mod args;
use std::{fs, path::Path};

use anyhow::anyhow;
use args::*;

use petgraph::{Directed, Graph};
use regex::Regex;
use walkdir::WalkDir;

fn main() -> Result<()> {
    let args = CliArgs::parse();
    let mut cmd = CliArgs::command();

    args.is_valid().with_context(|| cmd.render_long_help())?;

    let g = build_graph(args.path)?;

    Ok(())
}

fn build_graph<'a>(path: impl AsRef<Path>) -> Result<Graph<String, (), Directed>> {
    let mut g = Graph::new();
    // Regex for backlinks [[back-link]]
    let re = Regex::new(r"\[{2}([a-zA-z\-]*)\]{2}").unwrap();

    for file in WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_entry(|e| {
            // Only traverse directories or markdown files
            e.path().is_dir() || e.path().extension().map(|s| s == "md").unwrap_or(false)
        })
        .filter_map(|e| e.ok())
    {
        // We can still end up with directories because of how filter_entry works
        if file.path().is_file() {
            let contents = fs::read_to_string(file.path())?;
            // We actually don't need a markdown parser for now...
            // let root = parse_document(&arena, &contents, &ComrakOptions::default());

            let file_name = file
                .path()
                .file_stem()
                .ok_or(anyhow!(
                    "File stem failed to parse {}",
                    file.path().to_string_lossy()
                ))
                .and_then(|x| {
                    x.to_str().ok_or(anyhow!(
                        "String {} had non utf-8 characters or ended in ..",
                        file.path().to_string_lossy()
                    ))
                })?;

            let src_node = g.add_node(file_name.to_owned());

            let matches = re.captures_iter(&contents);

            for capture in matches {
                // Get the capture from the full match
                let back_link = &capture[1];
                // Prevents pet graph from panicking
                println!("Adding edge from {:?} to {:?}", file_name, back_link);
                let dest_node = g.add_node(back_link.to_owned());
                g.update_edge(src_node, dest_node, ());
            }
        }
    }

    Ok(g)
}
