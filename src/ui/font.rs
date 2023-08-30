#[derive(Clone)]
pub struct FontData {
    pub data: Vec<u8>,
    pub size: usize,
}
impl FontData {
    pub fn new(
        data: Vec<u8>
    ) -> FontData {
        let size = data.len();
        return FontData {
            data,
            size,
        }
    }
}