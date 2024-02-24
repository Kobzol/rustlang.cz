use std::io::BufWriter;
use std::path::PathBuf;

use atom_syndication::{FeedBuilder, LinkBuilder, PersonBuilder};
use chrono::Utc;
use clap::Parser;

#[derive(clap::Parser)]
struct Args {
    /// Path where the Atom feed should be generated
    output: PathBuf,

    /// Path to a data directory containing YAML files.
    #[clap(long = "data-path")]
    data_path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let last_update = Utc::now();
    let web_uri = "https://rustlang.cz";

    let feed = FeedBuilder::default()
        .title("Czech Rust language events")
        .subtitle(Some("Events and talk recordings related to the Rust language community in the Czech Republic".into()))
        .id(web_uri)
        .links(vec![LinkBuilder::default()
            .title("Atom feed".to_string())
            .href(format!("{web_uri}/feed.atom"))
            .build()])
        .authors(vec![PersonBuilder::default()
            .name("Jakub Beránek")
            .email("berykubik@gmail.com".to_string())
            .uri("https://github.com/kobzol".to_string())
            .build()])
        .updated(last_update)
        .build();
    let output = BufWriter::new(std::fs::File::create(args.output)?);
    feed.write_to(output)?;

    Ok(())
}
