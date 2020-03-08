use num_traits::Num; 

fn factorial<T: Num + Copy>(n: T) -> T {
    if n.is_zero() {
        T::one()
    } else {
        n * factorial(n - T::one())
    }
}

fn main() {
    println!("Generic: {}", factorial(5i64))
}
