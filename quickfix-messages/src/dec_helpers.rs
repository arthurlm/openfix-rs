use std::collections::HashMap;

pub type FixFieldItems = HashMap<u32, String>;

const SEP_CHAR: u8 = 0x01;

pub fn split_message_items(data: &[u8]) -> FixFieldItems {
    data.split(|x| *x == SEP_CHAR)
        .filter_map(|field| {
            let fields: Vec<_> = field.splitn(2, |x| *x == '=' as u8).collect();

            match fields[..] {
                [field_id, field_data] => {
                    let field_id = String::from_utf8(field_id.to_vec()).ok()?;
                    let field_id = field_id.parse::<u32>().ok()?;

                    let field_data = String::from_utf8(field_data.to_vec()).ok()?;

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
            HashMap::from_iter(IntoIter::new([(5, "foo".to_string())]))
        );
        assert_eq!(
            split_message_items(b"5=foo\x012631=bar"),
            HashMap::from_iter(IntoIter::new([
                (5, "foo".to_string()),
                (2631, "bar".to_string())
            ]))
        );
        assert_eq!(
            split_message_items(b"\x01\x01\x015=foo\x012631=bar\x01\x01\x01"),
            HashMap::from_iter(IntoIter::new([
                (5, "foo".to_string()),
                (2631, "bar".to_string())
            ]))
        );
    }

    #[test]
    fn test_split_message_items_weird_payload() {
        assert_eq!(
            split_message_items(b"5="),
            HashMap::from_iter(IntoIter::new([(5, "".to_string())]))
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
