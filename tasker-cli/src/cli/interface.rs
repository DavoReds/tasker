use camino::Utf8PathBuf;
use clap::{Args, Parser, Subcommand, ValueEnum};
use tasker_lib::todos::State;

/// A command-line application to manage your daily tasks.
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
    /// Add one To-Do
    #[command(arg_required_else_help = true, visible_alias = "a")]
    Add(AddToDo),

    /// Add multiple To-Do's
    #[command(arg_required_else_help = true, name = "addm", visible_alias = "am")]
    AddMultiple(AddMultipleToDo),

    /// Change the state of a To-Do
    #[command(arg_required_else_help = true, visible_alias = "t")]
    Toggle(ToggleToDo),

    /// Edit a To-Do
    #[command(arg_required_else_help = true, visible_alias = "e")]
    Edit(EditToDo),

    /// Delete To-Do's
    #[command(arg_required_else_help = true, visible_alias = "d")]
    Delete(DeleteToDo),

    /// Clean completed To-Do's
    #[command(visible_alias = "c")]
    Clean,
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
    /// Tasks to accomplish, wrap individual To-Dos in quotes for multi-word
    /// To-Dos.
    pub descriptions: Vec<String>,

    /// Project the To-Do's belongs to. Defaults to "Inbox"
    #[arg(short, long)]
    pub project: Option<String>,
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
    /// State to assign the To-Do
    #[arg(value_enum)]
    pub state: ToggleState,

    /// ID(s) of the To-Do(s) to toggle
    #[arg(name = "TO-DOS")]
    pub tasks: Vec<usize>,
}

#[derive(Debug, ValueEnum, Clone, Copy)]
pub enum ToggleState {
    /// This To-Do hasn't started
    #[value(name = "todo", alias = "t")]
    ToDo,

    /// This To-Do is in progress
    #[value(alias = "dg")]
    Doing,

    /// This To-Do is finished
    #[value(alias = "dn")]
    Done,

    /// This To-Do can't be accomplished due to external reasons
    #[value(name = "wait", alias = "w", alias = "waiting")]
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
    /// ID of the To-Do to edit
    #[arg(name = "TO-DO")]
    pub task: usize,

    /// New description for the To-Do
    #[arg(short, long)]
    pub description: Option<String>,

    /// New state for the To-Do
    #[arg(short, long, value_enum)]
    pub state: Option<ToggleState>,

    /// New project for the To-Do
    #[arg(short, long)]
    pub project: Option<String>,

    /// New tags for the To-Do
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
    /// Id's of To-Do(s) to delete
    #[arg(name = "TO-DOS")]
    pub tasks: Vec<usize>,
}
