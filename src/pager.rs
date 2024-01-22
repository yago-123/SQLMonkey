use std::cmp::Ordering;
use uuid::Uuid;
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Write};
use crate::freelist::FreeList;

const MAX_NUMBER_PAGES: usize = 1000;
const PAGE_SIZE: usize = 4096;

pub struct Page {
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
    pub fn new() -> Page {
        Page {
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
            pages: Vec::with_capacity(crate::pager::MAX_NUMBER_PAGES),
            persistence,
            freelist: FreeList::new(),
        })
    }

    pub fn insert_row(&mut self, data: Vec<u8>) -> Result<usize, String> {
        let mut cursor: usize;

        // search free space in order to accommodate data, otherwise, create new page
        if let Some(empty_space_cursor) = self.search_free_space(data.len()) {
            cursor = empty_space_cursor;
        } else if let Ok(new_page_current) = self.create_page() {
            cursor = new_page_current
        } else {
            return Err(String::from("error inserting row"))
        }

        self.insert_row_in_position(data, cursor);
        return Ok(cursor);
    }

    pub fn insert_row_in_position(&mut self, data: Vec<u8>, cursor: usize) -> Result<(), std::io::Error> {
        // check if page is in use and place lock (future)
        self.persistence.file.seek(SeekFrom::Start(cursor as u64))?;
        self.persistence.file.write_all(data.as_ref())?;

        Ok(())
    }

    pub fn delete_row_from_position(&mut self, cursor: usize, size: usize) {
        // just remove from FreeList, no need to overwrite, just use a clustered index


    }

    fn create_page(&mut self) -> Result<usize, String> {
        if self.pages.len() < MAX_NUMBER_PAGES {
            self.pages.push(Page::new());
            return Ok(self.pages.len() - 1)
        }

        return Err(String::from("error creating page, limit of pages reached"))
    }

    fn get_page(self, num: u64) {
        return
    }

    fn search_free_space(&self, size: usize) -> Option<usize> {
        // if does not find any spot in free list
        return None
    }

    fn commit_page(mut self, num_page: u64, content: &[u8]) {

    }

    pub fn flush() {

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