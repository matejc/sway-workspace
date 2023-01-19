# sway-workspace

Output aware Sway/i3wm workspace switcher with window move feature

## Install

```
cargo install sway-workspace
```

## Usage

```
Usage: sway-workspace [OPTIONS] <ACTION>

Arguments:
  <ACTION>  Action [possible values: next, prev, next-output, prev-output, next-on-output, prev-on-output]

Options:
  -e, --exec <EXEC>  Sway/i3 msg executable name or path [default: swaymsg]
  -m, --move         Move to new workspace
  -n, --no-focus     Do not focus to new workspace
  -s, --stdout       Print workspace number to stdout
  -h, --help         Print help
  -V, --version      Print version
```


## Example config

Put this in you sway config (`~/.config/sway/config`)

```
bindsym Mod1+Control+Up exec sway-workspace prev-output
bindsym Mod1+Control+Down exec sway-workspace next-output
bindsym Mod1+Control+Left exec sway-workspace prev-on-output
bindsym Mod1+Control+Right exec sway-workspace next-on-output

bindsym Mod1+Control+Shift+Up exec sway-workspace --move prev-output
bindsym Mod1+Control+Shift+Down exec sway-workspace --move next-output
bindsym Mod1+Control+Shift+Left exec sway-workspace --move prev-on-output
bindsym Mod1+Control+Shift+Right exec sway-workspace --move next-on-output
```
