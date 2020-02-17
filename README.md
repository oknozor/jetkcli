# Jetcli
Jira Embedded Terminal Client

Jet goal is to provide an opinionated cli to automate jira and git task.

### Features

#### Configuration

- `$XDG_DESKTOP_HOME/jet.toml` 

```toml
[my_jira_server]

url = "my-jira-server.example"
username = "me@example.org"
password = "MySuperSecretPass"
# predefined branches suffix
branching_model = ["fix", "feat", "chore", "style", "doc", "test"]
# {branch_suffix}/{jira_issue}
branch_separator = "/"
# Automatically assign those people to review your PRs
reviewers = ["john doe", "michel dupont", "alice white"]
# Automatically assign jira issue when you submit a PR
assignee =  ["kevin the techlead"]
```
#### Design 

Jet does less than you think. It  mostly uses git to automate some stuff in jira workflow, the core features are the following :

- [ ] starting a fresh issue with automated branch name. 
- [ ] transitioning jira issue state and assignment on picking and resolving issues.
- [ ] automated jira time tracking
- [ ] assigning people for review/validation when pushing fixes/features


#### Starting a project 

1. In a git repository : `jet init ${servername} -p project_name`

#### Command

- `jet init {server_name} --project {project_name}`
- `jet issues`
- `jet issues --all`
- `jet issues --search {text}`
- `jet issues --name {issue_name}`
- `jet checkout {issue_name}`
- `jet pr`
- `jet open`
- `jet open --name {issue_name}`
- `jet show users`
- `jet status`
- `jet config`
