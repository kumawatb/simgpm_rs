use std::{error::Error, fs,cmp};
use crate::config::Config;
use rand::seq::SliceRandom;
use statrs::distribution::{Poisson,Discrete};

pub struct Gpmap{
    gpmap: Vec<Vec<u64>>, // For a grid implementation the pid is obtained from a 2d vector where first index is x and second index is y
    pid_list: Vec<u64>
}


impl Gpmap{
    pub fn parse_gpmap(cfg: &Config) -> Result<Gpmap,Box<dyn Error>>{
        let mut gpmvec:Vec<Vec<u64>> = vec![vec![0; cfg.grid_y as usize]; cfg.grid_x as usize]; // Create an empty gpmap
        let mut pid_list: Vec<u64> = Vec::new();

        let gpmcfgcontent = fs::read_to_string(&cfg.gpfilepath)?;

        for(idy,row) in gpmcfgcontent.lines().enumerate(){
            let row_as_vec: Vec<&str> = row.split(",").collect();
            for(idx,cellval) in row_as_vec.iter().enumerate(){
                let pid = cellval.parse::<u64>().unwrap();
                gpmvec[idx as usize][idy as usize]= pid;
                if !pid_list.contains(&pid){
                    pid_list.push(pid);
                }
            }
        }

        let gpmap: Gpmap = Gpmap { gpmap: gpmvec, pid_list: pid_list };
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

    pub fn get_evol_at(&self, cfg:&Config, x: u64, y: u64) -> f64 {
        // get self pid
        let self_pid = self.gpmap[x as usize][y as usize];
        // get number of neighbors that have different pid
        let mut num_diff:f64  = 0.0;

        let north_pid:u64;
        let east_pid:u64;
        let south_pid:u64;
        let west_pid:u64;

        // Check north neighbor
        if y==cfg.grid_y-1{
            north_pid = self.gpmap[x as usize][0];
        } else {
            north_pid = self.gpmap[x as usize][(y+1) as usize];
        }

        // Check east neighbor
        if x==cfg.grid_x-1{
            east_pid = self.gpmap[0][y as usize];
        } else {
            east_pid = self.gpmap[(x+1) as usize][y as usize];
        }


        // Check south neighbor 
        if y==0{
            south_pid = self.gpmap[x as usize][(cfg.grid_y-1) as usize];
        } else {
            south_pid = self.gpmap[x as usize][(y-1) as usize];
        }

        // Check west neighbor
        if x==0{
            west_pid = self.gpmap[(cfg.grid_x-1) as usize][y as usize];
        } else {
            west_pid = self.gpmap[(x-1) as usize][y as usize];
        }

        if north_pid!=self_pid {num_diff += 1.0;}
        if east_pid!=self_pid {num_diff += 1.0;}
        if south_pid!=self_pid {num_diff += 1.0;}
        if west_pid!=self_pid {num_diff += 1.0;}

        num_diff/4.0
    }

    pub fn get_mutprob(&self, cfg:&Config, target_pid: u64, x_self: u64, y_self: u64) -> f64 {
        let mut mutprob: f64 = 0.0;

        for (idx,row) in self.gpmap.iter().enumerate(){
            for (idy, pid) in row.iter().enumerate(){
                if *pid==target_pid{
                    let del_x = cmp::min((x_self as i32-idx as i32).abs(),idx as i32+(cfg.grid_x as i32-x_self as i32));
                    let del_y = cmp::min((y_self as i32-idy as i32).abs(),idy as i32+(cfg.grid_y as i32-y_self as i32));

                    let k = del_x+del_y;

                    let lamb = (1.00/(1.00-cfg.mutprob)).ln();
                    let dist = Poisson::new(lamb).unwrap();
                    let prob = dist.pmf(k as u64);

                    mutprob += prob;
                }
            }
        }
        mutprob
    }

    pub fn get_pid_list(&self) -> &Vec<u64>{
        &self.pid_list
    }
}