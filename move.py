#!/usr/bin/env nix-shell
#!nix-shell -i python3 -p python3 python3Packages.xlib

import Xlib
import Xlib.display

dpy = Xlib.display.Display()
root = dpy.screen().root
geometry = root.get_geometry()
for win in root.query_tree().children:
    win.configure(x = 32, y = 32)
dpy.sync()
