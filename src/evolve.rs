
use crate::config::Config;
use crate::output::Output;
use crate::environment::Environment;
use crate::gpm::Gpmap;
use crate::pop::Population;
use std::error::Error;

use rand::prelude::*;
use rand::distributions::WeightedIndex;
use statrs::distribution::Poisson;




pub struct Simulation{
    config: Config,
    output: Output,
    env: Environment,
    gpmap: Gpmap,
    initpop: Population,
}


impl Simulation{
    pub fn setup(cfg: Config,output: Output,env: Environment,gpm: Gpmap,pop: Population) -> Result<Simulation,Box<dyn Error>>{
        Ok(Simulation {config:cfg, output:output, env:env, gpmap:gpm, initpop:pop })
    }

    pub fn run(&self){

        let mut envid = self.env.get_envid(0,-1); // Get environment at start
        let mut pfmap = self.env.get_pfmap(envid); // Get pffmap being used at start
        let mut gfmap = self.gpmap.get_gfmap(pfmap); // Get gfmap being used at the start
        let mut pop = self.initpop.clone(); // Make a copy of the initial population
        self.output.generate_output(0,&self.config,envid,&pop,&self.gpmap); // Generate an output

        for time in 1..(self.config.maxgens+1){
            println!("{}",time);
            // Note: Population selects on environment from last generation to create this generation!

            // SELECTION AND MUTATION
            self.select_mutate(&mut pop,&gfmap); // Perform selection and mutation on population using pfmap
            pop.add_to_average(&self.config,time); // Add new population to population average (checked if required using time)

            // UPDATE ENVIRONMENT
            envid = self.env.get_envid(time,envid); // Get environment from env
            pfmap = self.env.get_pfmap(envid); // Get pffmap
            gfmap = self.gpmap.get_gfmap(pfmap);

            // OUTPUTS
            let mut outflag = false;
            if time%self.config.saveevery==0 {
                self.output.generate_output(time,&self.config,envid,&pop,&self.gpmap);
                outflag = true;
            }
            // Output if not saved by saveevery but is final generation
            if time==self.config.maxgens && !outflag {
                self.output.generate_output(time,&self.config,envid,&pop,&self.gpmap);
            }
        }
    }

    pub fn select_mutate(&self,pop: &mut Population,gfmap: &Vec<Vec<f64>>){
        let mut gtypes_in_pop:Vec<(u64,u64)> = Vec::new();
        let mut weights: Vec<f64> = Vec::new();


        for (idx,row) in gfmap.iter().enumerate(){
            for (idy,fitness) in row.iter().enumerate(){
                let pop_at_xy = pop.get_at(idx as u64, idy as u64);
                if pop_at_xy!=0{
                    gtypes_in_pop.push((idx as u64,idy as u64));
                    weights.push(*fitness*(pop_at_xy as f64));
                }
            }
        }

        let dist = WeightedIndex::new(&weights).unwrap();
        let mut rng = thread_rng();

        let sampled_indices: Vec<usize> = dist.sample_iter(&mut rng).take(self.config.popsize as usize).collect();
        
        let mut new_pop: Vec<Vec<u64>> = vec![vec![0; self.config.grid_y as usize];self.config.grid_x as usize];

        for smp_index in sampled_indices.iter(){
            let (mut x, mut y) = gtypes_in_pop[*smp_index];


            let lamb = (1.00/(1.00-self.config.mutprob)).ln();
            let k = Poisson::new(lamb).unwrap(); // Pick from a geometric dist with p(k) = m^k (1-m)^k


            for _ in 0..k.sample(&mut rand::thread_rng()) as usize{
                self.mutate_xy(&mut x,&mut y);
            }


            new_pop[x as usize][y as usize] += 1;
        }

        pop.set_pop(new_pop);
    }

    pub fn mutate_xy(&self,x: &mut u64, y: &mut u64){
        let dir: i32 = *vec![0,1,2,3].choose(&mut rand::thread_rng()).unwrap() as i32;
    
        
        match dir{
            0 => { // North
                if *y == self.config.grid_y-1{
                    *y = 0;
                } else {
                    *y +=1;
                }
            }
            1 => { // East
                if *x == self.config.grid_x-1{
                    *x = 0;
                } else {
                    *x +=1;
                }
            }
            2 => { // South
                if *y == 0{
                    *y = self.config.grid_y-1;
                } else {
                    *y -=1;
                }
            }
            3 => { // West
                if *x == 0{
                    *x = self.config.grid_x-1;
                } else {
                    *x -=1;
                }
            }
            _ => {
            }
        }
    }
}


