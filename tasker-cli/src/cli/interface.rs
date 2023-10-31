use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

/// A command-line application to manage your daily tasks
#[derive(Debug, Parser)]
#[command(
    name = "Tasker CLI",
    author,
    version,
    about,
    long_about = None,
    help_template = "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}"
)]
pub struct Cli {
    /// Application subcommand
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Path to a file in which to look for and save To-Do's
    #[arg(short, long)]
    pub tasks_file: Option<PathBuf>,

    /// Path to an alternative configuration file
    #[arg(short, long)]
    pub config_file: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
#[command(help_template(
    "\
{name}
{about-with-newline}
{usage-heading} {usage}

{all-args}"
))]
pub enum Command {
    /// Add one To-Do
    #[command(arg_required_else_help = true)]
    Add(AddToDo),

    /// Add multiple To-Do's
    #[command(arg_required_else_help = true, name = "addm")]
    AddMultiple(AddMultipleToDo),
}

#[derive(Args, Debug)]
#[command(help_template(
    "\
{name}
{about-with-newline}
{usage-heading} {usage}

{all-args}"
))]
pub struct AddToDo {
    /// Task to accomplish, wrap in quotes for multi-word tasks.
    pub description: String,

    /// Project the To-Do belongs to. Defaults to "Inbox"
    #[arg(short, long)]
    pub project: Option<String>,

    /// Tags to classify the To-Do
    pub tags: Option<Vec<String>>,
}

#[derive(Args, Debug)]
#[command(help_template(
    "\
{name}
{about-with-newline}
{usage-heading} {usage}

{all-args}"
))]
pub struct AddMultipleToDo {
    /// Tasks to accomplish, wrap individual tasks in quotes for multi-word
    /// tasks.
    pub descriptions: Vec<String>,

    /// Project the To-Do's belongs to. Defaults to "Inbox"
    #[arg(short, long)]
    pub project: Option<String>,
}
