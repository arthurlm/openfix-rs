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

#[test]
fn test_serialize() {
    let message = build_hb();
    assert_eq!(message.encode_message(), b"8=FIX4.2\x019=100\x0135=0\x0149=BROKER\x0156=MARKET\x0134=23593\x0152=1618082857.9780622\x011128=4\x0110=XXX".to_vec());
}
