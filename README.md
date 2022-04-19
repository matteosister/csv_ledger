### Csv Ledger

The binary accept one argument (the csv source file) and output resulting accounts to stdout as csv

I had some problems with the csv library and serde, because I couldn't make it work with tagged enum representations.
I decided to implement a custom deserializer given that the format is pretty simple.
Another approach could also be having a middle representation more similar to the csv file, so one record for every row
with a field representing the type, and then convert it to the final enum representation with a TryFrom.

Errors are handled with the `thiserror` crate. See the `errors.rs` file for all possible error types.

I'm not streaming from the csv, but I'm creating an in-memory hashmap. 
Talking about efficiency, a better approach could be parsing the file and just store "empty" accounts in memory.
Then, when iterating over accounts to output the final state, apply rows directly from the csv file.
This is better from a memory perspective, because as soon as you complete an account you could drop all the transactions
you have in memory, but worst for the processor and IO, because probably you end up iterating over the csv file multiple times.
It's a tradeoff between efficiency and code complexity.

#### tests
    cargo test

#### execute
    cargo run -- data/example_data.csv

