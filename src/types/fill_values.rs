use crate::types::{FillValues, FillType};

// Possible errors during FillValues creation
#[derive(Debug, Clone)]
pub enum FillValuesError {
    NotEnoughtElements,
    IncorrectFillLimits,
}

impl std::fmt::Display for FillValuesError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FillValuesError::NotEnoughtElements => write!(f, "Number of values to fill must be at least 1"),
            FillValuesError::IncorrectFillLimits => write!(f, "Your border FillLimits must be [min value, <= max value]")
        }
    }
}

impl std::error::Error for FillValuesError {}

impl FillValues {
    pub fn new (fill_values: Vec<Vec<FillType>>, values_smooth: u16, is_preset_ordered: bool) -> 
        Result<FillValues, FillValuesError> 
    {
        for model_fill in &fill_values {
            for fill_value in model_fill {
                match fill_value {
                    FillType::RandomBetween(lower_bound, upper_bound) => {
                        if lower_bound > upper_bound {
                            return Err(FillValuesError::IncorrectFillLimits)
                        };
                    },
                    FillType::ValueFrom(values) => {
                        if values.is_empty() { return Err(FillValuesError::NotEnoughtElements) };
                    }
                }
            }
        }

        Ok(FillValues {
            fill_values,
            values_smooth,
            is_preset_ordered,
        })
    }
}

impl FillValues {
    pub fn fill_values(&self) -> &Vec<Vec<FillType>> {
        &self.fill_values
    }

    pub fn values_smooth(&self) -> u16 {
        self.values_smooth
    }

    pub fn is_preset_ordered(&self) -> bool {
        self.is_preset_ordered
    }
}
