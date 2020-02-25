@test "shall run jet --help" {
    run jet --help
    [ "$status" -eq 0 ]
}

@test "addition using dc" {
  result="$(echo 2 2+p | dc)"
  [ "$result" -eq 4 ]
}
