@test "shall run jet --help" {
    run jet --help
    [ "$status" -eq 0 ]
}
