# Jetcli
Jira Embedded Terminal Client

Jet is a strongly opinionated cli to automate jira and git task.

**Please don't rush into installing and using jet before you understand why and how to use it. Take time to read this 
section, this might save you some precious time.** 

## Shall I use Jet ? 

Jet is intended to work on a very specific context : 
- You are working with git and want to enforce naming conventions for branches and commits.
- You are working with Jira, and wants to get some repeating/boring tasks automated.

If your project doesn't match this description you probably don't want to use Jet (and won't be able to anyway). 

## What Jet can do for me ? 

1. When working on a project developers often try to enforce several good practices regarding commit messages, 
branching model Jira flow etc. Unfortunately people, come and go, and we humans do mistakes.

2. When working with Jira we do a lot of repeating tasks that matches an action we have already done in git. For
instance, I am working on a project where I have to assign a jira issue to the same reviewer every time I submit a PR
for this issue. 

Jet provide a higher level abstraction which allow you to perform tasks on both git objects and jira issues. 
It enforce a branching model and a commit message convention referencing your issues.
It would be pointless to respect such conventions if it was not to generate pretty change logs. So Jet allows you to 
generate change logs too.


## Configuration
TODO
### Global settings
TODO
### Private project settings
TODO
### Shared project settings
TODO

### TODOs

- [x] init command
- [x] template commit command
    - [x] generate commit prefix from config
    - [x] git2 implementation
    - [x] current jira issue
    - [x] optional scope
    - [x] default behavior for unmapped branch
- [ ] install commit hooks
    - [ ] validate commit message against template
- [ ] info command
    - [x] current branch name
    - [ ] dump config (ie. without credentials)
    - [ ] opt global
    - [ ] opt local
- [ ] status command
    - [ ] show current issue
    - [ ] issue state
    - [ ] issue description --details
    - [ ] show related commits
    - [ ] show git diff
- [ ] issues command
    - [x] open my issues
    - [ ] opt open
    - [ ] opt search
    - [ ] opt user
- [ ] checkout command
    - [x] fetch issue
    - [ ] if not assign issue to the current user 
    - [x] create and checkout branch from template
    - [x] checkout branch from template
    - [x] create and checkout branch from template
    - [ ] change issue state to ${WIP}
- [ ] submit command
    - [ ] create a new PR on the remote git platform
    - [ ] assign reviewers
    - [ ] assign jira issues default reporter
    - [ ] support bitbucket
    - [ ] support github
    - [ ] support gitlab
    - [ ] opt wip
    - [ ] unwip by default
- [ ] open command 
    - [ ] open jira issue in the browser
    - [ ] opt `--git` to open pull request page
- [ ] changelogs command
    - [ ] default from previous tag
    - [ ] opt --from 
    - [ ] opt --to  

### Command doc
TODO
