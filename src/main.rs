use brrr::browser::BrowserContext;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "brrr")]
#[command(about = "manage browser development environments")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Bootstrap,
    Build {
        #[arg(long)]
        release: bool,
    },
    Clean,
    Container {
        #[command(subcommand)]
        action: ContainerAction,
    },
    Git {
        #[arg(last = true)]
        args: Vec<String>,
    },
    Run {
        #[arg(last = true)]
        args: Vec<String>,
    },
    Status,
    Test {
        #[arg(last = true)]
        args: Vec<String>,
    },
    Worktree {
        #[command(subcommand)]
        action: WorktreeAction,
    },
}

#[derive(Debug, Subcommand)]
enum WorktreeAction {
    Create {
        name: String,
        branch: Option<String>,
    },
    List,
    Remove {
        name: String,
    },
    Switch {
        name: String,
        branch: Option<String>,
    },
}

#[derive(Debug, Subcommand)]
enum ContainerAction {
    Enter {
        #[arg(last = true)]
        args: Vec<String>,
    },
    Finish,
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
    let context = BrowserContext::detect()?;

    match cli.command {
        Commands::Bootstrap => {
            context.fetch_remote()?;
            context.container_setup()?;
        }
        Commands::Build { release: _ } => {}
        Commands::Clean => {}
        Commands::Container { action } => match action {
            ContainerAction::Enter { args } => context.container_enter(args)?,
            ContainerAction::Finish => context.container_finish()?,
            ContainerAction::Setup => context.container_setup()?,
        },
        Commands::Git { args: _ } => {}
        Commands::Run { args: _ } => {}
        Commands::Status => {}
        Commands::Test { args: _ } => {}
        Commands::Worktree { action: _ } => {}
    }

    Ok(())
}
