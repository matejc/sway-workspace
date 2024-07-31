use std::cmp::Ordering;
use std::env::var;

use clap::{Parser, ValueEnum};
use ksway::{Client, ipc_command};
use serde_json::{Value, from_str};


/// Simple command to switch workspaces with optional output awareness for Sway/i3
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Sway socket path
   #[arg(short, long, default_value_t = var("SWAYSOCK").unwrap())]
   sock: String,

   /// Action
   #[arg(value_enum)]
   action: Action,

   /// Move to new workspace
   #[arg(short, long = "move", default_value_t = false)]
   move_ws: bool,

   /// Do not focus to new workspace
   #[arg(short, long = "no-focus", default_value_t = false)]
   no_focus_ws: bool,

   /// Print workspace number to stdout
   #[arg(short = 'o', long = "stdout", default_value_t = false)]
   stdout_ws: bool,

   /// Skip empty workspaces
   #[arg(short = 'e', long = "skip-empty", default_value_t = false)]
   skip_empty: bool,
}

#[derive(ValueEnum, Clone)]
enum Action {
    Next,
    Prev,
    NextOutput,
    PrevOutput,
    NextOnOutput,
    PrevOnOutput,
}

fn get_workspaces(client: &mut Client) -> Vec<Value> {
    return from_str(&String::from_utf8_lossy(&client.ipc(ipc_command::get_workspaces()).unwrap())).unwrap();
}

fn focus_ws(client: &mut Client, num: i64) -> Result<Vec<u8>, ksway::Error> {
    return client.ipc(ipc_command::run(format!("workspace number {num}")));
}

fn move_ws(client: &mut Client, num: i64) -> Result<Vec<u8>, ksway::Error> {
    return client.ipc(ipc_command::run(format!("move workspace number {num}")));
}

fn find_by(workspaces: &Vec<Value>, current: i64, step: i64, skip_empty: bool) -> i64 {
    let existing: Vec<i64> = workspaces.into_iter().map(|w| w["num"].as_i64().unwrap()).collect();

    if skip_empty && step >= 0 {
        let other_nexts: Vec<i64> = existing.into_iter().filter(|e| *e >= (current + step)).collect();
        return match other_nexts.first() {
            Some(&value) => value,
            None => current,
        };
    } else if skip_empty {
        let other_prevs: Vec<i64> = existing.to_owned().into_iter().filter(|e| *e <= (current + step)).collect();
        return match other_prevs.last() {
            Some(&value) => value,
            None => current,
        };
    }

    let mut next: i64 = current + step;
    let first: i64 = 1;
    let last: i64 = existing.into_iter().max().unwrap();

    if current == last && step > 0 {
        next = last + step;
    } else if next < first {
        next = first;
    } else if next > last {
        next = last;
    }

    return next;
}

fn find_on_output(workspaces: &Vec<Value>, current: i64, step: i64, output: String, skip_empty: bool) -> i64 {
    let other_wss: Vec<&Value> = workspaces.into_iter().filter(|w| w["output"].to_string() != output).collect();
    let other_nums: Vec<i64> = other_wss.into_iter().map(|w| w["num"].as_i64().unwrap()).collect();
    let output_wss: Vec<&Value> = workspaces.into_iter().filter(|w| w["output"].to_string() == output).collect();
    let output_nums: Vec<i64> = output_wss.into_iter().map(|w| w["num"].as_i64().unwrap()).collect();

    if skip_empty && step >= 0 {
        let output_nexts: Vec<i64> = output_nums.into_iter().filter(|e| *e >= (current + step)).collect();
        return match output_nexts.first() {
            Some(&value) => value,
            None => current,
        };
    } else if skip_empty {
        let output_prevs: Vec<i64> = output_nums.to_owned().into_iter().filter(|e| *e <= (current + step)).collect();
        return match output_prevs.last() {
            Some(&value) => value,
            None => current,
        };
    }

    let mut next: i64 = current + step;

    let other_prevs: Vec<i64> = other_nums.to_owned().into_iter().filter(|e| *e < current).collect();
    let first: i64 = other_prevs.into_iter().max().unwrap() + 1;

    let other_nexts: Vec<i64> = other_nums.into_iter().filter(|e| *e > current).collect();
    let last: i64 = if other_nexts.len() == 0 {
        next
    } else {
        other_nexts.into_iter().min().unwrap() - 1
    };

    if next < first {
        next = first;
    } else if next > last {
        next = last;
    }

    return next;
}

fn find_output(workspaces: &Vec<Value>, current: i64, step: i64, output: String) -> i64 {
    let other_wss: Vec<&Value> = workspaces.into_iter().filter(|w| w["output"].to_string() != output && w["visible"] == true).collect();

    let other_prevs: Vec<&Value> = other_wss.to_owned().into_iter().filter(|w| w["num"].as_i64().unwrap() < current).collect();
    let other_nexts: Vec<&Value> = other_wss.into_iter().filter(|w| w["num"].as_i64().unwrap() > current).collect();

    match step.cmp(&0) {
        Ordering::Less => {
            return if other_prevs.len() == 0 { current } else { other_prevs.last().unwrap()["num"].as_i64().unwrap() }
        },
        Ordering::Greater => {
            return if other_nexts.len() == 0 { current } else { other_nexts.first().unwrap()["num"].as_i64().unwrap() }
        },
        Ordering::Equal => return current,
    }
}

fn main() {
    let args: Args = Args::parse();

    let mut client = Client::connect_to_path(args.sock.to_owned()).unwrap();

    let workspaces: &Vec<Value> = &get_workspaces(&mut client);

    let current_ws: &Value = workspaces.into_iter().filter(|w| w["focused"] == true).nth(0).unwrap();
    let current_ws_num: i64 = current_ws["num"].as_i64().unwrap();
    let current_output: String = current_ws["output"].to_string();

    let skip_empty: bool = args.skip_empty;

    let num: i64 = match args.action {
        Action::NextOnOutput => find_on_output(&workspaces, current_ws_num, 1, current_output, skip_empty),
        Action::PrevOnOutput => find_on_output(&workspaces, current_ws_num, -1, current_output, skip_empty),
        Action::NextOutput => find_output(&workspaces, current_ws_num, 1, current_output),
        Action::PrevOutput => find_output(&workspaces, current_ws_num, -1, current_output),
        Action::Next => find_by(&workspaces, current_ws_num, 1, skip_empty),
        Action::Prev => find_by(&workspaces, current_ws_num, -1, skip_empty),
    };

    if args.move_ws {
        move_ws(&mut client, num).unwrap();
    }

    if !args.no_focus_ws {
        focus_ws(&mut client, num).unwrap();
    }

    if args.stdout_ws {
        print!("{}", num);
    }
}
