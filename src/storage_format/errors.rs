use std::{
    fmt,
};

pub(crate) struct MissmatchHeaderSize {
    pub(crate) required_size: usize,
    pub(crate) received_size: usize,
}

impl fmt::Display for MissmatchHeaderSize {
    fn fmt(&self, f_out: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>{
        let req_size = self.required_size;
        let rec_size = self.received_size;
        write!(f_out, "Uncorrect header size. Required {req_size} | Received {rec_size}")
    }
}