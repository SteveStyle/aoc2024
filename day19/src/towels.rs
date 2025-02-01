use std::{
    fmt::{Debug, Display, Write},
    ops::{Deref, DerefMut},
};

#[derive(Clone, PartialEq, Eq)]
struct Word(Vec<u8>);

impl Deref for Word {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Word {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Word {
    fn as_chars(&self) -> Vec<char> {
        self.0.iter().map(|&b| b as char).collect()
    }
}

impl PartialOrd for Word {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Word {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.0.len().cmp(&other.0.len()) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => self.0.cmp(&other.0),
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        }
    }
}

impl Debug for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &c in &self.0 {
            f.write_char(c as char)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct Basis {
    source_list: Vec<Word>,
    single_letters: Vec<u8>,

    minimal_list: Vec<Word>,
}

impl Basis {
    fn new(input: &str) -> Self {
        let mut source_list = Vec::new();
        for word in input.split(", ") {
            source_list.push(Word(word.bytes().collect()));
        }
        source_list.sort();
        let single_letters = source_list
            .iter()
            .take_while(|w| w.len() == 1)
            .map(|w| w[0])
            .collect();
        Basis {
            source_list,
            single_letters,
            minimal_list: Vec::new(),
        }
    }
    fn is_only_single_letters(&self, word: &Word) -> bool {
        word.iter().all(|c| self.single_letters.contains(c))
    }

    fn can_be_made_from(word: &[u8], list: &Vec<Word>) -> bool {
        if word.is_empty() {
            return true;
        }
        let mut result = false;
        for try_word in list {
            if word.starts_with(try_word) && Self::can_be_made_from(&word[try_word.len()..], list) {
                return true;
            }
        }
        false
    }
    fn can_be_made(&self, word: &Word) -> bool {
        Self::can_be_made_from(word, &self.minimal_list)
    }
    fn count_ways(&self, word: &[u8]) -> usize {
        if word.is_empty() {
            return 1;
        }
        let mut result = 0;
        for try_word in &self.source_list {
            if word.starts_with(try_word) {
                result += self.count_ways(&word[try_word.len()..]);
            }
        }
        result
    }
    pub fn build_minimal_list(&mut self) {
        let mut new_list = Vec::new();
        for word in &self.source_list {
            if !Self::can_be_made_from(word, &new_list) {
                new_list.push(word.clone());
            }
        }

        self.minimal_list = new_list;
    }
}

#[derive(Debug, Clone)]
pub struct TowelWords {
    basis: Basis,
    target_list: Vec<Word>,
}

impl TowelWords {
    pub fn new(input: &str) -> Self {
        let mut split = input.split("\n\n");
        let basis = Basis::new(split.next().unwrap());
        let mut target_list: Vec<Word> = split
            .next()
            .unwrap()
            .lines()
            .map(|line| Word(line.bytes().collect()))
            .collect();
        target_list.sort();
        Self { basis, target_list }
    }
    pub fn count_possible_targets(&self) -> usize {
        let mut count = 0;
        for word in &self.target_list {
            if self.basis.can_be_made(word) {
                count += 1;
            }
        }
        count
    }
    pub fn count_possible_ways(&self) -> usize {
        let mut count = 0;
        for word in &self.target_list {
            count += self.basis.count_ways(word);
        }

        count
    }
    pub fn build_minimal_list(&mut self) {
        self.basis.build_minimal_list();
    }
}

#[cfg(test)]
mod tests {
    use crate::{towels::*, *};

    #[test]
    fn test_new() {
        let word = TowelWords::new(INPUT);
        println!("{:#?}", word.basis);
        println!(
            "source length {}, minimal list {}",
            word.basis.source_list.len(),
            word.basis.minimal_list.len(),
        )
    }
}
