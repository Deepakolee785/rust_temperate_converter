use crate::models::{Temperature, TemperatureUnit};

pub enum TemperatureConversionOption {
    CelsiusToFahrenheit,
    CelsiusToKelvin,

    FahrenheitToCelsius,
    FahrenheitToKelvin,

    KelvinToCelsius,
    KelvinToFahrenheit,

    Exit,
    Invalid,
}

impl TemperatureConversionOption {
    pub fn from_u32(value: u32) -> Self {
        match value {
            1 => Self::CelsiusToFahrenheit,
            2 => Self::CelsiusToKelvin,
            3 => Self::FahrenheitToCelsius,
            4 => Self::FahrenheitToKelvin,
            5 => Self::KelvinToCelsius,
            6 => Self::KelvinToFahrenheit,
            7 => Self::Exit,
            _ => Self::Invalid,
        }
    }

    pub fn convert_temperature(&self, value: f64) -> Temperature {
        match *self {
            Self::CelsiusToFahrenheit => Temperature::new(value, TemperatureUnit::Celsius)
                .convert(TemperatureUnit::Fahrenheit),
            Self::CelsiusToKelvin => {
                Temperature::new(value, TemperatureUnit::Celsius).convert(TemperatureUnit::Kelvin)
            }
            Self::FahrenheitToCelsius => Temperature::new(value, TemperatureUnit::Fahrenheit)
                .convert(TemperatureUnit::Celsius),
            Self::FahrenheitToKelvin => Temperature::new(value, TemperatureUnit::Fahrenheit)
                .convert(TemperatureUnit::Kelvin),
            Self::KelvinToCelsius => {
                Temperature::new(value, TemperatureUnit::Kelvin).convert(TemperatureUnit::Celsius)
            }
            Self::KelvinToFahrenheit => Temperature::new(value, TemperatureUnit::Kelvin)
                .convert(TemperatureUnit::Fahrenheit),
            _ => unreachable!(),
        }
    }
}
