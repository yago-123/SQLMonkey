use uuid::Uuid;

const MAX_NUMBER_PAGES: usize = 1000;
const PAGE_SIZE: usize = 4096;

pub struct Page {
    data: Vec<char>,
}

pub struct Pager {
    pages: Vec<Page>,
}

pub struct FileHeader {
    path: Uuid,
    page_size: u32,
}

impl Page {
    pub fn new(self) -> Page {
        Page {
            data: Vec::with_capacity(PAGE_SIZE),
        }
    }
}

impl Pager {
    pub fn new() -> Pager {
        Pager {
            // make this grow dynamically
            pages: Vec::with_capacity(MAX_NUMBER_PAGES),
        }
    }

    // get_page, write_page...

}

impl FileHeader {
    pub fn new() -> FileHeader {
        FileHeader {
            page_size: 0,
            path: Uuid::max(),
        }
    }
}