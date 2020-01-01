use std::vec::Vec;
use std::path::Path;
use std::fs::File;
use std::io::prelude::Write;

use crate::page::Page;
use crate::page_dimension::PageDimension;
use crate::document_info::DocumentInfo;


pub struct Document {
    pub info: DocumentInfo,
    pages: Vec<Page>,
    _secret: ()
}

impl Document {
    pub fn new() -> Document {
        Document { 
            info: DocumentInfo::new(), 
            pages: Vec::new(), 
            _secret: ()
        }
    }

    pub fn add_page(&mut self, dimension: PageDimension) -> &mut Page {
        self.pages.push(Page::new(self.pages.len(), dimension));

        let index = self.pages.len();
        &mut self.pages[index]
    }

    pub fn get_pages(&mut self, index: usize) -> Option<&mut Page> {
        self.pages.get_mut(index)
    }

    pub fn remove_page(&mut self, page: Page) {
        self.pages.remove(page.index);
    }

    pub fn total_pages(&self) -> i32 {
        self.pages.len() as i32
    }

    pub fn to_binary() -> Vec<u8> {
        Vec::new()
    }

    pub fn save(&mut self, file_path: &Path) -> std::io::Result<()> {
        let mut document_file = File::create(file_path)?;
        let document_data : Vec<u8> = Document::binary_data(self);

        document_file.write_all(document_data.as_slice())?;
        document_file.flush()
    }

    fn binary_data(&mut self) -> Vec<u8> {
        Vec::new()
    }
}