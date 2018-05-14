# git-repo-manager
GRM : A light git repository manager written in Rust for use in terminal.

Supports Linux, Max OSX, and Windows 10 (not tested from W7 but may works)

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

- [ ] implement [tui-rs](https://github.com/fdehau/tui-rs) or [termion](https://github.com/ticki/termion) : wait for a Windows implementation (see [termion issue "interest in windows port"](https://github.com/ticki/termion/issues/103))

# Getting started [WIP]
Waiting the final installation process, you can use the following steps to install GRM :

First clone the repository and build the project (with cargo or rustc, it's up to you) :
```
git clone git@github.com:theredfish/git-repo-manager.git
cargo build --release

# grm or grm.exe
./target/debug/grm
```

Then, because `cd` is a built-in command, we need to use a wrapper.

Under Windows, see DOSKEY to make an alias like Linux, or just execute the given script :
```
[TODO]
```