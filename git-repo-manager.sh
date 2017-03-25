#!/bin/bash

GRM_SH=$(basename $0)
GRM_DIR="$HOME/.gpm"
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
function add() {
	if [ -d $1 ]; then
		projects=$(find $1 -name ".git")
		find $1 -name '*.git' -print0 |
			while IFS= read -r -d $'\0' line; do
				project_path=${line%/*}
				project_name=${project_path##*/}
				project_hash=$(echo -n $project_name | sha1sum | tr -d ' -')

				if [ -z "$(grep $project_hash $GRM_FILE)" ]; then
					echo "$project_hash $(readlink -e $project_path)" >> $GRM_FILE
					echo "[OK] $project_name : added to your project list"
				else
					echo "[KO] $project_name : already in the list"
				fi
			done
	fi
}

# === Display help ===
function help() {
	cat <<-EndHelp
		Usage: $GRM_SH [OPTION]... [DIRECTORY]...

		Options:
		    -a, --add
			    Add the given repository to the list.

		    -h, --help
			    Display help.
	EndHelp
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
				echo "$GRM_SH: missing directory parameter" >&2
				echo
				help
			fi
		;;
		-h | --help)
			help
		;;
		*)
		  echo "$GRM_SH: unrecognized option: $1" >&2
		  echo
		  help
		;;
	esac
fi
