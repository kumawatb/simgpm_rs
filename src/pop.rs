use std::error::Error;
use crate::config::Config;
use crate::gpm::Gpmap;
use rand::{distributions::{Distribution, Uniform}};



#[derive(Clone)]
pub struct Population{
    pop: Vec<Vec<u64>>,
    avgpop: Vec<Vec<f64>>
}



impl Population{

    pub fn gen_initpop(cfg:&Config,gpmap:&Gpmap) -> Result<Population,Box<dyn Error>>{
        let mut popvec:Vec<Vec<u64>> = vec![vec![0; cfg.grid_y as usize]; cfg.grid_x as usize]; // Create an empty population
        let avgpopvec:Vec<Vec<f64>> = vec![vec![0.0; cfg.grid_y as usize]; cfg.grid_x as usize]; // Create an empty averaged population

        // Generate initial population based on popsize, and rndstrtpheno
        // Pick a random xy from rndstrtpheno genotypes
        let (randx,randy) = gpmap.get_random_xy(&cfg.rndstrtpheno);

        // Set popvec at rand x y to popsize
        popvec[randx as usize][randy as usize] = cfg.popsize;

        let pop: Population = Population { pop: popvec, avgpop: avgpopvec };
        Ok(pop)
    }

    pub fn gen_uniform(cfg: &Config) -> Result<Population, Box<dyn Error>>{ 
        let mut popvec: Vec<Vec<u64>> = vec![vec![0; cfg.grid_y as usize]; cfg.grid_x as usize]; // Create empty
        let avgpopvec:Vec<Vec<f64>> = vec![vec![0.0; cfg.grid_y as usize]; cfg.grid_x as usize]; // Create an empty averaged population

        let dist_x = Uniform::from(0..cfg.grid_x as usize);
        let dist_y = Uniform::from(0..cfg.grid_y as usize);

        let mut rng = rand::thread_rng();
        for _ in 0..cfg.popsize{
            popvec[dist_x.sample(&mut rng)][dist_y.sample(&mut rng)] += 1;
        }

        let pop: Population = Population { pop: popvec, avgpop: avgpopvec };
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

    pub fn avg_get_at(&self,x: u64, y: u64) -> f64{
        self.avgpop[x as usize][y as usize]
    }

    pub fn set_pop(&mut self, new_pop: Vec<Vec<u64>>){
        self.pop = new_pop;
    }

    pub fn add_to_average(&mut self, config: &Config, time: u64){
        if time==config.timeavgstart {
            self.avgpop = self.pop.iter().map(|a| a.iter().map(|&b| b as f64).collect()).collect();
        }
        if time>config.timeavgstart {
            let t_elapsed = time-config.timeavgstart;
            self.avgpop.iter_mut().for_each(|a| a.iter_mut().for_each(|b| *b *= t_elapsed as f64)); // Multiply avg_pop (in place) by time elapsed since first pop added

            // Add current pop to avg
            for idx in 0..config.grid_x{
                for idy in 0..config.grid_y{ 
                    self.avgpop[idx as usize][idy as usize]+= self.pop[idx as usize][idy as usize] as f64; // Add current population to the summed average
                }
            }

            
            self.avgpop.iter_mut().for_each(|a| a.iter_mut().for_each(|b| *b /= (t_elapsed+1) as f64 ));  // Divide by time_elapsed + 1 to get final average
        }
    }

}