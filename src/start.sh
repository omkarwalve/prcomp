#!/bin/zsh

SESSION="KILOWOG"
WINDOW=0

tmux new-session -d -s $SESSION
tmux rename-window -t $SESSION:$WINDOW 'kwe'
tmux send-keys -t $SESSION:$WINDOW 'cd kwe/ && cargo run --release' C-m
#tmux send-keys -t $SESSION:$WINDOW 'cd engine/ && cargo make start release' C-m
tmux split-window -h -t $SESSION:$WINDOW 'zsh'
tmux send-keys -t $SESSION:$WINDOW 'cd www/ && npm start' C-m
tmux attach-session -t $SESSION
