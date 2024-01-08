use rstest::*;
use crate::get_possible_corrections;

#[rstest]
#[case("laff", "laugh")]
#[case("fone", "phone")]
#[case("tonite", "tonight")]
#[case("tho", "though")]
#[case("thru", "through")]
#[case("ghoti", "fish")]
#[case("kumpoot", "compute")]
#[case("kumpliant", "compliant")]
#[case("kompost", "compost")]
#[case("compose", "compose")]
#[case("ubowt", "about")]
#[case("kryteria", "criteria")]
#[case("krismas", "christmas")] // FIXME - should be able to handle capital letters as well
#[case("won", "one")]
#[case("lite", "light")]
#[case("akwire", "acquire")]
#[case("bizar", "bizarre")]
#[case("semetary", "cemetery")]
#[case("definitly", "definitely")]
#[case("ekstasi", "ecstasy")]
#[case("forin", "foreign")]
#[case("gage", "gauge")]
#[case("haras", "harass")]
#[case("indipendunt", "independent")]
#[case("jewlry", "jewelry")]
#[case("yon", "yawn")]
#[case("epifany", "epiphany")]

fn test_phonetic_spell_check(#[case] original_word: &str, #[case] expected_correction: &str) {
    let possible_corrections = get_possible_corrections(original_word.to_string());
    assert!(possible_corrections.contains(expected_correction));
}
