use std::io;
use std::io::{prelude::*, BufReader};
use std::fs::File;

use super::markovchain::MarkovChain;

pub struct MarkovChainReader;

#[allow(dead_code)]
impl MarkovChainReader
{
    pub fn from_string(value : String) -> Box<MarkovChain>
    {
        let mut mc = MarkovChain::new();
        mc.feed_string(value);
        Box::new(mc)
    }

    pub fn from_file(filepath: &String) -> Result<Box<MarkovChain>, io::Error> 
    {
        let mut mc = MarkovChain::new();
        let file = File::open(filepath)?;
        let reader = BufReader::new(file);
    
        for line in reader.lines() {
            match line
            {
                Ok(s) => mc.feed_string(s),
                Err(e) => return Err(e.into())
            }
        }
    
        let boxed_mc = Box::new(mc);
        Ok(boxed_mc)
    }
}
