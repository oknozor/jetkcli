### TODOs

- config
    - [x] templatize message ( default : "prefix(scope) message  \[issue\]")
- [x] init command
- [x] template commit command
    - [x] generate commit prefix from config
    - [x] git2 implementation
    - [x] current jira issue
    - [x] optional scope
    - [x] default behavior for unmapped branch

- [x] checkout command
    - [x] fetch issue
    - [x] if not assign issue to the current user 
    - [x] create and checkout branch from template
    - [x] checkout branch from template
    - [x] create and checkout branch from template
    - [x] warn on simple checkout if issue is not assigned to current user
    - [x] change issue state to ${WIP}
        - [x] save transition id to internal settings
    - [x] checkout without prefix for issue with exactly one matching branch
