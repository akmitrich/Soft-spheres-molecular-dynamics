use d_vector::DVector;

fn main() {
    let v = DVector::from([1., 0.22, 1e-6]);
    println!("Hello, {:?}", v.components());
}
