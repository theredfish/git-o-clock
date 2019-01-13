script_dir=${BASH_SOURCE[0]%/*}
GRM=${script_dir}/grm

complete_git_repos() {
    repo_list=`${GRM} list`
    repo_concat_list=""

    for repo in ${repo_list}
    do
        repo_concat_list="${repo_concat_list} $repo"
    done

    echo ${repo_concat_list}
}

_grm() {
    local i cur prev opts cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${i}" in
            grm)
                cmd="grm"
                ;;
            add)
                cmd+="__add"
                ;;
            completions)
                cmd+="__completions"
                ;;
            goto)
                cmd+="__goto"
                ;;
            help)
                cmd+="__help"
                ;;
            list)
                cmd+="__list"
                ;;
            rm)
                cmd+="__rm"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        grm)
            opts=" -h -V  --help --version   add list goto rm completions help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in

                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;

        grm__add)
            opts=" -h -V  --help --version  <PATH> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in

                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        grm__completions)
            opts=" -h -V  --help --version  <SHELL> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in

                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        grm__goto)
            opts=" -h -V  --help --version "

            git_repo_list=$(complete_git_repos)

            # complete options
            if [[ ${cur} == -* ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            # complete git repo names
            elif [[ ${cur} != -* && ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${git_repo_list}" -- ${cur}) )
                return 0
            fi

            case "${prev}" in

                *)
                    COMPREPLY=()
                    ;;
            esac

            return 0
            ;;
        grm__help)
            opts=" -h -V  --help --version  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in

                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        grm__list)
            opts=" -h -V  --help --version  <PATTERN> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in

                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        grm__rm)
            opts=" -h -V  --help --version "

            git_repo_list=$(complete_git_repos)

            # complete options
            if [[ ${cur} == -* ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            # complete git repo names
            elif [[ ${cur} != -* && ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${git_repo_list}" -- ${cur}) )
                return 0
            fi

            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac

            return 0
            ;;
    esac
}

complete -F _grm -o bashdefault -o default grm
