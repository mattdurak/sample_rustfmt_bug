use std::borrow::Cow;

#[allow(dead_code)]
pub(crate) enum BlockStorageAllocateBlockAsyncCallbackResult {
    Ok = 1,
    AlreadyExists = 2,
    Error = 3,
}

#[allow(dead_code)]
pub struct DecodeError {
    pub msg: Cow<'static, str>,
    stack: Vec<(&'static str, &'static str)>,
}

#[allow(dead_code)]
impl DecodeError {
    pub fn new(msg: impl Into<Cow<'static, str>>) -> Self {
        DecodeError {
            msg: msg.into(),
            stack: Default::default(),
        }
    }

    pub fn extend(mut self, message: &'static str, field: &'static str) -> Self {
        self.stack.push((message, field));
        self
    }
}

#[allow(dead_code)]
#[allow(clippy::uninlined_format_args)]
impl BlockStorageAllocateBlockAsyncCallbackResult {
    fn decode(src: u32) -> Result<Self, DecodeError> {
        let result = match src {
            1 => BlockStorageAllocateBlockAsyncCallbackResult::Ok,
            2 => BlockStorageAllocateBlockAsyncCallbackResult::AlreadyExists,
            3 => BlockStorageAllocateBlockAsyncCallbackResult::Error,
            _ => return Err(DecodeError::new(format!("Decode BlockStorageAllocateBlockAsyncCallbackResult: Unsupported enum value: {}", src))),
        };
        Ok(result)
    }
}
