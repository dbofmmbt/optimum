use std::{path::PathBuf, str::FromStr};

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Opts {
    pub instance: PathBuf,
    #[clap(long)]
    pub decoder: DecoderChooser,
    #[clap(long)]
    pub execs: usize,
    #[clap(long)]
    pub population_size: usize,
    #[clap(long)]
    pub elites: usize,
    #[clap(long)]
    pub mutants: usize,
    #[clap(long)]
    pub crossover_bias: f64,
    #[clap(long)]
    pub seed: usize,
}

#[derive(Debug)]
pub enum DecoderChooser {
    New,
    Current,
}

impl FromStr for DecoderChooser {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "new" => Self::New,
            "current" => Self::Current,
            _ => return Err(anyhow::anyhow!("invalid value for decoder")),
        })
    }
}
