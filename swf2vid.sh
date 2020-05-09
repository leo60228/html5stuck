#!/usr/bin/env bash
trap 'kill $(jobs -p)' EXIT
set -e
shopt -s lastpipe

export DISPLAY=:1
SIZE=1024x768
BAR=61

Xvfb "$DISPLAY" -screen 0 "${SIZE}x24" &
export DISPLAY="$DISPLAY.0"
sleep 1
flashplayer "$1" &
sleep 0.1
./move.py
sleep 0.5

ffmpeg -video_size "$SIZE" -framerate 30 -draw_mouse 0 -f x11grab -i "${DISPLAY}" -t 0.25 -vf cropdetect=1:1:0 -f null - 2>&1 | grep -m1 'crop=' | tee /dev/stderr | IFS=': ' read _ _ _ _ x1 _ x2 _ y1 _ y2 _ >&/dev/null
tput init # ????????
w="$(($x2-$x1))"
h="$(($y2-$y1))"
CROP="crop=$w:$h:$x1:$y1"
ffmpeg -y -video_size "$SIZE" -draw_mouse 0 -f x11grab -i "${DISPLAY}" -filter:v "$CROP, crop=in_w:in_h-${BAR}:0:${BAR}" -t "$3" "$2"
