use format_bytes::format_bytes;

const NUM_BEGIN_STRING: &'static [u8] = b"8";
const NUM_BODY_LENGTH: &'static [u8] = b"9";
const NUM_CHECK_SUM: &'static [u8] = b"10";

/// This class help to add few standard fields to a already
/// generated message. Including:
/// - begin string (default: FIX.4.4)
/// - length
/// - check sum
///
/// Bug may remains when payload is empty or do not end with '\x01'.
#[derive(Debug)]
pub struct FixEnvelopeBuilder {
    begin_string: String,
}

impl FixEnvelopeBuilder {
    pub fn new() -> Self {
        Self {
            begin_string: "FIX.4.4".to_string(),
        }
    }

    pub fn begin_string(mut self, value: &str) -> Self {
        self.begin_string = value.into();
        self
    }

    pub fn build_message(&self, data: &[u8]) -> Vec<u8> {
        let message = format_bytes!(
            b"{}={}\x01{}={}\x01{}",
            NUM_BEGIN_STRING,
            self.begin_string.as_bytes(),
            NUM_BODY_LENGTH,
            data.len(),
            data,
        );

        let check_sum = message.iter().map(|x| *x as u64).sum::<u64>() % 256;
        let check_sum = format!("{:03}", check_sum); // format bytes does not have leading zero feature

        format_bytes!(b"{}{}={}\x01", message, NUM_CHECK_SUM, check_sum.as_bytes()).to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_envelope_defaults() {
        let builder = FixEnvelopeBuilder::new();

        assert_eq!(
            builder.build_message(b""),
            b"8=FIX.4.4\x019=0\x0110=200\x01".to_vec()
        );
        assert_eq!(
            builder.build_message(b"5=foo\x019=bar\x01"),
            b"8=FIX.4.4\x019=12\x015=foo\x019=bar\x0110=094\x01".to_vec()
        );
        assert_eq!(
            builder.build_message(
                b"35=A\x0149=SERVER\x0156=CLIENT\x0134=177\x0152=20090107-18:15:16\x0198=0\x01108=30\x01"
            ),
            b"8=FIX.4.4\x019=65\x0135=A\x0149=SERVER\x0156=CLIENT\x0134=177\x0152=20090107-18:15:16\x0198=0\x01108=30\x0110=064\x01".to_vec()
        );
    }

    #[test]
    fn test_add_envelope_with_params() {
        let builder = FixEnvelopeBuilder::new().begin_string("FIX.4.2");

        assert_eq!(
            builder.build_message(b""),
            b"8=FIX.4.2\x019=0\x0110=198\x01".to_vec()
        );
        assert_eq!(
            builder.build_message(b"5=foo\x019=bar\x01"),
            b"8=FIX.4.2\x019=12\x015=foo\x019=bar\x0110=092\x01".to_vec()
        );
        assert_eq!(
            builder.build_message(
                b"35=A\x0149=SERVER\x0156=CLIENT\x0134=177\x0152=20090107-18:15:16\x0198=0\x01108=30\x01"
            ),
            b"8=FIX.4.2\x019=65\x0135=A\x0149=SERVER\x0156=CLIENT\x0134=177\x0152=20090107-18:15:16\x0198=0\x01108=30\x0110=062\x01".to_vec()
        );
    }
}
