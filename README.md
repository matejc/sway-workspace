# sway-workspace

Output aware Sway workspace switcher with window move feature

## Install

```
cargo install sway-workspace
```

## Usage

By default, Sway has commands that you can go previous/next workspace no matter the output, sway-workspace implements virtual grouping of workspaces per output.

For example, these two key bindings will switch to previous/next workspace on output and will not go to other output's workspace:

```
bindsym Mod1+Control+Left exec sway-workspace prev-on-output
bindsym Mod1+Control+Right exec sway-workspace next-on-output
```

When you want to switch between output workspaces these two keybindings will switch/focus from visible workspace on one output to another:

```
bindsym Mod1+Control+Up exec sway-workspace prev-output
bindsym Mod1+Control+Down exec sway-workspace next-output
```

Additionally, you could also assign the workspaces to outputs, example:

```
workspace "1" output HDMI-A-2
workspace "2" output HDMI-A-2
workspace "3" output HDMI-A-2
workspace "4" output HDMI-A-2
workspace "5" output HDMI-A-1
```

Command's cli options:

```
Usage: sway-workspace [OPTIONS] <ACTION>

Arguments:
  <ACTION>  Action [possible values: next, prev, next-output, prev-output, next-on-output, prev-on-output]

Options:
  -s, --sock <SOCK>  Sway socket path [default: /run/user/1000/sway-ipc.1000.3062.sock]
  -m, --move         Move to new workspace
  -n, --no-focus     Do not focus to new workspace
  -o, --stdout       Print workspace number to stdout
  -e, --skip-empty   Skip empty workspaces
  -h, --help         Print help
  -V, --version      Print version
```


## Example config

Put this in your sway config (`~/.config/sway/config`)

```
bindsym Mod1+Control+Up exec sway-workspace prev-output
bindsym Mod1+Control+Down exec sway-workspace next-output
bindsym Mod1+Control+Left exec sway-workspace prev-on-output
bindsym Mod1+Control+Right exec sway-workspace next-on-output

bindsym Mod1+Control+Shift+Up exec sway-workspace --move prev-output
bindsym Mod1+Control+Shift+Down exec sway-workspace --move next-output
bindsym Mod1+Control+Shift+Left exec sway-workspace --move prev-on-output
bindsym Mod1+Control+Shift+Right exec sway-workspace --move next-on-output

workspace 1 output DP-1
workspace 2 output DP-1
workspace 3 output DP-1
workspace 4 output DP-1
workspace 5 output HDMI-A-1
```
