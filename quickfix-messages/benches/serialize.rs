#![feature(test)]

extern crate test;

use test::Bencher;

use quickfix_messages::test_spec::fields::*;
use quickfix_messages::test_spec::messages::*;
use quickfix_messages::AsFixMessage;

fn build_header() -> MessageHeader {
    MessageHeader {
        begin_string: BeginString::new("FIX4.2".into()),
        body_length: BodyLength::new(100),
        msg_type: MsgType::Heartbeat,
        sender_comp_id: SenderCompID::new("BROKER".into()),
        target_comp_id: TargetCompID::new("MARKET".into()),
        msg_seq_num: MsgSeqNum::new(23593),
        sending_time: SendingTime::new(1618082857.9780622),
        appl_ver_id: Some(ApplVerID::Fix42),
    }
}

fn build_trailer() -> MessageTrailer {
    MessageTrailer {
        check_sum: CheckSum::new("XXX".into()),
    }
}

fn build_hb() -> MessageHeartbeat {
    MessageHeartbeat {
        header: build_header(),
        trailer: build_trailer(),
        test_req_id: None,
    }
}

#[bench]
fn bench_serialize(bencher: &mut Bencher) {
    let message = build_hb();
    bencher.iter(|| message.encode_message());
}
