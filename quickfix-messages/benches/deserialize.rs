#![feature(test)]

extern crate test;

use test::Bencher;

use quickfix_messages::dec_helpers::split_message_items;
use quickfix_messages::test_spec_sig::messages::*;
use quickfix_messages::FromFixMessage;

#[bench]
fn bench_deserialize(bencher: &mut Bencher) {
    bench_data(bencher, b"8=FIX.4.4\x019=80\x0135=0\x0149=BROKER\x0156=MARKET\x0134=23593\x0152=1618082857.9780622\x011128=4\x0193=8\x0189=arthurlm\x0110=239\x01");
    bench_data(bencher, b"8=FIX.4.4\x019=80\x0135=0\x0149=BROKER\x0156=MARKET\x0134=23593\x0152=1618082857.9780622\x011128=4\x0193=8\x0189=arthurlm\x0110=239\x01\x01\x01\x01");
}

fn bench_data(bencher: &mut Bencher, data: &[u8]) {
    bencher.iter(|| {
        let _message = MessageHeartbeat::decode_message(&split_message_items(data)).unwrap();
    });
}
