# Gitm
Automatic git mirroring script.

### How it works
This is a simple script that intercepts commands like `git init` and `git push` and automatically configures and pushes to many different remotes.

### Usage
To use this script, create a file called `.gitm.toml` and populate it with repositories like so:
```toml
[repos]
origin = "https://gitea.arsenm.dev/Arsen6331/gitm.git"
gitlab = "https://gitea.arsenm.dev/moussaelianarsen/gitm.git"
```