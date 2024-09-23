#[cfg(test)]
#[path = "./tests.rs"]
mod solution_tests;

pub mod solutions {
    pub fn hoisting() -> i32 {
        println!("Rust does not support hoisting");
        5
    }

    //

    pub fn inc_by_value(mut n: i32) -> i32 {
        n += 1;
        n
    }

    //

    pub struct Num {
        pub n: i32,
    }

    pub fn inc_by_reference(num: &mut Num) {
        num.n += 1
    }

    //

    use std::any::{type_name_of_val, Any};
    use std::collections::HashMap;

    pub fn count_types_in_array(arr: &Vec<Box<dyn Any>>) -> HashMap<&'static str, i32> {
        let mut counter = HashMap::new();

        for elem in arr {
            // Well, it won't work in rust the same as in javascript, because
            // we can't get the type name without downcasting it with specified type.
            // As a result, we would always get "alloc::boxed::Box<dyn core::any::Any>" here.
            let type_name = type_name_of_val(elem);
            *counter.entry(type_name).or_insert(0) += 1;
        }

        counter
    }
}