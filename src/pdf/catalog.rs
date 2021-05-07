use std::collections::HashMap;
use super::{header::PdfVersion, object::ObjectRef};

pub mod page_layout;
pub mod page_mode;
pub mod open_action;

use page_layout::PageLayout;
use page_mode::PageMode;
use open_action::OpenAction;

#[derive(Debug, Clone, Default)]
pub struct Catalog {
    pub version: Option<PdfVersion>,
    pub pages: ObjectRef,
    pub page_layout: Option<PageLayout>,
    pub page_mode: Option<PageMode>,
    pub outlines: Option<ObjectRef>,
    pub threads: Option<ObjectRef>,
    pub open_action: Option<OpenAction>,
    pub acro_form: Option<HashMap<usize, ObjectRef>>,
    pub metadata: Option<ObjectRef>,
}

impl Catalog {
    pub fn set(&mut self, key: &str, value: &str) -> Result<(), std::num::ParseIntError> {
        match key {
            "/Type" => { assert_eq!(value.trim(), "/Catalog"); },
            "/Version" => { self.version = Some(value.parse().unwrap()); },
            "/Pages" => { self.pages = value.parse().unwrap(); },
            "/PageLayout" => { self.page_layout = Some(value.parse().unwrap()); },
            "/PageMode" => { self.page_mode = Some(value.parse().unwrap()); },
            "/Outlines" => { self.outlines = Some(value.parse().unwrap()); },
            _ => unreachable!(),
        }

        Ok(())
    }
}


impl std::str::FromStr for Catalog {
    type Err = std::num::ParseIntError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let mut catalog = Catalog::default();

        for line in content.lines() {
            let mut line = line.trim().splitn(2, ' ');
            let key = line.next().unwrap();
            let value = line.next().unwrap();

            catalog.set(key, value)?;
        }

        Ok(catalog)
    }
}


pub fn parse(buf: &[u8]) -> Catalog {
    let start = super::search(buf, b"<<", 5) + 2;
    let end = super::search(buf, b">>", start) - 1;

    let content = String::from_utf8_lossy(&buf[start..end]);
    let content = content.trim();

    content.parse().unwrap()
}
