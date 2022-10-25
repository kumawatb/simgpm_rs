use hdf5::{File,H5Type,Result};
use crate::config::Config;
use std::error::Error;

#[derive(Default)]
pub struct Output{
    pub domfile: Option<File>,
    pub evolfile: Option<File>,
    pub envfile: Option<File>,
    pub altmutfile: Option<File>,
    pub popfile: Option<File>

}

impl Output{
    pub fn createfiles(config: &Config) -> Result<Output, Box<dyn Error>>{
        let mut output = Output::default();

        if config.outdom{output.domfile = Some(File::create("./output/dominant.h5").unwrap());}
        if config.outevol{output.evolfile= Some(File::create("./output/evolvability.h5").unwrap());}
        if config.outenv{output.envfile = Some(File::create("./output/environment.h5").unwrap());}
        if config.outaltmut{output.altmutfile = Some(File::create("./output/altmut.h5").unwrap());}
        if config.outpopsave{output.popfile = Some(File::create("./output/population.h5").unwrap());}

        Ok(output)
    }
}