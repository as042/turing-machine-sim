use turing_machine::prelude::*;

fn main() {
    println!("{:?}", TuringMachine::chaitin_approx(3, 2, HaltSetting::AfterSteps(100)));
}