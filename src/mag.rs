use super::*;

// Use as blueprint of implementation for operation that only takes a single param (e.g. operates on self)
impl<T, const N: usize> Magnitude for &NVec<T, N> //note this is not define for i64 and u64 as it could cause overflows on it's conversion in to f64
//shouldn't consume the NVec
where
    T: Numerical, //
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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_type_test() {
    
        println!("\n\nTesting for magnitude\n\n");
        generate_same_type_mag_tests!(
            1, 2, 3, i8, i16, i32, u8, u16, u32, f32, f64
        );
        
    }

    #[test]
    fn casting_test(){
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