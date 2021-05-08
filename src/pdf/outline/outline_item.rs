use super::super::ObjectRef;

#[derive(Debug, Clone, Default)]
pub struct OutlineItem {
    pub title: String,
    pub parent: ObjectRef,
    pub prev: ObjectRef,
    pub next: ObjectRef,
    pub first: ObjectRef,
    pub last: ObjectRef,
    pub count: i32,
    pub dest: Option<Vec<u8>>,
    pub a: Option<usize>,
    pub c: Option<usize>,
    pub f: Option<u8>,
}

impl std::str::FromStr for OutlineItem { 
    type Err = std::num::ParseIntError;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}


