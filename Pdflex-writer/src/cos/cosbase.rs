use crate::cos::costype::COSType;

pub trait COSBase {
    fn cos_type(&self) -> &COSType;
    fn write(&self) -> Vec<u8>;
}