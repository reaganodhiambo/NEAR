fn main() {
    my_function();
    my_function_2(22);
    let x = my_function_3();
    let y = my_function_4();
}

fn my_function(){
    println!("Hello, world!");
}

fn my_function_2(number : i32){
    println!("Hello my function number {}", number);
}
fn my_function_3()-> String{
    println!("Hello my function number 3");
    return String::from ("Hello World");
}

fn my_function_4()-> i32{
    println!("Hello my function number 4");
   33
}