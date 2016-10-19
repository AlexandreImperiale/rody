extern crate mersh;
extern crate rody;

use rody::block::*;

fn forward(block: &mut Block, ts: f64)
{
    block.position.coords.add_in(ts, &block.velocity.coords);
}

fn main() {

    let mut block = rody::block::BlockBuilder::new()
        .set_mass_density(1.0)
        .set_lengths(1.0, 1.0, 1.0)
        .set_initial_velocity(-1.0, 0.0, 0.0)
        .get();

    let tl = rody::timeline::RegularTimeLine::new(0., 0.1, 10);

    for (i, time) in tl.enumerate()
    {
        // ======> CAN'T ACCESS TL INSIDE LOOP SINCE MOVED !!!!
        forward(&mut block, 0.1);
        println!("{:}", block.format("_", 3));
    }
}
