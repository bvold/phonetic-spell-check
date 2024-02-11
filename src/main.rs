use phonetic_spell_rs::get_possible_corrections;

fn main() {
    // let original_word = "grajuation".to_string();
    let original_word = "laff".to_string();
    let possible_corrections = get_possible_corrections(original_word);

    // Print out possible corrections from CMU Pronouncing dict
    // Check if the vector is empty and print a message
    if possible_corrections.is_empty() {
        println!("No corrections available.");
    } else {
        for correction in possible_corrections {
            println!("Possible correction: {:?}", correction);
        }
    }
}
