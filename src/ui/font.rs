#[derive(Clone)]
pub struct FontData {
    pub data: Vec<u8>,
    pub size: u32,
}
impl FontData {
    pub fn new(data: Vec<u8>) -> FontData {
        let size = data.len() as u32;
        return FontData {
            data,
            size,
        }
    }
}