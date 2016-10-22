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

    forward(&mut block, 0.1);
    println!("{:}", block.format("_", 3));
}
