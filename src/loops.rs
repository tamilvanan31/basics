pub fn loops() -> () {
    let mut count: i32 = 0;
    print!("--LOOPS--\n");
    let result = loop {
        if count == 10 {
            break count * 10;
        }
        count += 1;
        println!("Count value is {count}");
    };

    print!("\nThe result value is {result}");
}