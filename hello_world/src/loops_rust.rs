pub fn Loop(){
    let mut count=10;
    let value=loop{
        if count==40{
            //break will return the value
            break count*2;
        }

        count+=1;
        println!("value of count is {count}")
    };

    println!("Loop has been existed");
    println!("final value is {value}")

}