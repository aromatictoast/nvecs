use num_traits::AsPrimitive;
use std::ops::*;
//need num-traits = "0.2" under dependencies in cargo.toml

// TODO:
// magnitude,
// cross product,
// exponentiation e.g. squaring an NVec is taking dot with itself
// simd,
// additional operations,
// matrices??,
// tests for ops between multiple types,
// complex/dual numbers - wrap up in a new type

// This is a vector (fixed length array, mathematical vector) with N components
// It implements many useful methods used in vector maths, e.g. vector addition, subtraction, and the dot product

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NVec<T, const N: usize> {
    pub components: [T; N],
}

pub trait NewNVec<T, const N: usize> {
    fn new(components: [T; N]) -> NVec<T, N>;
}

// Multiply the elements to create a new vector
pub trait ElementMul<Rhs = Self>{
    type Output; //by defining this and setting the return type, we can make it far more generic
    fn element_multiply(self, rhs: Rhs) -> Self::Output; 
}



//constructor function
impl<T, const N: usize> NewNVec<T, N> for NVec<T, N> {
    fn new(components: [T; N]) -> Self {
        NVec::<T, N> { components }
    }
}

pub trait Magnitude{
    type Output;
    fn magnitude(self) -> Self::Output; 
    fn mag(self) -> Self::Output; //alias of magnitude
}


// This is an implementation that would mean that whenever an NVec was referenced, it would return its components
// However this isn't actually desireable although it may seem like it on the face - it would lead to cyclical references and impossible implementations for ops e.g. addition
// An NVec is its own type, not just a wrapper, because it implements special operations - it needs to be used as such
// The aim is that you should never have to deal directly with the component array because the type itself should implement everything you need

//impl<T, const N: usize> Deref for NVec<T, N> {
//    type Target = [T; N];
//
//    fn deref(&self) -> &Self::Target {
//        &self.components
//    }
//}

// Here we use the generics to overload the Add operator trait
// We use a spurious extra type V as a primitive (since we'll always get one of those anyway)
// Its guaranteed implementations as a primitive supplied by num-traits means we can convert in to it without trying and unwrapping
// It also means we can convert directly to a useful NVec<V/T, N>
// Otherwise we would have to convert to things like <T as Add<U>>::Output - in effect 'the type that returns when we do this'
// This way, we can just straight up guarantee that everything is fine
// The code is still safe however, as we can only operate on types with a promotion implementation
// In other words, how it should transform when operated on by other types
// See the Promote! stack....
// Therefore the compiler stops us operating on incompatible types, or crucially, NVecs of differing lengths

// Use this general format

// Implement Add for NVec
impl<T, U, V, const N: usize> Add<NVec<U, N>> for NVec<T, N>
where
    T: Copy + AsPrimitive<V> + Promote<U, Output = V>, // This ensures that we convert in to the correct type
    U: Copy + AsPrimitive<V>,
    V: Copy + Add<Output = V> + 'static,
{
    type Output = NVec<V, N>;

    fn add(self, rhs: NVec<U, N>) -> Self::Output {
        let result: [V; N] =
            core::array::from_fn(
                |i: usize|
                self.components[i].as_() + rhs.components[i].as_()
            );

        NVec::new(result)
    }
}

// Subtraction
impl<T, U, V, const N: usize> Sub<NVec<U, N>> for NVec<T, N>
where
    T: Copy + AsPrimitive<V> + Promote<U, Output = V>, // This ensures that we convert in to the correct type
    U: Copy + AsPrimitive<V>,
    V: Copy + Sub<Output = V> + 'static,
{
    type Output = NVec<V, N>;

    fn sub(self, rhs: NVec<U, N>) -> Self::Output {
        let result: [V; N] =
            core::array::from_fn(|i: usize| self.components[i].as_() - rhs.components[i].as_());

        //arr.into();

        NVec::new(result)
    }
}

// Multiplying elements - not the dot product
impl<T, U, V, const N: usize> ElementMul<NVec<U, N>> for NVec<T, N>
where
    T: Copy + AsPrimitive<V> + Promote<U, Output = V>, // This ensures that we convert in to the correct type
    U: Copy + AsPrimitive<V>,
    V: Copy + Mul<Output = V> + 'static,
{
    type Output = NVec<V, N>;

    fn element_multiply(self, rhs: NVec<U, N>) -> Self::Output {
        let result: [V; N] =
            core::array::from_fn(|i: usize| self.components[i].as_() * rhs.components[i].as_());

        //arr.into();

        NVec::new(result)
    }
}

// Multiplying vectors - the dot product
impl<T, U, V, const N: usize> Mul<NVec<U, N>> for NVec<T, N>
where
    T: Copy + AsPrimitive<V> + Promote<U, Output = V>, // This ensures that we convert in to the correct type
    U: Copy + AsPrimitive<V>,
    V: Copy + Mul<Output = V> + Add<Output = V> + Default + 'static, //numerical primitive
{
    type Output = V;

    fn mul(self, rhs: NVec<U, N>) -> Self::Output {
        self
        .components
        .iter()
        .zip(rhs.components.iter())
        .map(|(a, b)| a.as_() * b.as_())
        .fold(V::default(), |acc, elem| acc + elem) //sums the items of an array - fold moves left to right with an accumulator

        
    }
}


impl<T, const N: usize> Magnitude for NVec<T, N> //note this is not define for i64 and u64 as it could cause overflows on it's conversion in to f64
where
    T: Copy + Add<T> + Mul<Output = T>, //
    //V: Copy + AsPrimitive<V> + num_traits::real::Real + Mul<Output = V> + Add<Output = V> + Default + 'static + From<T>, //numerical primitive
    f64: From<T>
{
    type Output = f64; // Shares output type with sqrt()

    fn magnitude(self) -> Self::Output {

        let mut acc: f64 = 0.0; //don't bother with integers, no point since going to root

        for i in 0..N { //N - 1?
            acc = acc + (f64::from(self.components[i]) * f64::from(self.components[i])); //we could use square here but due to pow for ints and powi or powf for floats, simpler just to do this
        };

        acc.sqrt()
    }

    fn mag(self) -> Self::Output {

        let mut acc: f64 = 0.0; //don't bother with integers, no point since going to root

        for i in 0..N { //N - 1?
            acc = acc + (f64::from(self.components[i]) * f64::from(self.components[i])); //we could use square here but due to pow for ints and powi or powf for floats, simpler just to do this
        };

        acc.sqrt()
    }
}


pub trait Promote<T> {
    type Output;
}

// Implement Promote - builds the implementations from the promote! tower below

macro_rules! promote {
    ($t1:ty, $t2:ty => $result:ty) => {
        impl Promote<$t2> for $t1 {
            type Output = $result;
        }
        impl Promote<$t1> for $t2 {
            type Output = $result;
        }
    };
}

// This is a generic that sets any operation involving NVecs of the same T to yield one of that type
impl<T> Promote<T> for T {
    type Output = T;
}

// Integer and float combinations
// These are the rules for what the resultant type should be when an operation is performed between two components
// i.e. NVecs with a different T
// Here we preserve precision, but also minimise extra size where we don't need it
// There may be one or two edge cases where very large ints may get pushed in to an f64 causing an overflow
// Hopefully not...

//This needs to have every possible combination defined
// Integer + Integer => Larger Integer
promote!(i8, i16 => i16);
promote!(i8, i32 => i32);
promote!(i8, i64 => i64);
promote!(i8, i128 => i128);
promote!(i8, u8 => i16);
promote!(i8, u16 => i32);
promote!(i8, u32 => i64);
promote!(i8, u64 => i128);
promote!(i8, u128 => i128);

promote!(i16, i32 => i32);
promote!(i16, i64 => i64);
promote!(i16, i128 => i128);
promote!(i16, u8 => i16);
promote!(i16, u16 => i32);
promote!(i16, u32 => i64);
promote!(i16, u64 => i128);
promote!(i16, u128 => i128);

promote!(i32, i64 => i64);
promote!(i32, i128 => i128);
promote!(i32, u8 => i32);
promote!(i32, u16 => i32);
promote!(i32, u32 => i64);
promote!(i32, u64 => i128);
promote!(i32, u128 => i128);

promote!(i64, i128 => i128);
promote!(i64, u8 => i64);
promote!(i64, u16 => i64);
promote!(i64, u32 => i64);
promote!(i64, u64 => i128);
promote!(i64, u128 => i128);

promote!(i128, u8 => i128);
promote!(i128, u16 => i128);
promote!(i128, u32 => i128);
promote!(i128, u64 => i128);
promote!(i128, u128 => i128);

// Unsigned Integer + Unsigned Integer => Larger Unsigned Integer
promote!(u8, u16 => u16);
promote!(u8, u32 => u32);
promote!(u8, u64 => u64);
promote!(u8, u128 => u128);

promote!(u16, u32 => u32);
promote!(u16, u64 => u64);
promote!(u16, u128 => u128);

promote!(u32, u64 => u64);
promote!(u32, u128 => u128);

promote!(u64, u128 => u128);

// Integer + Float => Float
promote!(i8, f32 => f32);
promote!(i8, f64 => f64);
promote!(i16, f32 => f32);
promote!(i16, f64 => f64);
promote!(i32, f32 => f32);
promote!(i32, f64 => f64);
promote!(i64, f32 => f64); // i64 doesn't fit exactly in f32, so promote to f64
promote!(i64, f64 => f64); // may overflow - watch out
                           //promote!(i128, f32 => f64); // i128 also promotes to f64
                           //promote!(i128, f64 => f64);

promote!(u8, f32 => f32);
promote!(u8, f64 => f64);
promote!(u16, f32 => f32);
promote!(u16, f64 => f64);
promote!(u32, f32 => f32);
promote!(u32, f64 => f64);
promote!(u64, f32 => f64); // u64 doesn't fit exactly in f32, so promote to f64
promote!(u64, f64 => f64);
//promote!(u128, f32 => f64); // u128 also promotes to f64
//promote!(u128, f64 => f64);

// Float + Float => Larger Float
promote!(f32, f64 => f64);

#[cfg(test)]
mod tests{
    use super::*;
    //Need to generate tests for operations betwee NVecs with a different type T

    // This macro assembles 2 NVecs from passed values, applies a function passed as an operator to them, then applies a second function to their components together in turn
    // While making sure to convert to the correct types at any given moment
    // op1 is the operation to appply to the NVecs themselves
    // op2 is the operation to apply to the components of each matched elementwise to get the same effect
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

    //special versions needed here - hardcoded because I am lazy

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

    #[test]

    fn same_type_tests() {
        //declare all the operations, then call the tests to be assembled below
        fn add<A: std::ops::Add<B, Output = C>, B, C>(a: A, b: B) -> C {
            a + b
        }

        fn sub<A: std::ops::Sub<B, Output = C>, B, C>(a: A, b: B) -> C {
            a - b
        }

        fn mul<A: std::ops::Mul<B, Output = C>, B, C>(a: A, b: B) -> C {
            a * b
        }

        fn elemul<A: ElementMul<B, Output = C>, B, C>(a: A, b: B) -> C {
            a.element_multiply(b)
        }

        // Tests for addition
        println!("\n\nTesting for addition\n\n");
        generate_same_type_tests!(
            add, add, 1, 2, 3, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64
        );

        // Tests for subtraction
        println!("\n\nTesting for subtraction\n\n");
        generate_same_type_tests!(
            sub, sub, 1, 2, 3, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64
        );

        // Tests for elementwise multiplication
        println!("\n\nTesting for elementwise multiplication\n\n");
        generate_same_type_tests!(
            elemul, mul, 1, 2, 3, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64
        );

        // Tests for dot product
        println!("\n\nTesting for dot product\n\n");
        generate_same_type_dot_tests!(mul, 1, 2, 3, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);

        println!("\n\nTesting for magnitude\n\n");
        generate_same_type_mag_tests!(1, 2, 3, i8, i16, i32, u8, u16, u32, f32, f64);
    }

    #[test]
    fn cross_type_tests() {
        //generating the tests for operations across types is hard so we'll just do 2 manually

        println!("\n\nTesting for addition\n\n");
        let a: NVec<f64, 3> = NVec {
            components: [1.01 as f64, -2.65 as f64, 3.4 as f64],
        };
        let b: NVec<i32, 3> = NVec {
            components: [4, -5, 200],
        };

        assert_eq!(
            a + b,
            NVec {
                components: [1.01 + 4f64, -2.65 + -5f64, 3.4 + 200f64]
            }
        );

        assert_eq!(
            b + a,
            NVec {
                components: [1.01 + 4f64, -2.65 + -5f64, 3.4 + 200f64]
            }
        );

        println!("\n\nTesting for subtraction\n\n");
        let a: NVec<f64, 3> = NVec {
            components: [1.01 as f64, -2.65 as f64, 3.4 as f64],
        };
        let b: NVec<i32, 3> = NVec {
            components: [4, -5, 200],
        };
        println!("a, b\n");
        assert_eq!(
            a - b,
            NVec {
                components: [1.01 - 4f64, -2.65 - -5f64, 3.4 - 200f64]
            }
        );

        println!("b, a\n");
        assert_eq!(
            b - a,
            NVec {
                components: [-1.01 + 4f64, 2.65 + -5f64, -3.4 + 200f64]
            }
        );

        println!("\n\nTesting for elementwise multiplication\n\n");
        let a: NVec<f64, 3> = NVec {
            components: [1.01 as f64, -2.65 as f64, 3.4 as f64],
        };
        let b: NVec<i32, 3> = NVec {
            components: [4, -5, 200],
        };

        assert_eq!(
            a.element_multiply(b),
            NVec {
                components: [1.01 * 4f64, -2.65 * -5f64, 3.4 * 200f64]
            }
        );

        assert_eq!(
            b.element_multiply(a),
            NVec {
                components: [1.01 * 4f64, -2.65 * -5f64, 3.4 * 200f64]
            }
        );

        println!("\n\nTesting for dot product\n\n");
        let a: NVec<f64, 3> = NVec {
            components: [1.01 as f64, -2.65 as f64, 3.4 as f64],
        };
        let b: NVec<i32, 3> = NVec {
            components: [4, -5, 200],
        };

        assert_eq!(a * b, 1.01 * 4f64 + -2.65 * -5f64 + 3.4 * 200f64);

        assert_eq!(b * a, 1.01 * 4f64 + -2.65 * -5f64 + 3.4 * 200f64);

        println!("\n\nTesting for magnitude\n\n");
        let a: NVec<f64, 3> = NVec {
            components: [1.01 as f64, -2.65 as f64, 3.4 as f64],
        };
        let b: NVec<i32, 3> = NVec {
            components: [4, -5, 3],
        };

        assert_eq!(
            a.mag(),
            (1.01f64 * 1.01f64 + (-2.65f64) * (-2.65f64) + 3.4f64 * 3.4f64).sqrt()
        );

        assert_eq!(
            b.mag(),
            f64::from(4i32 * 4i32 + (-5i32) * (-5i32) + 3i32 * 3i32).sqrt()
        );
    }

}