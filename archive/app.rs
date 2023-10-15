use crate::components::navigation::TabState;
use crate::components::datatable::DataTable;


pub struct App<'a> {
    title: &'a str,
    pub tab: TabState<'a>,
    pub table: DataTable<'a>
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App {
            title,
            tab: TabState::new(),
            table: DataTable::new()
        }
    }
}
