// Using base tools of mersh.
use mersh::base::*;

/// Data structure for defining blocks.
#[derive(Clone, Default, Debug)]
pub struct Block {
    /// Total mass of the block.
    pub mass: f64,
    /// Associated length in each direction.
    pub lengths: [f64; 3],
    /// Associated position of the block center of mass.
    pub position: Pnt3d,
    /// Associated velocity of the block center of mass.
    pub velocity: Vec3d,
    // .... To Do : Angles and angular velocity.
}

/// Helper class for building blocks properly.
#[derive(Clone, Default, Debug)]
pub struct BlockBuilder {
    /// Block under construction.
    block: Block,
}

/// Helper class for formatting blocks.
#[derive(Clone, Debug)]
pub struct BlockFormatter<'a> {
    /// Reference to block to format.
    block: &'a Block,
    /// Index of data to be formmated.
    data_index: Vec<u8>,
    /// Number of decimal for formatting values.
    decimal: usize,
}

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// Implemntation of block builder.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

impl BlockBuilder {
    /// Creating new builder, internal component are initialized using default values.
    ///
    /// # Examples
    /// ```
    /// use rody::block::*;
    ///
    /// let block = BlockBuilder::new().get();
    ///
    /// assert!(block.mass.abs() < 1e-12);
    /// assert!(block.get_volume().abs() < 1e-12);
    /// assert!(block.position.coords.norm() < 1e-12);
    /// assert!(block.velocity.coords.norm() < 1e-12);
    /// ```
    pub fn new() -> Self
    {
        BlockBuilder::default()
    }

    /// Setting mass density of the block. The input paramater corresponds the the "per elementary
    /// volume" mass, the total mass of the block is the mass density times the volume of the block.
    ///
    /// * `mass_density` - the the "per elementary volume" mass, the total mass of the block is the
    /// mass density times the volume of the block.
    ///
    pub fn set_mass_density(&mut self, mass_density: f64) -> &mut Self
    {
        // Storing mass density, total mass computed when calling get() method.
        self.block.mass = mass_density;
        self
    }

    /// Setting lengths of the block.
    ///
    /// * `Lx` - length of the block in the first direction.
    /// * `Ly` - length of the block in the second direction.
    /// * `Lz` - length of the block in the third direction.
    ///
    pub fn set_lengths(&mut self, lx: f64, ly: f64, lz: f64) -> &mut Self
    {
        self.block.lengths = [lx, ly, lz];
        self
    }

    /// Setting initial position of the block.
    ///
    /// * `px` - First coordinate of the position of the block center of mass.
    /// * `py` - Second coordinate of the position of the block center of mass.
    /// * `pz` - Third coordinate of the position of the block center of mass.
    ///
    pub fn set_initial_position(&mut self, px: f64, py: f64, pz: f64) -> &mut Self
    {
        self.block.position = Pnt3d::new(px, py, pz);
        self
    }

    /// Setting initial velocity of the block.
    ///
    /// * `vx` - First coordinate of the initial velocity of the block.
    /// * `vy` - Second coordinate of the initial velocity of the block.
    /// * `vz` - Thrid coordinate of the initial velocity of the block.
    ///
    pub fn set_initial_velocity(&mut self, vx: f64, vy: f64, vz: f64) -> &mut Self
    {
        self.block.velocity = Vec3d::new(vx, vy, vz);
        self
    }

    /// Accessing built block.
    ///
    /// # Examples
    /// ```
    /// use rody::block::*;
    ///
    /// let block = BlockBuilder::new()
    ///     .set_mass_density(1.2)
    ///     .set_lengths(1., 0.5, 0.25)
    ///     .get();
    ///
    /// assert!((block.mass - 1.2 * 0.125).abs() < 1e-12);
    /// assert!((block.lengths[0] - 1.0).abs() < 1e-12);
    /// assert!((block.lengths[1] - 0.5).abs() < 1e-12);
    /// assert!((block.lengths[2] - 0.25).abs() < 1e-12);
    /// assert!(block.position.coords.norm() < 1e-12);
    /// assert!(block.velocity.coords.norm() < 1e-12);
    /// ```
    pub fn get(&mut self) -> Block
    {
        // Computing block mass from mass density.
        self.block.mass *= self.block.get_volume();

        // Returning built block.
        let built_block = self.block.clone();
        self.block = Block::default();
        built_block
    }
}

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// Implementation of block services.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

impl Block {
    /// Computing block volume.
    ///
    /// # Examples
    /// ```
    /// use rody::block::*;
    ///
    /// let block = BlockBuilder::new()
    ///     .set_lengths(1., 0.5, 0.25)
    ///     .get();
    ///
    /// assert!((block.get_volume() - 0.125).abs() < 1e-12);
    /// ```
    pub fn get_volume(&self) -> f64
    {
        self.lengths[0] * self.lengths[1] * self.lengths[2]
    }

    /// Creating formatter of current block instance.
    ///
    /// * `data_str` - TO DO !
    ///
    pub fn format(&self, data_str: &str, decimal: usize) -> BlockFormatter
    {
        BlockFormatter{ block: &self, data_index: BlockFormatter::parse_data_str(data_str), decimal: decimal }
    }
}

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// Implementation of block internal data formatter.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

use std::fmt;

impl<'a> BlockFormatter<'a> {
    /// Parsing input data string to data index.
    ///
    fn parse_data_str(data_str: &str) -> Vec<u8>
    {
        let mut data_index = Vec::new();
        let split : Vec<&str> = data_str.split_whitespace().collect();
        for s in split.iter()
        {
            match &*String::from(*s).to_lowercase() {
                "_" => for i in 0..6 { data_index.push(i); },
                "p" => for i in 0..3 { data_index.push(i); },
                "v" => for i in 3..6 { data_index.push(i); },
                "px" => data_index.push(0),
                "py" => data_index.push(1),
                "pz" => data_index.push(2),
                "vx" => data_index.push(3),
                "vy" => data_index.push(4),
                "vz" => data_index.push(5),
                _ => (),
            };
        }
        data_index
    }
}

impl<'a> fmt::Display for BlockFormatter<'a> {
    /// Implementation of display trait for a block formatter.
    ///
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        for index in self.data_index.iter()
        {
            match *index {
                0 => write!(f, " {:.*} ", self.decimal, self.block.position.coords.x).unwrap(),
                1 => write!(f, " {:.*} ", self.decimal, self.block.position.coords.y).unwrap(),
                2 => write!(f, " {:.*} ", self.decimal, self.block.position.coords.z).unwrap(),
                3 => write!(f, " {:.*} ", self.decimal, self.block.velocity.coords.x).unwrap(),
                4 => write!(f, " {:.*} ", self.decimal, self.block.velocity.coords.y).unwrap(),
                5 => write!(f, " {:.*} ", self.decimal, self.block.velocity.coords.z).unwrap(),
                _ => (),
            };
        }
        Ok(())
    }
}
