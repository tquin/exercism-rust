// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();

    // Total sum of words we have to play with
    for mag_word in magazine.iter() {
        *word_counts.entry(mag_word).or_insert(0) += 1;
    }

    // Remove individual words as needed from the note
    // If any are not available, immediate fail (ret false)
    for note_word in note.iter() {
        let value = word_counts.entry(note_word).or_insert(0);
        if *value == 0 { return false } else { *value -= 1 };
    }
    true
}
