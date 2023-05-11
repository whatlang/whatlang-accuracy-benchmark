use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

use whatlang::Lang;
use whatlang::dev::{Method, detect_with_options, Options, FilterList};

// A random ascii string of length 100 chars.
const ASCII_SHORT: &str = "It is a long established fact";
const ASCII_MEDIUM: &str = "It is a long established fact that a reader will be distracted by the readable content of a page when looking at its layout. The point of using Lorem Ipsum is that it has a more-or-less normal distribution of letters, as opposed to using 'Content here, content here', making it look like readable English. Many desktop publishing packages and web page editors now use Lorem Ipsum as their default model text, and a search for 'lorem ipsum' will uncover many web sites still in their infancy. Various versions have evolved over the years, sometimes by accident, sometimes on purpose (injected humour and the like).";
const JP_SHORT: &str = "日本ごです。　とても素敵な言葉ですね";
const JP_MEDIUM: &str = "日本ごです。　和名の由来は、太陽の動きにつれてその方向を追うように花が回るといわれたことから。ただしこの動きは生長に伴うものであるため、実際に太陽を追って動くのは生長が盛んな若い時期だけである。若いヒマワリの茎の上部の葉は太陽に正対になるように動き、朝には東を向いていたのが夕方には西を向く。日没後はまもなく起きあがり、夜明け前にはふたたび東に向く。この運動はつぼみを付ける頃まで続くが、つぼみが大きくなり花が開く素敵な言葉ですね.";

pub fn criterion_benchmark(c: &mut Criterion) {
    let langs_str: [&str; 16] = ["ara","cmn","deu","eng","fra","hin","ita","jpn","kor","nld","por","rus","spa","swe","tur","vie"];
    let mut langs = Vec::new();
    for lang_str in langs_str {
        langs.push(Lang::from_code(lang_str.to_string()).unwrap())
    }

    let options = Options::new()
        .set_method(Method::Trigram)
        .set_filter_list(FilterList::Allow(langs));

    let mut group = c.benchmark_group("whatlang");
    group
        .throughput(Throughput::Bytes(ASCII_SHORT.len() as u64))
        .bench_with_input("inference_short", ASCII_SHORT, |b, text| {
            b.iter(|| detect_with_options(black_box(text), black_box(&options)));
        });
    group
        .throughput(Throughput::Bytes(ASCII_MEDIUM.len() as u64))
        .bench_with_input("inference_long", ASCII_MEDIUM, |b, text| {
            b.iter(|| detect_with_options(black_box(text), black_box(&options)));
        });
    group
        .throughput(Throughput::Bytes(JP_SHORT.len() as u64))
        .bench_with_input("inference_jp_short", JP_SHORT, |b, text| {
            b.iter(|| detect_with_options(black_box(text), black_box(&options)));
        });
    group
        .throughput(Throughput::Bytes(JP_MEDIUM.len() as u64))
        .bench_with_input("inference_jp_medium", JP_MEDIUM, |b, text| {
            b.iter(|| detect_with_options(black_box(text), black_box(&options)));
        });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
