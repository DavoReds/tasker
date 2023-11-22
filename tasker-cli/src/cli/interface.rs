use camino::Utf8PathBuf;
use clap::{Args, Parser, Subcommand, ValueEnum};
use tasker_lib::todos::State;

/// A command-line application to manage your daily Tasks.
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

    /// Path to a file in which to look for and save Tasks
    #[arg(long = "todo-file")]
    pub to_do_file: Option<Utf8PathBuf>,

    /// Path to an alternative configuration file. Takes precedence over `todo-file`
    #[arg(long)]
    pub config_file: Option<Utf8PathBuf>,
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
    /// Add one Task
    #[command(arg_required_else_help = true, visible_alias = "a")]
    Add(AddToDo),

    /// Add multiple Tasks
    #[command(arg_required_else_help = true, name = "addm", visible_alias = "am")]
    AddMultiple(AddMultipleToDo),

    /// Clean completed Tasks
    #[command(visible_alias = "c")]
    Clean,

    /// Delete Tasks
    #[command(arg_required_else_help = true, visible_alias = "d")]
    Delete(DeleteToDo),

    /// Edit a Task
    #[command(arg_required_else_help = true, visible_alias = "e")]
    Edit(EditToDo),

    /// List Tasks
    #[command(visible_alias = "l")]
    List(ListToDo),

    /// Change the state of a Task
    #[command(arg_required_else_help = true, visible_alias = "t")]
    Toggle(ToggleToDo),
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

    /// Project the Task belongs to. Defaults to "Inbox"
    #[arg(short, long)]
    pub project: Option<String>,

    /// Tag to classify the Task. Can be called multiple times
    #[arg(short, long)]
    pub tag: Option<Vec<String>>,
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
    /// Tasks to accomplish, wrap individual Tasks in quotes for multi-word
    /// Tasks.
    pub descriptions: Vec<String>,

    /// Project the Task's belongs to. Defaults to "Inbox"
    #[arg(short, long)]
    pub project: Option<String>,

    /// Tag to assign the Task's. Can be called multiple times
    #[arg(short, long)]
    pub tag: Option<Vec<String>>,
}

#[derive(Args, Debug)]
#[command(help_template(
    "\
{name}
{about-with-newline}
{usage-heading} {usage}

{all-args}"
))]
pub struct ToggleToDo {
    /// State to assign the Task
    #[arg(value_enum)]
    pub state: ToggleState,

    /// ID(s) of the Task(s) to toggle
    #[arg(name = "TO-DOS")]
    pub tasks: Vec<usize>,
}

#[derive(Debug, ValueEnum, Clone, Copy)]
pub enum ToggleState {
    /// This Task hasn't started
    #[value(name = "todo", alias = "t")]
    ToDo,

    /// This Task is in progress
    #[value(alias = "dg")]
    Doing,

    /// This Task is finished
    #[value(alias = "dn")]
    Done,

    /// This Task can't be accomplished due to external reasons
    #[value(name = "wait", alias = "w")]
    Waiting,
}

impl From<ToggleState> for State {
    fn from(value: ToggleState) -> Self {
        match value {
            ToggleState::ToDo => Self::ToDo,
            ToggleState::Doing => Self::Doing,
            ToggleState::Done => Self::Done,
            ToggleState::Waiting => Self::Waiting,
        }
    }
}

#[derive(Args, Debug)]
#[command(help_template(
    "\
{name}
{about-with-newline}
{usage-heading} {usage}

{all-args}"
))]
pub struct EditToDo {
    /// ID of the Task to edit
    #[arg(name = "TO-DO")]
    pub task: usize,

    /// Change Task description
    #[arg(short, long)]
    pub description: Option<String>,

    /// Change Task progress
    #[arg(short, long, value_enum)]
    pub state: Option<ToggleState>,

    /// Change the Task's project
    #[arg(short, long)]
    pub project: Option<String>,

    /// Replace the Task's tags. Can be called multiple times
    #[arg(short, long)]
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
pub struct DeleteToDo {
    /// Id's of Task(s) to delete
    #[arg(name = "TASKS")]
    pub tasks: Vec<usize>,
}

#[derive(Args, Debug)]
#[command(help_template(
    "\
{name}
{about-with-newline}
{usage-heading} {usage}

{all-args}"
))]
pub struct ListToDo {
    /// Sort tasks by this field
    #[arg(short, long, value_enum)]
    pub sort_by: Option<SortToDo>,

    /// Only show tasks containing this text within their descriptions
    #[arg(short, long)]
    pub description: Option<String>,

    /// Only show tasks within this state of progress
    #[arg(short, long)]
    pub progress: Option<ToggleState>,

    /// Only show tags containing this tag. Can be called multiple times
    #[arg(short, long)]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, ValueEnum, Clone, Copy)]
pub enum SortToDo {
    /// Sort by description [aliases: desc, d]
    #[value(alias = "desc", alias = "d")]
    Description,

    /// Sort by project [aliases: pro, p]
    #[value(alias = "pro", alias = "p")]
    Project,

    /// Sort by state [aliases: s]
    #[value(alias = "s")]
    State,
}
