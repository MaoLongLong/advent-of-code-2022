use assert_cmd::Command;

#[test]
fn check_part1() {
    Command::cargo_bin("part1")
        .unwrap()
        .assert()
        .success()
        .stdout("8240\n");
}

#[test]
fn check_part2() {
    Command::cargo_bin("part2")
        .unwrap()
        .assert()
        .success()
        .stdout("2587\n");
}
