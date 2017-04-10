#!/bin/bash

GRM_SH=$(basename $0)
GRM_DIR="$HOME/.grm"
GRM_FILE="$GRM_DIR/projects.txt"

if [ ! -d $GRM_DIR ]; then
    mkdir $GRM_DIR
fi

if [ ! -f $GRM_FILE ]; then
	touch $GRM_FILE
fi

# === Add a git project to the list ===
#
# Parameters: $1 the path to the repository.
# Precondition: None.
# Postcondition: New .git projects are written to the $GRM_FILE.
add() {
	if [ -d $1 ]; then
		projects=$(find $1 -name ".git")
		find $1 -name '*.git' |
			while IFS= read -r line; do
				project_path=${line%/*}
				project_name=${project_path##*/}
				project_hash=$(echo -n $project_name | sha1sum | tr -d ' -')

				if [ -z "$(grep $project_hash $GRM_FILE)" ]; then
					path=$(readlink -e "$project_path")
					echo "$project_hash $path" >> $GRM_FILE
					echo "[OK] $project_name : added to your project list"
				else
					echo "[KO] $project_name : already in the list"
				fi
			done
	else
		error "$1 isn't a directory."
	fi
}

# === List git projects ===
#
# Parameters: None.
# Precondition: $GRM_FILE exists.
# Postcondition: Returns a list of project names (empty or not).
list() {
	while IFS= read -r line; do
		echo ${line##*/}
	done <$GRM_FILE
}

# === Switch to the given project ===
#
# Parameters: $@ the name of the project
# Precondition: None
# Postcondition: Change the current directory to the project directory
# 				 Returns an error if the project doesn't exist.
switch() {
	project_hash=$(echo -n $@ | sha1sum | tr -d ' -')
	project_line=$(grep $project_hash $GRM_FILE)

	if [ -n "$project_line" ]; then
		cd "/${project_line#*/}"
		git status
	else
		error "$1 not found in the list."
	fi
}

# === Display help ===
help() {
	cat <<-EndHelp
		Usage: $GRM_SH [OPTION]... [DIRECTORY]...

		Options:
		    -a, --add
		        Add the given repository to the list.

		    -h, --help
		        Display help.

		    -l, --list
	            List repositories.

		    -s, --switch
		        Switch to a repository by it's name. Examples :
		        - grm -s directory_name
		        - grm -s "directory name"

	EndHelp
}

# === Display error ===
#
# Parameters: $1 the error message.
# Precondition: None.
# Postcondition: Display the error to the stderr.
error() {
	echo "$GRM_SH: $1" >&2
	echo
}

# === Option processing ===
if [ "$#" = 0 ]; then
	help
else
	case $1 in
		-a | --add)
			if [ "$2" ]; then
				add $2
			else
				error "missing directory parameter"
				help
			fi
		;;
		-l | --list)
			if [ -f $GRM_FILE ]; then
				list
			else
				error "$GRM_FILE doesn't exist."
			fi
		;;
		-h | --help)
			help
		;;
		-s | --switch)
			switch $2
		;;
		*)
		  error "unrecognized option: $1"
		  help
		;;
	esac
fi
