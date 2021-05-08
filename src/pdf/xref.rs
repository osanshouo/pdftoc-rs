use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct XrefEntry {
    pub object_number: usize,
    pub offset: usize,
    pub generation_number: usize,
    pub in_use: bool,
}

pub fn parse(buf: &[u8]) -> HashMap<usize, XrefEntry> {
    assert_eq!(&buf[..4], b"xref");
    let start = seek_until(buf, 4, b'\n') + 1;
    let sp1 = seek_until(buf, start, b' ');
    let sp2 = seek_till(buf, sp1+1, |u| u != b' ');
    let end = seek_till(buf, sp2, |u| u == b' ' || u == b'\r' || u == b'\n');
    let obj_num_start: usize = String::from_utf8_lossy(&buf[start..sp1]).parse().unwrap();
    let obj_num_end: usize = String::from_utf8_lossy(&buf[sp2..end]).parse().unwrap();

    let mut idx = seek_until(buf, end, b'\n') + 1;
    let mut xref_table = HashMap::new();

    for obj_num in obj_num_start..obj_num_end {
        xref_table.insert(obj_num, XrefEntry {
            object_number: obj_num,
            offset: String::from_utf8_lossy(&buf[idx..idx+10]).parse().unwrap(),
            generation_number: String::from_utf8_lossy(&buf[idx+11..idx+16]).parse().unwrap(),
            in_use: buf[idx+17] == b'n',
        });

        idx += 20;
    }

    xref_table
}

fn seek_until(buf: &[u8], start: usize, end: u8) -> usize {
    let mut seeker = start;

    while buf[seeker] != end {
        seeker += 1;
    }

    seeker
}

fn seek_till<F>(buf: &[u8], start: usize, condition: F) -> usize where F: Fn(u8) -> bool {
    let mut seeker = start;

    while !condition(buf[seeker]) { seeker += 1; }

    seeker
}
