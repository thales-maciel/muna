use crate::{protocol::RespValueRef, operations::ReturnValue};

pub enum Command {
    COMMAND,
    GET(String),
    SET(String, String),
    HGET(String, String),
    HSET(String, Vec<String>),
}


impl From<ReturnValue> for RespValueRef {
    fn from(state_res: ReturnValue) -> Self {
        match state_res {
            ReturnValue::Ok => RespValueRef::String("OK".to_string()),
            ReturnValue::Nil => RespValueRef::NullBulkString,
            ReturnValue::StringRes(s) => RespValueRef::BulkString(s),
            ReturnValue::Error(e) => RespValueRef::Failure(e),
        }
    }
}

fn verify_size<T>(v: &[T], size: usize) -> Result<(), String> {
    if v.len() != size {
        return Err("Wrong number of args".to_string());
    }
    Ok(())
}

pub fn translate_array_to_command(array: &[RespValueRef]) -> Result<Command, String> {
    if array.is_empty() {
        return Err("Empty Array".to_string());
    }
    let head = &array[0];
    println!("head: {:?}", head);
    let tail: Vec<&RespValueRef> = array.iter().skip(1).collect();
    println!("tail: {:?}", tail);
    match head {
        RespValueRef::String(s) => match s.to_lowercase().as_ref() {
            "command" => Ok(Command::COMMAND),
            "get" => {
                verify_size(&tail, 1)?;
                let key = tail[0];
                match key {
                    RespValueRef::String(k) => Ok(Command::GET(k.to_string())),
                    _ => Err("Wrong Argument Type".to_string()),
                }
            }
            "set" => {
                verify_size(&tail, 2)?;
                let key = tail[0];
                let value = tail[1];
                match (&key, &value) {
                    (&RespValueRef::String(k), &RespValueRef::String(v)) => {
                        Ok(Command::SET(k.to_string(), v.to_string()))
                    }
                    _ => Err("Wrong Argument Type".to_string()),
                }
            }
            "hget" => {
                verify_size(&tail, 2)?;
                let key = tail[0];
                let value = tail[1];
                match (&key, &value) {
                    (&RespValueRef::String(k), &RespValueRef::String(v)) => {
                        Ok(Command::HGET(k.to_string(), v.to_string()))
                    }
                    _ => Err("Wrong Argument Type".to_string()),
                }
            }
            "hset" => {
                if tail.len() % 2 == 0 || tail.len() < 2 {
                    return Err("Wrong number of args".to_string());
                }
                let key = tail[0];
                let mut pairs: Vec<String> = vec![];

                for value in tail.iter().skip(1) {
                    match &value {
                        &RespValueRef::String(s) => pairs.push(s.to_string()),
                        _ => return Err("Wrong argument type".to_string())
                    }
                }
                
                match (&key, &pairs) {
                    (&RespValueRef::String(k), _) => {
                        Ok(Command::HSET(k.to_string(), pairs))
                    }
                    _ => Err("Wrong Argument Type".to_string()),
                }
            }
            _ => Err("Not implemented".to_string()),
        },
        _ => Err("Not implemented".to_string()),
    }
}
