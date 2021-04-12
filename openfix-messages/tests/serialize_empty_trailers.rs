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

#[test]
fn test_serialize() -> anyhow::Result<()> {
    let message = build_hb();
    let envelope_builder = FixEnvelopeBuilder::new();

    let mut message_content = Vec::with_capacity(1024);
    message.encode_message(&mut message_content)?;

    let mut data = Vec::with_capacity(1024);
    envelope_builder.build_message(&mut data, &message_content)?;

    assert_eq!(
        data,
        b"8=FIX.4.4\x019=63\x0135=0\x0149=BROKER\x0156=MARKET\x0134=23593\x0152=1618082857.9780622\x011128=4\x0110=240\x01".to_vec()
    );

    Ok(())
}
