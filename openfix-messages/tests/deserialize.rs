use openfix_messages::dec_helpers::split_message_items;
use openfix_messages::test_spec_sig::fields::*;
use openfix_messages::test_spec_sig::messages::*;
use openfix_messages::FromFixMessage;

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
    let author = "arthurlm".to_string();
    MessageTrailer {
        signature_length: Some(SignatureLength::new(author.len())),
        signature: Some(Signature::new(author)),
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
fn test_deserialize() {
    compare_data(b"8=FIX.4.4\x019=80\x0135=0\x0149=BROKER\x0156=MARKET\x0134=23593\x0152=1618082857.9780622\x011128=4\x0193=8\x0189=arthurlm\x0110=239\x01");
    compare_data(b"8=FIX.4.4\x019=80\x0135=0\x0149=BROKER\x0156=MARKET\x0134=23593\x0152=1618082857.9780622\x011128=4\x0193=8\x0189=arthurlm\x0110=239\x01\x01\x01\x01");
}

fn compare_data(data: &[u8]) {
    let expected = build_hb();
    let message = MessageHeartbeat::decode_message(&split_message_items(data)).unwrap();
    assert_eq!(message, expected);
}
