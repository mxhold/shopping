use std::ops::{Add, Sub};

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Quantity(String);

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
