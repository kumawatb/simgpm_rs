use simgpm::config::Config;
use simgpm::output::Output;
use simgpm::environment::Environment;
use simgpm::gpm::Gpmap;
use simgpm::pop::Population;
use simgpm::evolve::Simulation;
use std::fs;
use std::io::Write;
use std::process;




fn main() {

    print!("Initializing config... ");
    // Get configuration values from command-line
    let cfg = Config::getcli().unwrap_or_else(|err|{
        println!("Problem getting config from command line: {}",err);
        process::exit(1);
    });
    println!("Done!");


    print!("Creating output targets... ");
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
    println!("Done!");

    print!("Parsing environment file.. ");
    // Parse environment (and copy all pffiles to output directory)
    let env = Environment::parse_env(&cfg).unwrap_or_else(|err|{
        println!("Could not parse the environment file: {}",err);
        process::exit(1);
    });
    println!("Done!");

    print!("Parsing genotype-phenotype map.. ");
    // Parse genotype-phenotype map 
    let gpmap: Gpmap = Gpmap::parse_gpmap(&cfg).unwrap_or_else(|err|{
        println!("Could not parse the gpmap file: {}",err);
        process::exit(1);
    });
    println!("Done!");


    print!("Generating initial population.. ");
    // Parse/create initial population file
    let pop: Population = Population::gen_uniform(&cfg).unwrap_or_else(|err| {
        println!("Could not generate initial population!: {}", err);
        process::exit(1);
    });
    println!("Done!");

    print!("Running simulation...");
    // Run simulation(s) and save output files
    let sim: Simulation = Simulation::setup(cfg,output,env,gpmap,pop).unwrap_or_else(|err|{
        println!("Could not setup the simulation!: {}",err);
        process::exit(1);
    });
    sim.run();
    println!("Done!");
    

}
