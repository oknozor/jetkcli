setup() {
    mkdir test_repo && cd test_repo || exit;
    touch file.md;
    git init;
    git add file.md;
    git commit -m "chore: init test repository";
    echo '
[servers.local]
url = "http://localhost:8080"
username = "paul.delafosse"
password = "caca1000"
' > $HOME/.config/jet/config.toml;
}

teardown() {
    cd .. && rm -rf test_repo;
}  

@test "shall fail init when no arg" {
    run jet init
        [ $status -eq 1 ]
        [ ! -e .jet ]
        [ ! -f .jet/config.toml ]
        [ ! -f .jet/config.internal.toml ]
}

@test "shall fail init when no global config" {
    rm $HOME/.config/jet/config.toml
    run jet init
        [ $status -eq 1 ]
        [ ! -e .jet ]
        [ ! -f .jet/config.toml ]
        [ ! -f .jet/config.internal.toml ]
}

@test "shall init jet config files" {
    run jet init -p DUM --server "local"
        [ $status -eq 0 ]
        [ -e .jet ]
        [ -f .jet/config.toml ]
        [ -f .jet/config.internal.toml ]
}


@test "shall init jet config with implicit server arg" {
    run jet init -p DUM
        [ $status -eq 0 ]
        [ -e .jet ]
        [ -f .jet/config.toml ]
        [ -f .jet/config.internal.toml ]
}

