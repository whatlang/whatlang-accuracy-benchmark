use whatlang::{Script, Lang};
use whatlang::dev::Method;
use structopt::StructOpt;

use crate::benchmark;
use crate::report::OverallReport;

#[derive(Debug, StructOpt)]
#[structopt(name = "whatlang-occuracy-benchmark", about = "Runs occuracy benchmarks for whatlang library")]
struct Opt {
    #[structopt(short="s", long="script")]
    script: Option<Script>,

    #[structopt(short="l", long="lang", use_delimiter = true)]
    langs: Option<Vec<Lang>>,

    #[structopt(short="w", long="write")]
    write_report: bool,

    #[structopt(short="m", long="method", default_value = "trigram")]
    method: Method,
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
    let report = benchmark::run(langs, opt.method);

    if opt.write_report {
        save_report(&report);
        println!("Report is written in ./reports");
    }
}

fn save_report(report: &OverallReport) {
    use chrono::prelude::*;

    let time: DateTime<Utc> = Utc::now();
    let timestamp = time.format("%Y-%m-%d");

    let destination = format!("reports/{}.md", timestamp);
    let table = report.to_prettytable(false).to_string();

    std::fs::write(destination, &table).expect("Unable to save report in file");
    std::fs::write("reports/latest.md", &table).expect("Unable to save report in file");
}
