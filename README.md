# Study Rust Hanzi

A command-line tool for analyzing Chinese characters (Hanzi) written in Rust. This tool processes TSV data containing Chinese characters with their pinyin pronunciations and provides various analysis options.

## Features

- **By Pinyin Analysis**: List unique pinyin pronunciations with frequency counts and associated characters
- **By Tone Analysis**: Show characters grouped by tone for a specific pinyin pronunciation
- **Line Folding**: Wrap long character lists for better readability
- **Shell Completion**: Generate completion scripts for various shells
- **Robust Error Handling**: Handles broken pipes gracefully when used with tools like `head`

## Installation

### Prerequisites

- Rust (edition 2021 or later)
- Cargo package manager

### Building from Source

```bash
git clone https://github.com/suikan4github/study-rust-hanzi.git
cd study-rust-hanzi
cargo build --release
```

The binary will be available at `target/release/study-rust-kanji`.

## Usage

The tool operates on a TSV file named `hanzi.tsv` in the current directory. The TSV format should contain columns for simplified characters, pinyin with tone marks, pinyin without tone marks, and tone numbers.

### Commands

#### List Pinyin by Frequency

```bash
./study-rust-kanji by-pinyin
```

This command lists all unique pinyin pronunciations sorted by frequency (most common first), showing:
- The pinyin without tone marks
- Count of characters with that pronunciation
- All characters with that pronunciation

Example output:
```
ji      :  82 机己及几计基即记济极击集级技际纪继急激既积吉迹疾季辑鸡剂绩籍寄挤寂祭忌肌玑饥脊稽冀藉圾姬嫉妓棘讥畸缉叽矶羁伎汲诘悸暨亟笈戟唧骥稷瘠跻嵇髻鲫岌偈蓟箕畿觊乩犄霁麂楫芨屐
yi      :  79 一以意已义议易医依益疑异衣伊艺移亦遗亿译役仪宜翼忆椅抑疫乙毅矣谊姨夷逸溢蚁怡倚裔懿颐绎奕咦邑胰诣贻揖彝醫驿漪翌臆沂佚屹轶熠弋诒弈翊呓噫蜴壹薏迤刈咿铱旖羿苡缢翳
```

#### Fold Long Lines

Use the `--fold` option to wrap long character lists:

```bash
./study-rust-kanji by-pinyin --fold 50
./study-rust-kanji by-pinyin -f    # Uses default fold width of 50
```

#### Show Characters by Tone

```bash
./study-rust-kanji by-tone <pinyin>
```

This command shows all characters for a specific pinyin pronunciation, grouped by tone:

```bash
./study-rust-kanji by-tone ji
```

Example output:
```
jī: 机基击激积迹鸡绩肌玑饥稽圾姬讥畸缉叽矶羁唧跻嵇箕畿乩犄芨屐
jí: 及即极集级急吉疾辑籍脊藉嫉棘汲诘亟笈瘠岌楫
jǐ: 己几挤戟麂
jì: 计记济技际纪继既季剂寄寂祭忌冀妓伎悸暨骥稷髻鲫偈蓟觊霁
```

#### Generate Shell Completions

```bash
./study-rust-kanji generate-completion <shell>
```

Supported shells: `bash`, `zsh`, `fish`, `powershell`, `elvish`

See [COMPLETION.md](COMPLETION.md) for detailed installation instructions.

### Examples

```bash
# Get top 10 most common pinyin pronunciations
./study-rust-kanji by-pinyin | head -10

# Find all characters pronounced "ma"
./study-rust-kanji by-tone ma

# Generate bash completion script
./study-rust-kanji generate-completion bash > completion.bash

# Use with folding for better readability
./study-rust-kanji by-pinyin --fold 30 | less
```

## Data Format

The tool expects a TSV file named `hanzi.tsv` with the following format:

```tsv
frequency	simplified	traditional	pinyin	pinyin_without_tone	tone
1	的	的	de	de	5
2	一	一	yī	yi	1
3	是	是	shì	shi	4
4	不	不	bù	bu	4
5	了	了	le	le	5
```

The columns are:
- **frequency**: Usage frequency rank (1 = most common)
- **simplified**: Simplified Chinese character
- **traditional**: Traditional Chinese character 
- **pinyin**: Pinyin with tone marks
- **pinyin_without_tone**: Pinyin without tone marks
- **tone**: Tone number (1-4 for main tones, 5 for neutral tone)

See [HANZI_TSV_FORMAT.md](HANZI_TSV_FORMAT.md) for detailed format specifications.

## Development

### Project Structure

```
├── src/
│   ├── lib.rs          # Library code for reading TSV files
│   └── main.rs         # Main application logic
├── hanzi.tsv           # Sample data file
├── Cargo.toml          # Project configuration
├── README.md           # This file
├── COMPLETION.md       # Shell completion setup guide
└── HANZI_TSV_FORMAT.md # Data format documentation
```

### Dependencies

- `clap` - Command-line argument parsing with derive macros
- `clap_complete` - Shell completion generation

### Building and Testing

```bash
# Build in debug mode
cargo build

# Build optimized release
cargo build --release

# Run tests
cargo test

# Check code formatting
cargo fmt

# Run linter
cargo clippy
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Command-line interface powered by [clap](https://clap.rs/)
- Inspired by the need for efficient Hanzi learning tools
- Hanzi frequency and pinyin data sourced from [hanziDB](http://hanzidb.org/).
