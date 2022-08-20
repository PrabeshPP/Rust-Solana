pub fn forLoop(){
    let arr=["a","b","c","d","e"];

    for (index,&item) in arr.iter().enumerate(){
        println!("Character is {item}")
    }

    for x in 1..10{
        println!("value of x is {x}")
    }
}