pub mod document;
pub mod page;
pub mod page_dimension;
pub mod document_info;

mod cos;
mod writer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
