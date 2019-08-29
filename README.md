# Git Repository Manager
> Plase note that the API will strongly evolve until the stable version in `1.0.0`. Do not use if you're looking for a stable software.

GRM : A light git repository manager written in Rust for use in terminal.
Supports Linux, Max OSX, and Windows 10 (not tested from W7 but may works).

![Demo with bash](doc/demo_bash.gif)

```
USAGE:
    grm(.exe) [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add     Search and add your git repositories for a given path; default path is the current directory
    goto    Go to the repository directory
    help    Prints this message or the help of the given subcommand(s)
    list    List your saved repositories for the given pattern
    rm      Remove a git repository from the list
```

# Getting started
Waiting the final installation process, you can use the following steps to install GRM :

## Setup
### Download the repository and build the binary
Clone the repository and build the project (with cargo or rustc, it's up to you) :
```
git clone git@github.com:theredfish/git-repo-manager.git
cd git-repo-manager
cargo build --release
```

### Setup the installation folder
#### Linux
```
mkdir ~/.grm
cd git-repo-manager

# Copy the bash wrapper and the completion file
cp wrapper_scripts/grm.sh ~/.grm
cp completion_scripts/grm.bash-completion.sh ~/.grm

# Copy the binary
cp target/release/grm ~/.grm

# create the database file (if doesn't exist)
touch ~/.grm/grm.sqlite3
```

#### Windows (powershell)
```
mkdir ~/.grm

Copy the powershell wrapper
cp grm.ps1 ~/.grm

# Copy the binary
cp target/release/grm.exe ~/.grm

# create the database file
New-Item ~/.grm/grm.sqlite3 -ItemType file
```

## Setup grm alias
### Linux
```
# Edit your ~/.bashrc or your ~/.bash_aliases with these lines
source ~/.grm/grm.bash-completion.sh
alias grm='source ~/.grm/grm.sh'

# Reload your bashrc
source ~/.bashrc
```

### Windows

More information [here](https://stackoverflow.com/questions/24914589/how-to-create-permanent-powershell-aliases#29806921).

First you need to change your execution policy :
```
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
```

Next, create your Windows Powershell profile if doesn't exist :
```
# Microsoft.PowerShell_profile.ps1 already exists
cd $env:USERPROFILE\Documents
md WindowsPowerShell
cd WindowsPowerShell
New-Item Microsoft.PowerShell_profile.ps1 -ItemType "file"
```

Then edit `Microsoft.PowerShell_profile.ps1` to add your alias :
```
# You must split args one by one, grm can handle two arguments
function GrmCall {
    ~/.grm/grm.ps1 $args[0] $args[1]
}

Set-Alias grm GrmCall
```

Finally source your profile :
```
. $PROFILE
```

# Development
## Prerequisites
- Rust 1.37

### SQLite3
Install the following dependencies (it may change depending of your distribution): 
```bash
# OS dependencies
sudo apt-get install libsqlite3-dev

# Diesel CLI with sqlite only
cargo install diesel_cli --no-default-features --features sqlite
```

Or avoid installing OS dependencies by [playing with the bundled libsqlite3-sys in diesel_cli](https://github.com/theredfish/diesel/commit/3f7c365bb4df614574596fd27f716b000101e063).

## Manage wrapper scripts
TODO


# Features
This project is work in progress, here the list of expected features :

## must-have features
- [x] add repositories
- [x] list repositories
- [x] change directory for a given repository name
- [x] remove repository
- [ ] clean dead repositories
- [ ] update repositories (name and path)
- [ ] list repositories with filters / pattern
- [ ] auto-completion on repositories

## nice-to-have features
- [ ] parallelized search
- [ ] categorize repositories
- [ ] installation script
## migrations and updates
- [x] automatic migrations
- [ ] replace the automatic migrations with a prompt to ask if the user want to make the update
- [ ] always make a save of `grm.sqlite3` before a migration
- [ ] if a migration fails, restore the previous version

## system wrappers
- [x] grm powershell script wrapper for built-in commands
- [x] grm bash script wrapper for built-in commands
- [x] bash auto-completion
- [ ] powershell auto-completion
- [ ] zsh auto-completion

## not so sure :
- [ ] use [r2d2](https://github.com/sfackler/r2d2) : set of open db connections for repeated use
- [ ] implement [tui-rs](https://github.com/fdehau/tui-rs) or [termion](https://github.com/ticki/termion) : wait for a Windows implementation (see [termion issue "interest in windows port"](https://github.com/ticki/termion/issues/103))




