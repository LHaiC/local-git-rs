# local-git-rs

A local Git repository management center implemented in Rust. Create local bare repositories as backup hubs and configure remotes with one click.

## Features

- 🏠 **Local Backup Hub**: Create Git bare repositories locally for backup without network
- ⚡ **One-click Setup**: Quickly add local remotes or backup addresses to existing remotes
- 🔒 **No Root Required**: Uses user-level directories only, no admin permissions needed
- 🚀 **Rust Implementation**: High-performance, memory-safe Rust implementation

## Installation

### Method 1: Using Install Script

```bash
./install.sh
```

This will build the project and install the binary to `~/.local/bin`.

### Method 2: Using Cargo

```bash
cargo install --path .
```

## Quick Start

```bash
# 1. Initialize Hub
local-git-rs init

# 2. Create backup repository
local-git-rs create my-project

# 3. Add to current project
cd ~/projects/my-project
local-git-rs add-remote my-project

# 4. Push to local backup
git push local-hub main
```

## Detailed Usage

### 1. Initialize Hub

```bash
local-git-rs init
```

This creates a Hub directory at `~/.local-git-hub` (customizable via `--hub-path`).

**No prerequisites needed** - the directory will be created automatically.

### 2. Create Backup Repository

```bash
local-git-rs create my-project
```

This creates a bare repository named `my-project.git` in the Hub.

**Prerequisite**: Hub must be initialized first.

### 3. Add to Current Project

There are two ways to add local backup to your project:

#### Method A: Add Independent Remote

```bash
local-git-rs add-remote <name> [--remote-name <name>] [--path <path>]
```

**Parameters**:
- `<name>`: Repository name in hub (required)
- `--remote-name`: Remote name to create (default: `local-hub`)
- `--path`: Target repository path (default: current directory)

**How it works**:
- Creates a new remote in your project's `.git/config`
- Points to the local bare repository in the hub
- You can push/pull independently from other remotes

**Example**:
```bash
# Add with default remote name "local-hub"
local-git-rs add-remote my-project

# Add with custom remote name "backup"
local-git-rs add-remote my-project --remote-name backup

# Add to a different project
local-git-rs add-remote my-project --path ~/projects/other-project
```

**Modified .git/config**:
```ini
[remote "local-hub"]
    url = /home/user/.local-git-hub/my-project.git
    fetch = +refs/heads/*:refs/remotes/local-hub/*
```

**Usage**:
```bash
git push local-hub main
git pull local-hub main
```

#### Method B: Add Backup Push URL to Existing Remote

```bash
local-git-rs add-push-url <name> [--remote-name <name>] [--path <path>]
```

**Parameters**:
- `<name>`: Repository name in hub (required)
- `--remote-name`: Existing remote name to add backup to (default: `origin`)
- `--path`: Target repository path (default: current directory)

**How it works**:
- Adds a `pushurl` to an existing remote
- When you push to that remote, Git pushes to both the original URL and the local backup
- **Dual backup strategy** - push to GitHub/GitLab AND local hub simultaneously

**Example**:
```bash
# Add backup to origin (default)
local-git-rs add-push-url my-project

# Add backup to a different remote
local-git-rs add-push-url my-project --remote-name github
```

**Modified .git/config**:
```ini
[remote "origin"]
    url = https://github.com/user/repo.git
    fetch = +refs/heads/*:refs/remotes/origin/*
    pushurl = https://github.com/user/repo.git
    pushurl = /home/user/.local-git-hub/my-project.git  # Added
```

**Usage**:
```bash
# This pushes to BOTH GitHub and local hub
git push origin main
```

### 4. Clone from Local Hub

```bash
git clone ~/.local-git-hub/<project-name>.git <destination>
```

**Example**:
```bash
# Clone to current directory
git clone ~/.local-git-hub/my-project.git

# Clone to specific directory
git clone ~/.local-git-hub/my-project.git my-project-copy
```

### 5. List Repositories in Hub

```bash
local-git-rs list
```

### 6. Delete Repository

```bash
local-git-rs delete <name>
```

### 7. Other Remote Management Commands

```bash
# List remotes in current repository
local-git-rs list-remotes [--path <path>]

# Remove a remote
local-git-rs remove-remote <remote-name> [--path <path>]
```

## Command Dependencies

```
┌─────────────────────────────────────────────────────────┐
│                    Initialize Hub                       │
├─────────────────────────────────────────────────────────┤
│  local-git-rs init                                      │
│  → Creates ~/.local-git-hub/ directory                  │
│  → No prerequisites needed                              │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│                  Create Backup Repository               │
├─────────────────────────────────────────────────────────┤
│  local-git-rs create <name>                             │
│  → Creates ~/.local-git-hub/<name>.git/                 │
│  → Requires: init must be run first                     │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│            Add to Target Project (Two Methods)          │
├─────────────────────────────────────────────────────────┤
│  Method A: add-remote                                   │
│  local-git-rs add-remote <name>                         │
│  → Adds new remote to current project                   │
│  → Requires: backup repo created                        │
│  → Requires: target project exists                      │
│                                                         │
│  Method B: add-push-url                                 │
│  local-git-rs add-push-url <name>                       │
│  → Adds backup pushurl to existing remote               │
│  → Requires: backup repo created                        │
│  → Requires: target project exists                      │
│  → Requires: specified remote must exist                │
└─────────────────────────────────────────────────────────┘
```

## Usage Scenarios

### Scenario 1: Development Backup

```bash
# 1. Initialize Hub (one-time)
local-git-rs init

# 2. Create backup repository
local-git-rs create my-app

# 3. Add to current project
cd ~/projects/my-app
local-git-rs add-remote my-app

# 4. Push to local backup
git push local-hub main

# 5. Later, clone from local hub
git clone ~/.local-git-hub/my-app.git my-app-restored
```

### Scenario 2: Dual Backup Strategy

```bash
# 1. Initialize Hub
local-git-rs init

# 2. Create backup repository
local-git-rs create my-app

# 3. Add local backup to origin
cd ~/projects/my-app
local-git-rs add-push-url my-app

# 4. Now git push origin pushes to both GitHub and local
git push origin main
```

### Scenario 3: Offline Work

```bash
# 1. Push to local backup before going offline
git push local-hub main

# 2. Work offline...

# 3. Push to GitHub when network is restored
git push origin main
```

### Scenario 4: Multiple Project Backup

```bash
# Initialize Hub
local-git-rs init

# Create backup repos for multiple projects
local-git-rs create project1
local-git-rs create project2
local-git-rs create project3

# Add each project to its backup
cd ~/projects/project1
local-git-rs add-remote project1

cd ~/projects/project2
local-git-rs add-remote project2

cd ~/projects/project3
local-git-rs add-remote project3
```

### Scenario 5: Multiple Backup Remotes

```bash
# Create backup repository
local-git-rs create my-app

# Add multiple backup remotes
cd ~/projects/my-app
local-git-rs add-remote my-app --remote-name backup1
local-git-rs add-remote my-app --remote-name backup2

# Now you can push to multiple backups
git push backup1 main
git push backup2 main
```

## Parameter Reference

| Parameter | Command | Description | Default | Required |
|-----------|---------|-------------|---------|----------|
| `<name>` | create, add-remote, add-push-url, delete | Repository name in hub | - | Yes |
| `--remote-name` | add-remote, add-push-url | Remote name to create or modify | `local-hub` (add-remote)<br>`origin` (add-push-url) | No |
| `--path` | add-remote, add-push-url, list-remotes, remove-remote | Target repository path | Current directory | No |
| `--hub-path` | All | Hub root directory path | `~/.local-git-hub` | No |

## Common Errors and Solutions

### Error: Repository does not exist in hub

```bash
$ local-git-rs add-remote my-project
Error: Repository 'my-project' does not exist in hub
```

**Solution**: Create the repository first
```bash
local-git-rs create my-project
local-git-rs add-remote my-project
```

### Error: Remote already exists

```bash
$ local-git-rs add-remote my-project
Error: Remote 'local-hub' already exists
```

**Solution**: Remove existing remote or use different name
```bash
git remote remove local-hub
# OR
local-git-rs add-remote my-project --remote-name backup
```

### Error: Not a git repository

```bash
$ local-git-rs add-remote my-project
Error: Failed to open repository from current directory
```

**Solution**: Initialize Git repository or specify correct path
```bash
git init
# OR
local-git-rs add-remote my-project --path ~/projects/my-app
```

## Architecture

```
~/.local-git-hub/          # Hub root directory
├── project1.git/          # Bare repository 1
├── project2.git/          # Bare repository 2
└── project3.git/          # Bare repository 3
```

Each `.git` directory is a standard Git bare repository that can be cloned and pushed to like GitHub.

## Technical Implementation

- **git2-rs**: Git operations using libgit2
- **clap**: Modern CLI argument parsing
- **anyhow**: Elegant error handling

## License

MIT