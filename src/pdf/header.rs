const HEADER: &'static [u8] = b"%PDF-";

#[derive(Debug, Clone)]
pub struct PdfVersion (u8, u8);

impl PdfVersion {
    pub fn new(buf: &[u8]) -> Self {
        PdfVersion( 
            buf[0] - 48,
            buf[2] - 48
        )
    }
}

pub(crate) fn parse(buf: &[u8]) -> PdfVersion {
    assert_eq!(&buf[..5], HEADER);
    PdfVersion::new(&buf[5..8])
}
