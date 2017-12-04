use std::collections::{HashSet, BTreeSet, HashMap};
use std::collections::hash_map::Entry;

pub fn passphrase_valid_anagram(phrase: &str) -> bool {
    let mut phrases = Vec::new();

    for word in phrase.split_whitespace() {
        let mut word_decomp = HashMap::new();
        for letter in word.chars() {
            *word_decomp.entry(letter).or_insert(0) += 1;
        }

        if phrases.contains(&word_decomp) {
            return false;
        }

        phrases.push(word_decomp);
    }

    true
}

pub fn passphrase_valid(phrase: &str) -> bool {
    let mut found_phrases = HashSet::new();

    for word in phrase.split_whitespace() {
        if found_phrases.contains(&word) {
            return false;
        }

        found_phrases.insert(word);
    }

    true
}

pub fn puzzle() {
    println!("Part 1: {}", ::input_file().lines().map(passphrase_valid).fold(0, |acc, cur| acc + (cur as usize)));
    println!("Part 2: {}", ::input_file().lines().map(passphrase_valid_anagram).fold(0, |acc, cur| acc + (cur as usize)));
}

#[cfg(test)]
mod test {
    extern crate test;
    use super::*;

    #[test]
    fn passphrase_samples_part1() {
        assert_eq!(passphrase_valid("aa bb cc dd ee".into()), true);
        assert_eq!(passphrase_valid("aa bb cc dd aa".into()), false);
        assert_eq!(passphrase_valid("aa bb cc dd aaa".into()), true);
    }

    #[test]
    fn passphrase_samples_part2() {
        // words are anagrams of themselves, so these should still work
        assert_eq!(passphrase_valid_anagram("aa bb cc dd ee".into()), true);
        assert_eq!(passphrase_valid_anagram("aa bb cc dd aa".into()), false);
        assert_eq!(passphrase_valid_anagram("aa bb cc dd aaa".into()), true);

        assert_eq!(passphrase_valid_anagram("abcde fghij"), true);
        assert_eq!(passphrase_valid_anagram("abcde xyz ecdab"), false);
        assert_eq!(passphrase_valid_anagram("a ab abc abd abf abj"), true);
        assert_eq!(passphrase_valid_anagram("iiii oiii ooii oooi oooo"), true);
        assert_eq!(passphrase_valid_anagram("oiii ioii iioi iiio"), false);
    }

    #[bench]
    fn anagrams(b: &mut test::Bencher) {
        b.iter(|| test::black_box(passphrase_valid_anagram("bxebny akknwxw jeyxqvj syl cedps akknwxw akknwxw zpvnf kuoon pnkejn wqjgc")));
    }
}
