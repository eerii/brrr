use brrr::{Error, browser::BrowserContext};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "bx")]
#[command(about = "Manage browser development environments", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Worktree {
        #[command(subcommand)]
        action: WorktreeAction,
    },
    Container {
        #[command(subcommand)]
        action: ContainerAction,
    },
    Build {
        #[arg(long)]
        release: bool,
    },
    Test {
        #[arg(last = true)]
        args: Vec<String>,
    },
    Run {
        #[arg(last = true)]
        args: Vec<String>,
    },
    Clean,
    Status,
    Git {
        #[arg(last = true)]
        args: Vec<String>,
    },
    Exec {
        #[arg(last = true)]
        command: Vec<String>,
    },
}

#[derive(Debug, Subcommand)]
enum WorktreeAction {
    Create {
        name: String,
        branch: Option<String>,
    },
    Switch {
        name: String,
        branch: Option<String>,
    },
    List,
    Remove {
        name: String,
    },
}

#[derive(Debug, Subcommand)]
enum ContainerAction {
    Setup,
    Enter,
    Status,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> brrr::Result<()> {
    let cli = Cli::parse();
    let browser = BrowserContext::new().ok_or(Error::NotInBrowserDir)?;
    dbg!(browser.browser, browser.repo.workdir());

    match cli.command {
        Commands::Worktree { action: _ } => {}
        Commands::Container { action: _ } => {}
        Commands::Build { release: _ } => {}
        Commands::Test { args: _ } => {}
        Commands::Run { args: _ } => {}
        Commands::Clean => {}
        Commands::Status => {}
        Commands::Git { args: _ } => {}
        Commands::Exec { command: _ } => {}
    }

    Ok(())
}
