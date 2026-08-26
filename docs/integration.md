# Integration Guide

`soroban-cost-linter` integrates directly into your workspace and CI/CD pipelines.

## Shell Completions

`cargo-cost-lint` supports generating shell completion scripts. Use the `--completions` flag to generate a script for your shell.

### Bash
```bash
cargo cost-lint --completions bash > ~/.local/share/bash-completion/completions/cargo-cost-lint
```

### Zsh
```zsh
cargo cost-lint --completions zsh > _cargo-cost-lint
# Ensure it's in your fpath
```

### Fish
```fish
cargo cost-lint --completions fish > ~/.config/fish/completions/cargo-cost-lint.fish
```

## Colour Control

... (rest of the file remains same) ...