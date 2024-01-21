use std::fmt::Error;
use uuid::Uuid;
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Write};

const MAX_NUMBER_PAGES: usize = 1000;
const PAGE_SIZE: usize = 4096;

pub struct Page {
    data: Vec<char>,
}

pub struct Pager {
    pages: Vec<Page>,
    persistence: FileHeader,
    freelist: FreeList,
}

/// Contains cursor and space of empty spaces, to be used by VACUUM directive and insert
pub struct FreeList {

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
    pub fn new() -> Pager {
        // push initial pages to reserve space


        match FileHeader::new() {
            Ok(fh) => {
                Pager {
                    pages: Vec::with_capacity(MAX_NUMBER_PAGES),
                    persistence: fh,
                    freelist: FreeList::new(),
                }
            },
            Err(error) => panic!("Unable to create file header: {}", error),
        }

    }

    pub fn insert_row(mut self, data: Vec<char>) -> Result<usize, String> {
        let mut cursor: usize;

        // search free space in order to acomodate data, otherwise, create new page
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

    pub fn insert_row_in_position(&mut self, data: Vec<char>, cursor: usize) {
        self.persistence.file.seek(SeekFrom::Start(cursor as u64));
        return self.persistence.file.write_all(data.as_bytes())?;
    }


    fn create_page(&mut self) -> Result<usize, String> {
        if self.pages.len() < MAX_NUMBER_PAGES {
            let page = Page::new();
            self.pages.push(page);
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

    fn commit_page(mut self, num_page: u64, content: Vec<char>) {

    }

    pub fn flush() {

    }

    // get_page, write_page...

}

impl FileHeader {
    pub fn new() -> Result<FileHeader, String> {
        let mut file_handler = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .truncate(true) // remove this one
            .open(Uuid::max().to_string());

        match file_handler {
            Ok(file) => Ok(FileHeader{
                page_count: 0,
                file: file,
            }),
            Err(error) => Err(format!("error creating file header, {}", error)),
        }
    }
}

impl FreeList {
    pub fn new() -> FreeList {
        FreeList {

        }
    }
}