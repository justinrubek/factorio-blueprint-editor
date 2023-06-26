#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum Commands {
    Blueprint(Blueprint),
}

#[derive(clap::Args, Debug)]
pub(crate) struct Blueprint {
    #[clap(subcommand)]
    pub command: BlueprintCommands,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum BlueprintCommands {
    /// Creates a new blueprint book.
    /// The book will have variations on the blueprints you provide.
    /// For example, one variant would have all concrete removed.
    /// Another may have upgraded belts.
    GenerateBook(GenerateBook),
    /// Display the top-level information about a blueprint.
    Display(Display),
}

#[derive(clap::Args, Debug)]
pub(crate) struct GenerateBook {
    /// The blueprint book to modify.
    #[clap(long, short)]
    pub book: std::path::PathBuf,
    /// the file to save the book to.
    #[clap(long, short)]
    pub output: std::path::PathBuf,
}

#[derive(clap::Args, Debug)]
pub(crate) struct Display {
    /// The blueprint to display.
    #[clap(long, short)]
    pub blueprint: std::path::PathBuf,
}
