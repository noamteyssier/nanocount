use std::{ops::AddAssign, path::Path, sync::Arc};

use anyhow::Result;
use binseq::ParallelProcessor;
use parking_lot::Mutex;
use sassy::{Searcher, profiles::Iupac};
use serde::Serialize;

use crate::guides::Guides;

#[derive(Clone, Copy, Default, Debug)]
pub struct GuideStats {
    count_g1: usize,
    count_g2: usize,
    count_paired: usize,
    count_unpaired: usize,
}
impl GuideStats {
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}
impl AddAssign for GuideStats {
    fn add_assign(&mut self, rhs: Self) {
        self.count_g1 += rhs.count_g1;
        self.count_g2 += rhs.count_g2;
        self.count_paired += rhs.count_paired;
        self.count_unpaired += rhs.count_unpaired;
    }
}

#[derive(Serialize)]
pub struct OutputStats<'a> {
    construct: &'a str,
    alias: &'a str,
    g1: &'a str,
    g2: &'a str,
    count_g1: usize,
    count_g2: usize,
    count_paired: usize,
    count_unpaired: usize,
}
impl<'a> OutputStats<'a> {
    fn new(
        construct: &'a [u8],
        alias: &'a [u8],
        g1: &'a [u8],
        g2: &'a [u8],
        stats: GuideStats,
    ) -> Result<Self> {
        Ok(Self {
            construct: std::str::from_utf8(construct)?,
            alias: std::str::from_utf8(alias)?,
            g1: std::str::from_utf8(g1)?,
            g2: std::str::from_utf8(g2)?,
            count_g1: stats.count_g1,
            count_g2: stats.count_g2,
            count_paired: stats.count_paired,
            count_unpaired: stats.count_unpaired,
        })
    }
}

#[derive(Clone)]
pub struct PatternProcessor {
    /// All the guides to match
    guides: Arc<Guides>,

    /// Maximum alignment cost
    k: usize,

    /// Searcher for the guides (reusable buffer)
    searcher: Searcher<Iupac>,

    /// thread-local stats
    tl_counts: Vec<GuideStats>,

    /// global counts
    counts: Arc<Mutex<Vec<GuideStats>>>,
}
impl PatternProcessor {
    pub fn new(guides: Guides, k: usize) -> Self {
        let searcher = Searcher::new_fwd();
        let tl_counts = vec![GuideStats::default(); guides.len()];
        let counts = Arc::new(Mutex::new(tl_counts.clone()));
        Self {
            guides: Arc::new(guides),
            k,
            searcher,
            tl_counts,
            counts,
        }
    }

    pub fn pprint<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut writer = csv::WriterBuilder::new()
            .delimiter(b'\t')
            .has_headers(true)
            .from_path(path)?;

        let global = self.counts.lock();
        for (idx, construct, alias, g1, g2) in self.guides.iter_all() {
            let row = OutputStats::new(construct, alias, g1, g2, global[idx])?;
            writer.serialize(row)?;
        }
        writer.flush()?;
        Ok(())
    }
}
impl ParallelProcessor for PatternProcessor {
    fn process_record<R: binseq::BinseqRecord>(&mut self, record: R) -> binseq::Result<()> {
        let seq = record.sseq();
        for (idx, g1, g2) in self.guides.iter_patterns() {
            let match_g1 = !self.searcher.search(g1, seq, self.k).is_empty();
            let match_g2 = !self.searcher.search(g2, seq, self.k).is_empty();
            let stats = &mut self.tl_counts[idx];
            match (match_g1, match_g2) {
                (true, true) => {
                    stats.count_g1 += 1;
                    stats.count_g2 += 1;
                    stats.count_paired += 1;
                }
                (true, false) => {
                    stats.count_g1 += 1;
                    stats.count_unpaired += 1;
                }
                (false, true) => {
                    stats.count_g2 += 1;
                    stats.count_unpaired += 1;
                }
                _ => {
                    // do nothing if neither guide matches
                    continue;
                }
            }
        }
        Ok(())
    }
    fn on_batch_complete(&mut self) -> binseq::Result<()> {
        let mut counts = self.counts.lock();
        counts
            .iter_mut()
            .zip(self.tl_counts.iter_mut())
            .for_each(|(global, local)| {
                *global += *local;
                local.clear();
            });
        Ok(())
    }
}
