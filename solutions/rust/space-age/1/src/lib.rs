// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const EARTH_YEARS_IN_SECONDS: f64 = 31_557_600.0;

const MERCURY_YEARS_IN_SECONDS: f64 = EARTH_YEARS_IN_SECONDS * 0.2408467;
const VENUS_YEARS_IN_SECONDS: f64 = EARTH_YEARS_IN_SECONDS * 0.61519726;
const MARS_YEARS_IN_SECONDS: f64 = EARTH_YEARS_IN_SECONDS * 1.8808158;
const JUPITER_YEARS_IN_SECONDS: f64 = EARTH_YEARS_IN_SECONDS * 11.862615;
const SATURN_YEARS_IN_SECONDS: f64 = EARTH_YEARS_IN_SECONDS * 29.447498;
const URANUS_YEARS_IN_SECONDS: f64 = EARTH_YEARS_IN_SECONDS * 84.016846;
const NEPTUNE_YEARS_IN_SECONDS: f64 = EARTH_YEARS_IN_SECONDS * 164.79132;

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / MERCURY_YEARS_IN_SECONDS
    }
}

impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / VENUS_YEARS_IN_SECONDS
    }
}

impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / EARTH_YEARS_IN_SECONDS
    }
}

impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / MARS_YEARS_IN_SECONDS
    }
}

impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / JUPITER_YEARS_IN_SECONDS
    }
}

impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / SATURN_YEARS_IN_SECONDS
    }
}

impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / URANUS_YEARS_IN_SECONDS
    }
}

impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / NEPTUNE_YEARS_IN_SECONDS
    }
}
