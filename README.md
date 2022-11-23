### simgpm.rs

Wright-Fisher populations on arbitrary genotype spaces in a changing environment

Written by: Bhaskar Kumawat (https://kumawatb.com)

### Installation

To get this package up and running on your machine, you will need the latest installation of rust (https://www.rust-lang.org/tools/install). To build the program,

1. Clone the repository to your machine
```
git clone https://github.com/kumawatb/simgpm_rs.git
```
3. Switch to the root directory
```
cd simgpm_rs
```
4. Execute the following command
```
cargo build -r
```
The `simgpm` executable will be built in the `target/release` folder. 

### Simulation setup
Simulation setup requires three types of files in a `config` directory in the same location as the executable.
1. Environment file

The environment file specifies the different environments to be used during a simulation and how they change over evolutionary time. See `env.cfg` file in the `config` directory to learn how the environments are specified. A particular environment state essentially determines the phenotype-fitness map to be used in that state.

2. Phenotype-Fitness file(s)

The environment file relies on paths to Phenotype-Fitness (pf) files to specify how a particular phenotype is rewarded in the environment. pf-files are specified as comma separated values with the first column denoting a phenotype-id and the second column denoting the relative fitness of that phenotype in an environment state. See `pf_example1.csv` and `pf_example2.csv` along with the header for `env.cfg` to learn how to specify different phenotype-fitness maps.

3. Genotype-Phenotype file

The genotype-phenotype (gp) file specifies the 2D genotype-phenotype map to be used in a simulation as a comma separated file. The file contains X rows and Y columns, where X and Y are the width and height of the genotype space. Each (x,y) coordinate in the file is a phenotype-id that determines the phenotype of that genotype. The genotype space has periodic boundaries and loops around along both axes to form a toroid. To remove periodic boundaries, you can create a phenotype id with zero fitness around the entire genotype-phenotype map. See `gpm.csv` to learn how a gp-file is specified.

### Running the simulation

The simulation can be run with different parameter values for population size, mutation rates etc. To check the available parameters, type the following in the command line:

```
./simgpm --help
```
```
SimGpm: A program to simulate evolution on arbitrary genotype-phenotype maps

Optional arguments:
  -h,--help             Show this help message and exit
  -x,--grid_x GRID_X    Grid size in x
  -y,--grid_y GRID_Y    Grid size in y
  -g,--gpfilepath GPFILEPATH
                        Path to the genotype-phenotype map file
  -e,--envfilepath ENVFILEPATH
                        Path to the environment file
  -p,--popsize POPSIZE  Population size
  -m,--mutprob MUTPROB  Mutation probability
  -t,--maxgens MAXGENS  Maximum generations to run the simulation for
  -s,--saveevery SAVEEVERY
                        Time interval duration to create population saves (in
                        generations)
  -d,--outdom OUTDOM    0/1, output file containing dominant genotypes?
  -v,--outevol OUTEVOL  0/1, output file containing evolvabilities?
  -n,--outenv OUTENV    0/1, output file containing environments?
  -a,--outaltmut OUTALTMUT
                        0/1, output file containing alternate mutant
                        probabilities?
  -o,--outpopsave OUTPOPSAVE
                        0/1, output file containing population saves?
  -h,--rndstrtpheno RNDSTRTPHENO
                        Start simulation with all organisms having a random
                        genotype from given phenotypes?
  -l,--replid REPLID    If only one replicate, should this replicate be given
                        an id?

```
