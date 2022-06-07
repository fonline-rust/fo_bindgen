use std::{ffi::CStr, borrow::Cow};

#[cfg(feature = "cp1251")]
mod cp1251;
#[cfg(feature = "cp1251")]
pub use cp1251::CP1251;

mod map_chars;
pub use map_chars::MapChars;

pub trait StringAPI<C: Encoding, S> {
    fn allocate_engine_string(&self, bytes: &[u8]) -> S;
    fn engine_string_to_bytes(&self, str: &S) -> &[u8];
    fn char_mapper(&self) -> MapChars {
        map_chars::MAP_CHAR_FORP
    }
    fn new_engine_string(&self, str: &str) -> S {
        let mapped = map_chars::map_chars(Cow::Borrowed(str), self.char_mapper(), false);
        let encoded = C::encode(mapped);
        self.allocate_engine_string(&encoded)
    }
    fn from_engine_string(&self, str: &S) -> String {
        let bytes = self.engine_string_to_bytes(str);
        let decoded = C::decode(bytes);
        let mapped = map_chars::map_chars(decoded, self.char_mapper(), true);
        mapped.into_owned()
    }
}

pub trait Encoding {
    fn encode(str: Cow<str>) -> Cow<[u8]>;
    fn decode(bytes: &[u8]) -> Cow<str>;
}

pub struct UTF8;

impl Encoding for UTF8 {
    fn encode(str: Cow<str>) -> Cow<[u8]> {
        match str {
            Cow::Borrowed(str) => Cow::Borrowed(str.as_bytes()),
            Cow::Owned(str) => Cow::Owned(str.into_bytes())
        }        
    }
    fn decode(bytes: &[u8]) -> Cow<str> {
        String::from_utf8_lossy(bytes)
    }
}
