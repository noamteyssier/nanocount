# nanocount

A CLI to map single-end reads to dual guide protospacers.

## Installation

Quick installation with `cargo`:

```bash
# Requires target-cpu=native for compilation
export RUSTFLAGS="-C target-cpu=native";

# Install via cargo
cargo install nanocount

# Check the CLI documentation
nanocount --help
```
From github:

```bash
# Clone the repository
git clone https://github.com/noamteyssier/nanocount.git
cd nanocount

# Install with cargo
cargo install --path .

# Check the CLI documentation
nanocount --help
```

## Usage

This is expecting a [BINSEQ](https://github.com/arcinstitute/binseq) file with a single sequence per record.

The expected structure is:

```text
[...][protospacer_1][...][protospacer_2][...]
```

`nanocount` will count the number of occurences of each protospacer and the number of occurences of each pair of protospacers.

### Protospacer input

The protospacer input should be a TSV (no header) with four columns:

```text
1. Construct Name
2. Alias
3. Protospacer1 Sequence
4. Protospacer2 Sequence
```

> Note: The sequences **will be converted to uppercase** before counting and will appear in uppercase in the output.


### Output 

The output will be a TSV (with header) with the following columns:

```text
1. construct: Full construct name
2. alias: Alias of construct
3. g1: Protospacer1 Sequence
4. g2: Protospacer2 Sequence
5. count_g1: Number of sequences with Protospacer1
6. count_g2: Number of sequences with Protospacer2
7. count_paired: Count of sequences with both Protospacers
8. count_unpaired: Count of sequences with only one of the pairs
```
