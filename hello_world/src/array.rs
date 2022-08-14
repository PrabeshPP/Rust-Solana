pub fn array() {
    let letters = ['a', 'b', 'c', 'd'];
    let first_letter = letters[0];
    println!("{first_letter}");

    //initalizing an array in rust
    let numbers: [i32; 5];
    numbers = [0; 5];
    println!("{}", numbers[0]);

    //multi-dimensional array
    let matrix = [[1, 2, 3], [4, 5, 6]];
    let number=matrix[0][1];
    println!("{number}");


    //Tuples in Rust
    //Tuples in Rust Contains mixed data type
    let stuff=(1,"p",3.14);
    let second_item=stuff.1;
    println!("{}",second_item);
}
