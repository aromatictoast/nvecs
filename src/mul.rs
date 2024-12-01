use super::*;


// Multiplying vectors - the dot product
impl<T: Numerical, U: Numerical, V: Numerical, const N: usize> Mul<NVec<U, N>> for NVec<T, N>
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

//multiplying by a number e.g. 3x, -x, applies to all components

// this one is x * -1
impl<T, U, V, const N: usize> Mul<U> for NVec<T, N> 
where
    T: Numerical + AsPrimitive<V> + Promote<U, Output = V>, 
    U: Numerical + AsPrimitive<V>,
    V: Numerical + Mul<Output = V> + Add<Output = V> + 'static, //numerical primitive

{
    type Output = NVec<V, N>; //note that this may change the type of the component

    fn mul(self, rhs: U) -> Self::Output {
        let result: [V; N] =
        core::array::from_fn(|i: usize| self.components[i].as_() * rhs.clone().as_());

        NVec::new(result)
    }
}
// to implement the other way round, due to stupid orphan rules have to implement for each individual concrete type we support
// yay.....
//have to use a macro for each one - reinventing the generic...
// What I wouldn't give for an impl blah for U where U in {i8, i16.....}

//Use this when you give up on generics....
//Especially for commutative operations
macro_rules! right_handed_scalar_mul {
    ($( $type:ty ), *) => {
        $(
            impl<T, V, const N: usize> Mul<NVec<T, N>> for $type
            where
                T: Numerical + AsPrimitive<V> + Promote<$type, Output = V>, 
                V: Numerical + Mul<Output = V> + Add<Output = V> + 'static, //numerical primitive
                $type: num_traits::AsPrimitive<V>,

            {
                type Output = NVec<V, N>; //note that this may change the type of the component
            
                fn mul(self, rhs: NVec<T, N>) -> Self::Output {
                    let result: [V; N] =
                    core::array::from_fn(|i: usize| rhs.components[i].as_() * self.as_());
                
                    NVec::new(result)
                }
            }
        )*
    };
}

right_handed_scalar_mul!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64);











//impl<T, V, const N: usize, > Mul<NVec<T, N>> for U 
//    where
//        T: Numerical + AsPrimitive<V> + Promote<U, Output = V>, 
//        U: Numerical + AsPrimitive<V>,
//        V: Numerical + Mul<Output = V> + Add<Output = V> + Default + 'static, //numerical primitive
//
//{
//    type Output = NVec<V, N>; //note that this may change the type of the component
//
//    fn mul(self, rhs: NVec<T, N>) -> Self::Output {
//        let result: [V; N] =
//        core::array::from_fn(|i: usize| rhs.components[i].as_() * self.as_());
//
//        NVec::new(result)
//    }
//}












#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_type_test() {
        //declare all the operations, then call the tests to be assembled below
        fn mul<A: std::ops::Mul<B, Output = C>, B, C>(a: A, b: B) -> C {
            a * b
        }
    
        println!("\n\nTesting for dot product\n\n");
        generate_same_type_dot_tests!(
            mul, 1, 2, 3, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64
        );

        println!("\n\nTesting for scalar * NVec\n\n");
        let a: NVec<f64, 3> = NVec {
            components: [1.01 as f64, -2.65 as f64, 3.4 as f64],
        };
        let b: NVec<f64, 3> = NVec {
            components: [-1.01f64, 2.65f64, -3.4f64],
        };
        //let neg1: i32 = -1;
        
        assert_eq!(a, b * -1);
        assert_eq!(b * -1, a)


    }

    #[test]
    fn casting_test(){
        println!("\n\nTesting for dot product\n\n");
        let a: NVec<f64, 3> = NVec {
            components: [1.01 as f64, -2.65 as f64, 3.4 as f64],
        };
        let b: NVec<i32, 3> = NVec {
            components: [4, -5, 200],
        };
    
        assert_eq!(a * b, 1.01 * 4f64 + -2.65 * -5f64 + 3.4 * 200f64);
    
        assert_eq!(b * a, 1.01 * 4f64 + -2.65 * -5f64 + 3.4 * 200f64);
    }
}