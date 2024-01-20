const MAX_NUMBER_PAGES: usize = 1000;
const PAGE_SIZE: usize = 1024;

pub struct Page {
    data: Vec<char>,
}

pub struct Pager {
    pages: Vec<Page>,
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

}