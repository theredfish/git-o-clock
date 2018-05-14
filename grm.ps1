$scriptPath = split-path -parent $MyInvocation.MyCommand.Definition
$grm="$scriptPath\grm.exe"

if ($args.Count -eq 0) {
    & $grm -h
} else {
    $argsLine = ""

    # String arguments in one line
    foreach ($arg in $args[1..$args.length]) {
        if ($argsLine) {
            $argsLine += " " + $arg;
        } else {
            $argsLine += $arg;
        }
    }

    # Avoid bad path parsing.
    # For example the string "\my\path\" is ill-parsed and
    # give the following result in args : \my\path"
    # due to the escape character "\"
    if ($argsLine -and $argsLine.Substring($argsLine.length-1) -eq '\') {
        $argsLine = $argsLine.Substring(0, $argsLine.length-1)
    }

    switch($args[0]) {
        "goto" {
            $path = & $grm $args[0] $argsLine

            if ($path) {
                cd $path
            }
        }
        default {
            & $grm $args[0] $argsLine
        }
    }
}

