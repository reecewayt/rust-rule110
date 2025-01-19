

fn rule110(bits: u8) -> u8 {
    let mut next_gen = 0; 

    for i in 0..8 {
        // Get the left bit
        let left = if i == 7 {
            bits & 0b0000_0001 // mask all but the lsb
        } else {
            (bits >> (i + 1)) & 0b0000_0001
        };
        
        // Get the center bit
        let center = (bits >> i) & 0b0000_0001;

        // Get the right bit
        let right = if i == 0 {
            (bits & 0b1000_0000) >> 7 // mask all but the msb
        } else {
            (bits >> (i - 1)) & 0b0000_0001
        };

        // Apply rule 110 pattern
        let pattern = (left << 2) | (center << 1) | right;
        let temp_bit = match pattern {
            0b111 => 0,
            0b110 => 1, 
            0b101 => 1,
            0b100 => 0,
            0b011 => 1,
            0b010 => 1,
            0b001 => 1,
            0b000 => 0,
            _ => unreachable!(),
        }; 
        // Set the next generation bit
        next_gen |= temp_bit << i;
    };
    next_gen
}

fn main() {
    // Test with some initial pattern
    let mut current = 0b1010_0100u8;
    
    // Print several generations
    for _ in 0..8 {
        println!("{:08b}", current);
        current = rule110(current);
    }
}
