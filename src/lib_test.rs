use rstest::*;
use crate::get_possible_corrections;

#[rstest]
#[case("laff", "laugh")]
#[case("fone", "phone")]
#[case("tonite", "tonight")]
#[case("tho", "though")]
#[case("thru", "through")]
fn test_phonetic_spell_check(#[case] original_word: &str, #[case] expected_correction: &str) {
    let possible_corrections = get_possible_corrections(original_word.to_string());
    assert!(possible_corrections.contains(expected_correction));
}
