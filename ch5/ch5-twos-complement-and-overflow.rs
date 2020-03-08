macro_rules! print_binary {
    ($name:ident) => {
        println!(
            "{}: {:03}, bin_repr: {:08b}",
            stringify!($name),
            $name,
            $name
        );
    };
}

fn main() {
    // demo integer overflow
    let x = 124i8;
    print_binary!(x);
    for i in 1..7 {
        let y = x + i;
        print_binary!(y);
    }
    // demo two's complement
    let z1= 100i8;
    let z2 = -100i8;
    let z3 = !z1 + 1;
    print_binary!(z2);
    print_binary!(z3);
}
