use num_traits::Float;

/// Q1_7 - single byte representation of a fixed point number with range [-1, 1].
/// The name refers to the Texas Instrument representation
///
/// References:
///  - English Wikipedia: "Q (number format)" https://en.wikipedia.org/wiki/Q_(number_format)
#[derive(Debug)]
struct Q1_7(i8); // tuple struct holding a i8 value

impl<T> From<T> for Q1_7
where
    T: Float,
{
    fn from(n: T) -> Self {
        let one = T::one();
        let two = one + one;
        let val = if n > one {
            // out of bounds numbers are coerced to the maximum of the range
            one
        } else if n < -one {
            -one
        } else {
            n * (two.powi(7))
        };
        Q1_7(val.to_i8().expect("Failed to convert Float to i8"))
    }
}

macro_rules! impl_from_q1_7 {
    ($($t:ty)*) => ($(
        impl From<Q1_7> for $t {
            fn from(q: Q1_7) -> Self
            {
                q.0 as $t * (2.0).powf(-7.)
            }
        }
    )*)
}

impl_from_q1_7!(f32 f64);

fn main() {
    let f_orig = 0.84312f32;
    println!("f32 orig: {}", &f_orig);
    let nq0 = Q1_7::from(f_orig);
    println!("q1_7: {:?}", nq0);
    let f0 = f32::from(nq0);
    println!("f32: {}", &f0);
}
