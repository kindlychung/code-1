macro_rules! string {
    ( $($x:expr) * ) => {
        {
            let mut s = String::new();
            $(
                let s0 = stringify!($x);
                println!("Found: {}", &s0);
                s.push_str(s0); // <2>
                s.push(' ');
            )*
            s.pop();
            s
        }
    };
}

fn main() {
    let s = string!(hello there how are you?);
    println!("{}", s);
}
