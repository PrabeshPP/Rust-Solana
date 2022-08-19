pub fn say_hello(number1:i32,number2:i32){
    let total=number1+number2;
    println!("Sum of two numbers is {total}");
}

pub fn function1(num1:u8,num2:u8){
    let total=num1*num2;
    println!("Multiplication of two number is {total}");
}


pub fn square(x:i32)->i32{
    println!("I need to return a value");
    x*x
}