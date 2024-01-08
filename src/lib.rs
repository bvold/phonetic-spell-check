use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use strum::EnumMessage;
use crate::sound::Sound;

mod sound;
mod lib_test;

#[derive(Debug, Serialize, Deserialize)]
struct Spelling {
    sounds: Vec<Sound>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Spellings {
    spellings: BTreeMap<String, Spelling>,
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

    build_ipa_permutations("", btreemap.keys().collect::<Vec<_>>().as_slice(), btreemap, &mut permutations);

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

// lazy_static! {
//     static ref USR_SHARE_DICT_WORDS_HASHSET: Result<HashSet<String>, std::io::Error> = {
//         let file_contents = std::fs::read_to_string("/usr/share/dict/words")?;
//         let words = file_contents
//             .lines()
//             .map(|line| line.to_lowercase())
//             .collect();
//
//         Ok(words)
//     };
// }
// match dictionary {
// Ok(words) => {
// // Use the `words` HashSet here
// },
// Err(err) => {
// // Handle the error gracefully
// }
// }

lazy_static! {
    static ref USR_SHARE_DICT_WORDS_HASHSET: HashSet<String> = {
        let file = File::open("/usr/share/dict/words").unwrap();
        let reader = BufReader::new(file);

        let mut words = HashSet::new();
        for line in reader.lines() {
            let word = line.unwrap().to_lowercase();
            words.insert(word);
        }

        words
    };
}

fn phonemize(deserialized_spellings: &Spellings, original_word: String) -> Vec<String> {
    // Create a BTreeMap map for efficient diphthong lookup
    let mut key_map: BTreeMap<String, bool> = BTreeMap::new();
    for spelling in deserialized_spellings.spellings.keys() {
        key_map.insert(spelling.to_string(), true);
    }

    let mut diphthong_array = Vec::new();
    let mut offset = 0;
    let mut size = 0;

    let size_of_original_word = original_word.len();

    while offset + size < size_of_original_word {
        // Find the longest matching diphthong
        while offset + size + 1 <= size_of_original_word && key_map.contains_key(&original_word[offset..(offset + size + 1)]) {
            size += 1;
        }

        diphthong_array.push(original_word[offset..(offset + size)].to_string());
        offset += size;
        size = 0;
    }

    diphthong_array
}

// A recursive function that generates permutations of phoneme combinations after initial phoneme parse
fn gen_phoneme_permutations(original: &Vec<String>, current: Vec<String>, index: usize, result: &mut HashSet<Vec<String>>) {
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

fn read_spellings_yaml() -> Option<String> {
// Specify the path to the YAML file
    let file_path = "spellings.yaml";

    // Read the YAML file content into a string
    let yaml_string = match read_yaml_file(file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading YAML file: {}", err);
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
//             eprintln!("Error reading YAML file: {}", err);
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
    // println!("Deserialized Struct:");
    // for (spelling, spelling_data) in deserialized_example.spellings.iter() {
    //     println!("Spelling: {}", spelling);
    //     println!("Sounds: {:?}", spelling_data.sounds);
    // }

    let phoneme_array = phonemize(&deserialized_spellings, original_word.to_string());

    println!("{:?}", phoneme_array);

    let mut parsings_array_set: HashSet<Vec<String>> = HashSet::new();

    gen_phoneme_permutations(&phoneme_array, Vec::new(), 0, &mut parsings_array_set);

    println!("{:?}", parsings_array_set);

    let parsings_array: Vec<Vec<String>> = parsings_array_set.into_iter().collect();

    let mut possible_corrections = BTreeSet::new();

    for parse in parsings_array {
        let mut sounds_array: BTreeMap<usize, Vec<Sound>> = BTreeMap::new();

        // Add the sounds from each parse array
        for (i, phoneme) in parse.iter().enumerate() {
            sounds_array.insert(i, deserialized_spellings.spellings.get(phoneme).as_ref().map(|spelling| spelling.sounds.clone()).unwrap());
        }

        println!("sounds_array len {:?}", sounds_array.len());
        println!("sounds_array {:?}", sounds_array);

        for sound in sounds_array.iter() {
            println!("{:?}", sound);
        }

        // Create the spelling permutations:
        let mut word_spelling_permutations = gen_ipa_word_permutations(&sounds_array);

        // FIXME - should probably be a HashSet or equivalent data structure.
        word_spelling_permutations.sort();
        word_spelling_permutations.dedup();

        println!("{} permutations {:?}", word_spelling_permutations.len(), word_spelling_permutations);

        for ipa_char_word in word_spelling_permutations {
            // if word is in CMU Pronunciation dict, add the words that collate there to list of corrections
            let word_vector: Option<&Vec<String>> = IPA_TO_DICT_WORD_MAP.get(ipa_char_word.as_str());

            match word_vector {
                Some(words) => {
                    for word in words.iter() {
                        // CMU dict has words that are proper names and might not make sense here
                        if USR_SHARE_DICT_WORDS_HASHSET.contains(word) {
                            possible_corrections.insert(word.to_string());
                        }
                    }
                },
                None => {},
            };
        }
    }

    possible_corrections
}