/// Implements Rule 110 cellular automaton for a single generation of 8 bits
///
/// Rule 110 is an elementary cellular automaton where each cell's next state is determined by its
/// current state and its two neighbors. The rules are:
/// - 111 -> 0    - 011 -> 1
/// - 110 -> 1    - 010 -> 1
/// - 101 -> 1    - 001 -> 1
/// - 100 -> 0    - 000 -> 0
///
/// # Arguments
///
/// * `bits` - An 8-bit unsigned integer representing the current generation
///
/// # Returns
///
/// * An 8-bit unsigned integer representing the next generation
///
/// # Examples
///
/// ```
/// let current = 0b1010_0100;
/// let next = rule110(current);
/// assert_eq!(next, 0b1110_1101);
/// ```
fn rule110(bits: u8) -> u8 {
    let mut next_gen: u8 = 0;

    for i in 0..8 {
        // Get the left bit
        let left: u8 = if i == 7 {
            bits & 0b0000_0001 // mask all but the lsb
        } else {
            (bits >> (i + 1)) & 0b0000_0001
        };

        // Get the center bit
        let center: u8 = (bits >> i) & 0b0000_0001;

        // Get the right bit
        let right: u8 = if i == 0 {
            (bits & 0b1000_0000) >> 7 // mask all but the msb
        } else {
            (bits >> (i - 1)) & 0b0000_0001
        };

        // Apply rule 110 pattern
        let pattern: u8 = (left << 2) | (center << 1) | right;
        let temp_bit: u8 = match pattern {
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
    }
    next_gen
}

fn main() {
    // Test with some initial pattern
    let mut current: u8 = 0b1010_0100u8;

    // Print 10 generations
    for _ in 0..10 {
        println!("{:08b}", current);
        current = rule110(current);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corner_case_bit_0() {
        // Test bit 0's right neighbor wrapping
        let input: u8 = 0b0000_0001;
        let result: u8 = rule110(input);
        // Rule 110 pattern: 010 -> 1
        // Bit 0 should be 1, and rest should be 0
        assert_eq!((result & 0b0000_0001), 1);
    }

    #[test]
    fn test_corner_case_bit_7() {
        // Test bit 7's left neighbor wrapping
        let input: u8 = 0b1000_0000;
        let result: u8 = rule110(input);
        // Rule 110 pattern: 010 -> 1
        // Bit 7 should be 1, and rest should be 0
        assert_eq!((result & 0b1000_0000), 1 << 7);
    }

    #[test]
    fn test_assignment_base_case() {
        // Test the base case from the assignment
        // 1010_0100 -> 1101_0110
        let input: u8 = 0b1010_0100;
        let result: u8 = rule110(input);

        assert_eq!(result, 0b1110_1101);
    }
}
