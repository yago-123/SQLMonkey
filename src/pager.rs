use std::cmp::Ordering;
use uuid::Uuid;
use std::fs::{File, OpenOptions};
use std::io::{empty, Seek, SeekFrom, Write};
use crate::freelist::FreeList;

const MAX_NUMBER_PAGES: usize = 1000;
const PAGE_SIZE: usize = 4096;

pub struct Page {
    page_num: usize,
    data: Vec<u8>,
}

pub struct Pager {
    pages: Vec<Page>,
    persistence: FileHeader,
    freelist: FreeList,
}

/// Contains file that persists the pages
pub struct FileHeader {
    file: File,
    page_count: u32,
}

impl Page {
    pub fn new(page_num: usize) -> Page {
        Page {
            page_num: page_num,
            data: Vec::with_capacity(PAGE_SIZE),
        }
    }
}

impl Pager {
    pub fn new(datastore: Option<String>) -> Result<Pager, String> {
        // todo: reserve initial pages for metadata

        // Attempt to create a new FileHeader
        let persistence = FileHeader::new(datastore)
            .map_err(|error| format!("unable to start: {}", error))?;

        // Create a new Pager with initialized fields
        Ok(Pager {
            pages: Vec::with_capacity(MAX_NUMBER_PAGES),
            persistence,
            freelist: FreeList::new(),
        })
    }

    pub fn insert_row(&mut self, row: Vec<u8>) -> Result<usize, String> {
        let mut cursor: usize;

        // search free space in order to accommodate data, otherwise, create new page
        if let Some(empty_space_cursor) = self.freelist.retrieve_free_space(row.len()) {
            cursor = empty_space_cursor;
        } else if let Ok(new_page_cursor) = self.create_page() {
            cursor = new_page_cursor;
            // store the remaining page space that has been created as empty space in freelist
            self.freelist.insert_free_space(cursor + row.len(), PAGE_SIZE - row.len());
        } else {
            return Err(String::from("error inserting row not enough space available"))
        }

        self.insert_row_in_position(row, cursor);
        return Ok(cursor);
    }

    pub fn insert_bulk_rows() {

    }

    fn insert_row_in_position(&mut self, data: Vec<u8>, cursor: usize) -> Result<(), std::io::Error> {
        // todo(concurrency): check if page is in use and place lock (future)
        // todo(cache): check if the cache is catched
        self.persistence.file.seek(SeekFrom::Start(cursor as u64))?;
        self.persistence.file.write_all(data.as_ref())?;

        Ok(())
    }

    fn delete_row_from_position(&mut self, cursor: usize, space: usize) {
        // place the space in the freelist
        self.freelist.insert_free_space(cursor, space);
    }

    fn create_page(&mut self) -> Result<usize, String> {
        if self.pages.len() < MAX_NUMBER_PAGES {
            let page = Page::new(self.pages.len());
            self.pages.push(page);
            return Ok(self.pages.len() - 1);
        }

        return Err(String::from("error creating page, limit of pages reached"))
    }

    fn commit_page(mut self, num_page: u64, content: &[u8]) {

    }

    pub fn flush_page() {
        
    }

    pub fn compact() {

    }
}

impl FileHeader {
    pub fn new(datastore: Option<String>) -> Result<FileHeader, String> {
        let mut name = Uuid::new_v4().to_string();
        if let Some(ds_name) = datastore {
            name = ds_name
        }

        let mut file_handler = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .truncate(true) // remove this one
            .open(name);

        match file_handler {
            Ok(file) => Ok(FileHeader{
                page_count: 0,
                file: file,
            }),
            Err(error) => Err(format!("error creating file header, {}", error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_row() {
        let mut pager: Pager;
        if let Ok(ret) = Pager::new(Some("test_file".to_string())) {
            pager = ret;
        } else {
            panic!("pager have not been created successfully")
        }

        let data: Vec<u8>= vec![1, 2, 3, 4, 5];
        // test that new page is created if free space is not available
        assert_eq!(pager.insert_row(data), Ok(0 as usize));
        assert_eq!(pager.pages.len(), 1);

        // test that after new page is created, space is filled into newly created space
        let data2: Vec<u8>= vec![6, 7, 8, 9, 10];
        assert_eq!(pager.insert_row(data2), Ok(5 as usize));
        assert_eq!(pager.pages.len(), 1);
    }

    #[test]
    fn test_insert_bulk_rows() {

    }

    #[test]
    fn test_insert_row_in_position() {
        let mut pager: Pager;
        if let Ok(ret) = Pager::new(Some("test_file2".to_string())) {
            pager = ret;
        } else {
            panic!("pager have not been created successfully")
        }

        let data: Vec<u8>= vec![1, 2, 3, 4, 5];
        //pager.insert_row_in_position(data, 0)
    }

    #[test]
    fn test_delete_row_from_position() {

    }

    #[test]
    fn test_create_page() {

    }
}