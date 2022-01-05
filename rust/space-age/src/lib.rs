// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const SECONDS_AS_YEAR: f64 = (60 * 60 * 24) as f64 * 365.25;

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s }
    }
}

pub trait Planet {
    fn arbital_period_rate() -> f64;
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 * (1.0 / (SECONDS_AS_YEAR * Self::arbital_period_rate()))
    }
}

macro_rules! planets {
    [$(($p:ident, $d:expr)), *] => {
        $(impl Planet for $p {
                fn arbital_period_rate() -> f64 {
                    $d
                }
            }
         )*
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

planets![
    (Mercury, 0.2408467),
    (Venus, 0.61519726),
    (Earth, 1.0),
    (Mars, 1.8808158),
    (Jupiter, 11.862615),
    (Saturn, 29.447498),
    (Uranus, 84.016846),
    (Neptune, 164.79132)
];
