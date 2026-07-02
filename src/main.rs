use regex::Regex;
use std::collections::BTreeMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: Missing CSS URL.");
        eprintln!("Usage: {} <css_url>", args[0]);
        std::process::exit(1);
    }

    let url = &args[1];
    println!("Fetching CSS from CDN…");
    let css_text = ureq::get(url).call()?.body_mut().read_to_string()?;
    let re =
        Regex::new(r#"\.(icon-osu-[\w-]+)(?::+before)?\s*\{\s*content:\s*"\\([a-fA-F0-9]+)""#)?;

    let mut file = File::create("osu_icons.codepoints")?;
    let mut count = 0;

    println!("Parsing icons…");

    let code_points: BTreeMap<String, String> = re
        .captures_iter(&css_text)
        .map(|cap| (cap[1].to_string(), cap[2].to_string()))
        .collect();

    for (class, hex) in &code_points {
        writeln!(file, "{class} {hex}")?;
        count += 1;
    }

    println!("Successfully wrote {} icons to osu_icons.codepoints", count);

    Ok(())
}
