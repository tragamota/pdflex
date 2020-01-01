use crate::cos::costype::COSType;
use crate::cos::cosbase::COSBase;

use std::vec::Vec;

const TRUE: [u8; 4] = [116, 114, 117, 101];
const FALSE: [u8; 5] = [102, 97, 108, 115, 101];

struct COSBoolean {    
    value: bool,
    cos_type: COSType,
    _secret: ()
}

impl COSBase for COSBoolean {
    fn cos_type(&self) -> &COSType {
        &self.cos_type
    }

    fn write(&self) -> Vec<u8> {
        let mut binary_boolean = Vec::new();

        if self.value {
            binary_boolean.extend_from_slice(&TRUE);
        }
        else {
            binary_boolean.extend_from_slice(&FALSE);
        }

        binary_boolean
    }
}

impl COSBoolean {
    pub fn new(value: bool) -> COSBoolean {
        COSBoolean {
            value: value,
            cos_type: COSType::Boolean,
            _secret: ()
        }
    }
}
