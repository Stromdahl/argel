use crate::vec3::Vec3;

mod vec3;
fn main () {
    let a = Vec3::new(2, 2, 2);
    let b = Vec3::new(1, 2, 3);
    let c = a + b;

    println!("{:?}", c);
}
