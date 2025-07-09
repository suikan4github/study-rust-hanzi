# Shell Completion Setup

This program supports generating shell completion scripts for various shells.

## Usage

Generate completion scripts using the `generate-completion` subcommand:

```bash
# For Bash
./study-rust-hanzi generate-completion bash > study-rust-hanzi-completion.bash

# For Zsh
./study-rust-hanzi generate-completion zsh > _study-rust-hanzi

# For Fish
./study-rust-hanzi generate-completion fish > study-rust-hanzi.fish

# For PowerShell
./study-rust-hanzi generate-completion powershell > study-rust-hanzi-completion.ps1

# For Elvish
./study-rust-hanzi generate-completion elvish > study-rust-hanzi-completion.elv
```

## Installation

### Bash
```bash
# Copy to bash completion directory
sudo cp study-rust-hanzi-completion.bash /etc/bash_completion.d/

# Or source in your .bashrc
echo "source /path/to/study-rust-hanzi-completion.bash" >> ~/.bashrc
```

### Zsh
```bash
# Copy to a directory in your fpath
cp _study-rust-hanzi ~/.local/share/zsh/completions/

# Or add to your .zshrc
echo "fpath=(~/.local/share/zsh/completions $fpath)" >> ~/.zshrc
echo "autoload -U compinit && compinit" >> ~/.zshrc
```

### Fish
```bash
# Copy to fish completions directory
cp study-rust-hanzi.fish ~/.config/fish/completions/
```

## Features

The completion scripts support:
- Subcommand completion (`by-pinyin`, `by-tone`, `by-onset`, `generate-completion`)
- Option completion (`--fold`, `--help`, `--version`)
- Shell-specific completion for the `generate-completion` subcommand
