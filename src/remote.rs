use anyhow::{Context, Result};
use git2::Repository;
use std::path::Path;

/// Remote manager
/// Manages adding and remotes for local repositories
pub struct RemoteManager;

impl RemoteManager {
    /// Add local remote to current Git repository
    ///
    /// # Arguments
    /// * `repo_path` - Current repository path (None for current directory)
    /// * `remote_name` - Remote name (e.g., "local-hub")
    /// * `hub_repo_path` - Path to bare repository in local hub
    pub fn add_local_remote(
        repo_path: Option<&Path>,
        remote_name: &str,
        hub_repo_path: &Path,
    ) -> Result<()> {
        let repo = if let Some(path) = repo_path {
            Repository::open(path)
                .context("Failed to open repository")?
        } else {
            Repository::open_from_env()
                .context("Failed to open repository from current directory")?
        };

        let hub_repo_str = hub_repo_path
            .to_str()
            .context("Hub repo path is not valid UTF-8")?;

        // Check if remote already exists
        if repo.find_remote(remote_name).is_ok() {
            anyhow::bail!("Remote '{}' already exists", remote_name);
        }

        // Add remote
        repo.remote(remote_name, hub_repo_str)
            .context("Failed to add remote")?;

        Ok(())
    }

    /// Add extra push URL to existing remote
    /// Enables pushing to multiple destinations simultaneously
    ///
    /// # Arguments
    /// * `repo_path` - Current repository path
    /// * `remote_name` - Remote name (usually "origin")
    /// * `hub_repo_path` - Path to bare repository in local hub
    pub fn add_push_url(
        repo_path: Option<&Path>,
        remote_name: &str,
        hub_repo_path: &Path,
    ) -> Result<()> {
        let repo = if let Some(path) = repo_path {
            Repository::open(path)
                .context("Failed to open repository")?
        } else {
            Repository::open_from_env()
                .context("Failed to open repository from current directory")?
        };

        let hub_repo_str = hub_repo_path
            .to_str()
            .context("Hub repo path is not valid UTF-8")?;

        // git2-rs doesn't directly support multiple push URLs, need config file operation
        // Use git config command to add multiple push URLs
        let mut config = repo.config().context("Failed to open config")?;

        // Get current push URL config
        let push_url_key = format!("remote.{}.pushurl", remote_name);

        // Check if already exists
        if let Ok(existing) = config.get_string(&push_url_key) {
            if existing == hub_repo_str {
                anyhow::bail!("Push URL '{}' already exists for remote '{}'", hub_repo_str, remote_name);
            }
        }

        // Add new push URL
        // Note: git2-rs API is limited, using git config approach
        // Add extra push destination by setting pushurl
        config.set_str(&push_url_key, hub_repo_str)
            .context("Failed to add push URL")?;

        Ok(())
    }

    /// List all remotes in current repository
    pub fn list_remotes(repo_path: Option<&Path>) -> Result<Vec<(String, String)>> {
        let repo = if let Some(path) = repo_path {
            Repository::open(path)
                .context("Failed to open repository")?
        } else {
            Repository::open_from_env()
                .context("Failed to open repository from current directory")?
        };

        let mut remotes = Vec::new();

        for remote in repo.remotes()
            .context("Failed to list remotes")?
            .iter()
            .flatten()
        {
            let remote = repo.find_remote(remote)
                .context("Failed to find remote")?;

            if let Some(name) = remote.name() {
                if let Some(url) = remote.url() {
                    remotes.push((name.to_string(), url.to_string()));
                    // Also show push URL if exists
                    if let Some(push_url) = remote.pushurl() {
                        if push_url != url {
                            remotes.push((format!("{} (push)", name), push_url.to_string()));
                        }
                    }
                }
            }
        }

        Ok(remotes)
    }

    /// Delete remote
    pub fn remove_remote(repo_path: Option<&Path>, remote_name: &str) -> Result<()> {
        let repo = if let Some(path) = repo_path {
            Repository::open(path)
                .context("Failed to open repository")?
        } else {
            Repository::open_from_env()
                .context("Failed to open repository from current directory")?
        };

        repo.remote_delete(remote_name)
            .context(format!("Failed to delete remote '{}'", remote_name))?;

        Ok(())
    }
}