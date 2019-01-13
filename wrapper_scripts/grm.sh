#!/bin/bash
script_dir=${BASH_SOURCE[0]%/*}
GRM=${script_dir}/grm

if [ "$#" = 0 ]; then
  ${GRM} -h
else
  # handle spaces
  arg_value="${@:2:$#}"

  case $1 in
    goto)
      repo_path=`${GRM} goto "$arg_value"`

      if [ -n "$repo_path" ]; then
        cd "$repo_path"
      fi
    ;;
    *)
      ${GRM} $1 "$arg_value"
    ;;
  esac
fi
