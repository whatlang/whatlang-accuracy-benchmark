use whatlang::{Script, Lang};
use structopt::StructOpt;

use crate::benchmark;

#[derive(Debug, StructOpt)]
#[structopt(name = "whatlang-occuracy-benchmark", about = "Runs occuracy benchmarks for whatlang library")]
struct Opt {
    #[structopt(short="s", long="script")]
    script: Option<Script>
}

pub fn run() {
    let opt = Opt::from_args();

    println!("{:?}", opt.script);

    let langs: Vec<Lang> =
        match opt.script {
            None => Lang::values().collect(),
            // TODO: Provide ability to fetch langs by script from whatlang lib
            Some(Script::Cyrillic) => vec![Lang::Rus, Lang::Ukr, Lang::Bul, Lang::Bel, Lang::Srp, Lang::Mkd],
            Some(script) => panic!(format!("Script {} is not yet supported", script))
        };

    benchmark::run(langs);
}
