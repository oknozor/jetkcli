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

1. When working on a project developers often try to enforce good practices regarding commit messages, 
branching model Jira workflow etc. Unfortunately people, come and go, humans do mistakes and things get messy.

2. When working with Jira we do a lot of repeating tasks that conceptually matches actions made with git. 
Let us assume the following scenario :

- Alice the lead dev asks Bob to implement a new command in the jet project. She tells Bob to take the corresponding issue. 
- Bob start working on "JET-1" and assign it to himself transitioning the issue state to "Work in progress".
- He checks out a new local git branch with the name `feat/JET-1` 
- When he is done editing the code Bob creates a commit with the following message `feat(command): add jet 1 feature [JET-1]"
- When Bob is done working on an issue he pushes is work to the remote repository and submit a PR
- He assign Alice to review it.
- He change the issue state on jira to "Review" and assign it to Alice.

Jet's goal is to reduce repetition and automate some tasks by unifying related jira and git workflow when possible. 

Here are the planned feature to achieve this goal : 
- It enforce a branching model and a commit message convention referencing your issues 
(see [conventional commit](https://www.conventionalcommits.org/en/v1.0.0/))
- Handy commit generation command matching your convention model (all you need to type is the commit message).
- Automatic branch naming.
- Transition issue state and assignment automatically when you start working on one. 
- Submit PRs on the git platform where your project is hosted. 
- Automatically submit your PRs on your git platform (bitbucket, github, gitlab) and assign reviewers to it. 
- Transition issue state and assignment automatically when you submit a PR. 
- Read issue description and comments from the command line. 
- Generate markdown changelogs

## Example workflow

1. Init

```sh
jet init -p PRO
```

TODO
 
2. Checkout 

TODO 

```shell script
jet checkout -b feat JET-1
``` 

```shell script
jet checkout feat JET-1
``` 

3. Commit

f
Jet generate sub-commands matching your configured commit prefix (see [project settings shared](#shared-project-settings):

```shell script
jet feat "this is a cool feature"
```
Assuming you are working on the issue `JET-1` this will produce the following commit `feat: this is a cool feature [JET-1]`

Additionally you can provide a scope for your commit :

```shell script
jet fix "toml parse error" config
```

This will produce the following commit `fix(config): toml parse error [JET-1]`

4. Submit 

```shell script
jet sumbit
```

TODO



## Installation
TODO
## Configuration
TODO
### Global settings
TODO
### Private project settings
TODO
### Shared project settings
TODO


### TODOs

- [ ] Error message format 
- [ ] Colored message
- [ ] Internal state
    - [ ] Transitions id
- config
    - [ ] templatize message ( default : "prefix(scope) message  \[issue\]")
- [x] init command
- [x] template commit command
    - [x] generate commit prefix from config
    - [x] git2 implementation
    - [x] current jira issue
    - [x] optional scope
    - [x] default behavior for unmapped branch
    - [ ] fix empty repository scenario 
- [ ] install commit hooks
    - [ ] validate commit message against template
- [ ] info command
    - [ ] dump config (ie. without credentials)
    - [ ] opt global
    - [ ] opt local
- [ ] status command
    - [x] current branch name
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
- [ ] changelogs command (see [git journal](https://github.com/saschagrunert/git-journal))
    - [ ] default from previous tag
    - [ ] opt --from 
    - [ ] opt --to
