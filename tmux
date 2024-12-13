#!/bin/bash

years=()
cd_to=""
path="${0%/*}"
files=("$path"/*)

for file in "${files[@]}"; do
	[[ "${file##*/}" =~ ^[0-9]{4}$ ]] && years+=("${file##*/}")
done

# Get the largest year in the files
largest=0
for file in "${years[@]}"; do
	[[ "${file}" > "$largest" ]] && largest="${file}";
done
printf "Current Largest Year: ${largest}\n";
year_path="${path#*/}/${largest}/"

printf "$year_path\n"
cd "${year_path}"

function window1
{
	tmux send-keys "cd $PWD" ENTER
	tmux rename-window "Advent of Code"
}

# Are we already in a TMUX session
if [[ -n "$TMUX" ]]; then
	printf "In a TMUX session\n"; # Yes
	window1
else
	printf "Not in TMUX session\n"; # No
	tmux new -d # New TMUX session - detached
	
	window1	
	tmux new-window
	tmux rename-window "Testing"
	tmux send-keys "cargo run -- DAY"
	tmux select-window -t 0
	
	tmux a # Attach to the session
fi
