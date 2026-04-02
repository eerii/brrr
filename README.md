# brrr ✨

> **Could it have been a shell script?**
> Yes c:

CLI tool to manage my checkouts of browsers (and i guess other large open source projects).

Feature tracker:

- [x] Nice config
- [x] Getting the repos
- [x] Creating containers ([distrobox](https://distrobox.it/) or [webkit-container-sdk](https://github.com/Igalia/webkit-container-sdk))
- [x] Boostrapping and installing dependencies in the containers
- [x] Building, testing and other commands
- [ ] Managing git worktrees for parallel work
- [ ] Same interface to different project-specific tools

Supported browser repos:

- [Servo](https://github.com/servo/servo)
- [Firefox](https://github.com/mozilla-firefox/firefox)
- [WebKit](https://github.com/WebKit/WebKit)
- [Chromium](https://source.chromium.org/)

But everything is very work in progress.

> Browsers go _brrr_

```
Usage: brrr <COMMAND>

Commands:
  bootstrap  
  build      
  clean      
  container  
  git        
  run        
  status     
  test       
  worktree   
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

_I have to complete this :,)_
