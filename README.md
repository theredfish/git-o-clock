# Git Repository Manager
> Please note that I'm currently working on this project but my time is very limited at the moment. If you're interested by the project let me know, I'm looking for contributions! You can take a task from the Status section, create a PR and start coding. Thank you!

GRM : A light git repository manager written in Rust for use in terminal.
Supports Linux, Max OSX, and Windows 10 (not tested from W7 but may works).

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

# Status
This project is work in progress, here the list of expected features :


- [x] add repositories
- [x] list repositories
- [x] change directory for a given repository name
- [x] remove repository
- [ ] update repositories (name and path)
- [x] automatic migrations
- [ ] categorize repositories
- [ ] list repositories with filters / pattern
- [ ] installation script
- [ ] auto-completion on repositories name
- [x] grm powershell script wrapper for built-in commands
- [x] grm bash script wrapper for built-in commands

Not sure :
- [ ] use [r2d2](https://github.com/sfackler/r2d2) : set of open db connections for repeated use
- [ ] implement [tui-rs](https://github.com/fdehau/tui-rs) or [termion](https://github.com/ticki/termion) : wait for a Windows implementation (see [termion issue "interest in windows port"](https://github.com/ticki/termion/issues/103))

# Getting started [WIP]
Waiting the final installation process, you can use the following steps to install GRM :

## Setup

- Clone the repository and build the project (with cargo or rustc, it's up to you) :
```
git clone git@github.com:theredfish/git-repo-manager.git
cd git-repo-manager
cargo build --release
```

- Setup your installation folder, move build files into and create your database file :
```
mkdir ~/.grm

# grm.sh for Linux or grm.ps1 for Windows
cp grm.sh ~/.grm
cp target/release/grm ~/.grm

# On Windows : powershell.exe New-Item ~/.grm/grm.sqlite3 -ItemType file
touch ~/.grm/grm.sqlite3
```

## Alias

### Windows

Under Windows you need to be prepared by creating a guild, finishing 100 quests and killing 42 bosses. More information [here](https://stackoverflow.com/questions/24914589/how-to-create-permanent-powershell-aliases#29806921).

- First you need to change your execution policy :
```
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
```

- Next, create your Windows Powershell profile if doesn't exist :
```
# Microsoft.PowerShell_profile.ps1 already exists
cd $env:USERPROFILE\Documents
md WindowsPowerShell
cd WindowsPowerShell
New-Item Microsoft.PowerShell_profile.ps1 -ItemType "file"
```

- Then edit `Microsoft.PowerShell_profile.ps1` to add your alias :
```
# You must split args one by one, grm can handle two arguments
function GrmCall {
    ~/.grm/grm.ps1 $args[0] $args[1]
}

Set-Alias grm GrmCall
```

- Finally source your profile :
```
. $PROFILE
```

Now you are a warrior ... or a paladin since you won +100 in intelligence \*wink wink nudge nudge\*

### Linux
With Linux, it's like a butterfly hunt. It's relaxing.

```
# Edit your ~/.bashrc or your ~/.bash_aliases with this line
alias grm='source ~/.grm/grm'

# Reload your bashrc
source ~/.bashrc
```
