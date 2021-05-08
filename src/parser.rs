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
    path: String,
    region: Region,
    wrappers: (&'static str, &'static str),
}

impl Asset {
    fn new_script(path: String, region: Region) -> Asset {
        Asset {
            path,
            region,
            wrappers: ("<script>", "</script>"),
        }
    }

    fn new_style(path: String, region: Region) -> Asset {
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
    static ref SCRIPT_REGEX: Regex = regex("(<script.*?src\\s*?=\\s*?\"(.*?)\".*?>\\s*?</script>)");
    static ref STYLE_UNCHECKED_REGEX: Regex = regex("(<link.*?href\\s*?=\\s*?\"(.*?)\".*?>)");
    static ref STYLE_CHECK_REGEX: Regex = regex("(<link.*?rel=\"stylesheet\".*?>)");
}

pub fn parse_html(data: String) -> HtmlFile {
    let mut assets = Vec::new();

    // script search
    for cap in SCRIPT_REGEX.captures_iter(&data) {
        let all = cap.get(0).unwrap();
        let region = (all.start(), all.end());
        let path = &cap[1];

        println!(
            "({}, {}): \"{}\", path: {}",
            region.0,
            region.1,
            &data[region.0..region.1],
            path
        );
        assets.push(Asset::new_script(path.to_string(), region));
    }

    HtmlFile { data, assets }
}
