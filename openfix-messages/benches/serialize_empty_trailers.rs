#![feature(test)]

extern crate test;

use test::Bencher;

use openfix_messages::enc_helpers::FixEnvelopeBuilder;
use openfix_messages::test_spec::fields::*;
use openfix_messages::test_spec::messages::*;
use openfix_messages::AsFixMessage;

fn build_header() -> MessageHeader {
    MessageHeader {
        msg_type: MsgType::Heartbeat,
        sender_comp_id: SenderCompID::new("BROKER".into()),
        target_comp_id: TargetCompID::new("MARKET".into()),
        msg_seq_num: MsgSeqNum::new(23593),
        sending_time: SendingTime::new(1618082857.9780622),
        appl_ver_id: Some(ApplVerID::Fix42),
    }
}

fn build_trailer() -> MessageTrailer {
    MessageTrailer {}
}

fn build_hb() -> MessageHeartbeat {
    MessageHeartbeat {
        header: build_header(),
        trailer: build_trailer(),
        test_req_id: None,
    }
}

#[bench]
fn bench_serialize_no_envelope_empty_vec(bencher: &mut Bencher) {
    let message = build_hb();

    bencher.iter(|| {
        let mut payload = Vec::new();
        message.encode_message(&mut payload).unwrap();
    });
}

#[bench]
fn bench_serialize_no_envelope_reserved_vec(bencher: &mut Bencher) {
    let message = build_hb();

    bencher.iter(|| {
        let mut payload = Vec::with_capacity(1024);
        message.encode_message(&mut payload).unwrap();
    });
}

#[bench]
fn bench_serialize_full(bencher: &mut Bencher) {
    let message = build_hb();
    let envelope_builder = FixEnvelopeBuilder::new();

    bencher.iter(|| {
        let mut payload = Vec::with_capacity(1024);
        message.encode_message(&mut payload).unwrap();
        let _data = envelope_builder.build_message(&payload);
    });
}
