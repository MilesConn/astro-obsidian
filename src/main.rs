mod args;
use args::*;

fn main() -> Result<()> {
    let args = CliArgs::parse();
    let mut cmd = CliArgs::command();

    args.is_valid().with_context(|| cmd.render_long_help())?;

    Ok(())
}
