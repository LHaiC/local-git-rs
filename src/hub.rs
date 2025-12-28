use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use git2::{Repository, RepositoryInitOptions};
use std::fs;
use std::path::{Path, PathBuf};

/// Repository information
#[derive(Debug, Clone)]
pub struct RepoInfo {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub modified: DateTime<Local>,
    pub commits: Option<usize>,
}

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
        // Validate repository name
        self.validate_repo_name(name)?;

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

    /// List all repositories with detailed information
    pub fn list_repos_with_info(&self) -> Result<Vec<RepoInfo>> {
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
                    let name_str = name.to_string_lossy().to_string();
                    if let Ok(info) = self.get_repo_info(&name_str) {
                        repos.push(info);
                    }
                }
            }
        }

        repos.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(repos)
    }

    /// Search repositories by name pattern
    pub fn search_repos(&self, pattern: &str) -> Result<Vec<String>> {
        let all_repos = self.list_repos()?;
        let pattern_lower = pattern.to_lowercase();

        let filtered: Vec<String> = all_repos
            .into_iter()
            .filter(|name| name.to_lowercase().contains(&pattern_lower))
            .collect();

        Ok(filtered)
    }

    /// Delete repository with safety checks
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

        // Additional safety check: verify it's actually a git repository
        if !self.is_valid_git_repo(&repo_path)? {
            anyhow::bail!("Path '{}' is not a valid Git repository", repo_path.display());
        }

        fs::remove_dir_all(&repo_path)
            .context("Failed to delete repository")?;

        Ok(())
    }

    /// Get repository information
    pub fn get_repo_info(&self, name: &str) -> Result<RepoInfo> {
        let repo_name = if name.ends_with(".git") {
            name.to_string()
        } else {
            format!("{}.git", name)
        };

        let repo_path = self.hub_path.join(&repo_name);

        if !repo_path.exists() {
            anyhow::bail!("Repository '{}' does not exist", name);
        }

        // Get repository size
        let size = self.get_dir_size(&repo_path)?;

        // Get modification time
        let metadata = fs::metadata(&repo_path)?;
        let modified: DateTime<Local> = metadata.modified()?.into();

        // Get commit count
        let commits = self.get_commit_count(&repo_path);

        Ok(RepoInfo {
            name: repo_name,
            path: repo_path,
            size,
            modified,
            commits,
        })
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

    /// Validate repository name
    fn validate_repo_name(&self, name: &str) -> Result<()> {
        if name.is_empty() {
            anyhow::bail!("Repository name cannot be empty");
        }

        // Check for invalid characters
        let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
        for c in invalid_chars {
            if name.contains(c) {
                anyhow::bail!("Repository name cannot contain '{}'", c);
            }
        }

        // Check for reserved names
        if name == "." || name == ".." {
            anyhow::bail!("Repository name cannot be '.' or '..'");
        }

        // Check length
        if name.len() > 255 {
            anyhow::bail!("Repository name is too long (max 255 characters)");
        }

        Ok(())
    }

    /// Calculate directory size recursively
    fn get_dir_size(&self, path: &Path) -> Result<u64> {
        let mut total = 0;

        if path.is_dir() {
            for entry in fs::read_dir(path)
                .context("Failed to read directory")?
            {
                let entry = entry?;
                let entry_path = entry.path();

                if entry_path.is_dir() {
                    total += self.get_dir_size(&entry_path)?;
                } else {
                    total += entry.metadata()?.len();
                }
            }
        }

        Ok(total)
    }

    /// Check if path is a valid Git repository
    fn is_valid_git_repo(&self, path: &Path) -> Result<bool> {
        let head_path = path.join("HEAD");
        let objects_path = path.join("objects");
        let refs_path = path.join("refs");

        Ok(head_path.exists() && objects_path.exists() && refs_path.exists())
    }

    /// Get commit count from repository
    fn get_commit_count(&self, path: &Path) -> Option<usize> {
        match Repository::open(path) {
            Ok(repo) => {
                match repo.revparse_single("HEAD") {
                    Ok(commit) => {
                        match commit.as_commit() {
                            Some(c) => {
                                // Count commits in the history
                                let mut count = 0;
                                let mut revwalk = match repo.revwalk() {
                                    Ok(w) => w,
                                    Err(_) => return None,
                                };
                                if revwalk.push(c.id()).is_ok() {
                                    count = revwalk.count();
                                }
                                Some(count)
                            }
                            None => None,
                        }
                    }
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }
}
