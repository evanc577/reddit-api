use serde::Serialize;

use crate::structs::PageInfo;

pub(crate) trait Request: Serialize {
    fn set_cursor(&mut self, cursor: String);
}

pub(crate) trait Response<T> {
    fn page_info(&self) -> &PageInfo;

    fn items(self) -> Vec<T>;
}
