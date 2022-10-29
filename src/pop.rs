use std::{error::Error, fs};
use crate::config::Config;
use crate::gpm::Gpmap;



#[derive(Clone)]
pub struct Population{
    pop: Vec<Vec<u64>>,
}





impl Population{

    pub fn gen_initpop(cfg:&Config,gpmap:&Gpmap) -> Result<Population,Box<dyn Error>>{
        let mut popvec:Vec<Vec<u64>> = vec![vec![0; cfg.grid_y as usize]; cfg.grid_x as usize]; // Create an empty population

        // Generate initial population based on popsize, and rndstrtpheno
        // Pick a random xy from rndstrtpheno genotypes
        let (randx,randy) = gpmap.get_random_xy(&cfg.rndstrtpheno);

        // Set popvec at rand x y to popsize
        popvec[randx as usize][randy as usize] = cfg.popsize;

        let pop: Population = Population { pop: popvec };
        Ok(pop)
    }

    pub fn get_dominance_info(&self) -> (usize,usize,u64){

        let mut max_x:usize = 0;
        let mut max_y:usize = 0;
        let mut max_abund:u64 = 0;

        for (idx,row) in self.pop.iter().enumerate(){
            for (idy,popincell) in row.iter().enumerate(){
                if *popincell>max_abund{
                    max_x = idx;
                    max_y = idy;
                    max_abund = *popincell;
                }
            }
        }

        (max_x,max_y,max_abund)
    }

    pub fn get_at(&self,x: u64, y: u64) -> u64{
        self.pop[x as usize][y as usize]
    }

    pub fn set_pop(&mut self, new_pop: Vec<Vec<u64>>){
        self.pop = new_pop;
    }

}