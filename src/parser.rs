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
    static ref STYLE_CHECK_REGEX: Regex = regex("<link.*?rel=\"stylesheet\".*?>");
}

pub fn detect_script<'a>(data: &'a str) -> impl Iterator<Item = Asset> + 'a {
    SCRIPT_REGEX.captures_iter(data).map(|cap| {
        let all = cap.get(0).unwrap();
        let region = (all.start(), all.end());
        let path = cap[1].to_string();
        Asset::new_script(path, region)
    })
}

pub fn parse_html(data: String) -> HtmlFile {
    let mut assets = Vec::new();

    // script search
    let scripts = detect_script(&data).map(|a| {
        println!(
            "[{}, {}], {}: {}",
            a.region.0,
            a.region.1,
            &data[a.region.0..a.region.1],
            a.path
        );
        a
    });

    assets.extend(scripts);

    HtmlFile { data, assets }
}
