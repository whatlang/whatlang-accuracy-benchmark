use whatlang::{Script, Lang};
use structopt::StructOpt;

use crate::benchmark;

#[derive(Debug, StructOpt)]
#[structopt(name = "whatlang-occuracy-benchmark", about = "Runs occuracy benchmarks for whatlang library")]
struct Opt {
    #[structopt(short="s", long="script")]
    script: Option<Script>,

    #[structopt(short="l", long="lang", use_delimiter = true)]
    langs: Option<Vec<Lang>>
}

impl Opt {
    fn langs(&self) -> Vec<Lang> {
        if let Some(ref langs) = self.langs {
            return langs.to_vec();
        }
        if let Some(script) = self.script {
            return script.langs().to_vec();
        }

        // Return all
        Lang::values().to_vec()
    }
}

pub fn run() {
    let opt = Opt::from_args();
    let langs = opt.langs();
    benchmark::run(langs);
}
