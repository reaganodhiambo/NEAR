fn vectors(){
    let collected_iterator : Vec<i32> = (0..10).collect();
    println!("Collected  (0..10) into {:?}", collected_iterator);

    // Initialize a vector using vec!
    let mut xs = vec![1,2,3];
    println!("initial vector : {:?}", xs);

    // push new element into end of vector
    println!("push 4 into vector");
    xs.push(4);
    println!("New vector : {:?}", xs);

    // immutable vectors cannot grow

    // len() methods shows length of current vector
    println!("Length of vectors is {:?}", xs.len());

    //indexing is done using square brackets
    //indexing starts at 0
    println!("The second element is {}", xs[1]);
}

fn main() {
    vectors();
}


