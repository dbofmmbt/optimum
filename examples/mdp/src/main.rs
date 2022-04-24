mod cli;

use std::{fs::read_to_string, path::Path};

use clap::StructOpt;
use cli::{DecoderChooser, Opts};
use mdp::{CurrentDecoder, ExperimentalDecoder, MaximumDiversity};
use ndarray::Array2;
use optimum::{
    analysis::batch::{Batch, BatchResult, Statistics},
    core::{solver::IterHook, stop_criterion::IterCriterion, Problem},
    metaheuristics::genetic::{
        brkga::{Brkga, BrkgaHook, Params, RandomMemberBuilder},
        Decoder,
    },
};
use rand::SeedableRng;

fn main() -> std::io::Result<()> {
    let opts = Opts::parse();

    let problem = load_input(&opts.instance)?;

    let create_params = |member_size: usize| Params {
        population_size: opts.population_size.try_into().unwrap(),
        member_size: member_size.try_into().unwrap(),
        elites: opts.elites,
        mutants: opts.mutants,
        crossover_bias: opts.crossover_bias,
    };

    match opts.decoder {
        DecoderChooser::New => {
            println!("Using new decoder");
            let decoder = ExperimentalDecoder::new(&problem);
            run(decoder, create_params(problem.solution_size), opts.seed);
        }
        DecoderChooser::Current => {
            println!("Using current decoder");
            let decoder = CurrentDecoder::new(&problem);
            run(decoder, create_params(problem.input_size), opts.seed);
        }
    }

    Ok(())
}

fn run<D: Decoder<P = MaximumDiversity>>(decoder: D, params: Params, seed: usize) {
    let stop_criterion = IterCriterion::new(1000);

    let build_solver = |seed, exec_number| {
        let rng = rand_pcg::Pcg64::seed_from_u64((seed + exec_number) as u64);

        Brkga::new(&decoder, rng, params, RandomMemberBuilder)
    };

    let batch = Batch::builder()
        .base_seed(seed)
        .executions(10)
        .solver(build_solver)
        .stop_criterion(stop_criterion)
        .hook(LogHook::default())
        .build()
        .run()
        .unwrap();

    print_logs(&batch);

    let statistics = Statistics::new(&batch);
    let best_execution = statistics.best();
    println!(
        "average value: {} best value: {} average time: {}",
        statistics.average_value(),
        best_execution.evaluation().value(),
        statistics.average_time().as_secs_f64(),
    );
}

fn print_logs(batch: &BatchResult<MaximumDiversity, LogHook>) {
    for execution in batch.executions().iter() {
        for (idx, value) in execution.hook().values.iter().enumerate() {
            println!("ITER {number} LOCAL_SEARCH {value}", number = idx + 1);
        }
        println!(
            "EXEC {} VALUE {} TIME {}",
            execution.number(),
            execution.evaluation().value(),
            execution.duration().as_secs_f64()
        );
    }
}

fn load_input(path: &Path) -> std::io::Result<MaximumDiversity> {
    let file_name = { path.file_name().unwrap().to_string_lossy() };

    let (_, last) = file_name.split_once('n').unwrap();
    let (input_size, last) = last.split_once('_').unwrap();

    let input_size: usize = input_size.parse().unwrap();
    let solution_size: usize = last[1..].strip_suffix(".txt").unwrap().parse().unwrap();

    let input = read_to_string(path)?;

    let mut lines = input.lines();
    // skip first line, which is the problem size.
    lines.next();

    let mut vec = Vec::<f64>::with_capacity(input_size * input_size);

    for line in lines {
        let elements = line
            .split_ascii_whitespace()
            .map(|v| v.parse::<f64>().unwrap());

        vec.extend(elements);
    }

    Ok(MaximumDiversity {
        matrix: Array2::from_shape_vec((input_size, input_size), vec).unwrap(),
        solution_size,
        input_size,
    })
}

#[derive(Debug, Default, Clone)]
struct LogHook {
    values: Vec<f64>,
}

impl<P: Problem> IterHook<P> for LogHook
where
    P::Value: Into<f64>,
{
    fn iterated(&mut self, new: &optimum::core::Evaluation<P>) {
        self.values.push(new.value().into());
    }
}

impl<D: Decoder> BrkgaHook<D> for LogHook where <D::P as Problem>::Value: Into<f64> {}
