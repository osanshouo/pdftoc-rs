use std::collections::HashMap;
use super::{header::PdfVersion, object::ObjectRef};

#[derive(Debug, Clone)]
pub enum PageLayout {
    SinglePAge,
    OneColumn,
    TwoColumnLeft,
    TwoColumnRight,
    TwoPageLeft,
    TwoPageRight,
}

#[derive(Debug, Clone)]
pub enum PageMode {
    UseNone,
    UseOutlines,
    UseThumbs,
    FullScreen,
    UseOC,
    UseAttachements,
}

#[derive(Debug, Clone)]
pub enum OpenAction {
    Array,
    Dictionary,
}

#[derive(Debug, Clone)]
pub struct Catalog {
    pub version: PdfVersion,
    pub pages: HashMap<usize, usize>,
    pub page_layout: Option<PageLayout>,
    pub page_mode: Option<PageMode>,
    pub outlines: Option<ObjectRef>,
    pub threads: Option<ObjectRef>,
    pub open_action: Option<OpenAction>,
    pub acro_form: Option<HashMap<usize, ObjectRef>>,
    pub metadata: Option<ObjectRef>,
}

pub fn parse(_buf: &[u8]) -> Catalog {
    unimplemented!()
}
