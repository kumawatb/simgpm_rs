
use std::error::Error;
use std::fmt;
use std::path::PathBuf;
use argparse::{ArgumentParser, Store, List};



pub struct Config {
    // GP-map parameters

    pub grid_x: u32, // Grid size in X
    pub grid_y: u32, // Grid size in Y


    
    // File paths

    pub gpfilepath:  PathBuf, // Path to GP-map file
    pub envfilepath: PathBuf, // Path to Env file
    pub initfilepath: PathBuf, // Path to initial population file



    // Evolutionary parameters
    pub popsize: u64, // Population Size
    pub mutprob: f64, // Probability of a genotype changing per generation



    // Simulation Parameters
    pub maxgens: u64, // Maximum generations to run simulation for
    pub saveevery: u64, // Time interval between consecutive population saves
    pub numrepl: u32, // Number of replicates



    // Output file options
    pub outdom: bool, // Output a file containing dominant genotypes?
    pub outevol: bool, // Output a file containing evolvabilities?
    pub outenv: bool, // Output a file containing environment states?
    pub outaltmut: bool, // Output a file containing alternate mutation probabilities?
    pub outpopsave: bool, // Output population saves?



    // // Misc parameters
    pub rndstrtpheno: Vec<u32>, // start with population in a random genotypes belonging to a certain phenotype?
    pub replid: i32, // If using only one replicate should this replicate be assigned an ID? 
}


impl Default for Config{
    fn default() -> Config {
        Config{
            grid_x: 40,
            grid_y: 40,
            
            gpfilepath: PathBuf::from("./config/gpm.csv"),
            envfilepath: PathBuf::from("./config/env.cfg"),
            initfilepath: PathBuf::from("./config/initfile.csv"),

            popsize: 1000000,
            mutprob: 0.1,

            maxgens: 100000000,
            saveevery: 1,
            numrepl: 1,

            outdom: true,
            outevol: true,
            outenv: true,
            outaltmut: true,
            outpopsave: true,

            rndstrtpheno: vec![2], // Phenotype id 0 denotes inviable genotypes
            replid: -1
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
            ap.refer(&mut config.initfilepath).add_option(&["-i","--initfilepath"],Store,"Path to the initial population file");

            ap.refer(&mut config.popsize).add_option(&["-p","--popsize"],Store,"Population size");
            ap.refer(&mut config.mutprob).add_option(&["-m","--mutprob"],Store,"Mutation probability");
            
            ap.refer(&mut config.maxgens).add_option(&["-t","--maxgens"],Store,"Maximum generations to run the simulation for");
            ap.refer(&mut config.saveevery).add_option(&["-s","--saveevery"],Store,"Time interval duration to create population saves (in generations)");
            ap.refer(&mut config.numrepl).add_option(&["-r","--numrepl"],Store,"Number of replicates to run");

            ap.refer(&mut config.outdom).add_option(&["-d","--outdom"],Store,"0/1, output file containing dominant genotypes?");
            ap.refer(&mut config.outevol).add_option(&["-v","--outevol"],Store,"0/1, output file containing evolvabilities?");
            ap.refer(&mut config.outenv).add_option(&["-n","--outenv"],Store,"0/1, output file containing environments?");
            ap.refer(&mut config.outaltmut).add_option(&["-a","--outaltmut"],Store,"0/1, output file containing alternate mutant probabilities?");
            ap.refer(&mut config.outpopsave).add_option(&["-o","--outpopsave"],Store,"0/1, output file containing population saves?");

            ap.refer(&mut config.rndstrtpheno).add_option(&["-h","--rndstrtpheno"],List,"Start simulation with all organisms having a random genotype from given phenotypes?");
            ap.refer(&mut config.replid).add_option(&["-l","--replid"],Store,"If only one replicate, should this replicate be given an id?");


            ap.parse_args_or_exit();
        }
        Ok(config)
    }

}

impl fmt::Display for Config{
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"--grid_x {} --grid_y {} --gpfilepath {:?} --envfilepath {:?} --initfilepath {:?} --popsize {} --mutprob {} --maxgens {} --saveevery {} --numrepl {} --outdom {} --outevol {} --outenv {} --outaltmut {} --outpopsave  {} --rndstrtpheno {:?} --replid {}",
            self.grid_x,
            self.grid_y,
            self.gpfilepath,
            self.envfilepath,
            self.initfilepath,
            self.popsize,
            self.mutprob,
            self.maxgens,
            self.saveevery,
            self.numrepl,
            self.outdom,
            self.outevol,
            self.outenv,
            self.outaltmut,
            self.outpopsave,
            self.rndstrtpheno,
            self.replid    
        )
    } 
}