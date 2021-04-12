use std::io::{self, Write};

const NUM_BEGIN_STRING: &str = "8";
const NUM_BODY_LENGTH: &str = "9";
const NUM_CHECK_SUM: &str = "10";

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

    pub fn build_message<W>(&self, writer: &mut W, data: &[u8]) -> io::Result<()>
    where
        W: Write,
    {
        let header = format!(
            "{}={}\x01{}={}\x01",
            NUM_BEGIN_STRING,
            self.begin_string,
            NUM_BODY_LENGTH,
            data.len(),
        );

        macro_rules! bytes_sum {
            ($x:expr) => {
                $x.iter().map(|x| *x as u64).sum::<u64>()
            };
        }

        let check_sum = (bytes_sum!(header.as_bytes()) + bytes_sum!(data)) % 256;

        writer.write_all(header.as_bytes())?;
        writer.write_all(data)?;
        write!(writer, "{}={:03}\x01", NUM_CHECK_SUM, check_sum)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! build_message {
        ($builder:expr, $content:expr) => {{
            let mut data = Vec::new();
            $builder.build_message(&mut data, $content).unwrap();
            data
        }};
    }

    #[test]
    fn test_add_envelope_defaults() {
        let builder = FixEnvelopeBuilder::new();

        assert_eq!(
            build_message!(builder, b""),
            b"8=FIX.4.4\x019=0\x0110=200\x01".to_vec()
        );
        assert_eq!(
            build_message!(builder, b"5=foo\x019=bar\x01"),
            b"8=FIX.4.4\x019=12\x015=foo\x019=bar\x0110=094\x01".to_vec()
        );
        assert_eq!(
            build_message!(builder,
                b"35=A\x0149=SERVER\x0156=CLIENT\x0134=177\x0152=20090107-18:15:16\x0198=0\x01108=30\x01"
            ),
            b"8=FIX.4.4\x019=65\x0135=A\x0149=SERVER\x0156=CLIENT\x0134=177\x0152=20090107-18:15:16\x0198=0\x01108=30\x0110=064\x01".to_vec()
        );
    }

    #[test]
    fn test_add_envelope_with_params() {
        let builder = FixEnvelopeBuilder::new().begin_string("FIX.4.2");

        assert_eq!(
            build_message!(builder, b""),
            b"8=FIX.4.2\x019=0\x0110=198\x01".to_vec()
        );
        assert_eq!(
            build_message!(builder, b"5=foo\x019=bar\x01"),
            b"8=FIX.4.2\x019=12\x015=foo\x019=bar\x0110=092\x01".to_vec()
        );
        assert_eq!(
            build_message!(builder,
                b"35=A\x0149=SERVER\x0156=CLIENT\x0134=177\x0152=20090107-18:15:16\x0198=0\x01108=30\x01"
            ),
            b"8=FIX.4.2\x019=65\x0135=A\x0149=SERVER\x0156=CLIENT\x0134=177\x0152=20090107-18:15:16\x0198=0\x01108=30\x0110=062\x01".to_vec()
        );
    }
}
