mod hub;
mod remote;

use anyhow::Result;
use clap::{Parser, Subcommand};
use hub::LocalGitHub;
use remote::RemoteManager;
use std::path::PathBuf;

/// Local Git - Local Git repository management center
#[derive(Parser)]
#[command(name = "local-git-rs")]
#[command(about = "Manage local Git bare repositories as local backup hub", long_about = None)]
struct Cli {
    /// Hub root directory path (default: ~/.local-git-hub)
    #[arg(short, long, global = true)]
    hub_path: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize hub directory
    Init,

    /// Create new bare repository
    Create {
        /// Repository name
        name: String,
    },

    /// List all repositories
    List,

    /// Delete repository
    Delete {
        /// Repository name
        name: String,
    },

    /// Add local remote to current repository
    AddRemote {
        /// Repository name (name in hub)
        name: String,

        /// Remote name (default: local-hub)
        #[arg(short, long, default_value = "local-hub")]
        remote_name: String,

        /// Working directory path (default: current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,
    },

    /// Add local backup push URL to existing remote
    AddPushUrl {
        /// Repository name (name in hub)
        name: String,

        /// Remote name (default: origin)
        #[arg(short, long, default_value = "origin")]
        remote_name: String,

        /// Working directory path (default: current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,
    },

    /// List all remotes in current repository
    ListRemotes {
        /// Working directory path (default: current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,
    },

    /// Delete remote
    RemoveRemote {
        /// Remote name
        remote_name: String,

        /// Working directory path (default: current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
}

fn get_hub_path(cli_path: Option<PathBuf>) -> PathBuf {
    if let Some(path) = cli_path {
        path
    } else {
        // Default to ~/.local-git-hub
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(".local-git-hub")
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let hub_path = get_hub_path(cli.hub_path);

    match cli.command {
        Commands::Init => {
            let hub = LocalGitHub::new(&hub_path);
            hub.init()?;
            println!("✓ Local Git Hub initialized at: {}", hub_path.display());
        }

        Commands::Create { name } => {
            let hub = LocalGitHub::new(&hub_path);
            hub.init()?;
            let repo_path = hub.create_repo(&name)?;
            println!("✓ Repository '{}' created at: {}", name, repo_path.display());
            println!("  Tip: use 'local-git-rs add-remote {}' to add to current project", name);
        }

        Commands::List => {
            let hub = LocalGitHub::new(&hub_path);
            let repos = hub.list_repos()?;

            if repos.is_empty() {
                println!("No repositories in hub");
                println!("Use 'local-git-rs create <name>' to create new repository");
            } else {
                println!("Repositories in hub:");
                for repo in repos {
                    println!("  - {}", repo);
                }
            }
        }

        Commands::Delete { name } => {
            let hub = LocalGitHub::new(&hub_path);
            hub.delete_repo(&name)?;
            println!("✓ Repository '{}' deleted", name);
        }

        Commands::AddRemote {
            name,
            remote_name,
            path,
        } => {
            let hub = LocalGitHub::new(&hub_path);
            if !hub.repo_exists(&name) {
                anyhow::bail!("Repository '{}' does not exist in hub, please use 'local-git-rs create {}' first", name, name);
            }

            let hub_repo_path = hub.get_repo_path(&name)?;
            let path_ref = path.as_deref();

            RemoteManager::add_local_remote(path_ref, &remote_name, &hub_repo_path)?;

            println!("✓ Added remote '{}' -> {}", remote_name, hub_repo_path.display());
            println!("  Now you can use 'git push {} <branch>' to push to local backup", remote_name);
        }

        Commands::AddPushUrl {
            name,
            remote_name,
            path,
        } => {
            let hub = LocalGitHub::new(&hub_path);
            if !hub.repo_exists(&name) {
                anyhow::bail!("Repository '{}' does not exist in hub, please use 'local-git-rs create {}' first", name, name);
            }

            let hub_repo_path = hub.get_repo_path(&name)?;
            let path_ref = path.as_deref();

            RemoteManager::add_push_url(path_ref, &remote_name, &hub_repo_path)?;

            println!("✓ Added local backup push URL for remote '{}'", remote_name);
            println!("  Now every 'git push {}' will also push to local backup", remote_name);
        }

        Commands::ListRemotes { path } => {
            let path_ref = path.as_deref();
            let remotes = RemoteManager::list_remotes(path_ref)?;

            if remotes.is_empty() {
                println!("No remotes in current repository");
            } else {
                println!("Remotes in current repository:");
                for (name, url) in remotes {
                    println!("  {} -> {}", name, url);
                }
            }
        }

        Commands::RemoveRemote {
            remote_name,
            path,
        } => {
            let path_ref = path.as_deref();
            RemoteManager::remove_remote(path_ref, &remote_name)?;
            println!("✓ Remote '{}' deleted", remote_name);
        }
    }

    Ok(())
}