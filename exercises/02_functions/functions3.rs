fn main() {
    // TODO: Fix the function call.
    let num: u8 = 2;
    call_me(num);
}

fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
