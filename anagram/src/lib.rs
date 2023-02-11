use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {

    let lower_word: String = word.to_lowercase();
    let sorted_word: String = sort_str(&lower_word);

    let mut output = HashSet::new();

    for anagram in possible_anagrams.iter().copied() {
        let lower_anagram: String = anagram.to_lowercase();
        let sorted_anagram: String = sort_str(&lower_anagram.to_string());

        if sorted_word == sorted_anagram && lower_word != lower_anagram {
            output.insert(anagram);
        }
    }

    output
}

pub fn sort_str(word: &String) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort_unstable();

    String::from_iter(chars)
}