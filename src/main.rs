// extern crate mersh;

extern crate rody;
use rody::timeline::*;

fn main() {
    for (i, time) in RegularTimeLine::new(0., 1.0, 10).enumerate() {
        println!("time = {} {}", i, time);
    }
}




// for (i, time) in timeline.iter().enumerate()
// {
//      shape.move(time).write("shape_{}".format(i))
// }
