extern mod rustfrp;

use rustfrp::signal::*;

fn main() {
    let (input, four)   = constant(4);
    let timestwo        = lift(&four, |x| {println("timestwo");x*2});
    let times           = lift2(&four, &timestwo, |x,y| {println("times"); x*y});
    let _output         = lift(&times, |x| println!("{}", x));
    input.send(());
}

