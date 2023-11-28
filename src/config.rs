
use std::error::Error;
use std::fmt;
use std::path::PathBuf;
use argparse::{ArgumentParser, Store, List};



pub struct Config {
    // GP-map parameters

    pub grid_x: u64, // Grid size in X
    pub grid_y: u64, // Grid size in Y


    
    // File paths

    pub gpfilepath:  PathBuf, // Path to GP-map file
    pub envfilepath: PathBuf, // Path to Env file

    // Evolutionary parameters
    pub popsize: u64, // Population Size
    pub mutprob: f64, // Probability of a genotype changing per generation



    // Simulation Parameters
    pub maxgens: u64, // Maximum generations to run simulation for
    pub saveevery: u64, // Time interval between consecutive population saves



    // Output file options
    pub outdom: bool, // Output a file containing dominant genotypes?
    pub outevol: bool, // Output a file containing evolvabilities?
    pub outenv: bool, // Output a file containing environment states?
    pub outaltmut: bool, // Output a file containing alternate mutation probabilities?
    pub outpopsave: bool, // Output population saves?
    pub outtimeavg:bool, // Output time-averages?

    // Time-average options
    pub timeavgstart: u64, // Generation when to start time-averaging of population
    pub timeavgend: u64, // Generation when to end time-averaginf of population


    // // Misc parameters
    pub rndstrtpheno: Vec<u64>, // start with population in a random genotypes belonging to a certain phenotype?
    pub replid: u64, // If using only one replicate should this replicate be assigned an ID? 
}


impl Default for Config{
    fn default() -> Config {
        Config{
            grid_x: 40,
            grid_y: 20,
            
            gpfilepath: PathBuf::from("./config/gpm.csv"),
            envfilepath: PathBuf::from("./config/env.cfg"),

            popsize: 10000,
            mutprob: 0.01,

            maxgens: 1000000,
            saveevery: 1,

            outdom: true,
            outevol: true,
            outenv: true,
            outaltmut: true,
            outpopsave: true,
            outtimeavg: true,

            timeavgstart:0,
            timeavgend:1000000,

            rndstrtpheno: vec![0,1], // Phenotype id 0 denotes inviable genotypes
            replid: 0
        }
    }
}


impl Config{
    pub fn getcli() -> Result<Config,Box<dyn Error>>{
        let mut config = Config::default();
        {
            let mut ap = ArgumentParser::new();
            ap.set_description("SimGpm: A program to simulate evolution on arbitrary genotype-phenotype maps");

            ap.refer(&mut config.grid_x).add_option(&["-x","--grid_x"],Store,"Grid size in x");
            ap.refer(&mut config.grid_y).add_option(&["-y","--grid_y"],Store,"Grid size in y");

            ap.refer(&mut config.gpfilepath).add_option(&["-g","--gpfilepath"],Store,"Path to the genotype-phenotype map file");
            ap.refer(&mut config.envfilepath).add_option(&["-e","--envfilepath"],Store,"Path to the environment file");

            ap.refer(&mut config.popsize).add_option(&["-p","--popsize"],Store,"Population size");
            ap.refer(&mut config.mutprob).add_option(&["-m","--mutprob"],Store,"Mutation probability");
            
            ap.refer(&mut config.maxgens).add_option(&["-t","--maxgens"],Store,"Maximum generations to run the simulation for");
            ap.refer(&mut config.saveevery).add_option(&["-s","--saveevery"],Store,"Time interval duration to create population saves (in generations)");
            ap.refer(&mut config.outdom).add_option(&["-d","--outdom"],Store,"0/1, output file containing dominant genotypes?");
            ap.refer(&mut config.outevol).add_option(&["-v","--outevol"],Store,"0/1, output file containing evolvabilities?");
            ap.refer(&mut config.outenv).add_option(&["-n","--outenv"],Store,"0/1, output file containing environments?");
            ap.refer(&mut config.outaltmut).add_option(&["-a","--outaltmut"],Store,"0/1, output file containing alternate mutant probabilities?");
            ap.refer(&mut config.outpopsave).add_option(&["-o","--outpopsave"],Store,"0/1, output file containing population saves?");
            ap.refer(&mut config.outtimeavg).add_option(&["--outtimeavg"],Store, "0/1, output file containing the time average?");

            ap.refer(&mut config.timeavgstart).add_option(&["--timeavgstart"],Store,"Generation to start time-averaging at");
            ap.refer(&mut config.timeavgend).add_option(&["--timeavgend"],Store,"Generation to end time-averaging at");

            ap.refer(&mut config.rndstrtpheno).add_option(&["-h","--rndstrtpheno"],List,"Start simulation with all organisms having a random genotype from given phenotypes?");
            ap.refer(&mut config.replid).add_option(&["-l","--replid"],Store,"If only one replicate, should this replicate be given an id?");


            ap.parse_args_or_exit(); 
        }
        Ok(config)
    }

}

impl fmt::Display for Config{
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"--grid_x {} --grid_y {} --gpfilepath {:?} --envfilepath {:?} --popsize {} --mutprob {} --maxgens {} --saveevery {}  --outdom {} --outevol {} --outenv {} --outaltmut {} --outpopsave  {} --outtimeavg {} --timeavgstart {} --timeavgend {} --rndstrtpheno {:?} --replid {}",
            self.grid_x,
            self.grid_y,
            self.gpfilepath,
            self.envfilepath,
            self.popsize,
            self.mutprob,
            self.maxgens,
            self.saveevery,
            self.outdom,
            self.outevol,
            self.outenv,
            self.outaltmut,
            self.outpopsave,
            self.outtimeavg,
            self.timeavgstart,
            self.timeavgend,
            self.rndstrtpheno,
            self.replid    
        )
    } 
}