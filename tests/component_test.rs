use day_09::{calculate_all_fit_checksum, calculate_checksum};

#[test]
fn should_otpimise_checksum() {
    assert_eq!(calculate_checksum("tests/resources/puzzle.txt"), 1928);
}

#[test]
fn should_optimize_checksum_all_should_fit() {
    assert_eq!(calculate_all_fit_checksum("tests/resources/puzzle.txt"), 2858);
}