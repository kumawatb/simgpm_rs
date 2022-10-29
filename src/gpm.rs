use std::{error::Error, fs};
use crate::config::Config;
use rand::seq::SliceRandom;

pub struct Gpmap{
    gpmap: Vec<Vec<u64>>, // For a grid implementation the pid is obtained from a 2d vector where first index is x and second index is y
}


impl Gpmap{
    pub fn parse_gpmap(cfg: &Config) -> Result<Gpmap,Box<dyn Error>>{
        let mut gpmvec:Vec<Vec<u64>> = vec![vec![0; cfg.grid_y as usize]; cfg.grid_x as usize]; // Create an empty gpmap

        let gpmcfgcontent = fs::read_to_string(&cfg.gpfilepath)?;

        for(idy,row) in gpmcfgcontent.lines().enumerate(){
            let row_as_vec: Vec<&str> = row.split(",").collect();
            for(idx,cellval) in row_as_vec.iter().enumerate(){
                gpmvec[idx as usize][idy as usize]= cellval.parse::<u64>().unwrap();
            }
        }

        let gpmap: Gpmap = Gpmap { gpmap: gpmvec };
        Ok(gpmap)
    }


    pub fn get_random_xy(&self,vec_pids: &Vec<u64>) -> (u64,u64){
        let mut all_matching_xys: Vec<(u64,u64)> = Vec::new();

        for (idx,row) in self.gpmap.iter().enumerate(){
            for(idy,pidstr) in row.iter().enumerate(){
                if vec_pids.contains(pidstr){
                    all_matching_xys.push((idx as u64,idy as u64));
                }
            }
        }

        let chosen_xy = all_matching_xys.choose(&mut rand::thread_rng()).unwrap();
        *chosen_xy
    }


    pub fn get_gfmap(&self, pfmap: &Vec<f64>) -> Vec<Vec<f64>>{
        let mut gfmap: Vec<Vec<f64>> = vec![vec![0.0; self.gpmap[0].len()]; self.gpmap.len()];

        for (idx,row) in self.gpmap.iter().enumerate(){
            for (idy, pid) in row.iter().enumerate(){
                gfmap[idx][idy] = pfmap[*pid as usize];
            }
        }
        
        gfmap
    }
}