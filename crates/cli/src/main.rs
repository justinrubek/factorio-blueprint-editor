use crate::{
    commands::{BlueprintCommands, Commands},
    error::Result,
};
use clap::Parser;
use factorio_blueprint::{BlueprintCodec, Container};

mod commands;
mod error;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = commands::Args::parse();
    match args.command {
        Commands::Blueprint(blueprint) => match blueprint.command {
            BlueprintCommands::GenerateBook(generate_book) => {
                println!("Generating book from {}", generate_book.book.display());

                let book_data = tokio::fs::read(&generate_book.book).await?;
                let book_string = String::from_utf8(book_data)?;

                let container = BlueprintCodec::decode_string(&book_string)?;
                let no_concrete_container = rewrite_container(container.clone())?;
                println!("{container:#?}");

                let book_string = BlueprintCodec::encode_string(&no_concrete_container)?;
                let book_data = book_string.into_bytes();
                tokio::fs::write(&generate_book.output, book_data).await?;
            }
            BlueprintCommands::Display(display) => {
                println!("Displaying {}", display.blueprint.display());

                let blueprint_data = tokio::fs::read(&display.blueprint).await?;
                let blueprint_string = String::from_utf8(blueprint_data)?;

                let container = BlueprintCodec::decode_string(&blueprint_string)?;

                // Display - the container type, the label, and description.
                match container {
                    Container::BlueprintBook(book) => {
                        println!("Loaded blueprint book ({:?})", book.label);
                    }
                    Container::Blueprint(_) => todo!(),
                    Container::UpgradePlanner(_) => todo!(),
                    Container::DeconstructionPlanner(_) => todo!(),
                };
            }
        },
    }

    Ok(())
}

fn rewrite_container(container: Container) -> Result<Container> {
    match container {
        Container::BlueprintBook(book) => rewrite_book(book),
        Container::Blueprint(blueprint) => rewrite_blueprint(blueprint),
        Container::UpgradePlanner(_upgrade_planner) => todo!(),
        Container::DeconstructionPlanner(_deconstruction_planner) => todo!(),
    }
}

/// Creates a new blueprint book.
/// This book should container two sub-books.
/// The first is the original book, and the second is the modified book.
/// The modified book should be the same as the original, but with no concrete.
fn rewrite_book(book: factorio_blueprint::objects::BlueprintBook) -> Result<Container> {
    let mut book = book;

    // iterate over the blueprints book, and modify each blueprint.
    let blueprints = book
        .blueprints
        .iter()
        .map(|bp_value| {
            // bp_value.item is a container, and must be called recursively.
            let mut bp = bp_value.clone();
            let bp_container = rewrite_container(bp.item).unwrap();
            bp.item = bp_container;
            bp
        })
        .collect();

    book.blueprints = blueprints;

    Ok(Container::BlueprintBook(book))
}

/// Replaces the blueprint with a new blueprint that has no concrete.
fn rewrite_blueprint(blueprint: factorio_blueprint::objects::Blueprint) -> Result<Container> {
    let mut blueprint = blueprint;
    // iterate over the tiles, and remove any concrete.
    let new_titles = blueprint
        .tiles
        .iter()
        .filter(|tiles| tiles.name != "concrete")
        .cloned()
        .collect();

    blueprint.tiles = new_titles;

    Ok(Container::Blueprint(blueprint))
}
