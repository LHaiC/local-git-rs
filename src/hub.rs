use anyhow::{Context, Result};
use git2::{Repository, RepositoryInitOptions};
use std::fs;
use std::path::{Path, PathBuf};

/// Local Git repository manager
/// Manages creation, deletion, and query of local bare repositories
pub struct LocalGitHub {
    hub_path: PathBuf,
}

impl LocalGitHub {
    /// Create new LocalGitHub instance
    pub fn new<P: AsRef<Path>>(hub_path: P) -> Self {
        Self {
            hub_path: hub_path.as_ref().to_path_buf(),
        }
    }

    /// Initialize hub directory
    /// Create directory if it doesn't exist
    pub fn init(&self) -> Result<()> {
        if !self.hub_path.exists() {
            fs::create_dir_all(&self.hub_path)
                .context("Failed to create hub directory")?;
        }
        Ok(())
    }

    /// Create new bare repository
    ///
    /// # Arguments
    /// * `name` - Repository name (without .git suffix)
    pub fn create_repo(&self, name: &str) -> Result<PathBuf> {
        let repo_name = if name.ends_with(".git") {
            name.to_string()
        } else {
            format!("{}.git", name)
        };

        let repo_path = self.hub_path.join(&repo_name);

        if repo_path.exists() {
            anyhow::bail!("Repository '{}' already exists", name);
        }

        let mut opts = RepositoryInitOptions::new();
        opts.bare(true);
        opts.no_reinit(true);

        Repository::init_opts(&repo_path, &opts)
            .context("Failed to initialize bare repository")?;

        Ok(repo_path)
    }

    /// List all repositories
    pub fn list_repos(&self) -> Result<Vec<String>> {
        if !self.hub_path.exists() {
            return Ok(Vec::new());
        }

        let mut repos = Vec::new();

        for entry in fs::read_dir(&self.hub_path)
            .context("Failed to read hub directory")?
        {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() && path.extension().map_or(false, |e| e == "git") {
                if let Some(name) = path.file_name() {
                    repos.push(name.to_string_lossy().to_string());
                }
            }
        }

        repos.sort();
        Ok(repos)
    }

    /// Delete repository
    pub fn delete_repo(&self, name: &str) -> Result<()> {
        let repo_name = if name.ends_with(".git") {
            name.to_string()
        } else {
            format!("{}.git", name)
        };

        let repo_path = self.hub_path.join(&repo_name);

        if !repo_path.exists() {
            anyhow::bail!("Repository '{}' does not exist", name);
        }

        fs::remove_dir_all(&repo_path)
            .context("Failed to delete repository")?;

        Ok(())
    }

    /// Get full path of repository
    pub fn get_repo_path(&self, name: &str) -> Result<PathBuf> {
        let repo_name = if name.ends_with(".git") {
            name.to_string()
        } else {
            format!("{}.git", name)
        };

        let repo_path = self.hub_path.join(&repo_name);

        if !repo_path.exists() {
            anyhow::bail!("Repository '{}' does not exist", name);
        }

        Ok(repo_path)
    }

    /// Check if repository exists
    pub fn repo_exists(&self, name: &str) -> bool {
        let repo_name = if name.ends_with(".git") {
            name.to_string()
        } else {
            format!("{}.git", name)
        };

        self.hub_path.join(&repo_name).exists()
    }
}