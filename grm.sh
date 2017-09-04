#!/bin/bash

#TODO real implementation
export DATABASE_URL="/d/workspace/git-repo-manager/grm.sqlite3"

# TODO real implementation
goto() {
    path=$(/d/workspace/git-repo-manager/target/debug/show_repositories.exe)
    echo "Goto $path"
    cd $path
}

# === Display help ===
help() {
	cat <<-EndHelp
		Usage: $GRM_SH [OPTION]... [DIRECTORY]...

		Options:
		    goto
		        Go to the specified project.

		    help
		        Display help.
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
		goto)
			if [ "$2" ]; then
				goto $2
			else
				error "missing project's name parameter"
				help
			fi
		;;
		help)
			help
		;;
		*)
		  error "unrecognized option: $1"
		  help
		;;
	esac
fi