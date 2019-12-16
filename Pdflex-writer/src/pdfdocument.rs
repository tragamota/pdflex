use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;
use self::page;

mod document {

    struct PdfDocument {

    }

    impl PdfDocument {
        pub fn add_page(page: &Pdfpage) {

        }

        pub fn remove_page(page: &Pdfpage) {

        }

        fn save(path: &Path) -> std::io::Result<()> {
            let mut file = File::create(path)?;
            file.write_all(b"Hello World")?;
            OK(())
        }

        fn binary() -> Vec<u8> {
            let mut data = Vec::new();

            data
        }
    }
}