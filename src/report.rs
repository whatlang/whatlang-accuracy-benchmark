use whatlang::Lang;
use enum_map::{Enum, EnumMap};
use std::fmt;
// use prettytable::{Table, Row, Cell};
use prettytable::{row, cell, Table, Row, Cell};

#[derive(Debug, Clone, Copy, Enum)]
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
    pub correct: u32,

    pub reliable_correct: u32,
    pub reliable_false_postives: u32,
}

impl Counter {
    pub fn new() -> Self {
        Self { wrong: WrongCounter::new(), correct: 0, reliable_correct: 0, reliable_false_postives: 0 }
    }

    pub fn inc_wrong(&mut self, lang_opt: Option<Lang>, is_reliable: bool) {
        self.wrong.add(lang_opt);
        if is_reliable {
            self.reliable_false_postives += 1;
        }
    }

    pub fn inc_correct(&mut self, is_reliable: bool) {
        self.correct +=1 ;
        if is_reliable {
            self.reliable_correct += 1;
        }
    }

    pub fn total(&self) -> u32 {
        self.wrong.total() + self.correct
    }

    fn accuracy(&self) -> f64 {
        self.correct as f64 / self.total() as f64
    }

    pub fn reliable_total(&self) -> u32 {
        self.reliable_correct + self.reliable_false_postives
    }

    pub fn reliable_accuracy(&self) -> f64 {
        self.reliable_correct as f64 / self.reliable_total() as f64
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

    pub fn inc_correct(&mut self, size: Size, is_reliable: bool) {
        self.size_counters[size].inc_correct(is_reliable);
    }

    pub fn inc_wrong(&mut self, size: Size, lang_opt: Option<Lang>, is_reliable: bool) {
        self.size_counters[size].inc_wrong(lang_opt, is_reliable);
    }

    pub fn accuracy_for_size(&self, size: Size) -> f64 {
        self.size_counters[size].accuracy()
    }

    pub fn reliable_accuracy_for_size(&self, size: Size) -> f64 {
        self.size_counters[size].reliable_accuracy()
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
        use colored::Colorize;

        writeln!(f, "\n{}", self.lang.eng_name().bold())?;

        for (size, counter) in &self.size_counters {
            let pct = counter.accuracy() * 100.0;
            writeln!(f, "    {:?}: {:.2}%   (total = {})", size,  pct, counter.total())?;
            let mut wrong_langs: Vec<(Lang, u32)> =
                counter.wrong.langs
                    .into_iter()
                    .filter(|(_lang, count)| *count > 0)
                    .collect();

            let reliable_pct = counter.reliable_total() as f64 / counter.total() as f64 * 100.0;
            writeln!(f, "        is_reliable():")?;

            let mut accuracy_msg = format!("            accuracy: {:.2}%", counter.reliable_accuracy() * 100.0);
            if counter.reliable_accuracy() < 0.99 {
                accuracy_msg = accuracy_msg.red().to_string();
            }
            writeln!(f, "{}", accuracy_msg)?;

            wrong_langs.sort_by(|a, b| b.1.cmp(&a.1));
            writeln!(f, "        Top false detections:")?;
            for (wrong_lang, count) in wrong_langs.iter().take(2) {
                let wrong_pct = (*count as f64 / counter.total() as f64) * 100.0;
                writeln!(f, "            {:.2}% : {}", wrong_pct, wrong_lang.eng_name())?;
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

    pub fn avg_for_size(&self, size: Size) -> f64 {
        let len = self.lang_reports.len() as f64;
        let sum: f64 = self.lang_reports
            .iter()
            .map(|lr| lr.accuracy_for_size(size))
            .sum();

        sum / len
    }

    pub fn to_prettytable(&self, highlight: bool) -> Table {
        use prettytable::format::{FormatBuilder, LinePosition, LineSeparator};
        let markdown_format = FormatBuilder::new()
            .column_separator('|')
            .left_border('|')
            .right_border('|')
            .padding(1, 1)
            .separator(LinePosition::Title, LineSeparator::new('-', '|', '|', '|'))
            .build();

        let mut table = Table::new();
        //table.set_format(*prettytable::format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_format(markdown_format);
        table.set_titles(row!["LANG", "AVG", "<= 20", "21-50", "51-100", "> 100"]);

        for lang_report in &self.lang_reports {
            let lang = lang_report.lang;
            let lang_name = lang.eng_name();
            let under20 = lang_report.accuracy_for_size(Size::Under20);
            let under50 = lang_report.accuracy_for_size(Size::Under50);
            let under100 = lang_report.accuracy_for_size(Size::Under100);
            let over100 = lang_report.accuracy_for_size(Size::Over100);
            let avg = lang_report.avg_accuracy();

            table.add_row(
                Row::new(vec![
                    Cell::new(lang_name),
                    Cell::new(&format_accuracy(avg, highlight)),
                    Cell::new(&format_accuracy(under20, highlight)),
                    Cell::new(&format_accuracy(under50, highlight)),
                    Cell::new(&format_accuracy(under100, highlight)),
                    Cell::new(&format_accuracy(over100, highlight)),
                ])
            );
        }

        let avg = |size: Size| { format_accuracy(self.avg_for_size(size), highlight) };
        let avg_under20 = avg(Size::Under20);
        let avg_under50 = avg(Size::Under50);
        let avg_under100 = avg(Size::Under100);
        let avg_over100 = avg(Size::Over100);
        let avg_all = format_accuracy(self.avg_accuracy(), highlight);

        table.add_row(
            row!["AVG", avg_all, avg_under20, avg_under50, avg_under100, avg_over100]
        );
        table
    }
}

impl fmt::Display for OverallReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let table = self.to_prettytable(true);
        table.fmt(f)?;

        writeln!(f, "\nOVERALL: {} languages", self.lang_reports.len())?;
        writeln!(f, "AVG: {:.2}%", self.avg_accuracy() * 100.0)

    }
}

fn format_accuracy(accuracy: f64, highlight: bool) -> String {
    use colored::Colorize;

    let s = format!("{:.2}%", accuracy * 100.0);
    if highlight {
        if (0.0..=0.6).contains(&accuracy) {
            s.red().bold().to_string()
        } else if (0.6..=0.8).contains(&accuracy) {
            s.red().to_string()
        } else if (0.8..=0.95).contains(&accuracy) {
            s
        } else if (0.95..=0.99).contains(&accuracy) {
            s.green().to_string()
        } else {
            s.green().bold().to_string()
        }
    } else {
        s
    }
}
