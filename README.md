# Jetcli
Jira Embedded Terminal Client

Jet goal is to provide an opinionated cli to automate jira and git task.

### Features

#### Configuration

- `$XDG_DESKTOP_HOME/jet.toml` 

##### Global config

```toml
[my_jira_server]
url = "my-jira-server.example.org"
username = "me@example.org"

[another_one]
url = "another-jira-server.example.org"
username = "jira-server@example.org"
```


The global jet configuration contains a list of jira server and their credentials. For obvious security reasons it is 
not stored in the working repository. The `[server_name]` value is used to access credentials and ask the cli to 
perform task on a specific server.

For instance if you want to initialize a project on `my_jira_server` you would use the following : 
```sh
jet init --project my_project --server my_jira_server
```

Or the short version :
```sh
jet init -p project my_project -s my_jira_server
```

However if your global config contains only one server entry your can skip the `--server` argument : 
```sh 
jet init my_project
```

We will get back to this later but remember that `jet init` will contact the provided jira server's rest api and ensure your
project exists. The init command also expect to be run inside an existing repository.

This would produce an error.
```sh 
jet init oops
```

##### Project settings 

Once your project is initialized your should see a `.jet` directory containing two config files. `config.toml` contains 
your personal settings and `config.shared.toml`. 

`.jet/config.toml`
```toml
# jira user name of your default reviewers
reviewers = ["john doe", "michel dupont", "alice white"]
# user to assign your completed issues
default_assignee =  "kevin the techlead"
```

`.jet/config.shared.toml`
```toml
# define your branches prefixes
server_url = "my-jira-server.example.org"
# branch prefix convention
branch_types = ["fix", "feat", "chore", "style", "doc"]
# commit prefix convention
commit_types = ["fix", "feat", "chore", "style", "doc"]
# branch prefix separator : `feat/some-feature`
branch_separator = "/"
```

All settings in the shared config can be edited and shared with your team.



#### Design 

Jet does less than you think. It  mostly uses git to automate some stuff in jira workflow, the core features are the following :

- [ ] starting a fresh issue with automated branch name. 
- [ ] transitioning jira issue state and assignment on picking and resolving issues.
- [ ] automated jira time tracking
- [ ] assigning people for review/validation when pushing fixes/features


#### Starting a project 

1. In a git repository : `jet init ${servername} -p project_name`

#### Command

- `jet init --project {project_name} -s {server_name}`
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
