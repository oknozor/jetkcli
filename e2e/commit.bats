setup() {
    mkdir test_repo && cd test_repo || exit;
    touch file.md;
    git init;
    git add file.md;
    git commit -m "chore: init test repository";

    mkdir -p $HOME/.config/jet

    echo "
[servers.test]
url = \"$JIRA_SERVER_URL\"
username = \"$JIRA_USER\"
password = \"$JIRA_PASSWORD\"
" > $HOME/.config/jet/config.toml;

    jet init -p JETKCLI;
    sed -i '7s/.*/wip = "prendre en charge"/' .jet/config.shared.toml
}

teardown() {
    cd .. && rm -rf test_repo;
}

@test "feature commit works" {
    echo "add a nice txt file" > file.txt
    git add .

    run jet feat "this is a feature"
        [ $status -eq 0 ]
}

@test "chore commit works" {
    echo "add " > file.txt
    git add .

    run jet chore "some boring stuff"
        [ $status -eq 0 ]
}

@test "fix commit works" {
    echo "add " > file.txt
    git add .

    run jet fix "bug fixed"
        [ $status -eq 0 ]
}

@test "doc commit works" {
    echo "add " > file.txt
    git add .

    run jet doc "documentation for the win"
        [ $status -eq 0 ]
}

@test "style commit works" {
    echo "add " > file.txt
    git add .

    run jet style "this is a stylish"
        [ $status -eq 0 ]
}

