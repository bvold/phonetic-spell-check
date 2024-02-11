#[macro_use]
extern crate criterion;

use criterion::Criterion;

use phonetic_spell_rs::phonemize;
use phonetic_spell_rs::read_spellings_yaml;
use phonetic_spell_rs::Spellings;
use std::collections::BTreeMap;

fn criterion_benchmark(c: &mut Criterion) {
    // Prepare test data
    let spellings = read_spellings_yaml().unwrap();

    // Deserialize the YAML string back to a struct
    let deserialized_spellings: Spellings =
        serde_yaml::from_str(&spellings).expect("Failed to deserialize YAML");

    let original_word = "some_word";

    // Build BTreeMap for diphthong lookup
    let data_structure_btreemap: BTreeMap<String, bool> = deserialized_spellings
        .spellings
        .keys()
        .map(|spelling| (spelling.to_string(), true))
        .collect();

    c.bench_function("phonemize_btreemap", |b| {
        b.iter_batched(
            || {
                phonemize(
                    &deserialized_spellings,
                    original_word.clone(),
                    &data_structure_btreemap,
                )
            },
            1000,
        );
    });
}
