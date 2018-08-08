use num_rational::Rational32;
use regex::Regex;
use std::ops::{Add, Sub};

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Quantity(pub String);

impl<'a> Add<&'a Quantity> for Quantity {
    type Output = Quantity;

    fn add(mut self, other: &Quantity) -> Quantity {
        self.0.push_str("+");
        self.0.push_str(&other.0);
        self
    }
}

impl<'a> Sub<&'a Quantity> for Quantity {
    type Output = Quantity;

    fn sub(mut self, other: &Quantity) -> Quantity {
        self.0.push_str("-");
        self.0.push_str(&other.0);
        self
    }
}

#[derive(Debug, PartialEq, Clone)]
enum QuantityUnit {
    Tablespoon,
    Teaspoon,
    Cup,
    Gram,
    Count,
    Pound,
}

#[derive(Debug, PartialEq, Clone)]
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
}

impl<'a> Add<&'a RQuantity> for RQuantity {
    type Output = RQuantity;

    fn add(mut self, other: &RQuantity) -> RQuantity {
        // TODO: check that units match
        self.value = self.value + other.value;
        self
    }
}

impl<'a> Sub<&'a RQuantity> for RQuantity {
    type Output = RQuantity;

    fn sub(mut self, other: &RQuantity) -> RQuantity {
        // TODO: check that units match
        self.value = self.value - other.value;
        self
    }
}
