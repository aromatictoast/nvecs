use core::fmt;
use criterion::*;
use nvecslib::*;
use rand::random;

macro_rules! iterate_types {
    ($group:expr, $n:expr, $numlist:expr, $($types:ty), *) => {
        $(
            $group.bench_with_input(
                BenchmarkId::new(
                    format!("Adding NVecs of size {:?} with type {:?} and components {}", $n, std::any::type_name::<$types>(), $numlist), $numlist),
                    &$numlist,
                    |b, numlist|
                    b.iter(
                    ||
                    NVec{components: [numlist.v1 as $types; $n]}.element_multiply(
                     // <-------------------------------------------------- Here is the operation being benched and you need to change it here!
                    NVec{components: [numlist.v2 as $types; $n]})
                    )
            );
        )*

    };
}

macro_rules! benchmark {
    ($group:expr, $v1:expr, $v2:expr, $($n_vals:expr), *) => {
        $(
            iterate_types!($group, $n_vals, NumList{v1: $v1, v2: $v2}, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);
        )*

    };
}

struct NumList {
    //wrapper for the two values to be added together
    v1: i32,
    v2: i32,
}

impl std::fmt::Display for NumList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Components for NVec 1: {}, components for NVec 2: {}]", self.v1, self.v2)
    }
}


//// This is a benchmark for the NVector operation: Elementwise multiplication
///
/// The primary objective is to test the operation across NVecs of a specified set of sizes and numerical types
/// To allow us to do this, we can use macros to programmatically generate Criterion tests for us according to our parameters
/// 
/// Usage:
/// 
/// 
/// The main loop is controlled by a macro in bench_addition called benchmark:
/// 
///     benchmark!( criterion_group, randomly_generated_components_for_first_NVec, randomly_generated_components_for_second_NVec, 1, 2, 3......... ////a comma separated list of all the sizes of NVec you would like to test) 
///
/// To set the number of runs (loops through all specified number types with all specified NVec sizes), change number_of_runs in bench_addition
/// 
/// The benchmark macro iterates across the chosen sizes of NVec
/// Each pass it calls another macro called iterate_types:
///     
///     iterate_types!( criterion_group, length_of_nvec_this_pass, numlist_wrapper_of_component_values, i8, i16, ........ /////a comma separated list of all the types you wish to test)
/// 
/// To change the types tested, it is necessary to modify it there.
/// As the iterate_types macro is called, it is passed a NumList - this is a wrapper struct (with a Display implementation) for the randomly generated values of the components of the NVecs for that run
/// This generates the criterion objects, then they are benchmarked together afterwards
/// 


fn bench_addition(c: &mut Criterion) {
    let mut group: BenchmarkGroup<'_, measurement::WallTime> = c.benchmark_group("Elementwise multiplication");

    let number_of_runs: u32 = 1; //// Set this to choice - this is how many times we will generate numbers



    for _i in 0..number_of_runs {
        
        
        let nvec1_comps = random::<i32>(); // these generate randomised components - are kept the same for a whole loop of type and size for a proper comparison
        let nvec2_comps = random::<i32>();


        benchmark!(group, nvec1_comps, nvec2_comps, 1, 2, 3, 4, 5, 6);
    };

    group.finish();
}

criterion_group!(benches, bench_addition);
criterion_main!(benches);
