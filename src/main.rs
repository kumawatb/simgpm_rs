use simgpm::config::Config;
use simgpm::output::Output;
use std::fs;
use std::io::Write;
use std::process;



fn main() {

    
    // Get configuration values from command-line
    let cfg = Config::getcli().unwrap_or_else(|err|{
        println!("Problem getting config from command line: {}",err);
        process::exit(1);
    });


    // Create an output directory
    fs::create_dir_all("output").unwrap();

    // Create a copy of all config files in output directory
    fs::copy(&cfg.gpfilepath,"./output/gpm.csv").unwrap();
    fs::copy(&cfg.envfilepath,"./output/env.cfg").unwrap();

    // Create a cmd.log in output directory (containing all parameters used)
    let mut logfile = fs::File::create("output/cmd.log").unwrap();
    logfile.write(format!("{}",cfg).as_bytes()).unwrap();

    // Create output files based on config (hdf5 format)
    let output = Output::createfiles(&cfg).unwrap_or_else(|err|{
        println!("Could not create output files: {}",err);
        process::exit(1);
    });

    // Parse environment (and copy all pffiles to output directory)

    // Parse genotype-phenotype map

    // Parse initial population file

    // Run simulation(s)


}
