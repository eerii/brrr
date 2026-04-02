use brrr::{Error, config::Browser};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "brrr")]
#[command(about = "Manage browser development environments")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Setup the development environment")]
    Bootstrap,
    #[command(about = "Manage containers")]
    Container {
        #[command(subcommand)]
        action: ContainerAction,
    },
    #[command(flatten)]
    Exec(ExecCommands),
    #[command(about = "Display information about this browser")]
    Info,
    #[command(about = "Manage worktrees")]
    Worktree {
        #[command(subcommand)]
        action: WorktreeAction,
    },
}

#[derive(Parser)]
struct LastArgs {
    #[arg(last = true)]
    args: Vec<String>,
}

#[derive(Subcommand)]
enum ExecCommands {
    #[command(about = "Build the browser")]
    Build(#[command(flatten)] LastArgs),
    #[command(about = "LSP support outside of the container")]
    Check(#[command(flatten)] LastArgs),
    #[command(about = "Clean build artifacts")]
    Clean(#[command(flatten)] LastArgs),
    #[command(about = "Run the browser")]
    Run(#[command(flatten)] LastArgs),
    #[command(about = "Web platform tests")]
    Test(#[command(flatten)] LastArgs),
}

impl ExecCommands {
    fn name(&self) -> &'static str {
        match self {
            Self::Build(_) => "build",
            Self::Check(_) => "check",
            Self::Clean(_) => "clean",
            Self::Run(_) => "run",
            Self::Test(_) => "test",
        }
    }

    fn args(&self) -> &[String] {
        match self {
            Self::Build(a) | Self::Check(a) | Self::Clean(a) | Self::Run(a) | Self::Test(a) => {
                &a.args
            }
        }
    }
}

#[derive(Debug, Subcommand)]
enum WorktreeAction {
    #[command(about = "Create a new worktree")]
    Create {
        name: String,
        branch: Option<String>,
    },
    #[command(about = "List all worktrees")]
    List,
    #[command(about = "Remove a worktree")]
    Remove { name: String },
    #[command(about = "Switch to a worktree (and optionally to a branch)")]
    Switch {
        name: String,
        branch: Option<String>,
    },
}

#[derive(Debug, Subcommand)]
enum ContainerAction {
    #[command(about = "Run a command inside of the container")]
    Enter {
        #[arg(last = true)]
        args: Vec<String>,
    },
    #[command(about = "Finish bootstrapping the container (if interrupted)")]
    Finish,
    #[command(about = "Stop and remove the container")]
    Remove,
    #[command(about = "Create the container and bootstrap packages")]
    Setup,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> brrr::Result<()> {
    let cli = Cli::parse();
    let context = Browser::discover()?;

    match cli.command {
        Commands::Bootstrap => {
            context.fetch_remote()?;
            context.container_setup()?;
        }
        Commands::Container { action } => match action {
            ContainerAction::Enter { args } => context.run(&args.join("  "))?,
            ContainerAction::Finish => context.container_finish()?,
            ContainerAction::Remove => context.container_remove()?,
            ContainerAction::Setup => context.container_setup()?,
        },
        Commands::Exec(cmd) => {
            let name = cmd.name();
            let run = context
                .config
                .commands
                .get(name)
                .ok_or(Error::NoCommand(name.into()))?;
            if cmd.args().is_empty() {
                context.run(&run)?;
            } else {
                context.run(&format!("{} {}", run, cmd.args().join(" ")))?;
            }
        }
        Commands::Info => {
            println!("broswer: {}", context.name);
            println!("root: {}", context.root.display());
            // TODO: Add container status
            println!("container: {}", context.container_name());
            // TOOD: Print other worktrees and their branches
            println!(
                "worktrees:\n  - {} (main)",
                context.main_worktree().display()
            );
            let config_str = format!("{:#?}", context.config);
            println!("config: {}", config_str.trim_start_matches("Config "));
        }
        Commands::Worktree { action: _ } => {}
    }

    Ok(())
}
