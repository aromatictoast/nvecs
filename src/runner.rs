// file used to run anything needed during development
use nvecslib::*;


fn main(){
    let a = NVec::new([1, 2, 3]);
    let b = NVec::new([3, 2, 1]);


    println!("Mag A: {}", a.mag());
    println!("Mag B: {}", b.mag());

    let a_dot_b = a * b;
    println!("A dot B: {}", a_dot_b); 

    let a_cross_b = a.cross(b);
    println!("A x B: {:?}", a_cross_b);

    let a_times_2 = a * -2.0f32;
    println!("A * -2: {:?}", a_times_2);
    


    let c = NVec::new([2.0, 0.2, -3.2]);
    print!("{:?}\n", c.cross(a) * -1);
    print!("{:?}", a.cross(c));

    


}