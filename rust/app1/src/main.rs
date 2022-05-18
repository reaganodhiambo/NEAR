#[warn(unused_assignments)]
#[warn(dead_code)]
#[warn(unused_variables)]

fn main() {
    println!("Hello, world!");
    let my_immutable = 23;
    let mut my_mutable = 23;

    my_mutable = 40;
    println!("my immutable {} my mutable {}", my_immutable, my_mutable);


    let guess0 = 33;

    let guess: u8 = 33;
    let guess1:u16 = 33;
    let guess2:u32 = 33;
    let mut guess: u64 = 34;

    const  _numb: i8 = -45;
    const numb2 : i16 = 45;
    const numb3 : i32 = 45;
    const numb4 : i64 = 45;

    println!("guess {}, guess 0 {}", numb, guess0);

}

//signed and unsigned
//integers, float, string &str, booleans
//mutable and immnutable
//constants
