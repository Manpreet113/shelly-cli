#!/usr/bin/env bash

# Launch Zen Browser in workspace 1
hyprctl dispatch workspace 1
zen-browser &

sleep 1.5

# Launch VS Code and emulator in workspace 2
hyprctl dispatch workspace 2
code ~/app &
env QT_QPA_PLATFORM=xcb ~/Android/Sdk/emulator/emulator -avd Medium_Phone -gpu host -feature -Vulkan -no-snapshot &

sleep 1

emu_win=$(hyprctl clients -j | jq -r '.[] | select(.title | test("Android Emulator")) | .address')
if [ -n "$emu_win" ]; then
  hyprctl dispatch focuswindow address:$emu_win
  hyprctl dispatch movewindow r
  hyprctl dispatch splitratio +0.453  
fi
