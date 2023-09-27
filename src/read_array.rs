use std::io;

pub fn read_array() {
    let default_array: [i32; 5] = [0; 5];
    let sample_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("\n--ARRAY--\nDefault array: {:?}", default_array);

    let n = sample_array.len();
    for i in 0..n {
        print!("{} ", sample_array[i]);
    }

    let a = [1, 2, 3, 4, 5];

    println!("\nPlease enter an array index: ");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    if index > 4 {
        println!("Index is greater than 4. Program will now exit.");
        return;
    }

    let element = a[index];
    println!("The value at {} is {}", index, element);
}
