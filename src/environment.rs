use crate::config::Config;
use std::fmt;
use std::{error::Error, fs};
use std::path::PathBuf;
use std::collections::HashMap;
use rand::Rng;

enum EnvRegimeType{
    Constant, // A constant environment
    DetCyclic, // Cyclic environments with deterministic durations
    ProbSwitch, // Environments switch cyclically with certain transition probabilities (in order)
}

pub struct Environment{
    regimetype: EnvRegimeType, // Environment change regime type
    pfmaps: Vec<Vec<f64>>,     // Phenotype fitness maps (first index is envid, second index is pid)
    cyctimes: Option<Vec<u64>>,        // Cycling times (might be null)
    weights: Option<Vec<f64>>,         // Probabilities (might be null)
}

impl Environment{
    pub fn parse_env(cfg: &Config) -> Result<Environment,Box<dyn Error>>{
        let mut regimetype: Option<EnvRegimeType> = None;
        let mut pfmaps: Vec<Vec<f64>> = Vec::new();
        let mut cyctimes: Option<Vec<u64>> = None;
        let mut weights: Option<Vec<f64>> = None;



        // Read env cfg and get pf definitions and type of regime
        let envcfgcontent = fs::read_to_string(&cfg.envfilepath)?;
        let mut pfpaths: HashMap<&str,PathBuf> = HashMap::new();

        let mut typelineidx: Option<u64> = None;
        let mut endlineidx: Option<u64> = None;

        for (idx,line) in envcfgcontent.lines().enumerate(){
            let line_as_vec : Vec<&str> = line.split(" ").collect();

            match &line_as_vec[0][..]{
                "define" => {
                    let key = line_as_vec[1];
                    let fpath = PathBuf::from(line_as_vec[2]);
                    pfpaths.insert(key,fpath);
                },
                "type"   => {
                    let typekword = line_as_vec[1];
                    typelineidx = Some(idx as u64);
                    
                    match typekword{
                        "const" => {regimetype=Some(EnvRegimeType::Constant)},
                        "cycle" => {regimetype=Some(EnvRegimeType::DetCyclic)},
                        "switch_prob" => {regimetype=Some(EnvRegimeType::ProbSwitch)},
                        _  => {},
                    }
                },
                "end" => {
                    endlineidx = Some(idx as u64);
                }
                _=>{},
            }
        }


        // Copy pffiles to output directory
        for (k,v) in &pfpaths{
            let pffname = v.file_name().unwrap().to_str().unwrap();
            fs::copy(v,format!("./output/{pffname}")).unwrap();
        }


        // Parse env regime definition based on type
        let startidx = typelineidx.unwrap();
        let endidx = endlineidx.unwrap();

        let linevec: Vec<&str> = envcfgcontent.lines().collect();


        match regimetype.as_ref().unwrap(){
            // PARSE CONSTANT REGIME
            EnvRegimeType::Constant => {
                // Get each line in regimedef and convert to a pfmaps (a hashmap)
                for regline in &linevec[(startidx+1) as usize..endidx as usize]{
                    let regline_as_vec: Vec<&str> = regline.split(" ").collect();

                    

                    let pfpath = pfpaths.get(regline_as_vec[0]).unwrap();
                    let pffile = fs::read_to_string(pfpath)?;
    
                    let mut pfmap: Vec<f64> = vec![0.0; pffile.lines().count()];

                    for record in pffile.lines(){
                        let rec_as_vec : Vec<&str> = record.split(",").collect();
                        let pheno_id = rec_as_vec[0].parse::<u64>().unwrap();
                        let fitness = rec_as_vec[1].parse::<f64>().unwrap();
                        pfmap[pheno_id as usize] = fitness;
                    }
                    pfmaps.push(pfmap);
                }

                println!("{:?}",pfmaps);
            },

            // PARSE CYCLIC REGIME
            EnvRegimeType::DetCyclic => {
                let mut cyctimes_temp:Vec<u64> = Vec::new();

                for regline in &linevec[(startidx+1) as usize..endidx as usize]{
                    let regline_as_vec: Vec<&str> = regline.split(" ").collect();

                    let pfpath = pfpaths.get(regline_as_vec[1]).unwrap();
                    let pffile = fs::read_to_string(pfpath)?;

                    let mut pfmap: Vec<f64> = vec![0.0; pffile.lines().count()];
    
                    for record in pffile.lines(){
                        let rec_as_vec : Vec<&str> = record.split(",").collect();
                        let pheno_id = rec_as_vec[0].parse::<u64>().unwrap();
                        let fitness = rec_as_vec[1].parse::<f64>().unwrap();
                        pfmap[pheno_id as usize]=fitness;
                    }
                    pfmaps.push(pfmap);

                    cyctimes_temp.push(regline_as_vec[0].parse::<u64>().unwrap());

                }
                cyctimes = Some(cyctimes_temp);
            },

            // PARSE PROBABILISTICALLY SWITCHING REGIME
            EnvRegimeType::ProbSwitch => {
                let mut weights_temp:Vec<f64> = Vec::new();

                for regline in &linevec[(startidx+1) as usize..endidx as usize]{
                    let regline_as_vec: Vec<&str> = regline.split(" ").collect();

                    let pfpath = pfpaths.get(regline_as_vec[1]).unwrap();
                    let pffile = fs::read_to_string(pfpath)?;

                    let mut pfmap: Vec<f64> = vec![0.0; pffile.lines().count()];
    
                    for record in pffile.lines(){
                        let rec_as_vec : Vec<&str> = record.split(",").collect();
                        let pheno_id = rec_as_vec[0].parse::<u64>().unwrap();
                        let fitness = rec_as_vec[1].parse::<f64>().unwrap();
                        pfmap[pheno_id as usize]=fitness;
                    }
                    pfmaps.push(pfmap);

                    weights_temp.push(regline_as_vec[0].parse::<f64>().unwrap());

                }
                weights = Some(weights_temp);
            }
        }

        // Convert to struct and return
        let env = Environment { 
            regimetype: regimetype.unwrap(), 
            pfmaps: pfmaps, 
            cyctimes: cyctimes, 
            weights: weights
        };

        Ok(env)
    }

    pub fn get_envid(&self,time: u64,last_envid: i32)-> i32{
        match self.regimetype{


            // Constant environment
            EnvRegimeType::Constant => {
                return 0
            },



            // Determinstic cyclic environment 
            EnvRegimeType::DetCyclic => {
                // Implement this from the julia version if required
                todo!();
                return 0
            },


            // Probabilistically switching environment
            EnvRegimeType::ProbSwitch => {
                let mut new_envid = -1;
                if last_envid!=-1{
                    let switch_prob = self.weights.as_ref().unwrap()[last_envid as usize];
                    let next_envid = if last_envid<(self.weights.as_ref().unwrap().len() as i32 -1){
                        last_envid+1
                    } else {
                        0
                    };
                    
                    new_envid = if switch_prob>rand::thread_rng().gen::<f64>() {
                        next_envid
                    } else{
                        last_envid
                    }

                }
                else{ // If no envid chose, sample randomly
                    new_envid = rand::thread_rng().gen_range(0..self.weights.as_ref().unwrap().len() as i32);
                }
                return new_envid
            }
        }
        
    }

    pub fn get_pfmap(&self,envid: i32) -> &Vec<f64> {
        return &self.pfmaps[envid as usize];
    }
}

