use assert_cmd::Command;

fn check_stdout(bin_name: &'static str, expected: &'static str) {
    Command::cargo_bin(bin_name)
        .unwrap()
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn check_part1() {
    check_stdout("part1", "494\n");
}

#[test]
fn check_part2() {
    check_stdout("part2", "833\n");
}
