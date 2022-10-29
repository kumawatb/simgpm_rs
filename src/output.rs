
use crate::config::Config;
use crate::gpm::Gpmap;
use crate::pop::Population;
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[derive(Default)]
pub struct Output{
    pub domfile: Option<File>,
    pub evolfile: Option<File>,
    pub envfile: Option<File>,
    pub altmutfile: Option<File>,
    pub popfile: Option<File>

}



impl Output{
    pub fn createfiles(config: &Config) -> Result<Output, Box<dyn Error>>{
        let mut output = Output::default();

        if config.outdom{
            output.domfile = Some(File::create("./output/dominant.csv")?);
            output.domfile.as_ref().unwrap().write(b"repl,time,domx,domy,abundance\n").unwrap();
        }
        if config.outevol{
            output.evolfile = Some(File::create("./output/evolvability.csv")?);
            output.evolfile.as_ref().unwrap().write(b"repl,time,popevol\n").unwrap(); 
        }
        if config.outenv{
            output.envfile = Some(File::create("./output/environment.csv")?);
            output.envfile.as_ref().unwrap().write(b"repl,time,envid\n").unwrap();
        }
        if config.outaltmut{
            output.altmutfile = Some(File::create("./output/altmut.csv")?);
            output.altmutfile.as_ref().unwrap().write(b"repl,time,pid,probmut\n").unwrap();
        }
        if config.outpopsave{
            output.popfile = Some(File::create("./output/population.csv")?);
            output.popfile.as_ref().unwrap().write(b"repl,time,x,y,pop\n").unwrap();
        }

        Ok(output)
    }

    pub fn generate_output(&self,time: u64,config: &Config,envid: i32, pop: &Population, gpmap: &Gpmap){
        // Write dominant file
        if !self.domfile.is_none(){
            self.write_domfile(config,time,pop);
        }

        // Write environment file
        if !self.envfile.is_none(){
            self.write_envfile(config,time,envid);
        }

        //Write population file
        if !self.popfile.is_none(){
            self.write_popfile(config,time,pop);
        }

        // Write altmut file
        if !self.altmutfile.is_none(){
            self.write_altmutfile(config,time,pop,gpmap);
        }

        // Write evolvability file
        if !self.evolfile.is_none(){
            self.write_evolfile(config,time,pop,gpmap);
        }
    }

    pub fn write_domfile(&self,config: &Config, time:u64,pop: &Population){
        let (domx,domy,abundance) = pop.get_dominance_info();
        self.domfile.as_ref().unwrap().write(format!("{},{},{},{},{}\n",config.replid,time,domx,domy,abundance).as_bytes()).unwrap();
    }

    pub fn write_envfile(&self,config: &Config,time: u64,envid: i32){
        self.envfile.as_ref().unwrap().write(format!("{},{},{}\n",config.replid,time,envid).as_bytes()).unwrap();
    }

    pub fn write_popfile(&self,config: &Config,time: u64,pop: &Population){
        for idx in 0..config.grid_x{
            for idy in 0..config.grid_y{
                let size = pop.get_at(idx,idy);
                if size!=0{
                    self.popfile.as_ref().unwrap().write(
                        format!("{},{},{},{},{}\n",config.replid,time,idx,idy,size).as_bytes()
                    ).unwrap();
                }
            }
        }
    }

    pub fn write_evolfile(&self, config: &Config, time: u64, pop: &Population,gpmap: &Gpmap){
        // @TODO
    }

    pub fn write_altmutfile(&self, config: &Config, time: u64, pop: &Population, gpmap: &Gpmap){
        // @TODO
    }
}
