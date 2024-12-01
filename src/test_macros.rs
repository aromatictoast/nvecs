#[cfg(test)]

// Each macro needs #[macro_export] so it can be used by the tests in each implementation file

//Need to generate tests for operations betwee NVecs with a different type T

// This macro assembles 2 NVecs from passed values, applies a function passed as an operator to them, then applies a second function to their components together in turn
// While making sure to convert to the correct types at any given moment
// op1 is the operation to appply to the NVecs themselves
// op2 is the operation to apply to the components of each matched elementwise to get the same effect

#[macro_export]
macro_rules! generate_same_type_tests {
    ($op1:ident, $op2:ident, $val1:expr, $val2:expr, $val3:expr, $($type:ty), *) => {
        $(
            let a = NVec{components: [$val1 as $type, $val2 as $type, $val3 as $type]};
            let b = NVec{components: [$val1 as $type, $val2 as $type, $val3 as $type]};
            let c = NVec{components: [$op2($val1 as $type, $val1 as $type), $op2($val2 as $type, $val2 as $type), $op2($val3 as $type, $val3 as $type)]};
            let d = $op1(a, b);
            let e = $op1(b, a);
            println!("A: {:?}\nB: {:?}\n\nC: {:?}\nA op B: {:?}\nB op A: {:?}\n\n\n", a, b, c, d, e);


            assert_eq!(d, c);
            assert_eq!(e, c);
        )*

    };
}
//pub(crate) use generate_same_type_tests;

//special versions needed here - hardcoded because I am lazy
#[macro_export]
macro_rules! generate_same_type_dot_tests {
    ($op1:ident, $val1:expr, $val2:expr, $val3:expr, $($type:ty), *) => {
        $(
            let a = NVec{components: [$val1 as $type, $val2 as $type, $val3 as $type]};
            let b = NVec{components: [$val1 as $type, $val2 as $type, $val3 as $type]};
            let c = $val1 as $type * $val1 as $type + $val2 as $type * $val2 as $type + $val3 as $type * $val3 as $type;
            let d = $op1(a, b);
            let e = $op1(b, a);
            println!("A: {:?}\nB: {:?}\n\nC: {:?}\nA op B: {:?}\nB op A: {:?}\n\n\n", a, b, c, d, e);


            assert_eq!(d, c);
            assert_eq!(e, c);
        )*

    };
}

#[macro_export]
macro_rules! generate_same_type_mag_tests {
    ($val1:expr, $val2:expr, $val3:expr, $($type:ty), *) => {
        $(
            let a = NVec{components: [$val1 as $type, $val2 as $type, $val3 as $type]};
            let b = ($val1 as $type * $val1 as $type + $val2 as $type * $val2 as $type + $val3 as $type * $val3 as $type) as f64;
            let c = a.mag();

            println!("A: {:?}\n|A|: {:?}\nB: {:?}", a, c, b);


            assert_eq!(c, b.sqrt()); //probably don't need both but why not
            assert_eq!(b.sqrt(), c);
        )*

    };
}

#[macro_export]
macro_rules! generate_same_type_cross_tests {
    ($op1:ident, $val1:expr, $val2:expr, $val3:expr, $($type:ty), *) => {
        $(
            let a = NVec{components: [$val1 as $type, $val2 as $type, $val3 as $type]};
            let b = NVec{components: [$val3 as $type, $val2 as $type, $val1 as $type]}; //swapped around!
            let c = NVec{components: [
                                    $val2 as $type * $val1 as $type - $val3 as $type * $val2 as $type,
                                    $val3 as $type * $val3 as $type - $val1 as $type * $val1 as $type,
                                    $val1 as $type * $val2 as $type - $val2 as $type * $val3 as $type 
                                    ]};
            let d = $op1(a, b);
            let e = $op1(b, a);

            println!("A: {:?}\nB: {:?}\n\nC: {:?}\nA op B: {:?}\n\n\n", a, b, c, d);


            assert_eq!(d, c);
            println!("D: {:?}\nE: {:?}\n\n\n", d, e);
            assert_eq!(e * -1, c * 1); //by multiplying c by 1, we make sure we are always working with at least i32 in the final comparison, since that is the default for macros
            assert_eq!(-1 * e, 1 * c);

            // Would expect assert_eq!(e * -1, c); to work, however this generates a type mismatch
            // This is because, unlike in normal usage where the compiler can infer that -1 should be of a type that matches or promotes up to the components of what it's multiplying to
            // In a macro, it assumes the default type of i32
            // Which promotes the components of e up to an i32
            // Which means that it no longer matches c, which is constructed out of a varying type across the test

            //assert_eq!(-1 * e, c);
        )*

    };
}
