use super::*;

// Use this general format
// Implement Add for NVec
impl<T, U, V, const N: usize> Add<NVec<U, N>> for NVec<T, N>
where
    T: Numerical + AsPrimitive<V> + Promote<U, Output = V>, // This ensures that we convert in to the correct type
    U: Numerical + AsPrimitive<V>,
    V: Numerical + Add<Output = V> + 'static,
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







#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_type_test() {
        //declare all the operations, then call the tests to be assembled below
        fn add<A: std::ops::Add<B, Output = C>, B, C>(a: A, b: B) -> C {
            a + b
        }
    
        println!("\n\nTesting for addition\n\n");
        generate_same_type_tests!(
            add, add, 1, 2, 3, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64
        );
    }

    #[test]
    fn casting_test(){
        println!("\n\nTesting for addition casting\n\n");
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
    }
}