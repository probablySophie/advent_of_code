
day_text=("one" "two" "three" "four" "five" "six" "seven" "eight" "nine" "ten" "eleven" "twelve" "thirteen" "fourteen" "fifteen" "sixteen" "seventeen" "eighteen" "nineteen" "twenty" "twenty_one" "twenty_two" "twenty_three" "twenty_four" "twenty_five")


printf "Please enter the new day's num\n[1-25] "
read day_num

rs_file="src/days/${day_text[$day_num-1]}.rs"
txt_file="input/$day_num.txt"

printf "\n\t$rs_file\n"
printf "\t$txt_file\n\n"

if [[ -e "$rs_file" ]]; then
	printf "That day already exists!\n"
	return
else
	printf "That day does not exist!\nMaking it now!\n"
fi

cp src/days/empty.rs $rs_file
printf "Made $rs_file\n"
sed -i "s/~DAY_NUM~/${day_num-1}/g" $rs_file
printf "Replaced '~DAY_NUM~' with ${day_num-1} in $rs_file\n"
touch $txt_file
printf "Made $txt_file\n"

# Does the file end with a new-line?
if [[ "$(tail -c 1 src/days/mod.rs)" == "" ]]; then
	printf "pub mod ${day_text[$day_num-1]};" >> src/days/mod.rs
else
	printf "\npub mod ${day_text[$day_num-1]};" >> src/days/mod.rs
fi
printf "Updated src/days/mod.rs\n";

printf "Open created files? [y/n]\n(n) > "
read answer
if [[ "$answer" == y* ]]; then
	$EDITOR "$rs_file" "$txt_file" "src/main.rs" "README.md"
fi
