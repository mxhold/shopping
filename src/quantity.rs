use num_rational::Rational32;
use regex::Regex;
use std::fmt;
use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
enum QuantityUnit {
    Tablespoon,
    Teaspoon,
    Cup,
    Gram,
    Count,
    Pound,
}

impl fmt::Display for QuantityUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &QuantityUnit::Tablespoon => write!(f, "T"),
            &QuantityUnit::Teaspoon => write!(f, "t"),
            &QuantityUnit::Cup => write!(f, "c"),
            &QuantityUnit::Gram => write!(f, "g"),
            &QuantityUnit::Count => write!(f, "ct"),
            &QuantityUnit::Pound => write!(f, "lb"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RQuantity {
    unit: QuantityUnit,
    value: Rational32,
}

impl RQuantity {
    pub fn new(string: &str) -> RQuantity {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"\A(?P<num>\d+)(/(?P<denom>\d+))?(?P<unit>\w+)\z").unwrap();
        }

        let mut value: Option<Rational32> = None;
        let mut unit: Option<QuantityUnit> = None;

        for caps in RE.captures_iter(string) {
            let num = caps.name("num").expect("missing numerator");
            let num: i32 = num.as_str().parse().expect("numerator must be i32");
            let denom: i32 = match caps.name("denom") {
                Some(denom) => denom.as_str().parse().expect("denominator must be i32"),
                None => 1,
            };

            value = Some(Rational32::new(num, denom));

            unit = match caps.name("unit").expect("missing unit").as_str() {
                "t" => Some(QuantityUnit::Teaspoon),
                "T" => Some(QuantityUnit::Tablespoon),
                "ct" => Some(QuantityUnit::Count),
                "g" => Some(QuantityUnit::Gram),
                "lb" => Some(QuantityUnit::Pound),
                "c" => Some(QuantityUnit::Cup),
                _ => None,
            };
        }

        RQuantity {
            unit: unit.expect("unit failed to parse"),
            value: value.expect("quantity failed to parse"),
        }
    }

    pub fn is_positive(&self) -> bool {
        self.value > Rational32::from_integer(0)
    }
}

impl fmt::Display for RQuantity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.value, self.unit)
    }
}

impl<'a> Add<&'a RQuantity> for RQuantity {
    type Output = RQuantity;

    fn add(self, other: &RQuantity) -> RQuantity {
        // TODO: check that units match
        RQuantity {
            value: self.value + other.value,
            unit: self.unit,
        }
    }
}

impl<'a> Sub<&'a RQuantity> for RQuantity {
    type Output = RQuantity;

    fn sub(self, other: &RQuantity) -> RQuantity {
        RQuantity {
            value: self.value - other.value,
            unit: self.unit,
        }
    }
}
