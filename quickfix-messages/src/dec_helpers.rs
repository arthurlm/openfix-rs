use std::collections::HashMap;

pub type FixFieldItems = HashMap<u32, Vec<u8>>;

const SEP_CHAR: u8 = 0x01;

pub fn split_message_items(data: &[u8]) -> FixFieldItems {
    data.split(|x| *x == SEP_CHAR)
        .filter_map(|field| {
            let fields: Vec<_> = field.splitn(2, |x| *x == '=' as u8).collect();

            match fields[..] {
                [field_id, field_data] => {
                    let field_id = std::str::from_utf8(field_id).ok()?;
                    let field_id = field_id.parse::<u32>().ok()?;

                    let field_data = field_data.to_vec();

                    Some((field_id, field_data))
                }
                _ => None,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::array::IntoIter;
    use std::iter::FromIterator;

    // rustc v1.51 required to run this tests

    #[test]
    fn test_split_message_items_std_payload() {
        assert_eq!(
            split_message_items(b""),
            HashMap::from_iter(IntoIter::new([]))
        );
        assert_eq!(
            split_message_items(b"\x01"),
            HashMap::from_iter(IntoIter::new([]))
        );
        assert_eq!(
            split_message_items(b"\x01\x01\x01\x01"),
            HashMap::from_iter(IntoIter::new([]))
        );
        assert_eq!(
            split_message_items(b"5=foo"),
            HashMap::from_iter(IntoIter::new([(5, b"foo".to_vec())]))
        );
        assert_eq!(
            split_message_items(b"5=foo\x012631=bar"),
            HashMap::from_iter(IntoIter::new([
                (5, b"foo".to_vec()),
                (2631, b"bar".to_vec())
            ]))
        );
        assert_eq!(
            split_message_items(b"\x01\x01\x015=foo\x012631=bar\x01\x01\x01"),
            HashMap::from_iter(IntoIter::new([
                (5, b"foo".to_vec()),
                (2631, b"bar".to_vec())
            ]))
        );
    }

    #[test]
    fn test_split_message_items_weird_payload() {
        assert_eq!(
            split_message_items(b"5="),
            HashMap::from_iter(IntoIter::new([(5, b"".to_vec())]))
        );
        assert_eq!(
            split_message_items(b"foo=bar"),
            HashMap::from_iter(IntoIter::new([]))
        );
        assert_eq!(
            split_message_items(b"foobar"),
            HashMap::from_iter(IntoIter::new([]))
        );
    }
}
