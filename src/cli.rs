use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// BINSEQ file to process
    pub ipath: String,

    /// Output file to write results to (as TSV)
    #[clap(short, long, default_value = "nanocount.tsv")]
    pub opath: String,

    /// Patterns CSV [name_pair1, name_pair2, seq_pair1, seq_pair2]
    #[clap(short, long, required = true)]
    pub patterns: String,

    /// Maximum alignment cost per protospacer
    #[clap(short, default_value_t = 1)]
    pub k: usize,

    /// Number of threads to use, (0 = all available)
    #[clap(short = 'T', default_value_t = 0)]
    pub threads: usize,
}
