pub fn read_tuple() -> () {
    let tup: (i32, f32, u8, &str) = (500, 6.4, 1, "Hello");
    // let (x, y,z, message) = tup;

    let prt: u8 = tup.2;

    println!("--TUPLE--\nA sample tuple: {:?}", tup);
    println!("Value at index 2: {prt}");
    let mut scores = (93, 84, 85, 90, 96);
    scores.4 = 99;

    println!("Tuple: {:?}", scores);

    let unit_tuple = ();
    println!("Unit tuple: {:?}", unit_tuple);

}