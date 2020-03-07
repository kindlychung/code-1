extern crate rand;

use rand::Rng; // <> Make an external crate available to our code

static mut ERROR: isize = 0;

struct File;

#[allow(unused_variables)]
fn read(f: &File, save_to: Vec<u8>) -> usize {
    if rand::thread_rng().gen_bool(0.5) {
        unsafe {
            ERROR = 1;
        }
    }

    0 // <> Always read() 0 bytes
}

#[allow(unused_mut)]
fn main() {
    let mut f = File;
    let mut buffer = vec![];

    read(&f, buffer);
    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred!")
        }
    }
}
