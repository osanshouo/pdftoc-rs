#[derive(Debug, Clone)]
pub enum PageMode {
    UseNone,
    UseOutlines,
    UseThumbs,
    FullScreen,
    UseOC,
    UseAttachments,
}

impl std::str::FromStr for PageMode { 
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let field = match s {
            "/UseNone" => PageMode::UseNone,
            "/UseOutlines" => PageMode::UseOutlines,
            "/UseThumbs" => PageMode::UseThumbs,
            "/FullScreen" => PageMode::FullScreen,
            "/UseOC" => PageMode::UseOC,
            "/UseAttachments" => PageMode::UseAttachments,
            _ => unreachable!(),
        };

        Ok(field)
    }
}
