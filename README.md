# Whatlang accuracy benchmark

This is a fork of [whatlang-accuracy-benchmark](https://github.com/whatlang/whatlang-accuracy-benchmark) that integrates [Whichlang](https://github.com/quickwit-oss/whichlang) along with few small changes to make the comparison fair. 

## Setup 

First clone the following repositories at the same directory level as this repository:
- https://github.com/greyblake/whatlang-rs
- https://github.com/whatlang/whatlang-corpora
 

## How to run the benchmarks 

Run `cargo bench` to execute the basic Whichlang benchmark for Whatlang.

To run the accuracy benchmarks, you will need to run the following commands respectively for Whichlang and Whatlang.


```bash
cargo run --release -- --crate whichlang --save-report --lang ara,cmn,deu,eng,fra,hin,ita,jpn,kor,nld,por,rus,spa,swe,tur,vie
```

```bash
cargo run --release -- --crate whatlang --save-report --lang ara,cmn,deu,eng,fra,hin,ita,jpn,kor,nld,por,rus,spa,swe,tur,vie
```
