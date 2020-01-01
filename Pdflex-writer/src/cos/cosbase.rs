
trait CosBase {
    fn cos_type(&self) -> COStype;
    fn write(&self) -> Vec<u8>;
}