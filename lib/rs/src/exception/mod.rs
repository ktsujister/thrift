// #![allow(unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types)]
#![allow(dead_code, non_camel_case_types)]

use protocol::{self, Protocol};
use Transport;
use Result as ThriftResult;
use std::default::Default;
use std::fmt;

enom! {
  name = TApplicationExceptionType,
  values = [
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
  ],
  default = UNKNOWN
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct TApplicationException {
    _type: TApplicationExceptionType,
    message: Option<String>,
}

pub type Result<T> = ::std::result::Result<T, TApplicationException>;

impl TApplicationException {
    pub fn new(_type: TApplicationExceptionType, message: Option<String>) -> TApplicationException {
        TApplicationException { _type: _type, message: message }
    }
}

impl fmt::Display for TApplicationException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}

impl Default for TApplicationException {
    fn default() -> TApplicationException {
        TApplicationException { _type: TApplicationExceptionType::UNKNOWN, message: None }
    }
}

impl protocol::ThriftTyped for TApplicationException {
    fn typ(&self) -> protocol::Type {
        protocol::Type::Struct
    }
}

impl protocol::Encode for TApplicationException {
    fn encode<P, T>(&self, protocol: &mut P, transport: &mut T) -> ThriftResult<()>
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

impl protocol::Decode for TApplicationException {
    fn decode<P, T>(&mut self, protocol: &mut P, transport: &mut T) -> ThriftResult<()>
        where P: Protocol, T: Transport {
        try!(protocol.read_struct_begin(transport));
        loop {
            let (_, typ, id) = try!(protocol.read_field_begin(transport));

            if typ == protocol::Type::Stop {
                break;
            } else if (typ, id) == (protocol::helpers::typ::<String>(), 1) {
                try!(protocol::Decode::decode(&mut self.message, protocol, transport));
            } else if (typ, id) == (protocol::helpers::typ::<i32>(), 2) {
                try!(protocol::Decode::decode(&mut self._type, protocol, transport));
            } else {
                try!(protocol.skip(transport, typ));
            }

            try!(protocol.read_field_end(transport));
        }

        try!(protocol.read_struct_end(transport));

        Ok(())
    }
}
