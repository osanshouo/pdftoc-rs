use super::object::ObjectRef;

#[derive(Debug, Clone, Default)]
pub struct Trailer {
    /// クロスリファレンス・テーブルのエントリ数
    pub size: usize,
    /// 以前のクロスリファレンス・テーブルの位置を示すオフセット値
    pub prev: Option<usize>,
    /// カタログ(Catalog)ディクショナリ
    pub root: ObjectRef,
    /// 暗号化ディクショナリ
    pub encrypt: Option<ObjectRef>,
    /// 文書情報(Document Information)ディクショナリ
    pub info: ObjectRef,
    /// ２っのバイトストリング（「<」と「>」で囲まれた）で構成されたファイル識別子
    /// 差し当たりこれを連結した単一の `Vec<u8>` として取り扱う.
    pub id: Vec<u8>,
}

impl Trailer {
    pub fn set(&mut self, key: &str, value: &str) -> Result<(), std::num::ParseIntError> {
        match key {
            "/Size" => { self.size = value.parse()?; },
            "/Prev" => { self.prev = Some(value.parse()?); },
            "/Root" => { self.root = value.parse()?; },
            "/Encrypt" => { self.encrypt = Some(value.parse()?); },
            "/Info" => { self.info = value.parse()?; },
            "/ID" => { self.id = value.as_bytes().to_vec(); },
            _ => unreachable!(),
        }

        Ok(())
    }
}


impl std::str::FromStr for Trailer {
    type Err = std::num::ParseIntError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let mut trailer = Trailer::default();

        for line in content.lines() {
            let mut line = line.splitn(2, ' ');
            let key = line.next().unwrap();
            let value = line.next().unwrap();

            trailer.set(key, value)?;
        }

        Ok(trailer)
    }
}


pub(crate) fn parse(buf: &[u8]) -> (Trailer, usize) {
    let n = buf.len();

    // フッターの取得
    let footer = {
        let offset = super::search(buf, b"trailer", n - 512);
        &buf[offset..]
    };
    
    // トレイラーの解析
    let trailer: Trailer = {
        let start = super::search(&footer, b"<<", 9) + 2;
        let end = super::search(&footer, b">>", start) - 1;

        let content = String::from_utf8_lossy(&footer[start..end]);
        let content = content.trim();

        content.parse().unwrap()
    };
    
    // // クロスリファレンスの解析
    let xref: usize = {
        let start = super::search(&footer, b"startxref", 0);
        let footer = String::from_utf8_lossy(&footer[start..]);
        let mut footer = footer.lines();

        assert_eq!(footer.next(), Some("startxref"));
        let xref = footer.next().unwrap().parse::<usize>().unwrap();
        assert_eq!(footer.next(), Some("%%EOF"));
        xref
    };

    (trailer, xref)
}
