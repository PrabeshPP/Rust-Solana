pub fn ownership_rust(){
    //ownership in rust
    let outer_planet:String;
    {
        let inner_planet=String::from("Mercury");
        /* 
            if we assign the value  
            outer_planet=inner_planet // the ownership would  be moved to the outer_planet variable
            so,to solve this problem in rust
            we need to use the .clone() function in Rust
        */

        /*
        if the value happens to be integer,we don't need to use  the clone function
        since having the integer value ,the data would lives on the stack
        
        */
        outer_planet=inner_planet.clone();
        println!("Inner planet is {inner_planet}");
    }
    println!("outer_planet is {outer_planet}");
}