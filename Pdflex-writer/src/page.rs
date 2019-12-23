use crate::pageDimension::PageDimension;

pub struct Page {
    pub dimension: PageDimension,
    pub index: usize,
    _secret: ()
}

impl Page {
    pub fn new(page_index: usize, dimension: PageDimension) -> Page {
        Page { dimension: dimension, index: page_index, _secret: () }
    }

    fn page_number(&self) -> i32 {
        (self.index + 1) as i32
    }
}