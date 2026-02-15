use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unit {
    dims: BTreeMap<String, i32>,
}

impl Unit {
    pub fn dimensionless() -> Self {
        Self { dims: BTreeMap::new() }
    }

    pub fn base(symbol: &str) -> Self {
        let mut dims = BTreeMap::new();
        dims.insert(symbol.to_string(), 1);
        Self { dims }
    }

    pub fn is_dimensionless(&self) -> bool {
        self.dims.is_empty()
    }

    pub fn mul(&self, other: &Unit) -> Unit {
        let mut dims = self.dims.clone();
        for (name, power) in &other.dims {
            let entry = dims.entry(name.clone()).or_insert(0);
            *entry += *power;
            if *entry == 0 {
                dims.remove(name);
            }
        }
        Unit { dims }
    }

    pub fn div(&self, other: &Unit) -> Unit {
        let mut dims = self.dims.clone();
        for (name, power) in &other.dims {
            let entry = dims.entry(name.clone()).or_insert(0);
            *entry -= *power;
            if *entry == 0 {
                dims.remove(name);
            }
        }
        Unit { dims }
    }

    pub fn pow(&self, exp: i32) -> Unit {
        if exp == 0 {
            return Unit::dimensionless();
        }

        let mut dims = BTreeMap::new();
        for (name, power) in &self.dims {
            let next = power * exp;
            if next != 0 {
                dims.insert(name.clone(), next);
            }
        }
        Unit { dims }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.dims.is_empty() {
            return write!(f, "1");
        }

        let mut numerators = Vec::new();
        let mut denominators = Vec::new();

        for (name, power) in &self.dims {
            if *power > 0 {
                if *power == 1 {
                    numerators.push(name.clone());
                } else {
                    numerators.push(format!("{}^{}", name, power));
                }
            } else if *power < 0 {
                let abs_power = power.abs();
                if abs_power == 1 {
                    denominators.push(name.clone());
                } else {
                    denominators.push(format!("{}^{}", name, abs_power));
                }
            }
        }

        let numerator = if numerators.is_empty() {
            "1".to_string()
        } else {
            numerators.join("*")
        };

        if denominators.is_empty() {
            return write!(f, "{}", numerator);
        }

        let denominator = denominators.join("*");
        write!(f, "{}/{}", numerator, denominator)
    }
}
