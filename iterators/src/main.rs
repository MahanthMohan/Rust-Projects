#[derive(PartialEq, Debug)]
pub struct Bike {
    name: String,
    size: f32,
}

impl Bike {
    pub fn new(name: String, size: f32) -> Bike {
        Bike {
            name,
            size,
        }
    }
}

pub fn filter_by_size(bikes: Vec<Bike>, target_size: f32) -> Vec<Bike> {
    bikes.into_iter().filter(|b| b.size == target_size).collect()
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
pub fn test_iterator() {
    let mut bikes: Vec<Bike> = Vec::new();
    bikes.push(Bike::new(String::from("Trek Marlin 5"), 29.0));
    bikes.push(Bike::new(String::from("Giant Boulder"), 26.5));
    bikes.push(Bike::new(String::from("Kent 700C Walmart Bike"), 29.0));
    let matches = filter_by_size(bikes, 26.5);
    for m in matches.into_iter() {
        assert_eq!(m.size, 26.5);
    }
}
}
