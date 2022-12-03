use assert_cmd::Command;

#[test]
fn check_part1() {
    Command::cargo_bin("part1")
        .unwrap()
        .assert()
        .success()
        .stdout("12458\n");
}

#[test]
fn check_part2() {
    Command::cargo_bin("part2")
        .unwrap()
        .assert()
        .success()
        .stdout("12683\n");
}
