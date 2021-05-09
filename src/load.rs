use std::{fs, path::PathBuf};

use anyhow::Context;
use beau_collector::BeauCollector;

use crate::{
    errors::Error,
    parser::{Asset, HtmlFile},
};

#[derive(Debug)]
pub struct LoadedHtml {
    data: String,
    assets: Vec<LoadedAsset>,
}

#[derive(Debug)]
pub struct LoadedAsset {
    data: String,
    region: (usize, usize),
    wrappers: (&'static str, &'static str),
}

impl LoadedHtml {
    pub fn load(root: PathBuf, HtmlFile { data, assets }: HtmlFile) -> anyhow::Result<LoadedHtml> {
        let assets = assets
            .into_iter()
            .map(|asset| LoadedAsset::load(root.clone(), asset))
            .bcollect()?;
        Ok(LoadedHtml { data, assets })
    }
}

impl LoadedAsset {
    pub fn load(
        mut root: PathBuf,
        Asset {
            path,
            region,
            wrappers,
        }: Asset,
    ) -> anyhow::Result<LoadedAsset> {
        root.push(path);
        let data = fs::read_to_string(&root).with_context(|| Error::FileNotFound(root.clone()))?;
        Ok(LoadedAsset {
            data,
            region,
            wrappers,
        })
    }
}

impl LoadedHtml {
    pub fn chunks<'a>(&'a self) -> Vec<&'a str> {
        let mut chunks = Vec::with_capacity(4 * self.assets.len() + 1);
        let mut ptr = 0;
        for asset in &self.assets {
            chunks.push(&self.data[ptr..asset.region.0]);

            chunks.push(asset.wrappers.0);
            chunks.push(&asset.data);
            chunks.push(asset.wrappers.1);

            ptr = asset.region.1;
        }

        chunks.push(&self.data[ptr..self.data.len()]);

        chunks
    }
}
