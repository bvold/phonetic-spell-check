use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs::File;
use std::io::Read;
use sound::Sound;
use std::collections::HashMap;

mod sound;

#[derive(Debug, Serialize, Deserialize)]
struct Spelling {
    sounds: Vec<Sound>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Spellings {
    spellings: std::collections::BTreeMap<String, Spelling>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Graphemes {
    graphemes: std::collections::HashMap<String, Grapheme>,
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

fn main() {
    let spellings = match read_spellings_yaml() {
        Some(value) => value,
        None => return,
    };

    // Deserialize the YAML string back to a struct
    let deserialized_example: Spellings =
        serde_yaml::from_str(&spellings).expect("Failed to deserialize YAML");

    let graphemes = match read_graphemes_yaml() {
        Some(value) => value,
        None => return,
    };

    let deserialized_graphemes: Graphemes = serde_yaml::from_str(&graphemes).expect("Failed to deserialize YAML");

    // // Access fields and print the deserialized struct
    // println!("Deserialized Struct:");
    // for (spelling, spelling_data) in deserialized_example.spellings.iter() {
    //     println!("Spelling: {}", spelling);
    //     println!("Sounds: {:?}", spelling_data.sounds);
    // }

    // Get all sounds for 'l'
    let l_sounds = deserialized_example.spellings.get("l");
    println!("l_sounds: {:?}", l_sounds.unwrap().sounds);

    // get all sounds for 'a'
    let a_sounds = deserialized_example.spellings.get("a");
    println!("a_sounds: {:?}", a_sounds.unwrap().sounds);

    // get all sounds for 'ff'
    let ff_sounds = deserialized_example.spellings.get("ff");
    println!("ff_sounds: {:?}", ff_sounds.unwrap().sounds);

    let mut sounds_array: HashMap<usize, Vec<Sound>> = HashMap::new();
    sounds_array.insert(0, l_sounds.as_ref().map(|spelling| spelling.sounds.clone()).unwrap());
    sounds_array.insert(1, a_sounds.as_ref().map(|spelling| spelling.sounds.clone()).unwrap());
    sounds_array.insert(2, ff_sounds.as_ref().map(|spelling| spelling.sounds.clone()).unwrap());

    println!("sounds_array len {:?}", sounds_array.len());
    println!("sounds_array {:?}", sounds_array);

    let mut spellings_array: HashMap<usize, Vec<String>> = HashMap::new();

    let l_spellings = deserialized_graphemes.graphemes.get("L").unwrap().spellings.clone();
    let schwa_l_spellings = deserialized_graphemes.graphemes.get("SchwaL").unwrap().spellings.clone();

    add_unique_strings(&mut spellings_array, 0, l_spellings);
    add_unique_strings(&mut spellings_array, 0, schwa_l_spellings);

    println!("{:?}", spellings_array);


    // for a_sound in a_sounds {
    //     spellings_array.insert(0, deserialized_graphemes.graphemes.get(a_sound).unwrap().spellings.clone());
    // }
    // for ff_sound in ff_sounds {
    //     spellings_array.insert(0, deserialized_graphemes.graphemes.get(ff_sound).unwrap().spellings.clone());
    // }

    // println!("{:?}", spellings_array);
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

fn read_graphemes_yaml() -> Option<String> {
// Specify the path to the YAML file
    let file_path = "graphemes.yaml";

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


fn add_unique_strings(data: &mut HashMap<usize, Vec<String>>, index: usize, new_strings: Vec<String>) {
    for s in new_strings {
        if !data.get(&index).unwrap_or(&Vec::new()).contains(&s) {
            data.entry(index).or_insert_with(Vec::new).push(s);
        }
    }
}