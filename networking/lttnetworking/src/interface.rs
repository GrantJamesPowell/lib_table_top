use futures_util::{Sink, Stream};
use serde::{Deserialize, Serialize};

pub struct ParseError;

pub trait Encoding {
    fn serialize<T>(value: &T) -> Vec<u8>
    where
        T: Serialize;
    fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<T, ParseError>
    where
        T: Deserialize<'a>;
}

#[derive(Debug)]
pub struct ReaderWriter<Read: Stream<Item = Result<Vec<u8>, ParseError>>, Write: Sink<Vec<u8>>> {
    pub read: Read,
    pub write: Write,
}
