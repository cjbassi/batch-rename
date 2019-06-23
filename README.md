# batch-rename

Quickly rename a batch of files in your default text editor.

Takes in a list of filenames on the command line, which are copied into the default text editor, and renames the files based on edits done to the filenames.

## Example

Running the command:

```bash
batch-rename foo bar baz
```

opens a text editor with:

```
foo
bar
baz
```

and changing any of the filenames causes those files to be renamed when the editor is closed.

## Installation

### Prebuilt binaries:

Note: (currently only a binary for Linux-x86_64 is available)

Run the following to download the correct binary for your system from the releases tab into `$CARGO_HOME/bin`, courtesy of [japaric/trust](https://github.com/japaric/trust):

```bash
bash <(curl -LSfs https://japaric.github.io/trust/install.sh) \
  -f --git cjbassi/batch-rename
```

### From source:

```bash
cargo install --git https://github.com/cjbassi/batch-rename
```
