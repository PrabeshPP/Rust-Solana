pub  fn challenge2(){
    let numbers=[1,9,-2,0,23,20,-7,13,37,20,56,-18,20,3];
    let mut max:i32;
    let mut min:i32;
    let mut mean:f64;
    max=0;
    min=0;
    mean=0.0;

    //For finding the Max Number
    for num in numbers{
        if num>max{
            max=num;
        }
    }

    //For finding the Minimum Number
    for num in numbers{
        if num<min{
            min=num;
        }
    }


    //For finding the mean
    let mut total=0;
    for num in numbers{
        total+=num;
    }

    mean=total  as f64 / numbers.len() as f64;

    




    assert_eq!(max,56);
    assert_eq!(min,-18);
    assert_eq!(mean,12.5);
    println!("Test passed")
    
}