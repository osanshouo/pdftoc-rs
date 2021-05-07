#[derive(Debug, Clone)]
pub enum OpenAction {
    Array,
    Dictionary,
}

impl std::str::FromStr for OpenAction { 
    type Err = std::num::ParseIntError;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}
