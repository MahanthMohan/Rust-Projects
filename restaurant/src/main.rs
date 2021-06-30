mod sandwich_shop {

    // Implementing std::fmt::Debug trait for the Breakfast struct
    #[derive(std::fmt::Debug)]
    pub struct Breakfast {
        toast: String,
        fruit: String,
    }

    impl Breakfast {
        pub fn create(toast: &str, fruit: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                fruit: String::from(fruit),
            }
        }

        // Mutable borrow here
        pub fn set_toast_type(&mut self, toast_type: &str) {
            self.toast = String::from(toast_type);
        }

        // Mutable borrow here
        pub fn set_fruit(&mut self, fruit: &str) {
            self.fruit = String::from(fruit);
        }
    }
}

pub fn modify_order(meal: &mut sandwich_shop::Breakfast, toast_type: &str, fruit: &str) {
    meal.set_toast_type(toast_type);
    meal.set_fruit(fruit);
}

fn main() {
    let mut meal = sandwich_shop::Breakfast::create("Wheat", "Apple");
    println!("{:#?}", meal);
    println!();
    modify_order(&mut meal, "Italian Bread", "Pineapple");
    println!("{:#?}", meal);
}
