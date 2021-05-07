const HEADER: &'static [u8] = b"%PDF-";

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct PdfVersion (u8, u8);

impl PdfVersion {
    pub fn new(buf: &[u8]) -> Self {
        PdfVersion( 
            buf[0] - 48,
            buf[2] - 48
        )
    }
}

impl std::str::FromStr for PdfVersion {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_eq!(s.len(), 3);
        let b = s.as_bytes();
        Ok(PdfVersion(
            b[0] - 48,
            b[2] - 48
        ))
    }
}

pub(crate) fn parse(buf: &[u8]) -> PdfVersion {
    assert_eq!(&buf[..5], HEADER);
    PdfVersion::new(&buf[5..8])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let s = "1.7";
        let answer = PdfVersion(1, 7);
        assert_eq!(PdfVersion::new(s.as_bytes()), answer);
        assert_eq!(s.parse::<PdfVersion>().unwrap(), answer);
    }
}
