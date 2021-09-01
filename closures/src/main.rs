use std::cmp::Ordering;

struct Cacher<T>
where
    T: Fn(u8) -> u8,
{
    calculation: T,
    argument: u8,
    value: Option<u8>
}

impl<T> Cacher<T>
where
    T: Fn(u8) -> u8,
{
    pub fn new(calculation: T) -> Self {
        Self {
            calculation,
            argument: 0,
            value: None,
        }
    }

    pub fn value(&mut self, arg: u8) -> u8 {
        match self.value {
            Some(v) => {
                match self.argument.cmp(&arg) {
                    Ordering::Equal => {
                        v
                    },
                    // Not equal (Unique argument)
                    _ => {
                        let v = (self.calculation)(arg);
                        self.value = Some(v);
                        v
                    }
                }
            },
            None => {
                self.argument = arg;
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        } 
    }
}

#[test]
fn test_values() {
    let mut c = Cacher::new(|v| v * v);
    c.value(1);
    assert_eq!(c.value(5), 25);
}
