// Patterns crashing the code found via fuzzing (cargo-fuzz)

#[test]
fn test_crash() {
    assert_eq!(
        french_numbers::french_number(&std::i128::MIN),
        "-170141183460469231731687303715884105728"
    );
    assert_eq!(french_numbers::french_number(&std::i8::MIN), "-128");
}
