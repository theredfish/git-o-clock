#!/bin/bash
script_dir=`dirname $0`
GRM=${script_dir}/grm

if [ "$#" = 0 ]; then
  ${GRM} -h
else
    case $1 in
      goto)
        repo_path=`${GRM} goto $2`
        cd "$repo_path"
      ;;
      *)
        ${GRM} $1 $2
      ;;
    esac
fi
