mod hub;
mod remote;

use anyhow::Result;
use chrono::{DateTime, Local};
use clap::{Parser, Subcommand};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm};
use hub::LocalGitHub;
use humansize::format_size;
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
    List {
        /// Show detailed information
        #[arg(short, long)]
        detailed: bool,
    },

    /// Search repositories by name pattern
    Search {
        /// Search pattern
        pattern: String,
    },

    /// Show repository information
    Info {
        /// Repository name
        name: String,
    },

    /// Delete repository
    Delete {
        /// Repository name
        name: String,

        /// Skip confirmation prompt
        #[arg(short, long)]
        force: bool,
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

fn print_success(message: &str) {
    println!("{} {}", "✓".green(), message);
}

fn print_error(message: &str) {
    eprintln!("{} {}", "✗".red(), message);
}

fn print_warning(message: &str) {
    println!("{} {}", "⚠".yellow(), message);
}

fn print_info(message: &str) {
    println!("{} {}", "ℹ".blue(), message);
}

fn print_header(title: &str) {
    println!("\n{}", title.bold().cyan());
    println!("{}", "=".repeat(title.len()).cyan());
}

fn format_datetime(dt: DateTime<Local>) -> String {
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let hub_path = get_hub_path(cli.hub_path);

    match cli.command {
        Commands::Init => {
            let hub = LocalGitHub::new(&hub_path);
            hub.init()?;
            print_success(&format!("Local Git Hub initialized at: {}", hub_path.display()));
        }

        Commands::Create { name } => {
            let hub = LocalGitHub::new(&hub_path);
            hub.init()?;
            let repo_path = hub.create_repo(&name)?;
            print_success(&format!("Repository '{}' created at: {}", name, repo_path.display()));
            print_info(&format!("Use 'local-git-rs add-remote {}' to add to current project", name));
        }

        Commands::List { detailed } => {
            let hub = LocalGitHub::new(&hub_path);

            if detailed {
                print_header("Repositories in Hub");
                let repos = hub.list_repos_with_info()?;

                if repos.is_empty() {
                    print_warning("No repositories in hub");
                    print_info("Use 'local-git-rs create <name>' to create new repository");
                } else {
                    println!(
                        "{:<30} {:>12} {:>10} {:>20}",
                        "Name".bold(),
                        "Size".bold(),
                        "Commits".bold(),
                        "Modified".bold()
                    );
                    println!("{}", "-".repeat(75));

                    for repo in &repos {
                        let size_str = format_size(repo.size, humansize::DECIMAL);
                        let commits_str = repo.commits.map_or("N/A".to_string(), |c| c.to_string());
                        let modified_str = format_datetime(repo.modified);

                        println!(
                            "{:<30} {:>12} {:>10} {:>20}",
                            repo.name.dimmed(),
                            size_str,
                            commits_str.yellow(),
                            modified_str.dimmed()
                        );
                    }

                    println!("\nTotal: {} repositories", repos.len());
                }
            } else {
                let repos = hub.list_repos()?;

                if repos.is_empty() {
                    print_warning("No repositories in hub");
                    print_info("Use 'local-git-rs create <name>' to create new repository");
                } else {
                    print_header("Repositories in Hub");
                    for repo in &repos {
                        println!("  {}", repo.green());
                    }
                    println!("\nTotal: {} repositories", repos.len());
                }
            }
        }

        Commands::Search { pattern } => {
            let hub = LocalGitHub::new(&hub_path);
            let repos = hub.search_repos(&pattern)?;

            print_header(&format!("Search Results for '{}'", pattern));

            if repos.is_empty() {
                print_warning("No repositories found");
            } else {
                for repo in &repos {
                    println!("  {}", repo.green());
                }
                println!("\nFound: {} repositories", repos.len());
            }
        }

        Commands::Info { name } => {
            let hub = LocalGitHub::new(&hub_path);

            if !hub.repo_exists(&name) {
                print_error(&format!("Repository '{}' does not exist", name));
                anyhow::bail!("Repository not found");
            }

            let info = hub.get_repo_info(&name)?;

            print_header(&format!("Repository: {}", info.name));
            println!("  Path:     {}", info.path.display().to_string().dimmed());
            println!("  Size:     {}", format_size(info.size, humansize::DECIMAL).cyan());
            println!("  Commits:  {}", info.commits.map_or("N/A".to_string(), |c| c.to_string()).yellow());
            println!("  Modified: {}", format_datetime(info.modified).dimmed());
        }

        Commands::Delete { name, force } => {
            let hub = LocalGitHub::new(&hub_path);

            if !hub.repo_exists(&name) {
                print_error(&format!("Repository '{}' does not exist", name));
                anyhow::bail!("Repository not found");
            }

            // Get repository info before deletion
            let info = hub.get_repo_info(&name)?;

            if !force {
                print_warning(&format!("You are about to delete repository '{}'", name));
                println!("  Size:    {}", format_size(info.size, humansize::DECIMAL));
                println!("  Commits: {}", info.commits.map_or("N/A".to_string(), |c| c.to_string()));

                let confirm = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Are you sure you want to delete this repository?")
                    .default(false)
                    .interact()?;

                if !confirm {
                    print_info("Deletion cancelled");
                    return Ok(());
                }
            }

            hub.delete_repo(&name)?;
            print_success(&format!("Repository '{}' deleted", name));
        }

        Commands::AddRemote {
            name,
            remote_name,
            path,
        } => {
            let hub = LocalGitHub::new(&hub_path);
            if !hub.repo_exists(&name) {
                print_error(&format!("Repository '{}' does not exist in hub", name));
                print_info(&format!("Use 'local-git-rs create {}' to create it first", name));
                anyhow::bail!("Repository not found");
            }

            let hub_repo_path = hub.get_repo_path(&name)?;
            let path_ref = path.as_deref();

            RemoteManager::add_local_remote(path_ref, &remote_name, &hub_repo_path)?;

            print_success(&format!("Added remote '{}' -> {}", remote_name, hub_repo_path.display()));
            print_info(&format!("Now you can use 'git push {} <branch>' to push to local backup", remote_name));
        }

        Commands::AddPushUrl {
            name,
            remote_name,
            path,
        } => {
            let hub = LocalGitHub::new(&hub_path);
            if !hub.repo_exists(&name) {
                print_error(&format!("Repository '{}' does not exist in hub", name));
                print_info(&format!("Use 'local-git-rs create {}' to create it first", name));
                anyhow::bail!("Repository not found");
            }

            let hub_repo_path = hub.get_repo_path(&name)?;
            let path_ref = path.as_deref();

            RemoteManager::add_push_url(path_ref, &remote_name, &hub_repo_path)?;

            print_success(&format!("Added local backup push URL for remote '{}'", remote_name));
            print_info(&format!("Now every 'git push {}' will also push to local backup", remote_name));
        }

        Commands::ListRemotes { path } => {
            let path_ref = path.as_deref();
            let remotes = RemoteManager::list_remotes(path_ref)?;

            if remotes.is_empty() {
                print_warning("No remotes in current repository");
            } else {
                print_header("Remotes in Current Repository");
                for (name, url) in remotes {
                    println!("  {} -> {}", name.cyan(), url.dimmed());
                }
            }
        }

        Commands::RemoveRemote {
            remote_name,
            path,
        } => {
            let path_ref = path.as_deref();
            RemoteManager::remove_remote(path_ref, &remote_name)?;
            print_success(&format!("Remote '{}' removed", remote_name));
        }
    }

    Ok(())
}
