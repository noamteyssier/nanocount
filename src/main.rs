mod cli;
mod guides;
mod processor;

use anyhow::Result;
use binseq::{BinseqReader, ParallelReader};
use clap::Parser;

use crate::{cli::Cli, guides::Guides, processor::PatternProcessor};

pub type Pattern = Vec<u8>;
pub type Patterns = Vec<Pattern>;

fn main() -> Result<()> {
    let args = Cli::parse();
    let guides = Guides::from_path(&args.patterns)?;
    let reader = BinseqReader::new(&args.ipath)?;
    let proc = PatternProcessor::new(guides, args.k);
    reader.process_parallel(proc.clone(), args.threads)?;
    proc.pprint(&args.opath)?;
    Ok(())
}
