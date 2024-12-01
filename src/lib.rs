use num_traits::AsPrimitive;
use std::ops::*;
//need num-traits = "0.2" under dependencies in cargo.toml

// TODO:
// figure out how to implement for two types
//MAKE PARAMETERS IN TO REFERENCES FOR THE ONES WHERE IT IS NATURAL
// exponentiation e.g. squaring an NVec is taking dot with itself
// simd + inlining ( #[inline] ),
// additional operations,
// matrices??,
// tests for ops between multiple types,
// complex/dual numbers - wrap up in a new type

// This is a vector (fixed length array, mathematical vector) with N components
// It implements many useful methods used in vector maths, e.g. vector addition, subtraction, and the dot product

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NVec<T: Numerical, const N: usize> {
    pub components: [T; N],
}
pub trait IsNvec{} //type signifier to help with different implementations of operation orders, e.g. 1 * a, a * 1
impl<T: Numerical, const N: usize> IsNvec for NVec<T, N> {} // a local trait to satisfy orphan rule things - impl mul<NVec<T, N>> for U (where nlkdslkdsgnlk)

//used to define multiple implementations sharing an op, e.g. multiplication
//this defines any numerical primitive we support
//super-trait - it requires that all the other listed traits be implemented to be able to be implemented itself
pub trait Numerical: Add<Output = Self> + Mul<Output = Self> + Sub<Output = Self> + Div<Output = Self> + Copy + PartialEq + PartialOrd {}

//implements the Numerical trait - it defines all supported numerical primitive types

macro_rules! numerical {
    ($($type:ty), *) => {
        $(
            impl Numerical for $type {}
        )*
    };
}

numerical!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64);
// This is the list of all number types supported
// If you want some more, add them here



pub trait NewNVec<T: Numerical, const N: usize> {
    fn new(components: [T; N]) -> NVec<T, N>;
}

//constructor function
impl<T: Numerical, const N: usize> NewNVec<T, N> for NVec<T, N> {
    fn new(components: [T; N]) -> Self {
        NVec::<T, N> { components }
    }
}

//impl<T: Numerical, const N: usize> std::fmt::Display for NVec<T, N> {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        write!(f, "{:?}", self.components)
//    }
//} 






// Operations //


// Multiply the elements to create a new vector
pub trait ElementMul<Rhs = Self>{
    type Output; //by defining this and setting the return type, we can make it far more generic
    fn element_multiply(self, rhs: Rhs) -> Self::Output; 
}

pub trait Magnitude{
    type Output;
    fn magnitude(self) -> Self::Output; 
    fn mag(self) -> Self::Output; //alias of magnitude
}

pub trait CrossProduct<Rhs = Self>{
    type Output; 
    fn cross(self, rhs: Rhs) -> Self::Output; 
}


// Implementations

// split up implementations for better readability
pub mod add;
pub mod sub;
pub mod element_multiply;
pub mod mul;
pub mod cross;
pub mod mag;

pub mod test_macros; //the tests should be in each implementation file, but they use a common set of macros linked here






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






// Promotion System for Generic Numerical Primitives //


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

macro_rules! promote_self { //for promotion of self to self e.g. i8 to i8 - needed for implementations that cross types e.g. mul by nvec for nvec or mul by numerical for nvec etc etc
    ($t1:ty, $t2:ty => $result:ty) => {
        impl Promote<$t2> for $t1 {
            type Output = $result;
        } 
    };
}


promote_self!(i8, i8 => i8);
promote_self!(i16, i16 => i16);
promote_self!(i32, i32 => i32);
promote_self!(i64, i64 => i64);
promote_self!(i128, i128 => i128);

promote_self!(u8, u8 => u8);
promote_self!(u16, u16 => u16);
promote_self!(u32, u32 => u32);
promote_self!(u64, u64 => u64);
promote_self!(u128, u128 => u128);

promote_self!(f32, f32 => f32);
promote_self!(f64, f64 => f64);


// This is a generic that sets any operation involving NVecs of the same T to yield one of that type
//impl<T: Numerical> Promote<T> for T {
//    type Output = T;
//}

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