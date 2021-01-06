use whatlang_corpora::Corpus;
use whatlang::Lang;
use rayon::prelude::*;
use enum_map::{Enum, EnumMap};

#[derive(Debug, Enum)]
enum Size {
    // <20 chars
    Under20,

    // 21-50 chars
    Under50,

    // 51-100 chars
    Under100,

    // > 100 chars
    Over100
}

struct Counter {
    wrong: u32,
    correct: u32
}

impl Counter {
    fn new() -> Self {
        Self { wrong: 0, correct: 0 }
    }

    fn inc_wrong(&mut self) {
        self.wrong +=1 ;
    }

    fn inc_correct(&mut self) {
        self.correct +=1 ;
    }

    fn total(&self) -> u32 {
        self.wrong + self.correct
    }

    fn correctness(&self) -> f64 {
        self.correct as f64 / self.total() as f64
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

fn size(sentence: &str) -> Size {
    match sentence.chars().count() {
        0..=20 => Size::Under20,
        21..=50 => Size::Under50,
        51..=100 => Size::Under100,
        _ => Size::Over100
    }
}

fn benchmark_lang(lang: Lang) {
    let mut counters: EnumMap<Size, Counter> = EnumMap::new();

    let corpus = Corpus::load(lang);

    for (_num, sentence) in corpus.sentences().enumerate() {
        let size = size(sentence);

        if whatlang::detect_lang(sentence) == Some(lang) {
            counters[size].inc_correct();
        } else {
            counters[size].inc_wrong();
        }
    }

    println!("{}", lang);
    for (size, counter) in &counters {
        let pct = counter.correctness() * 100.0;
        println!("  {:?}: {:.2}%   (total = {})", size,  pct, counter.total());
    }
}

fn main() {
    // let langs: Vec<Lang> = Lang::values().collect();

    langs.par_iter().for_each(|&lang| {
        benchmark_lang(lang);
    });
}
