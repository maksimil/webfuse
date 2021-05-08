use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};

#[derive(Debug, Clone)]
pub struct HtmlFile {
    data: String,
    assets: Vec<Asset>,
}

type Region = (usize, usize);

#[derive(Debug, Clone)]
pub struct Asset {
    pub path: String,
    pub region: Region,
    pub wrappers: (&'static str, &'static str),
}

impl Asset {
    pub fn new_script(path: String, region: Region) -> Asset {
        Asset {
            path,
            region,
            wrappers: ("<script>", "</script>"),
        }
    }

    pub fn new_style(path: String, region: Region) -> Asset {
        Asset {
            path,
            region,
            wrappers: ("<style>", "</style>"),
        }
    }
}

fn regex(s: &str) -> Regex {
    RegexBuilder::new(s)
        .multi_line(true)
        .dot_matches_new_line(true)
        .build()
        .unwrap()
}

lazy_static! {
    static ref SCRIPT_REGEX: Regex = regex("<script.*?src\\s*?=\\s*?\"(.*?)\".*?>\\s*?</script>");
    static ref STYLE_UNCHECKED_REGEX: Regex = regex("<link.*?href\\s*?=\\s*?\"(.*?)\".*?>");
    static ref STYLE_CHECK_REGEX: Regex = regex("<link.*?rel\\s*?=\\s*?\"stylesheet\".*?>");
}

pub fn detect_script<'a>(data: &'a str) -> impl Iterator<Item = Asset> + 'a {
    SCRIPT_REGEX.captures_iter(data).map(|cap| {
        let all = cap.get(0).unwrap();
        let region = (all.start(), all.end());
        let path = cap[1].to_string();
        Asset::new_script(path, region)
    })
}

pub fn detect_styles<'a>(data: &'a str) -> impl Iterator<Item = Asset> + 'a {
    STYLE_UNCHECKED_REGEX.captures_iter(data).filter_map(|cap| {
        let all = cap.get(0).unwrap();

        if STYLE_CHECK_REGEX.is_match(all.as_str()) {
            let region = (all.start(), all.end());
            let path = cap[1].to_string();
            Some(Asset::new_style(path, region))
        } else {
            None
        }
    })
}

pub fn parse_html(data: String) -> HtmlFile {
    let mut assets = Vec::new();

    #[cfg(debug_assertions)]
    let debug_asset = |asset: Asset| {
        println!(
            "region: {:?}, asset: {:?}",
            &data[asset.region.0..asset.region.1],
            asset
        );
        asset
    };

    // script search
    #[cfg(not(debug_assertions))]
    let scripts = detect_script(&data);

    #[cfg(debug_assertions)]
    let scripts = detect_script(&data).map(debug_asset);

    assets.extend(scripts);

    // style search
    #[cfg(not(debug_assertions))]
    let styles = detect_styles(&data);

    #[cfg(debug_assertions)]
    let styles = detect_styles(&data).map(debug_asset);

    assets.extend(styles);

    HtmlFile { data, assets }
}
