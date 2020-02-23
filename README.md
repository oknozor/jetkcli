# Jetkcli : Jira Embedded Terminal Kung fu Command-line interface
---
⚠️ **Jetkcli is still a work in progress** ⚠️

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

Alice the lead dev asks Bob to implement a new command in the jet project. 
She tells Bob to take the corresponding issue. 
Bob start working on **JET-1** and assign it to himself transitioning the issue state to **"Work in progress"**.
He checks out a new local git branch with the name `feat/JET-1`.
When he is done editing the code Bob creates a commit with the following message : `feat(command): add jet 1 feature [JET-1]`.
Bob is done coding so he pushes is work to the remote repository and submit a PR.
He assign Alice to review it.
He change the issue state on jira to "Review" and assign it to Alice.

Jet's goal is to reduce this kind of repetition and automate some tasks by unifying related jira and git workflow when
 possible. 

Here are the planned feature to achieve this goal : 
- It enforce a branching model and a commit message convention referencing your issues
(see [conventional commit](https://www.conventionalcommits.org/en/v1.0.0/)) ✔️
- Handy commit generation command matching your convention model (all you need to type is the commit message) ✔️
- Automatic branch naming ✔️
- Transition issue state and assignment automatically when you start working on one ✔ ️
- Submit PRs on the git platform where your project is hosted. 
- Automatically submit your PRs on your git platform (bitbucket, github, gitlab) and assign reviewers to it. 
- Transition issue state and assignment automatically when you submit a PR. 
- Read issue description and comments from the command line ✔️
- Generate markdown changelogs


## Installation

To install Jetkcli you will need [rust installed on your system](https://www.rust-lang.org/tools/install).

you can then run `cargo install jetkcli`

### Global settings

Before using jet you will need to create the following config file :

|Platform | Value                                 | Location                                           |
| ------- | ------------------------------------- | --------------------------------                   |
| Linux   | `$XDG_CONFIG_HOME` or `$HOME/.config` | `/home/alice/.config/jet/config.toml`              |
| macOS   | `$HOME/Library/Preferences`           | `/Users/Alice/Library/Preferences/jet/config.toml` |
| Windows | `{FOLDERID_RoamingAppData}`           | `C:\Users\Alice\AppData\Roaming\jet\config.toml`   |

You can copy the example from [doc/example.config.toml](doc/example.config.toml)

```shell script
# "my_corporate_jira" is an arbitrary value, put whatever you want here
[servers.my_corporate_jira]

url = "corporate.jira.com"
username = "bob.smith@yopmail.com"
password = "hunter2"

[servers.local]

url = "http://localhost:8080"
username = "bob.smith"
password = "hunter2"
```

### Private project settings
TODO

### Shared project settings

This file is generated in `${project_dir}/.jetp/config.shared.toml`. 
The example above show the default generated file.

```toml
[jira]
# project short name in jira, generated from `jet init` -p arg
project_name = "MYP"
# infered for global settings with one server or from `jet init` -s arg
server_name = "local"
server_url = "http://localhost:8080"

# Default values, change this if you have custom issue types
[jira.workflow]
wip = "In Progress"
done = "Done"
todo = "To Do"

[git]
# Default conventional commit prefix
commit_types = ["fix", "feat", "chore", "style", "doc"]
# Default branch prefix
branch_types = ["fix", "feat", "chore", "style", "doc"]
# Default branch separator (ex: feat/JET-1)
branch_separator = "/"
```

## Commands

### Init

The init command does the following : 
- check that current dir is a git repository. 
- check that your Jira project exist in the remote jira server.  
- create `.jet` directory and default local configurations. 

```sh
jet init -p PRO -s my_jira_server
```
If you have only one Jira server configured in `$XDG_CONFIG_HOME/jet/config.toml` you can omit the `--server` flag.
 
### Checkout

Like `git checkout` the  `jet checkout` allow you to change the working branch and create new ones with the `-b` flag.
When checking out a new branch jet requires a branch prefix followed by an issue key

This does the following :
- Assign the issue to the user defined in `$XDG_CONFIG_HOME/jet/config.toml`. 
- Transition the issue to the state `wip` set in `.jet/config.shared.toml`
- Create and checkout a new local branch with the following name `{prefix}{separator}{issue_key}` see([shared settings](#shared-project-settings))

```shell script
jet checkout -b feat JET-1  # create and checkout a branch named "feat/JET-1"
``` 

When you checkout an existing branch :
- Warn if the current user is not assigned to the issue
- perform git checkout

```shell script
jet checkout feat JET-1
``` 

If there is exactly one branch for this issue you can use the short version : 
 ```shell script
 jet checkout JET-1
 ``` 

### Commit

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

### Submit 

TODO