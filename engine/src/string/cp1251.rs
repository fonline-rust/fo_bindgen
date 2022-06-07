use super::{Encoding, Cow, CStr};

pub struct CP1251;

impl Encoding for CP1251 {
    fn encode(str: Cow<str>) -> Cow<[u8]> {
        utf8_to_cp1251(str)
    }
    fn decode(bytes: &[u8]) -> Cow<str> {
        cp1251_to_utf8(bytes)
    }
}

fn cp1251_to_utf8(bytes: &[u8]) -> Cow<str> {
    use encoding_rs::*;

    let (cow, encoding_used, had_errors) = WINDOWS_1251.decode(bytes);
    assert_eq!(encoding_used, WINDOWS_1251);
    assert!(!had_errors);
    cow
}

fn utf8_to_cp1251(string: Cow<str>) -> Cow<[u8]> {
    use encoding_rs::*;

    let (cow, encoding_used, had_errors) = WINDOWS_1251.encode(&string);
    assert_eq!(encoding_used, WINDOWS_1251);
    assert!(!had_errors);
    {
        let _ = CStr::from_bytes_with_nul(cow.as_ref()).expect("Null terminated cp1251 string");
    }
    if let Cow::Owned(owned) = cow {
        Cow::Owned(owned)
    } else {
        match string {
            Cow::Borrowed(str) => Cow::Borrowed(str.as_bytes()),
            Cow::Owned(string) => Cow::Owned(string.into_bytes()),
        }
    }
}
