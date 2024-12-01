use super::*;

impl<T, U, V> CrossProduct<NVec<U, 3>> for NVec<T, 3> // simple cross product in VA form for 3d only
where
    T: Numerical + AsPrimitive<V> + Promote<U, Output = V>,
    U: Numerical + AsPrimitive<V>,
    V: Numerical + 'static,
{
    type Output = NVec<V, 3>;

    fn cross(self, rhs: NVec<U, 3>) -> Self::Output {
        let result: [V; 3] = [ //just using a formula rather than building more fancily

            self.components[1].as_() * rhs.components[2].as_() - self.components[2].as_() * rhs.components[1].as_(),
            self.components[2].as_() * rhs.components[0].as_() - self.components[0].as_() * rhs.components[2].as_(),
            self.components[0].as_() * rhs.components[1].as_() - self.components[1].as_() * rhs.components[0].as_(),
        ];

        

        NVec::new(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_type_test() {
        //declare all the operations, then call the tests to be assembled below
        fn cross<A: CrossProduct<B, Output = C>, B, C>(a: A, b: B) -> C {
            a.cross(b)
        }

    
        println!("\n\nTesting for cross product\n\n");
        generate_same_type_cross_tests!(
            cross, 1, 2, 3, i8, i16, i32, i64, f32, f64 // no unsigned integers
            //used to have mul, may need
        );
    }

    #[test]
    fn casting_test(){
        println!("\n\nTesting for cross product casting\n\n");
        let a: NVec<f64, 3> = NVec {
            components: [1.01 as f64, -2.65 as f64, 3.4 as f64],
        };
        let b: NVec<i32, 3> = NVec {
            components: [4, -5, 200],
        };
    
        assert_eq!(
            a.cross(b),
            NVec {
                components: [-513.0f64, -188.4f64, 5.55f64] //if it's not working, check these
            }
        );
        //TODO - get to work both ways

    }
}