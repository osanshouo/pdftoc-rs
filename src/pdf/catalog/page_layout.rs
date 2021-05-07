#[derive(Debug, Clone)]
pub enum PageLayout {
    SinglePage,
    OneColumn,
    TwoColumnLeft,
    TwoColumnRight,
    TwoPageLeft,
    TwoPageRight,
}

impl std::str::FromStr for PageLayout { 
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let field = match s {
            "/SinglePage" => PageLayout::SinglePage,
            "/OneColumn" => PageLayout::OneColumn,
            "/TwoColumnLeft" => PageLayout::TwoColumnLeft,
            "/TwoColumnRight" => PageLayout::TwoColumnRight,
            "/TwoPageLeft" => PageLayout::TwoPageLeft,
            "/TwoPageRight" => PageLayout::TwoPageRight,
            _ => unreachable!(),
        };

        Ok(field)
    }
}
