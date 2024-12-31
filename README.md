# 'A Tier' – Tier List app in Rust

Simple Terminal-CLI app for making tier lists

## Functions:
- print visualization of tier list in CLI interface with colored tier names
- set custom tiers or pass empty string in tier prompt to use default S,A,B,C,D tiers
- interactively enter tier items in format TIER NAME=tier item
- or read pre-made structure from a JSON file


Functions to be added:
- setting custom height-width of tier blocks
- multi-line tiers like in tiermaker (now only supports single-line tiers, limiting the number of items per tier by terminal width)
- setting custom colors

## Usage

### Interactive CLI mode

run the binary / cargo

```sh
./tier_list
```

```sh
cargo run
```

and follow the instructions to define tiers and items one-by-one

### Load from JSON

save tiers and items in a JSON in format

```json
{
  "data": {
    "S 10/10": [
        "Item 1",
        "Item 2",
        "Item 3"
    ],
    "A": [
        "Item 1"
    ],
    "some other tier": [
        "Item 1"
    ]
  }
}
```

run the app
```sh
./tier_list --json /path/to/.json
```

```sh
cargo run --  --json /path/to/.json
```

> app will auto-order the JSON file tiers according to the first letter of tier name, checking it against S-A-B-C-D naming convention (S then A-Z), other names will be put last by default

