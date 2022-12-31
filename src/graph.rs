use crate::args::*;
use anyhow::anyhow;
use petgraph::{visit::EdgeRef, Directed, Graph};
use regex::Regex;
use serde::Serialize;
use std::{
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub type BackLinksGraph = Graph<String, (), Directed>;

#[derive(Debug, Serialize)]
pub struct JsonGraphContainer {
    pub graph: JsonGraph,
}

#[derive(Debug, Serialize)]
pub struct JsonGraph {
    pub directed: bool,
    pub nodes: Vec<JsonNode>,
    pub edges: Vec<JsonEdge>,
}

#[derive(Debug, Serialize)]
pub struct JsonNode {
    // https://github.com/jsongraph/json-graph-specification#nodesedges-single-graph
    // Technically we aren't following the spec to a T
    // the key for the JsonNode should also be label used in edges
    // instead I'm just making them an array so the keys will be indices
    // and we'll extract the actual name from this label
    pub label: String,
}

#[derive(Debug, Serialize)]
pub struct JsonEdge {
    pub source: String,
    pub directed: bool,
    pub target: String,
}

pub fn build_graph<'a>(path: impl AsRef<Path>) -> Result<BackLinksGraph> {
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

pub fn output_graph(g: BackLinksGraph) -> JsonGraphContainer {
    let nodes: Vec<JsonNode> = g
        .node_weights()
        .map(|w| JsonNode {
            label: w.to_owned(),
        })
        .collect();

    let edges: Vec<JsonEdge> = g
        .edge_references()
        .map(|er| JsonEdge {
            source: g[er.source()].to_owned(),
            directed: true,
            target: g[er.target()].to_owned(),
        })
        .collect();

    JsonGraphContainer {
        graph: JsonGraph {
            directed: true,
            nodes: nodes,
            edges: edges,
        },
    }
}

impl JsonGraphContainer {
    pub fn to_file(&self, output_path: PathBuf) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(&output_path, json)
            .with_context(|| format!("Failed to write to file {}", output_path.display()))?;

        Ok(())
    }
}
