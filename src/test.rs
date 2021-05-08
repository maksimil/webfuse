#![cfg(test)]

use crate::parser::{detect_script, detect_styles, parse_html, HtmlFile};

#[test]
fn script_detection() {
    let data = vec![
        ("<html><script src=  \"lib/index.js\" type=\"module\" defer></script></html>", vec![(6, 64)]),
        ("<head><script src=\"lib/index.js\">  \n</script><meta stuff></head>", vec![(6, 45)]),
        ("<head></head><script  type=\"module\"  src =\"lib/index.js\"defer></script><script src=\"lib/index.js\">\n\n\n</script><meta stuff>", vec![(13, 71), (71, 110)]),
    ];

    for (s, regions) in data {
        let assets = detect_script(s);
        let regions = regions.into_iter();

        for (asset, region) in assets.zip(regions) {
            println!("asset: {:?}", asset);
            println!("data: {:?}", s);
            println!("part: {:?}", &s[asset.region.0..asset.region.1]);

            assert_eq!(asset.path, "lib/index.js");
            assert_eq!(asset.region, region);
        }
    }
}

#[test]
fn style_detection() {
    let data = vec![
        ("<link rel=  \"stylesheet\" href =\"style.css\">", 1),
        ("<link href=\"style.css\" defer rel =\"stylesheet\"/>", 1),
        ("<link rel=\"stylesheet\" href =\"style.css\">", 1),
        ("<link href=\"style.css\" rel=\"stylesheet\"/><link meta href =\"style.css\"><link rel=  \"stylesheet\" href =\"style.css\">", 2)
    ];

    for (s, len) in data {
        let assets = detect_styles(s).collect::<Vec<_>>();

        println!("data: {:?}", s);
        println!("assets: {:?}", assets);

        assert_eq!(assets.len(), len);
    }
}

#[test]
fn big_test() {
    let data = include_str!("../test/index.html");

    let HtmlFile { assets, data } = parse_html(data.to_string());

    {
        let asset = &assets[0];
        let region = &data[asset.region.0..asset.region.1];

        assert_eq!(region, "<script src=\"index.js\"></script>");
        assert_eq!(asset.wrappers, ("<script>", "</script>"));
        assert_eq!(asset.path, "index.js");
    }

    {
        let asset = &assets[1];
        let region = &data[asset.region.0..asset.region.1];

        assert_eq!(region, "<link rel=\"stylesheet\" href=\"style.css\" />");
        assert_eq!(asset.wrappers, ("<style>", "</style>"));
        assert_eq!(asset.path, "style.css");
    }

    {
        let asset = &assets[2];
        let region = &data[asset.region.0..asset.region.1];

        assert_eq!(region, "<link href=\"a.css\" rel=\"stylesheet\" />");
        assert_eq!(asset.wrappers, ("<style>", "</style>"));
        assert_eq!(asset.path, "a.css");
    }

    {
        let asset = &assets[3];
        let region = &data[asset.region.0..asset.region.1];

        assert_eq!(region, "<script src=\"lib/lib.min.js\" defer></script>");
        assert_eq!(asset.wrappers, ("<script>", "</script>"));
        assert_eq!(asset.path, "lib/lib.min.js");
    }
}
