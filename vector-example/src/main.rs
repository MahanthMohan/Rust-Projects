#[derive(Debug, Copy, Clone)]
pub struct Vector {
    ihat: f64,
    jhat: f64,
}

impl Vector {
    pub fn create(x: f64, y: f64) -> Vector {
        Vector { ihat: x, jhat: y }
    }
}

static PI: f64 = 3.14159;

#[allow(non_snake_case)]
pub mod VecOps {
    impl crate::Vector {
        pub fn add(&mut self, other: &crate::Vector) {
            self.ihat += other.ihat;
            self.jhat += other.jhat;
        }

        pub fn subtract(&mut self, other: &crate::Vector) {
            self.ihat -= other.ihat;
            self.jhat -= other.jhat;
        }

        pub fn dot(self, other: &crate::Vector) -> f64 {
            (self.ihat * other.ihat) + (self.jhat * other.jhat)
        }

        pub fn nullify(&mut self) {
            self.ihat = 0.0;
            self.jhat = 0.0;
        }

        pub fn magnitude(self) -> f64 {
            let squared_mag = self.ihat.powi(2) + self.jhat.powi(2);
            squared_mag.powf(0.5)
        }

        pub fn calculate_angle(self) -> f64 {
            (self.jhat / self.ihat).atan() * (180.0 / PI)
        }
    }
}

fn main() {
    let a = Vector::create(3.0, 4.0);
    println!("Vector a: {:#?}", a);
    println!();
    let b = Vector::create(5.0, 6.0);
    println!("Vector b: {:#?}", b);
    println!();
    let res = a.dot(&b);
    println!("Dot Product of a and b: {}", res);
    println!("Angle Between a and x axis: {}", a.calculate_angle() as f32);
}
