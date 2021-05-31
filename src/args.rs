use structopt::StructOpt;

#[derive(StructOpt, Debug)]
/// Top Level of CLI
pub struct GitArgs {
    /// Print gitrs version
    #[structopt(short, long)]
    pub version: bool,
    // #[structopt(subcommand)]
    // pub init: Option<InitOptions>,
}

// #[derive(StructOpt, PartialEq, Debug)]
// #[structopt(subcommand, name = "init")]
// /// init Subcommand
// pub struct InitOptions {}
