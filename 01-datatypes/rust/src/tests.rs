#[cfg(test)]
mod solution_tests {
    use super::super::solutions;

    #[test]
    fn test_hoisting() {
        let result = solutions::hoisting();
        assert_eq!(result, 5);
    }

    #[test]
    fn test_inc_by_value() {
        let cases = vec![
            (0, 1),
            (1, 2),
            (-1, 0),
            (10000, 10001),
        ];

        for (input, expected) in cases {
            let result = solutions::inc_by_value(input);
            assert_ne!(result, input, "Result should not be equal to argument");
            assert_eq!(result, expected, "Unexpected result for input {}", input);
        }
    }

    #[test]
    fn test_inc_by_reference() {
        let cases = vec![
            (0, 1),
            (1, 2),
            (-1, 0),
            (10000, 10001),
        ];

        for (input, expected) in cases {
            let mut num = solutions::Num { n: input };
            solutions::inc_by_reference(&mut num);
            assert_eq!(num.n, expected, "Expected Num.n to be {}, got {}", expected, num.n);
        }
    }

    use std::any::Any;

    macro_rules! vec_of_boxes {
        ($($x:expr),*) => {
            vec![$(Box::new($x) as Box<dyn Any>),*]
        };
    }

    #[test]
    fn test_count_types_in_array() {
        let cases = vec![
            (
                vec_of_boxes!(true, true, false),
                vec![("alloc::boxed::Box<dyn core::any::Any>", 3)]
            ),
            (
                vec_of_boxes!(1, true, "hello"),
                vec![("alloc::boxed::Box<dyn core::any::Any>", 3)]
            )
        ];

        for (input, expected) in cases {
            let result = solutions::count_types_in_array(&input);

            for (type_name, count) in expected {
                assert_eq!(
                    result.get(type_name).cloned().unwrap_or(0),
                    count,
                    "Expected {} occurrences of type {}, got {:?}",
                    count, type_name, result
                );
            }
        }
    }
}