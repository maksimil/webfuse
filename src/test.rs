#![cfg(test)]

use crate::parser::detect_script;

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
