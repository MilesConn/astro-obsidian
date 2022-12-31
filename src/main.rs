mod args;
mod graph;

use args::*;
use graph::*;

fn main() -> Result<()> {
    let args = CliArgs::parse();
    let mut cmd = CliArgs::command();

    args.is_valid().with_context(|| cmd.render_long_help())?;

    let g = build_graph(args.path)?;

    let json_graph = output_graph(g);

    json_graph.to_file(args.output)?;

    Ok(())
}
