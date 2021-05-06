#[derive(Debug, Clone, Default)]
pub struct ObjectRef {
    pub object_number: usize,
    pub generation_number: usize,
}

use std::num::ParseIntError;

impl std::str::FromStr for ObjectRef {
    type Err = ParseIntError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut value = value.split_whitespace();
        let object_number = value.next().unwrap().parse()?;
        let generation_number = value.next().unwrap().parse()?;
        assert_eq!(value.next(), Some("R"));

        Ok(ObjectRef {
            object_number, generation_number
        })
    }
}
