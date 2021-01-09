mod report;
mod benchmark;
mod cli;

fn main() {
    cli::run();


    // Refine corpora
    //
    // use whatlang::{Lang, Script, detect_script};
    // use std::fs;
    // use whatlang_corpora::Corpus;
    // use std::io::prelude::*;

    // for lang in Script::Cyrillic.langs().iter() {
    //     let dest = format!("{}.txt", lang.code());
    //     let corpus = Corpus::load(*lang);

    //     let mut refined_sentences: Vec<String> = vec![];

    //     for text in corpus.sentences() {
    //         if let Some(Script::Cyrillic) = detect_script(text) {
    //             refined_sentences.push(text.to_string());
    //         }
    //     }

    //     let all_text = refined_sentences.join("\n");
    //     fs::write(&dest, all_text);
    // }
}
