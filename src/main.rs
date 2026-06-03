mod tree;
mod db;

use anyhow::Result;

use db::postgres::load_schema;

use tree::diff::diff_trees;

use tree::render::{
    build_diff_lines,
    render_boxed_tree,
    render_legend,
    RenderSide,
};

#[tokio::main]
async fn main() -> Result<()> {

    /*
        LOAD DATABASES
    */

   let source_db =
    load_schema(
        "postgres://postgres:password@localhost:5433/db1"
    )
    .await?;

    let target_db =
    load_schema(
        "postgres://postgres:password@localhost:5434/db2"
    )
    .await?;
    /*
        BUILD DIFF
    */

    let diff =
        diff_trees(
            Some(&source_db),
            Some(&target_db),
        );

    /*
        SOURCE VIEW
    */

    let mut source_lines = vec![];

    build_diff_lines(
        &diff,
        &RenderSide::Source,
        String::new(),
        true,
        &mut source_lines,
    );

    /*
        TARGET VIEW
    */

    let mut target_lines = vec![];

    build_diff_lines(
        &diff,
        &RenderSide::Target,
        String::new(),
        true,
        &mut target_lines,
    );

    /*
        RENDER
    */

    println!();

    render_boxed_tree(
        "SOURCE",
        &source_lines,
        70,
    );

    println!();

    render_boxed_tree(
        "TARGET",
        &target_lines,
        70,
    );

    render_legend();

    Ok(())
}