mod report;
mod benchmark;
mod cli;

use whatlang_corpora::Corpus;
use whatlang::Lang;
use rayon::prelude::*;

use report::{LangReport, OverallReport, size};

fn main() {
    cli::run();
}
