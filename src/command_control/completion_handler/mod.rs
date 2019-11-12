pub mod process;

    pub use process::CompletionProcess;
    use super::cmd_model;

    pub struct CompletionScript {

    }

    impl CompletionScript {
        pub fn bash() {
            println!("{}",r#"
    _complete() {
    local i cur prev opts cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${i}" in
            complete)
                cmd="complete"
                ;;
            
            bash)
                cmd+="__bash"
                ;;
            completions)
                cmd+="__completions"
                ;;
            elvish)
                cmd+="__elvish"
                ;;
            fish)
                cmd+="__fish"
                ;;
            powershell)
                cmd+="__powershell"
                ;;
            zsh)
                cmd+="__zsh"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        complete)
            opts=" -v -h  --verbose --help   completions"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        
        complete__completions)
            opts=" -h -V  --help --version   bash fish zsh powershell elvish"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        complete__completions__bash)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        complete__completions__elvish)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        complete__completions__fish)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        complete__completions__powershell)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        complete__completions__zsh)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

complete -F _complete -o bashdefault -o default complete

    "#);
        }

        pub fn fish() {
            println!("{}",r#"
    complete -c complete -n "__fish_use_subcommand" -s v -l verbose -d 'Enable verbose logging.'
complete -c complete -n "__fish_use_subcommand" -s h -l help -d 'Prints help information'
complete -c complete -n "__fish_use_subcommand" -f -a "completions" -d 'Completion scripts for various shells.'
complete -c complete -n "__fish_seen_subcommand_from completions" -s h -l help -d 'Prints help information'
complete -c complete -n "__fish_seen_subcommand_from completions" -s V -l version -d 'Prints version information'
complete -c complete -n "__fish_seen_subcommand_from completions" -f -a "bash" -d 'Bash completion script.'
complete -c complete -n "__fish_seen_subcommand_from completions" -f -a "fish" -d 'Fish completion script.'
complete -c complete -n "__fish_seen_subcommand_from completions" -f -a "zsh" -d 'Zsh completion script.'
complete -c complete -n "__fish_seen_subcommand_from completions" -f -a "powershell" -d 'PowerShell completion script.'
complete -c complete -n "__fish_seen_subcommand_from completions" -f -a "elvish" -d 'Elvish completion script.'
complete -c complete -n "__fish_seen_subcommand_from bash" -s h -l help -d 'Prints help information'
complete -c complete -n "__fish_seen_subcommand_from bash" -s V -l version -d 'Prints version information'
complete -c complete -n "__fish_seen_subcommand_from fish" -s h -l help -d 'Prints help information'
complete -c complete -n "__fish_seen_subcommand_from fish" -s V -l version -d 'Prints version information'
complete -c complete -n "__fish_seen_subcommand_from zsh" -s h -l help -d 'Prints help information'
complete -c complete -n "__fish_seen_subcommand_from zsh" -s V -l version -d 'Prints version information'
complete -c complete -n "__fish_seen_subcommand_from powershell" -s h -l help -d 'Prints help information'
complete -c complete -n "__fish_seen_subcommand_from powershell" -s V -l version -d 'Prints version information'
complete -c complete -n "__fish_seen_subcommand_from elvish" -s h -l help -d 'Prints help information'
complete -c complete -n "__fish_seen_subcommand_from elvish" -s V -l version -d 'Prints version information'

    "#);
        }

        pub fn zsh() {
            println!("{}",r#"
    #compdef complete

autoload -U is-at-least

_complete() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" \
'-v[Enable verbose logging.]' \
'--verbose[Enable verbose logging.]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
":: :_complete_commands" \
"*::: :->complete" \
&& ret=0
    case $state in
    (complete)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:complete-command-$line[1]:"
        case $line[1] in
            (completions)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_complete__completions_commands" \
"*::: :->completions" \
&& ret=0
case $state in
    (completions)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:complete-completions-command-$line[1]:"
        case $line[1] in
            (bash)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- Bash completion script.:_files' \
&& ret=0
;;
(fish)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- Fish completion script.:_files' \
&& ret=0
;;
(zsh)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- Zsh completion script.:_files' \
&& ret=0
;;
(powershell)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- PowerShell completion script.:_files' \
&& ret=0
;;
(elvish)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- Elvish completion script.:_files' \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
}

(( $+functions[_complete_commands] )) ||
_complete_commands() {
    local commands; commands=(
        "completions:Completion scripts for various shells." \
    )
    _describe -t commands 'complete commands' commands "$@"
}
(( $+functions[_complete__completions__bash_commands] )) ||
_complete__completions__bash_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'complete completions bash commands' commands "$@"
}
(( $+functions[_complete__completions_commands] )) ||
_complete__completions_commands() {
    local commands; commands=(
        "bash:Bash completion script." \
"fish:Fish completion script." \
"zsh:Zsh completion script." \
"powershell:PowerShell completion script." \
"elvish:Elvish completion script." \
    )
    _describe -t commands 'complete completions commands' commands "$@"
}
(( $+functions[_complete__completions__elvish_commands] )) ||
_complete__completions__elvish_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'complete completions elvish commands' commands "$@"
}
(( $+functions[_complete__completions__fish_commands] )) ||
_complete__completions__fish_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'complete completions fish commands' commands "$@"
}
(( $+functions[_complete__completions__powershell_commands] )) ||
_complete__completions__powershell_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'complete completions powershell commands' commands "$@"
}
(( $+functions[_complete__completions__zsh_commands] )) ||
_complete__completions__zsh_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'complete completions zsh commands' commands "$@"
}

_complete "$@"
    "#);
        }

        pub fn powershell() {
            println!("{}",r#"
    
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'complete' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'complete'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-')) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'complete' {
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Enable verbose logging.')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Enable verbose logging.')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('completions', 'completions', [CompletionResultType]::ParameterValue, 'Completion scripts for various shells.')
            break
        }
        'complete;completions' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('bash', 'bash', [CompletionResultType]::ParameterValue, 'Bash completion script.')
            [CompletionResult]::new('fish', 'fish', [CompletionResultType]::ParameterValue, 'Fish completion script.')
            [CompletionResult]::new('zsh', 'zsh', [CompletionResultType]::ParameterValue, 'Zsh completion script.')
            [CompletionResult]::new('powershell', 'powershell', [CompletionResultType]::ParameterValue, 'PowerShell completion script.')
            [CompletionResult]::new('elvish', 'elvish', [CompletionResultType]::ParameterValue, 'Elvish completion script.')
            break
        }
        'complete;completions;bash' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'complete;completions;fish' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'complete;completions;zsh' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'complete;completions;powershell' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'complete;completions;elvish' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}

    "#);
        }

        pub fn elvish() {
            println!("{}",r#"
            
edit:completion:arg-completer[complete] = [@words]{
    fn spaces [n]{
        repeat $n ' ' | joins ''
    }
    fn cand [text desc]{
        edit:complex-candidate $text &display-suffix=' '(spaces (- 14 (wcswidth $text)))$desc
    }
    command = 'complete'
    for word $words[1:-1] {
        if (has-prefix $word '-') {
            break
        }
        command = $command';'$word
    }
    completions = [
        &'complete'= {
            cand -v 'Enable verbose logging.'
            cand --verbose 'Enable verbose logging.'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand completions 'Completion scripts for various shells.'
        }
        &'complete;completions'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
            cand bash 'Bash completion script.'
            cand fish 'Fish completion script.'
            cand zsh 'Zsh completion script.'
            cand powershell 'PowerShell completion script.'
            cand elvish 'Elvish completion script.'
        }
        &'complete;completions;bash'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'complete;completions;fish'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'complete;completions;zsh'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'complete;completions;powershell'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'complete;completions;elvish'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
    ]
    $completions[$command]
}

    "#);
        }
    }
    