mod animal;
mod human;

use crate::animal::animal::Animal;
use crate::animal::structs::{Cat, Dog};
use crate::human::human::Human;

fn main() {
    //Create a new instance of a cat.
    let mut cat_1 = Cat::new("Lio");

    //Create a new instance of a dog.
    let mut dog_1 = Dog::new("Stella");

    //Set ages
    dog_1.set_age(11.to_owned());
    cat_1.set_age(2.to_owned());

    //Print (execute abstract methods)
    println!("{}",format!("Hello, i am {} and {} years old", cat_1.get_name(), cat_1. get_age()));
    println!("{}",format!("Hello, i am {} and {} years old", dog_1.get_name(), dog_1. get_age()));
    cat_1.sound();
    dog_1.sound();

    let mut humanity = Human::i;

}
