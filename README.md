# turing_machine

A Rust Turing machine simulator with no dependencies. It provides a transition function, an
infinite tape, configurable forced-halt settings, and a `Recording` type that replays a run
step by step in the terminal. The experimental part is `chaitin_approx`, which comes out of an
interest in Chaitin's constant, the halting probability. Since the constant is uncomputable, the
function does the only thing available: it brute-force enumerates every machine with a given number
of states and symbols, runs each one under a step cap, and reports what fraction halted alongside
the fraction that were still running when the cap hit. That second number is the part no amount of
compute will ever get rid of.

## Example

```rust
use turing_machine::prelude::*;

fn main() {
    // The 2-state busy beaver, played back in the terminal.
    let trans_fn = TransitionFn::new(&vec![
        ((0, 0), (1, 1, true)),
        ((1, 0), (0, 1, false)),
        ((0, 1), (1, 1, false)),
    ]);

    let mut machine = TuringMachine::new(trans_fn);
    let record = machine.run_with_halt_setting_and_record(&mut Tape::default(), HaltSetting::AfterSteps(100));
    record.play_in_console(std::time::Duration::from_millis(500), true);

    // (fraction halted, fraction undecided) over all 3-state, 2-symbol machines.
    println!("{:?}", TuringMachine::chaitin_approx(3, 2, HaltSetting::AfterSteps(100)));
}
```