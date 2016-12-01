// #![allow(unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types)]
#![allow(dead_code, non_camel_case_types)]

use protocol::{self, Protocol};
use Transport;
use Result;

#[derive(Debug, Copy, Clone)]
pub enum TApplicationExceptionType {
    UNKNOWN = 0,
    UNKNOWN_METHOD = 1,
    INVALID_MESSAGE_TYPE = 2,
    WRONG_METHOD_NAME = 3,
    BAD_SEQUENCE_ID = 4,
    MISSING_RESULT = 5,
    INTERNAL_ERROR = 6,
    PROTOCOL_ERROR = 7,
    INVALID_TRANSFORM = 8,
    INVALID_PROTOCOL = 9,
    UNSUPPORTED_CLIENT_TYPE = 10,
}

#[derive(Debug)]
pub struct TApplicationException {
    _type: TApplicationExceptionType,
    message: Option<String>,
}

impl TApplicationException {
    pub fn new(_type: TApplicationExceptionType, message: Option<String>) -> TApplicationException {
        TApplicationException { _type: _type, message: message }
    }
}

impl protocol::ThriftTyped for TApplicationException {
    fn typ(&self) -> protocol::Type {
        protocol::Type::Struct
    }
}

impl protocol::Encode for TApplicationException {
    fn encode<P, T>(&self, protocol: &mut P, transport: &mut T) -> Result<()>
        where P: Protocol, T: Transport {
        try!(protocol.write_struct_begin(transport, stringify!(TApplicationException)));
        if protocol::Encode::should_encode(&self.message) {
            try!(protocol.write_field_begin(transport, stringify!(message),
                                            protocol::helpers::typ::<String>(), 1));
            try!(protocol::Encode::encode(&self.message, protocol, transport));
            try!(protocol.write_field_end(transport));
        }
        try!(protocol.write_field_begin(transport, stringify!(type),
                                        protocol::helpers::typ::<i32>(), 2));
        let type_val: i32 = self._type as i32;
        try!(protocol::Encode::encode(&type_val, protocol, transport));
        try!(protocol.write_field_end(transport));

        try!(protocol.write_field_stop(transport));
        try!(protocol.write_struct_end(transport));
        Ok(())
    }
}
