use whatlang_corpora::Corpus;
use whatlang::Lang;
use whatlang::{Method, detect_with_options, Options};
use rayon::prelude::*;

use crate::report::{LangReport, OverallReport, size};

pub fn run(langs: Vec<Lang>, method: Method) -> OverallReport {
    let lang_reports: Vec<LangReport> =
        langs.par_iter().map(|&lang| {
            let lang_report = benchmark_lang(lang, method);
            println!("{}", lang_report);
            lang_report
        }).collect();

    let overall_report = OverallReport::new(lang_reports);
    println!("{}", overall_report);
    println!("Method used: {}", method);
    overall_report
}

fn benchmark_lang(lang: Lang, method: Method) -> LangReport {
    let mut lang_report = LangReport::new(lang);
    let corpus = Corpus::load(lang);

    let options = Options::new().method(method);

    for (_num, sentence) in corpus.sentences().enumerate() {
        let size = size(sentence);

        let output = detect_with_options(sentence, &options);
        let lang_opt = output.map(|o| o.lang());
        if lang_opt == Some(lang) {
            lang_report.inc_correct(size);
        } else {
            lang_report.inc_wrong(size, lang_opt);
        }
    }
    lang_report
}
