use structopt::StructOpt;
use rand::distributions::{Distribution, Uniform};
use std::fmt;

impl std::string::ToString for ArgError {
    fn to_string(&self) -> String {
        match self {
            ArgError::InvalidArgs { details } => details.to_string(),
        }
    }
}

#[derive(Debug)]
enum ArgError {
    InvalidArgs { details: String },
}

impl std::str::FromStr for Strategy {
    type Err = ArgError;

    fn from_str(s: &str) -> Result<Strategy, ArgError> {
        match s {
            "stay" => Ok(Strategy::Stay),
            "switch" => Ok(Strategy::Switch),
            "both" => Ok(Strategy::Both),
            _ =>{
                let details = "Please choose a strategy - 'stay', 'switch', or 'both'";
                Err(ArgError::InvalidArgs { details: details.to_string() })
            }
        }
    }
}

impl fmt::Display for Strategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
enum Strategy {
    Stay,
    Switch,
    Both,
}

#[derive(Debug, StructOpt)]
struct CLIArg {
    #[structopt(short = "i", long = "iterations", help = "The number of times the simulation shall be run", default_value = "9001")]
    iterations: u32,
    #[structopt(short = "s", long = "strategy", help = "Which strategy to use", default_value = "both")]
    strategy: Strategy,
}

enum Prize {Goat, NotGoat}

fn main() {
    let cliarg = CLIArg::from_args();
    let runiters = &cliarg.iterations;
    let runstrat = &cliarg.strategy;
    let mut rng = rand::thread_rng();
    let doors = Uniform::from(0..3);
    let cardoor = doors.sample(&mut rng);
//    println!("{}", runiters);
//    println!("{:?}", cliarg);
    println!("{}", runiters);
    println!("{}", runstrat);
    println!("{}", cardoor);
}
