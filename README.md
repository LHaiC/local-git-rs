# local-git-rs

A local Git repository management center implemented in Rust. Create local bare repositories as backup hubs and configure remotes with one click.

## Features

- 🏠 **Local Backup Hub**: Create Git bare repositories locally for backup without network
- ⚡ **One-click Setup**: Quickly add local remotes or backup addresses to existing remotes
- 🔒 **No Root Required**: Uses user-level directories only, no admin permissions needed
- 🚀 **Rust Implementation**: High-performance, memory-safe Rust implementation
- 🎨 **Beautiful UI**: Colorful output with enhanced user experience
- 🔍 **Search & Filter**: Search repositories by name pattern
- 📊 **Detailed Info**: View repository size, commit count, and modification time
- ✅ **Safety First**: Confirmation prompts for destructive operations
- 🛡️ **Input Validation**: Robust validation to prevent errors

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
local-git-rs create <name>
```

Creates a bare repository named `<name>.git` in the Hub.

**Features**:
- Validates repository name (no invalid characters)
- Checks for duplicates
- Prevents reserved names (`.`, `..`)

**Prerequisite**: Hub must be initialized first.

### 3. List Repositories

```bash
# Simple list
local-git-rs list

# Detailed list with size, commits, and modification time
local-git-rs list --detailed
```

**Output Examples**:

Simple list:
```
Repositories in Hub
═══════════════════════════════════════════════════════════════════════════════
  my-project.git
  another-project.git

Total: 2 repositories
```

Detailed list:
```
Repositories in Hub
═══════════════════════════════════════════════════════════════════════════════
Name                           Size      Commits            Modified
──────────────────────────────────────────────────────────────────────────────────
my-project.git                 1.2 MB          42    2025-12-27 15:30:45
another-project.git           256 KB           8    2025-12-26 10:15:20

Total: 2 repositories
```

### 4. Search Repositories

```bash
local-git-rs search <pattern>
```

Search repositories by name pattern (case-insensitive).

**Example**:
```bash
local-git-rs search my
# Output:
# Search Results for 'my'
# ═══════════════════════════════════════════════════════════════════════════════
#   my-project.git
#   my-other-project.git
#
# Found: 2 repositories
```

### 5. View Repository Information

```bash
local-git-rs info <name>
```

Shows detailed information about a repository.

**Example**:
```bash
local-git-rs info my-project
# Output:
# Repository: my-project.git
# ═══════════════════════════════════════════════════════════════════════════════
#   Path:     /home/user/.local-git-hub/my-project.git
#   Size:     1.2 MB
#   Commits:  42
#   Modified: 2025-12-27 15:30:45
```

### 6. Delete Repository

```bash
# With confirmation prompt (default)
local-git-rs delete <name>

# Force delete without confirmation
local-git-rs delete <name> --force
```

**Safety Features**:
- Confirmation prompt with repository details
- Shows size and commit count before deletion
- Validates it's a valid Git repository before deletion
- Use `--force` to skip confirmation (use with caution!)

### 7. Add to Current Project

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

### 8. Clone from Local Hub

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

### 9. Remote Management

```bash
# List remotes in current repository
local-git-rs list-remotes [--path <path>]

# Remove a remote
local-git-rs remove-remote <remote-name> [--path <path>]
```

## Command Dependencies

```
┌─────────────────────────────────────────────────────────┐
│                    Initialize Hub                        │
├─────────────────────────────────────────────────────────┤
│  local-git-rs init                                      │
│  → Creates ~/.local-git-hub/ directory                  │
│  → No prerequisites needed                              │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│                  Create Backup Repository                 │
├─────────────────────────────────────────────────────────┤
│  local-git-rs create <name>                             │
│  → Creates ~/.local-git-hub/<name>.git/                 │
│  → Requires: init must be run first                     │
│  → Validates: name, duplicates, reserved names          │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│            Add to Target Project (Two Methods)           │
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

# 5. View repository info
local-git-rs info my-app

# 6. Later, clone from local hub
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

# List all repositories with details
local-git-rs list --detailed
```

### Scenario 5: Search and Manage

```bash
# Search for projects
local-git-rs search project

# View detailed info
local-git-rs info project1

# Delete old backup (with confirmation)
local-git-rs delete old-project
```

## Safety Features

### Input Validation

- **Repository Name Validation**:
  - No empty names
  - No invalid characters (`/`, `\`, `:`, `*`, `?`, `"`, `<`, `>`, `|`)
  - No reserved names (`.`, `..`)
  - Maximum length 255 characters

### Deletion Protection

- **Confirmation Prompt**: Always asks before deletion
- **Repository Details**: Shows size and commit count before deletion
- **Valid Git Check**: Verifies it's a valid Git repository before deletion
- **Force Option**: `--force` flag to skip confirmation (use carefully!)

### Error Handling

- Clear, colorful error messages
- Helpful suggestions for common errors
- Graceful handling of edge cases

## Parameter Reference

| Parameter | Command | Description | Default | Required |
|-----------|---------|-------------|---------|----------|
| `<name>` | create, add-remote, add-push-url, delete, info | Repository name in hub | - | Yes |
| `<pattern>` | search | Search pattern (case-insensitive) | - | Yes |
| `--remote-name` | add-remote, add-push-url | Remote name to create or modify | `local-hub` (add-remote)<br>`origin` (add-push-url) | No |
| `--path` | add-remote, add-push-url, list-remotes, remove-remote | Target repository path | Current directory | No |
| `--hub-path` | All | Hub root directory path | `~/.local-git-hub` | No |
| `--detailed` | list | Show detailed information | false | No |
| `--force` | delete | Skip confirmation prompt | false | No |

## Common Errors and Solutions

### Error: Repository does not exist in hub

```bash
$ local-git-rs add-remote my-project
✗ Repository 'my-project' does not exist in hub
ℹ Use 'local-git-rs create my-project' to create it first
```

**Solution**: Create the repository first
```bash
local-git-rs create my-project
local-git-rs add-remote my-project
```

### Error: Remote already exists

```bash
$ local-git-rs add-remote my-project
✗ Remote 'local-hub' already exists
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
✗ Failed to open repository from current directory
```

**Solution**: Initialize Git repository or specify correct path
```bash
git init
# OR
local-git-rs add-remote my-project --path ~/projects/my-app
```

### Error: Invalid repository name

```bash
$ local-git-rs create my/project
✗ Repository name cannot contain '/'
```

**Solution**: Use valid characters only
```bash
local-git-rs create my-project
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
- **clap**: Modern CLI argument parsing with derive features
- **anyhow**: Elegant error handling
- **colored**: Beautiful colored terminal output
- **dialoguer**: Interactive confirmation prompts
- **chrono**: Date and time handling
- **humansize**: Human-readable file size formatting

## License

MIT