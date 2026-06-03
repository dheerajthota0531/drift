use colored::*;

use crate::tree::diff::{
    DiffNode,
    DiffStatus,
};

pub enum RenderSide {
    Source,
    Target,
}

pub fn build_diff_lines(
    node: &DiffNode,
    side: &RenderSide,
    prefix: String,
    is_last: bool,
    lines: &mut Vec<String>,
) {

    let connector =
        if prefix.is_empty() {
            ""
        } else if is_last {
            "└── "
        } else {
            "├── "
        };

    let value =
        match side {

            RenderSide::Source => {
                node.old_value.clone()
            }

            RenderSide::Target => {
                node.new_value.clone()
            }
        };

    let mut line =
        format!(
            "{}{}{}",
            prefix,
            connector,
            node.name
        );

    if let Some(v) = value {
        line.push_str(&format!(":{}", v));
    }

    let colored_line =
        match node.status {

            DiffStatus::Added => {

                match side {

                    RenderSide::Source => {
                        line.dimmed().to_string()
                    }

                    RenderSide::Target => {
                        line.green().to_string()
                    }
                }
            }

            DiffStatus::Removed => {

                match side {

                    RenderSide::Source => {
                        line.red().to_string()
                    }

                    RenderSide::Target => {
                        line.dimmed().to_string()
                    }
                }
            }

            DiffStatus::Modified => {
                line.yellow().to_string()
            }

            DiffStatus::Unchanged => line,
        };

    lines.push(colored_line);

    let child_prefix =
        if prefix.is_empty() {
            String::from(" ")
        } else if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        };

    let len = node.children.len();

    for (index, child)
        in node.children.iter().enumerate()
    {

        build_diff_lines(
            child,
            side,
            child_prefix.clone(),
            index == len - 1,
            lines,
        );
    }
}

pub fn render_boxed_tree(
    title: &str,
    lines: &[String],
    width: usize,
) {

    let title_text =
        format!(" {} ", title);

    let remaining =
        width.saturating_sub(title_text.len());

    let left =
        remaining / 2;

    let right =
        remaining - left;

    println!(
        "┌{}{}{}┐",
        "─".repeat(left),
        title_text.bold(),
        "─".repeat(right),
    );

    for line in lines {

        println!(
            "│ {:<width$} │",
            line,
            width = width - 2
        );
    }

    println!(
        "└{}┘",
        "─".repeat(width)
    );
}
pub fn render_legend() {

    println!();

    println!("{}", "Legend".bold());

    println!(
        "  {} Added",
        "●".green()
    );

    println!(
        "  {} Removed",
        "●".red()
    );

    println!(
        "  {} Modified",
        "●".yellow()
    );

    println!(
        "  {} Unchanged",
        "●".normal()
    );

    println!();
}