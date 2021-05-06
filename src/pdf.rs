pub mod header;
pub mod object;
pub mod trailer;
pub mod dictionary;

pub fn parse(buf: &[u8]) {
    // ヘッダーの解析
    let _pdf_version = dbg!(header::parse(buf));

    // フッターの解析
    let (trailer, xref) = trailer::parse(buf);
    dbg!(trailer);
    dbg!(xref);
}

pub(crate) fn search(buf: &[u8], pat: &[u8], start: usize) -> usize {
    let mut idx = start;

    while &buf[idx..idx+pat.len()] != pat {
        idx += 1;
    }

    idx
}


#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn parse_test() {
        let buf = std::fs::read("./experiment/HelloWorld.pdf").unwrap();
        parse(&buf);

        let buf = std::fs::read("./experiment/HelloWorld.toc.pdf").unwrap();
        parse(&buf);

        let buf = std::fs::read("./experiment/Weinberg - The Quantum Theory of Fields - Volume 3 Supersymmetry.toc.pdf").unwrap();
        parse(&buf);
    }
}
