use super::*;

// Subtraction
impl<T, U, V, const N: usize> Sub<NVec<U, N>> for NVec<T, N>
where
    T: Numerical + AsPrimitive<V> + Promote<U, Output = V>, // This ensures that we convert in to the correct type
    U: Numerical + AsPrimitive<V>,
    V: Numerical + Sub<Output = V> + 'static,
{
    type Output = NVec<V, N>;

    fn sub(self, rhs: NVec<U, N>) -> Self::Output {
        let result: [V; N] =
            core::array::from_fn(|i: usize| self.components[i].as_() - rhs.components[i].as_());

        //arr.into();

        NVec::new(result)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_type_test() {
        //declare all the operations, then call the tests to be assembled below
        fn sub<A: std::ops::Sub<B, Output = C>, B, C>(a: A, b: B) -> C {
            a - b
        }
    
        println!("\n\nTesting for subtraction\n\n");
        generate_same_type_tests!(
            sub, sub, 1, 2, 3, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64
        );
    
    }

    #[test]
    fn casting_test(){
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
    }
}
