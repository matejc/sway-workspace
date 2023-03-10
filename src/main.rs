use std::cmp::Ordering;
use std::env::var;

use clap::{Parser, ValueEnum};
use ksway::{Client, ipc_command};
use serde_json::{Value, from_str};


/// Simple command to switch workspaces with optional output awareness for Sway/i3
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Sway/i3 socket path
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

fn find_by(workspaces: &Vec<Value>, current: i64, step: i64) -> i64 {
    let existing: Vec<i64> = workspaces.into_iter().map(|w| w["num"].as_i64().unwrap()).collect();

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

fn find_on_output(workspaces: &Vec<Value>, current: i64, step: i64, output: String) -> i64 {
    let other_wss: Vec<&Value> = workspaces.into_iter().filter(|w| w["output"].to_string() != output).collect();
    let other_nums: Vec<i64> = other_wss.into_iter().map(|w| w["num"].as_i64().unwrap()).collect();

    let other_nums_prev: Vec<i64> = [
        Vec::from([0]),
        other_nums.to_owned().into_iter().filter(|n| n < &current).collect()
    ].concat();
    let other_nums_next: Vec<i64> = other_nums.into_iter().filter(|n| n > &current).collect();

    let mut next: i64 = current + step;

    let first: i64 = other_nums_prev.into_iter().max().unwrap() + 1;

    let last: i64 = if other_nums_next.len() == 0 {
        next
    } else {
        other_nums_next.into_iter().min().unwrap() - 1
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

    let num: i64 = match args.action {
        Action::NextOnOutput => find_on_output(&workspaces, current_ws_num, 1, current_output),
        Action::PrevOnOutput => find_on_output(&workspaces, current_ws_num, -1, current_output),
        Action::NextOutput => find_output(&workspaces, current_ws_num, 1, current_output),
        Action::PrevOutput => find_output(&workspaces, current_ws_num, -1, current_output),
        Action::Next => find_by(&workspaces, current_ws_num, 1),
        Action::Prev => find_by(&workspaces, current_ws_num, -1),
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
