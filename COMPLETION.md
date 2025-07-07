# Shell Completion Setup

This program supports generating shell completion scripts for various shells.

## Usage

Generate completion scripts using the `generate-completion` subcommand:

```bash
# For Bash
./study-rust-kanji generate-completion bash > study-rust-kanji-completion.bash

# For Zsh
./study-rust-kanji generate-completion zsh > _study-rust-kanji

# For Fish
./study-rust-kanji generate-completion fish > study-rust-kanji.fish

# For PowerShell
./study-rust-kanji generate-completion powershell > study-rust-kanji-completion.ps1

# For Elvish
./study-rust-kanji generate-completion elvish > study-rust-kanji-completion.elv
```

## Installation

### Bash
```bash
# Copy to bash completion directory
sudo cp study-rust-kanji-completion.bash /etc/bash_completion.d/

# Or source in your .bashrc
echo "source /path/to/study-rust-kanji-completion.bash" >> ~/.bashrc
```

### Zsh
```bash
# Copy to a directory in your fpath
cp _study-rust-kanji ~/.local/share/zsh/completions/

# Or add to your .zshrc
echo "fpath=(~/.local/share/zsh/completions $fpath)" >> ~/.zshrc
echo "autoload -U compinit && compinit" >> ~/.zshrc
```

### Fish
```bash
# Copy to fish completions directory
cp study-rust-kanji.fish ~/.config/fish/completions/
```

## Features

The completion scripts support:
- Subcommand completion (`by-pinyin`, `by-tone`, `generate-completion`)
- Option completion (`--fold`, `--help`, `--version`)
- Shell-specific completion for the `generate-completion` subcommand
