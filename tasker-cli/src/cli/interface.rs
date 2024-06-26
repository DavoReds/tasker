use camino::Utf8PathBuf;
use clap::{Args, Parser, Subcommand, ValueEnum};
use lib_tasker::todos::State;

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
    #[arg(short = 'T', long)]
    pub todo_file: Option<Utf8PathBuf>,

    /// Path to an alternative configuration file. Takes precedence over `todo-file`
    #[arg(short = 'C', long)]
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
    /// Add Task(s)
    #[command(arg_required_else_help = true, visible_alias = "a")]
    Add(AddTasks),

    /// Clean completed Tasks
    #[command(visible_alias = "c")]
    Clean,

    /// Delete Tasks
    #[command(arg_required_else_help = true, visible_alias = "d")]
    Delete(DeleteTasks),

    /// Edit a Task
    #[command(arg_required_else_help = true, visible_alias = "e")]
    Edit(EditTask),

    /// List Tasks
    #[command(visible_alias = "l")]
    List(ListTasks),

    /// Print default paths for the application
    #[command(visible_alias = "p")]
    Paths,

    /// Change the state of a Task
    #[command(arg_required_else_help = true, visible_alias = "t")]
    Toggle(ToggleTasks),
}

#[derive(Args, Debug)]
#[command(help_template(
    "\
{name}
{about-with-newline}
{usage-heading} {usage}

{all-args}"
))]
pub struct AddTasks {
    /// Task(s) to accomplish
    pub descriptions: Vec<String>,

    /// Project the Task(s) belongs to. Defaults to "Inbox"
    #[arg(short, long)]
    pub project: Option<String>,

    /// Tag to assign the Task(s). Can be called multiple times
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
pub struct ToggleTasks {
    /// State to assign the Task(s)
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
pub struct EditTask {
    /// ID of the Task to edit
    #[arg(name = "TO-DO")]
    pub task: usize,

    /// New description
    #[arg(short, long)]
    pub description: Option<String>,

    /// New state
    #[arg(short, long, value_enum)]
    pub state: Option<ToggleState>,

    /// New project
    #[arg(short, long)]
    pub project: Option<String>,

    /// New tags
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
pub struct DeleteTasks {
    /// Ids of the Task(s) to delete
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
pub struct ListTasks {
    /// Sort Tasks by this field
    #[arg(short = 'S', long, value_enum)]
    pub sort_by: Option<SortTasks>,

    /// Only show Tasks containing this text within their descriptions
    #[arg(short, long)]
    pub description: Option<String>,

    /// Only show Tasks with this state of progress
    #[arg(short, long)]
    pub state: Option<ToggleState>,

    /// Only show Tasks containing these tags. Can be called multiple times
    #[arg(short, long)]
    pub tag: Option<Vec<String>>,

    /// Only show Tasks belonging to this project
    #[arg(short, long)]
    pub project: Option<String>,
}

#[derive(Debug, ValueEnum, Clone, Copy)]
pub enum SortTasks {
    /// Sort by description [aliases: desc, d]
    #[value(alias = "desc", alias = "d")]
    Description,

    /// Sort by project [aliases: pro, p]
    #[value(alias = "pro", alias = "p")]
    Project,

    /// Sort by state [aliases: s]
    #[value(alias = "s")]
    State,

    /// Sort by ID [aliases: i]
    #[value(alias = "i")]
    ID,
}
