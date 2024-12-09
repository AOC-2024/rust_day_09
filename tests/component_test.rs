use day_09::optimize_checksum;

#[test]
fn should_otpimise_checksum() {
    assert_eq!(optimize_checksum("tests/resources/puzzle.txt"), 1928);
}