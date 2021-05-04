use yaml_rust::{Yaml, YamlLoader};
use std::path::Path;
const PARSEERROR: &'static str = "Error: Unexpected YAML structure.";

#[derive(Debug, Clone)]
pub struct Bookmark<'a> {
    title: &'a str,
    level: u32,
    page: &'a str,
}

use std::fmt;
impl<'a> fmt::Display for Bookmark<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, 
            "BookmarkBegin\r\nBookmarkTitle: {}\r\nBookmarkLevel: {}\r\nBookmarkPageNumber: {}\r\n", 
            self.title, self.level, self.page
        )
    }
}

pub fn yaml_to_pdftk(toc: &Path) -> String {
    let toc = std::fs::read_to_string(&toc)
        .expect(&format!("Error: can't read {:?}", &toc));
    let toc: Vec<Yaml> = YamlLoader::load_from_str(&toc)
        .expect("Error: Invalid YAML,");
    
    let mut buf: Vec<Bookmark> = Vec::new();
    parse(&toc, &mut buf);
    let toc = buf.iter()
        .fold(String::new(),|acc, bm| format!("{}{}", acc, bm));    
    
    toc
}

fn parse<'a>(yamls: &'a [Yaml], buf: &mut Vec<Bookmark<'a>>) {
    for entry in yamls.iter() {
        parse_bookmark(entry, buf, 0);
    }
}

fn parse_bookmark<'a>(entry: &'a Yaml, buf: &mut Vec<Bookmark<'a>>, level: u32) {
    match entry {
        Yaml::Array(array) => {
            for entry in array.iter() {
                parse_bookmark(entry, buf, level + 1);
            }
        },
        Yaml::String(title) => {
            let mut title = title.trim().rsplitn(2, ' ');
            let page = title.next().expect(PARSEERROR);
            let bookmark = Bookmark {
                title: title.next().expect(PARSEERROR), level, page,
            };
            buf.push(bookmark);
        },
        _ => std::panic::panic_any(PARSEERROR),
    }
}
