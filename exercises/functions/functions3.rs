// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.



fn main() {
    // Could also pass a literal int and rust would be able to infer that
    // it is a u32 (it could also infer the type even if you removed it from
    // the variable declaration)
    let num: u32 = 8;
    call_me(num);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
