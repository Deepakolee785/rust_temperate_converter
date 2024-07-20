#[derive(Debug)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

#[derive(Debug)]
pub struct Temperature {
    pub value: f64,
    pub unit: TemperatureUnit,
}

impl Temperature {
    pub fn new(value: f64, unit: TemperatureUnit) -> Temperature {
        Temperature { value, unit }
    }
    // (F-32)/180 = C/100 = (k-273.15)/100
    pub fn convert(&self, to: TemperatureUnit) -> Temperature {
        let new_value: f64 = match self.unit {
            TemperatureUnit::Celsius => match to {
                TemperatureUnit::Fahrenheit => ((self.value * 180.0) / 100.0) + 32.0,
                TemperatureUnit::Kelvin => self.value + 273.15,
                _ => self.value,
            },
            TemperatureUnit::Fahrenheit => match to {
                TemperatureUnit::Celsius => ((self.value - 32.0) * 100.0) / 180.0,
                TemperatureUnit::Kelvin => ((self.value - 32.0) * 100.0) / 180.0,
                _ => self.value,
            },
            TemperatureUnit::Kelvin => match to {
                TemperatureUnit::Celsius => self.value - 273.15,
                TemperatureUnit::Fahrenheit => ((self.value - 273.15) * 180.0) / 100.0,
                _ => self.value,
            },
        };
        Temperature {
            value: new_value,
            unit: to,
        }
    }

    pub fn display(&self) {
        let unit = match self.unit {
            TemperatureUnit::Celsius => "°C",
            TemperatureUnit::Fahrenheit => "°F",
            TemperatureUnit::Kelvin => "K",
        };
        println!("{:.2}{}", self.value, unit);
    }
}
