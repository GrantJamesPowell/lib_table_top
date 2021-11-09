use crate::interface::{ParseError, ReaderWriter};
use crate::User;
use futures_util::{Sink, Stream};

pub async fn authorize<Read, Write, Encoding>(
    _encoding: Encoding,
    _io: ReaderWriter<Read, Write>,
) -> Option<(User, ReaderWriter<Read, Write>)>
where
    Read: Stream<Item = Result<Vec<u8>, ParseError>>,
    Write: Sink<Vec<u8>>,
{
    todo!()
}
