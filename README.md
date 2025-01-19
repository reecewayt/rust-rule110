# Rust-Rule110
Author: Reece Wayt  
Contact: reecwayt@pdx.edu  
Date: 1/18/2025 
GitHub Repo: https://github.com/reecewayt/rust-rule110 
---
## Algorithm Description
A simple rust application for assignment 1 of CS 523.

Rule 110 is a elementary cellular automata which means each cell is binary 0 or 1. Each generation of a string of binary is created iteratively by cycling through individual cells, considering each cells neighboring cells (i.e left and right) to determine what the next cell value will be in the next generation. Its important to note that in the case of 8-bits, where each bit is a cell, the 7th bit's right neighbor wraps around to bit position 0; vis-avis the 0th bit's left neighbor wraps around which would be the 7th bit. There are many rule sets to chose from, but this implementation uses rule 110 as summarized pictorally below.  

  ![Rule 110 Pattern](./docs/images/rule-110-wolfram-ref.png)  
  *Image source: [Wolfram MathWorld - Rule 110](https://mathworld.wolfram.com/Rule110.html)*

## Implementation Notes
- Uses `u8` ddata type to store each cellular automata generation
- Includes unit tests for corner cases to test wrap around behavior and base case from assignment requirements

## Running application, tests, and rustdoc
```bash
#run application main() entry
cargo run
# run #[test] cases
cargo test
# run rustdoc
cargo doc --no-deps --open
```
**Note**: I'm running wsl which requires some additional steps to natively launch the rustdoc html in a browser. If you're also on wsl follow this guide to run browsers from wsl. [Run Linux Gui Apps from WSL](https://learn.microsoft.com/en-us/windows/wsl/tutorials/gui-apps)

### Sources
- Algorithm Reference: [Wolfram MathWorld - Rule 110](https://mathworld.wolfram.com/Rule110.html)
- Development Tools:
  - Rustdoc boilerplate and documentation formatting assistance provided by [Claude.ai](https://claude.ai)
  - rust-analyzer (vs-code extension)