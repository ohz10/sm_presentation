# State Machines in Rust

A presentation which introduces a few different techniques for implementing state machines in Rust.

- tennis kata
    - naive implementation
    - naive implementation using enumeration for score
    - a state machine solution
        - fixing problems with naive solution
    - another state machine solution
        - uses minimal memory
        - uses type system to prevent invalid transitions

- type state pattern
    - useful idiom for implementing stateful interfaces
    - prevents incorrect use of stateful interfaces

- asynchronous state machine parser
    - a state machine driven parser
    - uses function pointers to force a jump table
