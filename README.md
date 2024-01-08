# Phonetic Spell Check

[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## Overview

Phonetic Spell Check is a tool for performing spell checks with a focus on phonetic similarity. It is implemented in Rust, leveraging the language's performance and safety features. The tool provides the `phonetic-spell-check-rs` library to achieve accurate phonetic comparisons, making it suitable for applications where traditional spell-checking methods may fall short.

## Features
- **Converted from an earlier Java implementation written in 1999** : 
- **Phonetic Comparison:** Leverage advanced phonetic algorithms implemented in Rust to identify similar-sounding words.
- **(TBD) Custom Dictionary:** Use your own dictionary for domain-specific spell checking.
- **(TBD) Command-Line Interface:** Integrate seamlessly into your workflow with a user-friendly command-line interface.

## Getting Started

### Prerequisites

- Uses the CMU Pronouncing Dictionary found here: 

### Installation

```bash
$ cargo install phonetic_spell_rs

```

### Usage

```bash
$ phonetic_spell_rs --input text.txt --dictionary custom_dict.txt
```

### (TBD) Configuration
[Explain any configuration options here, e.g., how to customize phonetic algorithms, specify dictionaries, etc.]

### Contributing
We welcome contributions! Please follow the Contribution Guidelines to get started.

### (TBD) License
This project is licensed under the MIT License - see the LICENSE file for details.

### (TBD) Acknowledgments
[List any acknowledgments or credits for Rust libraries/tools used, inspiration, etc.]