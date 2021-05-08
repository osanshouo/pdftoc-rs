pub mod header;
pub mod trailer;
pub mod xref;
pub mod catalog;
pub mod outline;
pub mod object;
pub mod dictionary;

use object::ObjectRef;
use xref::XrefEntry;

pub fn parse(buf: &[u8]) {
    // ヘッダーを解析し PDF バージョンを取得
    let pdf_version = header::parse(buf);
    eprintln!("{:?}", pdf_version);

    // フッターを解析しトレイラーとクロスリファレンステーブルのオフセットを取得
    let (trailer, xref_offset) = trailer::parse(buf);
    
    // クロスリファレンステーブルを取得
    let xref_table = xref::parse(&buf[xref_offset..]);
    
    
    // カタログを取得
    let catalog = {
        let catalog_ref: &ObjectRef = &trailer.root;
        let xref: &XrefEntry = &xref_table[&catalog_ref.object_number];
        catalog::parse(&buf[xref.offset..])
    };
    eprintln!("{:?}", &catalog);

    if let Some(outlines) = catalog.outlines {
        eprintln!("Note: Input PDF already have ToC. PDFToC replace it in output PDF.");

        let xref = &xref_table[&outlines.object_number];
        let outlines = outline::parse(&buf[xref.offset..]);
        eprintln!("{:?}", outlines);
    }
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

        // let buf = std::fs::read("./experiment/HelloWorld.toc.pdf").unwrap();
        // parse(&buf);

        let buf = std::fs::read("./experiment/Weinberg - The Quantum Theory of Fields - Volume 3 Supersymmetry.toc.pdf").unwrap();
        parse(&buf);
    }
}
