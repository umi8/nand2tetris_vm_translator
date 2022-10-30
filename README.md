# Nand2Tetris VM Translator

## Overview

VM Translator for Nand2Tetris

## Usage

```shell
Nand2Tetris VM Translator

Usage: nand2tetris_vm_translator --path <PATH>

Options:
  -p, --path <PATH>  Sets a path to be translated
  -h, --help         Print help information
  -V, --version      Print version information
```

## Example

### If the target is a directory

```shell
$ nand2tetris_vm_translator --path path/to/dir
# File translation succeeded: path/to/dir/dir.asm
```

### If the target is a vm file

```shell
$ nand2tetris_vm_translator --path path/to/file.vm
# File translation succeeded: path/to/file.asm
```

## Reference

- https://www.nand2tetris.org/project07
- https://www.nand2tetris.org/project08
