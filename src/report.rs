use whatlang::Lang;
use enum_map::{Enum, EnumMap};
use std::fmt;
// use prettytable::{Table, Row, Cell};
use prettytable::{table, row, cell};

#[derive(Debug, Enum)]
pub enum Size {
    // <20 chars
    Under20,

    // 21-50 chars
    Under50,

    // 51-100 chars
    Under100,

    // > 100 chars
    Over100
}

pub fn size(sentence: &str) -> Size {
    match sentence.chars().count() {
        0..=20 => Size::Under20,
        21..=50 => Size::Under50,
        51..=100 => Size::Under100,
        _ => Size::Over100
    }
}

pub struct WrongCounter {
    pub langs: EnumMap<Lang, u32>,
    pub none: u32
}

impl WrongCounter {
    pub fn new() -> Self {
        Self { langs: EnumMap::default(), none: 0 }
    }

    pub fn add(&mut self, lang_opt: Option<Lang>) {
        match lang_opt {
            Some(lang) => self.langs[lang] += 1,
            None => self.none += 1
        }
    }

    pub fn total(&self) -> u32 {
        self.langs.values().sum::<u32>() +  self.none
    }
}

pub struct Counter {
    pub wrong: WrongCounter,
    pub correct: u32
}

impl Counter {
    pub fn new() -> Self {
        Self { wrong: WrongCounter::new(), correct: 0 }
    }

    pub fn inc_wrong(&mut self, lang_opt: Option<Lang>) {
        self.wrong.add(lang_opt);
    }

    pub fn inc_correct(&mut self) {
        self.correct +=1 ;
    }

    pub fn total(&self) -> u32 {
        self.wrong.total() + self.correct
    }

    fn accuracy(&self) -> f64 {
        self.correct as f64 / self.total() as f64
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

pub struct LangReport {
    lang: Lang,
    size_counters: EnumMap<Size, Counter>,
}

impl LangReport {
    pub fn new(lang: Lang) -> Self {
        let size_counters: EnumMap<Size, Counter> = EnumMap::new();
        Self { lang, size_counters }
    }

    pub fn inc_correct(&mut self, size: Size) {
        self.size_counters[size].inc_correct();
    }

    pub fn inc_wrong(&mut self, size: Size, lang_opt: Option<Lang>) {
        self.size_counters[size].inc_wrong(lang_opt);
    }

    pub fn accuracy_for_size(&self, size: Size) -> f64 {
        self.size_counters[size].accuracy()
    }

    pub fn avg_accuracy(&self) -> f64 {
        let count = self.size_counters
            .values()
            .count();

        let sum: f64 = self.size_counters
            .values()
            .map(|c| c.accuracy())
            .sum();

        sum / count as f64
    }
}

impl fmt::Display for LangReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "\n{}", self.lang.eng_name())?;

        for (size, counter) in &self.size_counters {
            let pct = counter.accuracy() * 100.0;
            writeln!(f, "  {:?}: {:.2}%   (total = {})", size,  pct, counter.total())?;
            let mut wrong_langs: Vec<(Lang, u32)> =
                counter.wrong.langs
                    .into_iter()
                    .filter(|(lang, count)| *count > 0)
                    .collect();

            wrong_langs.sort_by(|a, b| b.1.cmp(&a.1));

            for (wrong_lang, count) in &wrong_langs {
                let wrong_pct = (*count as f64 / counter.total() as f64) * 100.0;
                // println!("    {:.2}% : {}", wrong_pct, wrong_lang.eng_name());
            }
        }
        writeln!(f, "  Avg: {:.2}%", self.avg_accuracy() * 100.0)?;
        Ok(())
    }
}


pub struct OverallReport {
    lang_reports: Vec<LangReport>
}

impl OverallReport {
    pub fn new(lang_reports: Vec<LangReport>) -> Self {
        Self { lang_reports }
    }

    pub fn avg_accuracy(&self) -> f64 {
        let len = self.lang_reports.len();

        let sum: f64 = self.lang_reports
            .iter()
            .map(|lr| lr.avg_accuracy())
            .sum();

        sum / len as f64
    }
}

impl fmt::Display for OverallReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "\n\nOVERALL: {} languages", self.lang_reports.len())?;
        writeln!(f, "  Avg: {:.2}%", self.avg_accuracy() * 100.0)?;

        let t = table![
            ["LANG", "<= 20", "21-50", "51-100", "> 100"],
            ["Eng", "93.54%"]
        ];

        t.fmt(f)
    }
}
