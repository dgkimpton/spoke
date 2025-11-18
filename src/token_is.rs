use crate::{error::ProcessingError, token::Token};

pub(crate) enum TokenIs {
    Consumed,
    ConsumedAndFinished,
    Rejected(Token),
    FailedProcessing(ProcessingError),
}

impl TokenIs {
    pub(crate) fn failed(msg:String, at:Token) -> Self {
        Self::FailedProcessing(ProcessingError::new(msg, at))
    }
    
    pub(crate) fn failed_at_end(msg:String) -> Self {
        Self::FailedProcessing(ProcessingError::new(msg, Token::EndOfStream))
    }
}
