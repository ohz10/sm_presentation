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
    - iterating on 20-state solution to replace big match statement with [a dynamic dyn call](src/bin/tennis_sm20_enumtrait.rs)
    - iterating on 20-state solution to replace big match statement with [a table lookup](src/bin/tennis_sm20_table.rs)
- [super simple state machine](src/bin/simple.rs)
    - redemonstrate technique from tennis kata on simple state machine

- [type state pattern](src/bin/type_state_pattern.rs)
    - useful idiom for implementing stateful interfaces
    - prevents incorrect use of stateful interfaces

- [asynchronous state machine parser](src/bin/asm_parser.rs)
    - a state machine driven parser
    - uses function pointers to force a jump table

## Tennis Kata Images

Use graphviz to generate images for tennis kata state machines:

    $ circo -Tpng -o sm4.png sm4.dot
    $ dot -Tpng -o sm20.png sm20.dot
    $ dot -Tpng -o simple.png simple.dot

View the images using feh:

    $ feh -Z -F sm4.png
    $ feh -Z -F sm20.png
    $ feh -Z -f simple.png

## PDF Slides

The slides can be generated using TexMaker.

### Code Slides

The code slides require `bat` and `ksh` and can be viewed by running `./code_slides.sh`

## Presentation Monitor Configuration

Configure secondary monitor to mirror the main monitor using xrandr:

    $ xrandr --output SECONDARY --same-as PRIMARY

Where SECONDARY is the name of the second monitor and PRIMARY is the name of the primary monitor.
