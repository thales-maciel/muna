use thiserror::Error;

use crate::{request::Request, operations::OperationResult};

#[derive(PartialEq, Clone, Debug)]
pub enum RespValueRef {
    String(String),
    BulkString(String),
    Failure(String),
    Int(i64),
    Array(Vec<RespValueRef>),
    NullArray,
    NullBulkString,
}

impl From<OperationResult> for RespValueRef {
    fn from(state_res: OperationResult) -> Self {
        match state_res {
            OperationResult::Ok => RespValueRef::String("OK".to_string()),
            OperationResult::Nil => RespValueRef::NullBulkString,
            OperationResult::StringRes(s) => RespValueRef::BulkString(s),
            OperationResult::Error(e) => RespValueRef::Failure(e),
            OperationResult::Int(i) => RespValueRef::Int(i),
        }
    }
}

impl TryInto<Request> for RespValueRef {
    type Error = ();

    fn try_into(self) -> Result<Request, Self::Error> {
        if let RespValueRef::Array(command_refs) = self {
            let mut command_chunks: Vec<String> = vec![];
            for v in command_refs {
                if let RespValueRef::String(s) = v {
                    command_chunks.push(s);
                    continue;
                }
                return Err(());
            }
            return Ok(Request::new(command_chunks));
        }
        Err(())
    }
}

#[derive(Error, Debug)]
pub enum RESPError {
    #[error("Unknown Starting Byte")]
    UnknownStartingByte,
    #[error("Unparseable Int")]
    IntParseFailure,
    #[error("Bad Bulkstring size `{0}`")]
    BadBulkStringSize(i64),
    #[error("Bad Array size `{0}`")]
    BadArraySize(i64),
}

#[derive(Debug)]
struct BufSplit(usize, usize);
impl BufSplit {
    #[inline]
    fn as_slice<'a>(&self, buf: &'a [u8]) -> &'a [u8] {
        &buf[self.0..self.1]
    }
}

#[derive(Debug)]
enum RespBufSplit {
    String(BufSplit),
    Error(BufSplit),
    Int(i64),
    Array(Vec<RespBufSplit>),
    NullArray,
    NullBulkString,
}

impl RespBufSplit {
    fn redis_value(self, buf: &[u8]) -> RespValueRef {
        match self {
            RespBufSplit::String(bfs) => {
                RespValueRef::String(String::from_utf8_lossy(&buf[bfs.0..bfs.1]).to_string())
            }
            RespBufSplit::Error(bfs) => {
                RespValueRef::Failure(String::from_utf8_lossy(&buf[bfs.0..bfs.1]).to_string())
            }
            RespBufSplit::Array(arr) => {
                RespValueRef::Array(arr.into_iter().map(|bfs| bfs.redis_value(buf)).collect())
            }
            RespBufSplit::NullArray => RespValueRef::NullArray,
            RespBufSplit::NullBulkString => RespValueRef::NullBulkString,
            RespBufSplit::Int(i) => RespValueRef::Int(i),
        }
    }
}

type RespResult = Result<Option<(usize, RespBufSplit)>, RESPError>;

fn memchr(needle: u8, haystack: &[u8]) -> Option<usize> {
    haystack.iter().position(|&b| b == needle)
}

fn word(buf: &mut [u8], pos: usize) -> Option<(usize, BufSplit)> {
    if buf.len() <= pos {
        return None;
    }

    memchr(b'\r', &buf[pos..]).and_then(|end| {
        if end + 1 < buf.len() {
            Some((pos + end + 2, BufSplit(pos, pos + end)))
        } else {
            None
        }
    })
}

fn simple_string(buf: &mut [u8], pos: usize) -> RespResult {
    Ok(word(buf, pos).map(|(pos, word)| (pos, RespBufSplit::String(word))))
}

fn error(buf: &mut [u8], pos: usize) -> RespResult {
    Ok(word(buf, pos).map(|(pos, word)| (pos, RespBufSplit::Error(word))))
}

fn int(buf: &mut [u8], pos: usize) -> Result<Option<(usize, i64)>, RESPError> {
    match word(buf, pos) {
        Some((pos, word)) => {
            let s =
                std::str::from_utf8(word.as_slice(buf)).map_err(|_| RESPError::IntParseFailure)?;
            let i = s.parse().map_err(|_| RESPError::IntParseFailure)?;
            Ok(Some((pos, i)))
        }
        None => Ok(None),
    }
}

fn array(buf: &mut [u8], pos: usize) -> RespResult {
    match int(buf, pos)? {
        None => Ok(None),
        Some((pos, -1)) => Ok(Some((pos, RespBufSplit::NullArray))),
        Some((pos, num_elements)) if num_elements >= 0 => {
            let mut values = Vec::with_capacity(num_elements as usize);
            let mut curr_pos = pos;
            for _ in 0..num_elements {
                match parse(buf, curr_pos)? {
                    Some((new_pos, value)) => {
                        curr_pos = new_pos;
                        values.push(value);
                    }
                    None => return Ok(None),
                }
            }
            Ok(Some((curr_pos, RespBufSplit::Array(values))))
        }
        Some((_pos, bad_num_elements)) => Err(RESPError::BadArraySize(bad_num_elements)),
    }
}

fn parse(buf: &mut [u8], pos: usize) -> RespResult {
    if buf.is_empty() {
        return Ok(None);
    }

    match buf[pos] {
        b'+' => simple_string(buf, pos + 1),
        b'-' => error(buf, pos + 1),
        b'$' => bulk_string(buf, pos + 1),
        b':' => resp_int(buf, pos + 1),
        b'*' => array(buf, pos + 1),
        _ => Err(RESPError::UnknownStartingByte),
    }
}

fn resp_int(buf: &mut [u8], pos: usize) -> RespResult {
    Ok(int(buf, pos)?.map(|(pos, int)| (pos, RespBufSplit::Int(int))))
}

fn bulk_string(buf: &mut [u8], pos: usize) -> RespResult {
    match int(buf, pos)? {
        Some((pos, -1)) => Ok(Some((pos, RespBufSplit::NullBulkString))),
        Some((pos, size)) if size >= 0 => {
            let total_size = pos + size as usize;
            if buf.len() < total_size + 2 {
                Ok(None)
            } else {
                let bb = RespBufSplit::String(BufSplit(pos, total_size));
                Ok(Some((total_size + 2, bb)))
            }
        }
        Some((_pos, bad_size)) => Err(RESPError::BadBulkStringSize(bad_size)),
        None => Ok(None),
    }
}

pub fn decode(buf: &mut [u8]) -> Result<Option<RespValueRef>, RESPError> {
    if buf.is_empty() {
        return Ok(None);
    }

    match parse(buf, 0)? {
        Some((pos, value)) => {
            let our_data = buf.split_at(pos).0;
            Ok(Some(value.redis_value(&our_data)))
        }
        None => Ok(None),
    }
}

impl RespValueRef {
    pub fn write_resp_value(&self) -> String {
        let mut return_value = String::new();
        match &self {
            RespValueRef::Failure(e) => {
                return_value.push_str("-");
                return_value.push_str(&e);
                return_value.push_str("\r\n");
            }
            RespValueRef::String(s) => {
                return_value.push_str("+");
                return_value.push_str(&s);
                return_value.push_str("\r\n");
            }
            RespValueRef::BulkString(s) => {
                return_value.push_str("$");
                return_value.push_str(s.len().to_string().as_str());
                return_value.push_str("\r\n");
                return_value.push_str(&s);
                return_value.push_str("\r\n");
            }
            RespValueRef::Array(array) => {
                return_value.push_str("*");
                return_value.push_str(array.len().to_string().as_str());
                return_value.push_str("\r\n");
                for redis_value in array {
                    redis_value.write_resp_value();
                }
            }
            RespValueRef::Int(i) => {
                return_value.push_str(":");
                return_value.push_str(i.to_string().as_str());
                return_value.push_str("\r\n");
            }
            RespValueRef::NullArray => return_value.push_str("*-1\r\n"),
            RespValueRef::NullBulkString => return_value.push_str("$-1\r\n"),
        }

        return_value
    }
}
