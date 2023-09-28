pub fn bitwise() -> () {
    let mut value = 0b1111_0101u8;
    println!("--BITWISE--\n Normal representation: {value}");
    println!("Binary representation: {:08b}", value);

    // Bitwise NOT - ! 
    // inverts the bits
    value = !value;

    /* Bitwise AND operator - &
     * When you want to invert a particular without affecting other bits
     *
     *  Clear value for a specific bit
     *    00001010
     *AND 11110111
     *    00000010
     * 
     * Check value of a specific bit
     *      | This bit 
     *      v 
     *     00000010 
     * AND 01000000    
     *     00000000
     * */

    value = value & 0b1111_0111u8;
    println!("Value: {:08b}", value);
    println!("{0:08b} & {2:08b}: {1:08b}", value, value & 0b0100_0000, 0b0100_0000);

    /* Bitwise OR operator - |
     *Set value of a specific bit     
     *   00000010
     *OR 01000000    
     *   00000010 
     * */
    println!("{:08b} | {:08b}: {:08b}", value, 0b0100_0000, value | 0b0100_0000);
    
    /*
     *Bitwise XOR - ^
     * */
    println!("{:08b} ^ {:08b}: {:08b}", value, 0b0100_0000, value ^ 0b0101_0101);
    /*
     *Bitwise shift operators - left: <<, right: >> 
     * */  
    let left_shift = value << 4;
    println!("Left shift by 4 bits: {:08b}", left_shift);

    let right_shift = value >> 4;
    println!("Right shift by 4 bits: {:08b}", right_shift);
}

