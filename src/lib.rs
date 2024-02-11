use crate::sound::Sound;
use flexi_logger::{colored_opt_format, Duplicate, Logger, LoggerHandle};
use lazy_static::lazy_static;
use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use strum::EnumMessage;

mod lib_test;
mod sound;

#[derive(Debug, Serialize, Deserialize)]
pub struct Spelling {
    sounds: Vec<Sound>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Spellings {
    pub spellings: BTreeMap<String, Spelling>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Graphemes {
    graphemes: HashMap<String, Grapheme>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Grapheme {
    spellings: Vec<String>,
}

fn read_yaml_file(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn gen_ipa_word_permutations(btreemap: &BTreeMap<usize, Vec<Sound>>) -> Vec<String> {
    let mut permutations = Vec::new();

    // Helper function to recursively build permutations
    fn build_ipa_permutations(
        prefix: &str,
        remaining_keys: &[&usize],
        values_by_key: &BTreeMap<usize, Vec<Sound>>,
        permutations: &mut Vec<String>,
    ) {
        if remaining_keys.is_empty() {
            permutations.push(prefix.to_string());
            return;
        }

        let current_key = remaining_keys[0];
        for value in values_by_key[current_key].iter() {
            let new_prefix = format!("{}{}", prefix, value.get_message().unwrap());
            build_ipa_permutations(
                &new_prefix,
                &remaining_keys[1..],
                values_by_key,
                permutations,
            );
        }
    }

    build_ipa_permutations(
        "",
        btreemap.keys().collect::<Vec<_>>().as_slice(),
        btreemap,
        &mut permutations,
    );

    permutations
}

lazy_static! {
    static ref IPA_TO_DICT_WORD_MAP: HashMap<String, Vec<String>> = {
        let mut cmu_dict: HashMap<String,Vec<String>> = HashMap::new();
        let file = File::open("en_US.txt").expect("Error opening file");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            // Split the line into words
            let words: Vec<&str> = line.split_whitespace().collect();

            // Get the word and its IPA representation(s)
            let word = words[0].to_string();
            let ipa = words[1..].to_vec();

            // Add the word and its IPA representation(s) to the hashmap
            for i in 0..ipa.len() {
let pronunciation = ipa[i].replace(['/', '\'', 'ˈ','ˌ',','], "");
                cmu_dict.entry(pronunciation.to_string()).or_insert(vec![]).push(word.clone());
            }
                }

        cmu_dict
    };
}

lazy_static! {
    static ref USR_SHARE_DICT_WORDS_HASHSET: HashSet<String> = {
        let file = File::open("words").unwrap();
        let reader = BufReader::new(file);

        let mut words = HashSet::new();
        for line in reader.lines() {
            let word = line.unwrap().to_lowercase();
            words.insert(word);
        }

        words
    };
}

trait DiphthongLookup {
    fn contains(&self, diphthong: &str) -> bool;
}

pub fn phonemize<T>(
    deserialized_spellings: &Spellings,
    original_word: &str,
    data_structure: &T,
) -> Vec<String>
where
    T: DiphthongLookup, // Define a trait for diphthong lookup
{
    // // Create a BTreeMap map for efficient diphthong lookup
    // let mut key_map: BTreeMap<String, bool> = BTreeMap::new();
    // for spelling in deserialized_spellings.spellings.keys() {
    //     key_map.insert(spelling.to_string(), true);
    // }

    let mut diphthong_array = Vec::new();
    let mut offset = 0;
    let mut size = 0;

    let size_of_original_word = original_word.len();

    while offset + size < size_of_original_word {
        // Find the longest matching diphthong
        while offset + size + 1 <= size_of_original_word
            && data_structure.contains(&original_word[offset..(offset + size + 1)])
        {
            size += 1;
        }

        diphthong_array.push(original_word[offset..(offset + size)].to_string());
        offset += size;
        size = 0;
    }

    diphthong_array
}

impl DiphthongLookup for BTreeMap<String, bool> {
    fn contains(&self, diphthong: &str) -> bool {
        self.contains_key(diphthong)
    }
}

// A recursive function that generates permutations of phoneme combinations after initial phoneme parse
fn gen_phoneme_permutations(
    original: &Vec<String>,
    current: Vec<String>,
    index: usize,
    result: &mut HashSet<Vec<String>>,
) {
    if index == original.len() {
        result.insert(current.clone());
        return;
    }

    let mut chars = Vec::new();
    for c in original[index].chars() {
        chars.push(c.to_string());
    }

    let mut next_current = current.clone();
    next_current.extend_from_slice(&chars);
    gen_phoneme_permutations(original, next_current, index + 1, result);

    let mut next_current = current.clone();
    next_current.push(original[index].clone());
    gen_phoneme_permutations(original, next_current, index + 1, result);
}

pub fn read_spellings_yaml() -> Option<String> {
    // Specify the path to the YAML file
    let file_path = "spellings.yaml";

    // Read the YAML file content into a string
    let yaml_string = match read_yaml_file(file_path) {
        Ok(content) => content,
        Err(err) => {
            error!("Error reading YAML file: {}", err);
            return None;
        }
    };
    Some(yaml_string)
}

// fn read_graphemes_yaml() -> Option<String> {
//     // Specify the path to the YAML file
//     let file_path = "graphemes.yaml";
//
//     // Read the YAML file content into a string
//     let yaml_string = match read_yaml_file(file_path) {
//         Ok(content) => content,
//         Err(err) => {
//             error!("Error reading YAML file: {}", err);
//             return None;
//         }
//     };
//     Some(yaml_string)
// }

// fn add_unique_strings(data: &mut HashMap<usize, Vec<String>>, index: usize, new_strings: Vec<String>) {
//     for s in new_strings {
//         if !data.get(&index).unwrap_or(&Vec::new()).contains(&s) {
//             data.entry(index).or_insert_with(Vec::new).push(s);
//         }
//     }
// }

pub fn get_possible_corrections(original_word: String) -> BTreeSet<String> {
    // Initialize logger with color configuration
    initialize_logger();

    let spellings = read_spellings_yaml().unwrap();

    // Deserialize the YAML string back to a struct
    let deserialized_spellings: Spellings =
        serde_yaml::from_str(&spellings).expect("Failed to deserialize YAML");

    // let graphemes = match read_graphemes_yaml() {
    //     Some(value) => value,
    //     None => return,
    // };

    // let deserialized_graphemes: Graphemes = serde_yaml::from_str(&graphemes).expect("Failed to deserialize YAML");

    // // Access fields and print the deserialized struct
    // debug!("Deserialized Struct:");
    // for (spelling, spelling_data) in deserialized_example.spellings.iter() {
    //     debug!("Spelling: {}", spelling);
    //     debug!("Sounds: {:?}", spelling_data.sounds);
    // }

    let data_structure_btreemap: BTreeMap<String, bool> = deserialized_spellings
        .spellings
        .keys()
        .map(|spelling| (spelling.to_string(), true))
        .collect();

    let phoneme_array = phonemize(
        &deserialized_spellings,
        &original_word.to_string(),
        &data_structure_btreemap,
    );

    debug!("{:?}", phoneme_array);

    let mut parsings_array_set: HashSet<Vec<String>> = HashSet::new();

    parsings_array_set.insert(phoneme_array);
    //gen_phoneme_permutations(&phoneme_array, Vec::new(), 0, &mut parsings_array_set);  // FIXME - only create the permutations if words not found for first parse, save time, more efficient

    debug!("{:?}", parsings_array_set);

    let parsings_array: Vec<Vec<String>> = parsings_array_set.into_iter().collect();

    let mut possible_corrections = BTreeSet::new();

    for parse in parsings_array {
        let mut sounds_array: BTreeMap<usize, Vec<Sound>> = BTreeMap::new();

        // Add the sounds from each parse in the array
        for (i, phoneme) in parse.iter().enumerate() {
            sounds_array.insert(
                i,
                deserialized_spellings
                    .spellings
                    .get(phoneme)
                    .as_ref()
                    .map(|spelling| spelling.sounds.clone())
                    .unwrap(),
            );
        }

        // debug!("sounds_array len {:?}", sounds_array.len());
        debug!("sounds_array {:?}", sounds_array);

        for sound in sounds_array.iter() {
            debug!("{:?}", sound);
        }

        // Create the spelling permutations:
        let mut word_spelling_permutations = gen_ipa_word_permutations(&sounds_array);

        // FIXME - should probably be a HashSet or equivalent data structure.
        word_spelling_permutations.sort();
        word_spelling_permutations.dedup();

        debug!(
            "{} permutations {:?}",
            word_spelling_permutations.len(),
            word_spelling_permutations
        );

        for ipa_char_word in word_spelling_permutations {
            // if word is in CMU Pronunciation dict, add the words that collate there to list of corrections
            let word_vector: Option<&Vec<String>> =
                IPA_TO_DICT_WORD_MAP.get(ipa_char_word.as_str());

            match word_vector {
                Some(words) => {
                    for word in words.iter() {
                        // CMU dict has words that are proper names and might not make sense here
                        if USR_SHARE_DICT_WORDS_HASHSET.contains(word) {
                            possible_corrections.insert(word.to_string());
                        }
                    }
                }
                None => {}
            };
        }
    }

    debug!("{:?}", possible_corrections);

    possible_corrections
}

fn initialize_logger() {
    Logger::try_with_str("trace")
        .unwrap()
        .duplicate_to_stdout(Duplicate::Trace)
        .format_for_stderr(colored_opt_format)
        .set_palette("b9;b11;b2;b4;b6".parse().unwrap())
        .start()
        .unwrap();
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn phonemizes_words_with_known_diphthongs() {
//         let deserialized_spellings = Spellings {
//             spellings: vec![
//                 ("ea".to_string(), "iy".to_string()),
//                 ("ai".to_string(), "ey".to_string()),
//             ]
//             .into_iter()
//             .collect(),
//         };
//
//         assert_eq!(
//             phonemize(&deserialized_spellings, "steak".to_string()),
//             vec!["st", "iy", "k"]
//         );
//         assert_eq!(
//             phonemize(&deserialized_spellings, "rain".to_string()),
//             vec!["r", "ey", "n"]
//         );
//     }
// }
