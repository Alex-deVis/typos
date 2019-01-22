#![feature(test)]

extern crate test;

mod data;

#[bench]
fn tokenize_empty(b: &mut test::Bencher) {
    b.iter(|| scorrect::tokenize(data::EMPTY.as_bytes()).collect::<Vec<_>>());
}

#[bench]
fn tokenize_no_tokens(b: &mut test::Bencher) {
    b.iter(|| scorrect::tokenize(data::NO_TOKENS.as_bytes()).collect::<Vec<_>>());
}

#[bench]
fn tokenize_single_token(b: &mut test::Bencher) {
    b.iter(|| scorrect::tokenize(data::SINGLE_TOKEN.as_bytes()).collect::<Vec<_>>());
}

#[bench]
fn tokenize_sherlock(b: &mut test::Bencher) {
    b.iter(|| scorrect::tokenize(data::SHERLOCK.as_bytes()).collect::<Vec<_>>());
}

#[bench]
fn tokenize_code(b: &mut test::Bencher) {
    b.iter(|| scorrect::tokenize(data::CODE.as_bytes()).collect::<Vec<_>>());
}

#[bench]
fn tokenize_corpus(b: &mut test::Bencher) {
    b.iter(|| scorrect::tokenize(data::CORPUS.as_bytes()).collect::<Vec<_>>());
}