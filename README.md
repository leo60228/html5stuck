# html5stuck

Very WIP OSS mirror of Homestuck with proper HTML5 [S] pages

## `move.py`
Moves every window in the current X11 session to 32, 32. You'll probably want to change the shebang if you're not on NixOS.

## `swf2vid.sh`
Converts an swf to a video. Currently does not implement sound (it'll be played on the speakers). [TARDIS](https://github.com/DavidBuchanan314/TARDIS) can be used to speed up the playback, but I don't do this currently due to framerate issues, and my system can't run Flash at more than 2-3x speed anyway. Flash is entirely single-threaded, so parallelizing is a much clearer path to a speed improvement.

Usage:
```
./swf2vid.sh wake.swf wake.mp4 170 # input, output, length
```
