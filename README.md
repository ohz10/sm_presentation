# State Machines in Rust

A presentation which introduces a few different techniques for implementing state machines in Rust.

- tennis kata
    - [naive implementation](src/bin/tennis_naive.rs)
    - [naive implementation using enumeration](src/bin/tennis_naive_enum.rs) for score
    - a [state machine solution](src/bin/tennis_sm4.rs)
        - fixing problems with naive solution
    - another [state machine solution](src/bin/tennis_sm20.rs)
        - uses minimal memory
        - uses type system to prevent invalid transitions

- [type state pattern](src/bin/type_state_pattern.rs)
    - useful idiom for implementing stateful interfaces
    - prevents incorrect use of stateful interfaces

- [asynchronous state machine parser](src/bin/asm_parser.rs)
    - a state machine driven parser
    - uses function pointers to force a jump table
