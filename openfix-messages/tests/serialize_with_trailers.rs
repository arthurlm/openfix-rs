use openfix_messages::enc_helpers::FixEnvelopeBuilder;
use openfix_messages::test_spec_sig::fields::*;
use openfix_messages::test_spec_sig::messages::*;
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

fn build_empty_trailer() -> MessageTrailer {
    MessageTrailer {
        signature_length: None,
        signature: None,
    }
}

fn build_signed_trailer() -> MessageTrailer {
    let author = "arthurlm".to_string();
    MessageTrailer {
        signature_length: Some(SignatureLength::new(author.len())),
        signature: Some(Signature::new(author)),
    }
}

#[test]
fn test_serialize_empty_trailer() -> anyhow::Result<()> {
    let envelope_builder = FixEnvelopeBuilder::new();
    let message = MessageHeartbeat {
        header: build_header(),
        trailer: build_empty_trailer(),
        test_req_id: None,
    };

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

#[test]
fn test_serialize_signed_trailer() -> anyhow::Result<()> {
    let envelope_builder = FixEnvelopeBuilder::new();
    let message = MessageHeartbeat {
        header: build_header(),
        trailer: build_signed_trailer(),
        test_req_id: None,
    };

    let mut message_content = Vec::with_capacity(1024);
    message.encode_message(&mut message_content)?;

    let mut data = Vec::with_capacity(1024);
    envelope_builder.build_message(&mut data, &message_content)?;

    assert_eq!(
        data,
        b"8=FIX.4.4\x019=80\x0135=0\x0149=BROKER\x0156=MARKET\x0134=23593\x0152=1618082857.9780622\x011128=4\x0193=8\x0189=arthurlm\x0110=239\x01".to_vec()
    );

    Ok(())
}
