use super::*;

// Multiplying elements - not the dot product - not sure what used for but made it accidentally, free code
// modify to include multiplying out in form of (ai + bj) * (ci + dj) ?


impl<T, U, V, const N: usize> ElementMul<NVec<U, N>> for NVec<T, N>
where
    T: Numerical + AsPrimitive<V> + Promote<U, Output = V>, // This ensures that we convert in to the correct type
    U: Numerical + AsPrimitive<V>,
    V: Numerical + 'static,
{
    type Output = NVec<V, N>;

    fn element_multiply(self, rhs: NVec<U, N>) -> Self::Output {
        let result: [V; N] =
            core::array::from_fn(|i: usize| self.components[i].as_() * rhs.components[i].as_());

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
        fn mul<A: std::ops::Mul<B, Output = C>, B, C>(a: A, b: B) -> C {
            a * b
        }
        fn elemul<A: ElementMul<B, Output = C>, B, C>(a: A, b: B) -> C {
            a.element_multiply(b)
        }

        println!("\n\nTesting for element multiplication\n\n");
        generate_same_type_tests!(
            elemul, mul, 1, 2, 3, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64
        );
    }

    #[test]
    fn casting_test(){
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
    }
}
