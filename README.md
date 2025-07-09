# Study Rust Hanzi

A command-line tool for analyzing Chinese characters (Hanzi) written in Rust. This tool processes TSV data containing Chinese characters with their pinyin pronunciations and provides various analysis options.

## Features

- **Pinyin Analysis**: List unique pinyin pronunciations with frequency counts and associated characters
- **Tone Analysis**: Show characters grouped by tone for a specific pinyin pronunciation
- **Onset Analysis**: Group and count characters by their pinyin onset (initial consonant sound)
- **Traditional/Simplified Character Support**: Switch between simplified and traditional Chinese characters
- **V-to-Ü Replacement**: Automatically converts 'v' to 'ü' in pinyin input for easier typing
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

The binary will be available at `target/release/study-rust-hanzi`.

## Usage

The tool operates on a TSV file named `hanzi.tsv` in the current directory. The TSV format should contain columns for simplified characters, pinyin with tone marks, pinyin without tone marks, and tone numbers.

### Commands

#### List Pinyin by Frequency

```bash
./study-rust-hanzi pinyin [OPTIONS] [PINYIN]
```

This command lists all unique pinyin pronunciations sorted by frequency (most common first), showing:
- The pinyin without tone marks
- Count of characters with that pronunciation
- All characters with that pronunciation

**Options:**
- `--traditional`, `-t`: Use traditional characters instead of simplified
- `--fold [WIDTH]`, `-f [WIDTH]`: Fold long lines (default width: 50)
- `[PINYIN]`: Optional specific pinyin to filter results (e.g., `ji`, `yi`, `yu`)

Example output:
```
ji      :  82 机己及几计基即记济极击集级技际纪继急激既积吉迹疾季辑...
yi      :  79 一以意已义议易医依益疑异衣伊艺移亦遗亿译役仪宜翼忆椅...
yu      :  69 于与语育预余域鱼遇予雨玉欲宇愈御狱誉渔羽愚郁豫愉裕...
```

#### Fold Long Lines

Use the `--fold` option to wrap long character lists:

```bash
./study-rust-hanzi pinyin --fold 50
./study-rust-hanzi pinyin -f    # Uses default fold width of 50
```

Example with folding at 30 characters:
```bash
./study-rust-hanzi pinyin -f 30
```

Output:
```
ji      :  82 机己及几计基即记济极击集级技际纪继急激既积吉迹疾季辑鸡剂绩籍
              寄挤寂祭忌肌玑饥脊稽冀藉圾姬嫉妓棘讥畸缉叽矶羁伎汲诘悸暨亟笈
              戟唧骥稷瘠跻嵇髻鲫岌偈蓟箕畿觊乩犄霁麂楫芨屐
yi      :  79 一以意已义议易医依益疑异衣伊艺移亦遗亿译役仪宜翼忆椅抑疫乙毅
              矣谊姨夷逸溢蚁怡倚裔懿颐绎奕咦邑胰诣贻揖彝醫驿漪翌臆沂佚屹轶
              熠弋诒弈翊呓噫蜴壹薏迤刈咿铱旖羿苡缢翳
```

#### Show specific pinyin by Tone

With the `[PINYIN]` argument, you can filter results to show characters for a specific pinyin pronunciation, grouped by tone.


**Options:**
- `--traditional`, `-t`: Use traditional characters instead of simplified

**V-to-Ü Replacement:** You can use 'v' as a substitute for 'ü' when typing. For example, `nv` will be automatically converted to `nü`.

```bash
./study-rust-hanzi pinyin ji
./study-rust-hanzi pinyin nv    # Automatically converted to "nü"
./study-rust-hanzi pinyin lv    # Automatically converted to "lü"
```

Example output:
```
jī: 机基击激积迹鸡绩肌玑饥稽圾姬讥畸缉叽矶羁唧跻嵇箕畿乩犄芨屐
jí: 及即极集级急吉疾辑籍脊藉嫉棘汲诘亟笈瘠岌楫
jǐ: 己几挤戟麂
jì: 计记济技际纪继既季剂寄寂祭忌冀妓伎悸暨骥稷髻鲫偈蓟觊霁
```

#### Group Characters by Onset

```bash
./study-rust-hanzi onset [OPTIONS] [ONSET]
```

This command groups and counts all Chinese characters by their pinyin onset (initial consonant sound). The onset is the consonant or consonant cluster that begins a syllable. This analysis is useful for understanding the distribution of initial sounds in Chinese.

**Options:**
- `--traditional`, `-t`: Use traditional characters instead of simplified
- `--fold [WIDTH]`, `-f [WIDTH]`: Fold long lines when showing specific onset (default width: 50)
- `[ONSET]`: Optional specific onset to filter by (e.g., `j`, `zh`, `none`)

**What is an Onset?**
In Chinese phonology, the onset is the initial consonant or consonant cluster of a syllable:
- `m` in `ma` (妈)
- `zh` in `zhōng` (中)
- `shr` in `shū` (书)
- Empty onset for vowel-initial syllables like `ān` (安)

Example output:
```
y: 447
j: 400
l: 343
x: 303
zh: 291
```

When an onset is specified, the command shows all pinyin within that onset:
```bash
./study-rust-hanzi onset j
```
Output:
```
ji      :  82 机己及几计基即记济极击集级技际纪继急激既积吉迹疾...
jian    :  47 件俭健僭兼减剑剪坚奸尖建拣捡柬检歼涧渐溅煎监睑...
jing    :  35 井京儆兢净境径惊憬敬旌景晶泾痉睛竞竟精經经胫...
```

This command processes all 5000 characters in the dataset and shows the frequency distribution of initial sounds, helping with pronunciation pattern analysis and phonetic studies.

#### Generate Shell Completions

```bash
./study-rust-hanzi generate-completion <shell>
```

Supported shells: `bash`, `zsh`, `fish`, `powershell`, `elvish`

See [COMPLETION.md](COMPLETION.md) for detailed installation instructions.

#### Character Set Options

The `pinyin` commands support character set selection:

- **Default**: Shows simplified Chinese characters
- **Traditional**: Use `--traditional` or `-t` flag to show traditional Chinese characters

```bash
# Simplified characters (default)
./study-rust-hanzi pinyin

# Traditional characters
./study-rust-hanzi pinyin --traditional
./study-rust-hanzi pinyin ma -t
```

#### Input Convenience Features

**V-to-Ü Replacement**: When using the `pinyin` command with `[PINYIN]` option, you can type 'v' instead of 'ü' for easier keyboard input. The tool automatically converts:
- `nv` → `nü` (女)
- `lv` → `lü` (律, 旅, etc.)
- `xv` → `xü` (虚, etc.)

This feature is especially helpful when using keyboards without easy access to the ü character.

### Examples

```bash
# Get top 10 most common pinyin pronunciations
./study-rust-hanzi pinyin | head -10

# Show traditional characters instead of simplified
./study-rust-hanzi pinyin --traditional

# Find all characters pronounced "ma"
./study-rust-hanzi pinyin ma

# Find characters with "ü" sound using "v" replacement
./study-rust-hanzi pinyin nv     # Same as "nü"
./study-rust-hanzi pinyin lv     # Same as "lü"

# Use traditional characters for tone analysis
./study-rust-hanzi pinyin ma --traditional

# Analyze onset distribution of all characters
./study-rust-hanzi onset

# Show all characters with onset 'j'
./study-rust-hanzi onset j

# Show vowel-initial characters (no onset)
./study-rust-hanzi onset none

# Analyze onset distribution using traditional characters
./study-rust-hanzi onset --traditional

# Get the most common onsets
./study-rust-hanzi onset | head -10

# Generate bash completion script
./study-rust-hanzi generate-completion bash > completion.bash

# Use with folding for better readability
./study-rust-hanzi pinyin --fold 30 | less

# Show onset 'j' with folding
./study-rust-hanzi onset j --fold 40
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
