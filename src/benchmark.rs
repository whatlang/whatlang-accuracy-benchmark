use whatlang_corpora::Corpus;
use whatlang::Lang;
use rayon::prelude::*;

use crate::report::{LangReport, OverallReport, size};

pub fn run(langs: Vec<Lang>) -> OverallReport {
    let lang_reports: Vec<LangReport> =
        langs.par_iter().map(|&lang| {
            let lang_report = benchmark_lang(lang);
            println!("{}", lang_report);
            lang_report
        }).collect();

    let overall_report = OverallReport::new(lang_reports);
    println!("{}", overall_report);
    overall_report
}

fn benchmark_lang(lang: Lang) -> LangReport {
    let mut lang_report = LangReport::new(lang);
    let corpus = Corpus::load(lang);
    let detector = whatlang::Detector::new();

    for (_num, sentence) in corpus.sentences().enumerate() {
        let size = size(sentence);

        let lang_opt = detector.detect_lang(sentence);
        if lang_opt == Some(lang) {
            lang_report.inc_correct(size);
        } else {
            lang_report.inc_wrong(size, lang_opt);
        }
    }
    lang_report
}
