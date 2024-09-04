use crate::types::{Borders, BorderType};

// Possible errors during Axis creation
#[derive(Debug, Clone)]
pub enum BorderError {
    IncorrectBordersCount,
}

impl std::fmt::Display for BorderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BorderError::IncorrectBordersCount => write!(f, "Number of borders must be between 1 and 255")
        }
    }
}

impl std::error::Error for BorderError {}

impl Default for Borders {
    fn default() -> Borders {
        Borders {
            number_of_borders: 2,
            border_same_pattern: false,
            borders_type: vec![BorderType::Random, BorderType::Random],
            borders_limits: vec![[5, 10], [15, 20]]
        }
    }
}

impl Borders {
    pub fn new<T> (number_of_borders: T, borders_type: &[BorderType], borders_limits:&[[u32; 2]]) -> Result<Borders, Box<dyn std::error::Error>>
    where 
        T: TryInto<u8>,
    {
        let number_of_borders = match number_of_borders.try_into() {
            Ok(value) => {
                if value == 0 {
                    return Err(Box::new(BorderError::IncorrectBordersCount))
                }
                value
            },
            Err(_) => return Err(Box::new(BorderError::IncorrectBordersCount))
        };

        //TODO: Check border limits
        Ok(Borders {
            border_same_pattern: false,
            number_of_borders,
            borders_type: borders_type.to_vec(),
            borders_limits: borders_limits.to_vec(),
        })
    }
}
