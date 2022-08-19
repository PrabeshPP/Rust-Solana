mod arthimetic_average;
mod array;
mod functions_rust;

fn main() {
    //variable in Rust
    //Integer Data-Types
    let mut x:i8=90;
    // By default variables are immutable in Rust
    x=20;
    println!("X value is {x}");

    //DataType in Rust
    //Floating Data Types
    let  number:f32=20.6555;
    println!("{}",number);

    //Arthmetic Operaitons

    let a=20;
    let b=40;
    let c=(b+1)/a;
    println!("{}",c);


    //Formating print statements
    let i=30.33;
    let j=3.3;
    let k=i/j;
    println!("k is {:.2}",k);

    //Positional Parameters

    let x=20;
    let y=40;
    
    let result=y*x;
    
    println!("x is {0} y is {1} and {0} multiply {1} is {2}",x,y,result);

    //BitWise Operations
    //Not
    //Or
    //And
    //Xor
    //Shift

    let mut value=10;
    value=!value;
    print!("value is {}",value);
    
    // And wise bit operation
    let first_bit=2;
    let second_bit=3;
    println!("And Bit Wise Operation is {}",first_bit&second_bit);

    //Checking whether the given number is odd or even using bitwise operator
    let number=100;
    let result=number&1;
    if result==0 {
        println!("Number is even");
    }else {
        println!("Number is Odd");
    }


    //XOR Operator
    let num1=2;
    let num2=1;
    let xor_result=num1^num2;
    println!("XOR of {0} and {1} is {2}",num1,num2,xor_result);

    //Bitwise shift in Rust
    //Left Shift Operator
    let random=2;
    println!("Value after shifting with left shifter {}",random<<4);

    //Right Shift Operator
    let random1=2;
    println!("Value after shifting with Right Shifter {}",random1>>4);


    //Characters in Rust
    let letter='a';
    let finger='\u{261D}';
    println!("{letter}");
    println!("{finger}");


    //Average Solution of three numbers
    arthimetic_average::sum_average();
    
    array::array();
    functions_rust::say_hello(20, 40);
    let num1=2;
    let num2=2;
    functions_rust::function1(num1,num2);

    let value_square=functions_rust::square(20);
    println!("The square of the given  number is {value_square}")
}
