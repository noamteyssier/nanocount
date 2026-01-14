use std::path::Path;

use anyhow::Result;
use serde::Deserialize;

use crate::{Pattern, Patterns};

#[derive(Clone, Deserialize)]
pub struct GuideRow {
    construct: String,
    alias: String,
    g1: String,
    g2: String,
}

#[derive(Default, Clone)]
pub struct Guides {
    pub construct: Patterns,
    pub alias: Patterns,
    pub g1: Patterns,
    pub g2: Patterns,
}
impl Guides {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut guides = Self::default();
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .from_path(path)?;
        for result in reader.deserialize() {
            let record: GuideRow = result?;
            guides.add_record(
                record.construct.as_bytes(),
                record.alias.as_bytes(),
                record.g1.to_uppercase().as_bytes(),
                record.g2.to_uppercase().as_bytes(),
            );
        }
        Ok(guides)
    }

    pub fn len(&self) -> usize {
        self.construct.len()
    }

    pub fn add_record(&mut self, construct: &[u8], alias: &[u8], g1: &[u8], g2: &[u8]) {
        // insert the guide pair name
        {
            self.construct.push(construct.into());
            self.alias.push(alias.into());
        }

        // insert the guide sequences
        {
            self.g1.push(g1.into());
            self.g2.push(g2.into());
        }
    }

    pub fn iter_patterns(&self) -> impl Iterator<Item = (usize, &Pattern, &Pattern)> {
        (0..self.len()).map(|idx| {
            let g1 = &self.g1[idx];
            let g2 = &self.g2[idx];
            (idx, g1, g2)
        })
    }

    pub fn iter_all(
        &self,
    ) -> impl Iterator<Item = (usize, &Pattern, &Pattern, &Pattern, &Pattern)> {
        (0..self.len()).map(|idx| {
            let construct = &self.construct[idx];
            let alias = &self.alias[idx];
            let g1 = &self.g1[idx];
            let g2 = &self.g2[idx];
            (idx, construct, alias, g1, g2)
        })
    }
}
