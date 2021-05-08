use super::object::ObjectRef;

#[derive(Debug, Clone, Default)]
pub struct Outlines {
    pub first: ObjectRef,
    pub last: ObjectRef,
    pub count: usize,
}

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

impl Outlines {
    pub fn set(&mut self, key: &str, value: &str) {
        match key {
            "/Type" => assert_eq!(value, "/Outlines"),
            "/First" => { self.first = value.parse().unwrap(); },
            "/Last" => { self.last = value.parse().unwrap(); },
            "/Count" => { self.count = value.parse().unwrap(); },
            _ => unreachable!(),
        }
    }
}

impl std::str::FromStr for Outlines { 
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut outlines = Outlines::default();

        for line in s.lines() {
            let mut line = line.trim().splitn(2, ' ');
            let key = line.next().unwrap();
            let value = line.next().unwrap();

            outlines.set(key, value);
        }

        Ok(outlines)
    }
}

impl std::str::FromStr for OutlineItem { 
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

pub fn parse(buf: &[u8]) -> Outlines {
    let start = super::search(buf, b"<<", 3) + 2;
    let end = super::search(buf, b">>", start) - 1;

    let content = String::from_utf8_lossy(&buf[start..end]);
    let content = content.trim();

    content.parse().unwrap()
}