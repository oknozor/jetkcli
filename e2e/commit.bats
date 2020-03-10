setup() {
    mkdir test_repo && cd test_repo || exit;
    touch file.md;
    git init;
    git add file.md;
    git commit -m "chore: init test repository";
    echo '
[servers.test]
url = "$JIRA_SERVER_URL"
username = "$JIRA_USER"
password = "$JIRA_PASSWORD"
' > $HOME/.config/jet/config.toml;

    jet init -p JETKCLI;
    sed -i '7s/.*/wip = "prendre en charge"/' .jet/config.shared.toml
}

teardown() {
    cd .. && rm -rf test_repo;
}

@test "feature commit works" {
    echo "add some feature" > file.txt
    git add .
    run jet feat "this is a feature"
        [ $status -eq 0 ]
}

