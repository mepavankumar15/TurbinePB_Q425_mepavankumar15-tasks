## User Story

Its primary use case is to allow users to perform basic arithmetic operations (like addition, subtraction, multiplication, and division) directly on the blockchain.
A user initializes a dedicated "calculator" account that stores a value (e.g., a "result"). 
They can then send subsequent transactions to the program to modify this stored value, 
with the program ensuring the calculations are processed and the new result is saved on-chain.



## Architectural Diagram
<img width="918" height="607" alt="architectural-diagram" src="https://github.com/user-attachments/assets/fa5a3661-479c-4323-a344-888ffe0e2642" />




## Quick start
```bash
# Install dependencies
npm install

# Build the program
anchor build

# Run tests
anchor test
```
