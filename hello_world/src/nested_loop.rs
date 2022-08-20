pub fn nested_loop(){
    let mut matrix=[
        [1,2,3],
        [4,5,6],
        [7,8,9]
    ];

    //modifying the multi-dimensional array
    for row in matrix.iter_mut(){
        for num in row.iter_mut(){
            *num+=10;
            print!("{num}\t")
        }
        println!();
    }
}