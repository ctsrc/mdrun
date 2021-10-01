# mdrun

[![Crates.io](https://img.shields.io/crates/v/mdrun.svg)](https://crates.io/crates/mdrun) [![Docs.rs](https://docs.rs/mdrun/badge.svg)](https://docs.rs/mdrun/)

Markdown notebook runner

## Work in progress

This tool is not yet ready for use, as at present time
no code for it has yet been written.

## Table of contents

Table of contents will be added as soon as `mdrun` is able to generate it.

## Introduction

`mdrun` is a tool that reads Markdown and CommonMark documents and:

  - Runs shell commands from one or more specified sections of your document.
  - Captures the output of said shell commands.
  - Inserts the output of the shell commands into the document.
  - Updates the output of the shell commands in the document.

Additionally, it performs the following tasks:

  - Generates a table of contents for your document.
  - Updates the table of contents in your document.

## Dogfooding

We wrote this tool to scratch an itch, and we use it ourselves.
Both in other projects, and on the very README you are presently reading.

(Also, when I say "we" I actually mean "I", because there is only me working
on this as of yet. Contributions are welcome though, as long as they stick to
the essence of the project. So perhaps one day it really will be "we".)

## Use cases

The foremost usecase for `mdrun` lies in keeping documentation up to date,
by capturing and updating the output of shell commands that your docmuents
contain.

Aside from this, one of many other possible use cases include using `mdrun`
with a Markdown or CommonMark document in the style of what IPython provides
when you choose to "Run all cells" on an IPython notebook. Albeit more simple
and less interactive compared to IPython of course. Simpler and less
interactive for the time being, at least.

## Usage

Since one might not want to execute all commands in any given document,
the tool requires that you specify which section(s) of the document
to run shell commands from. By default, no commands will be executed
unless sections are specified, and furthermore, only the commands
that belong to the section itself, and not any commands inside of
any subsections will be executed.

Additionally, for safety and convenience, `mdrun` will only execute
fenced commands that satisfy the following conditions:

  1. The code block containing the command is fenced and has an info string
     indicating which shell the command is to be run in.
  2. The aforementioned info string has a value corresponding to either
     of the following: `zsh`, `fish`, `bash`, `sh`, `ksh`, `tcsh` or `csh`.
  3. The code block containing the command is immediatelly followed
     by a code block that has an info string with a value of `text`.

As such, assuming `mdrun` has been installed to a directory that is
in your `$PATH`, and you were to run the following command in this
directory, specifying that `mdrun` run on the section titled "Usage"
of this README:

```zsh
mdrun -s Usage README.md
```

And further assuming that you have `zsh` installed on your system,
then you will find that the following commands will execute and
the output will be updated correspondingly.

```zsh
date +%s
```

```text

```

```zsh
uname -a
```

```text

```

Whereas the following command will not be executed, because there is no
corresponding fenced code block with info string `text` immediatelly
following it.

```zsh
echo This command is not executed.
```

Likewise, the following command will not be executed either, because
the info string for the command code block has a value that is different
from the set of accepted values for the info string of a command code block:

```python3
print("Hello from Python 3.")
```

```text

```

Although... Python may be considered to be in scope. So in that case we'll
have to come up with another example instead.

This one may be a safer bet for something that will probably remain
considered out of scope:

```sql
select current_date;
```

```text

```

Since in order for that command to really make sense, we'd at the very least
need to know which sql client to use (PostgreSQL here), and usually we'd need
to know which database to run the sql command on etc.

Either way, in both the case of Python 3 and any command-line capable sql client,
if you wanted to run commands and capture output you could specify a shell command
that invokes the tool in question.

So for example Python 3:

```zsh
python3 -c "print('Hello from Python 3.')"
```

```text

```

and a multi-line Python 3 example:

```zsh
python3 <<EOF
for i in range(2):
  print(i)
EOF
```

```text

```

and SQLite:

```zsh
sqlite3 <<EOF
select date('now');
EOF
```

```text

```

etc, etc.

That being said, and given that we mentioned PostgreSQL I might also note that
you'd probably usually *not* want to connect to a persistent database in the
context of sections of documents that you run `mdrun`. But even that will
sometimes be appropriate. It all depends on what you are trying to do with
`mdrun` and your Markdown or CommonMark documents.

### A couple of other examples

Likewise to the above, when you run:

```zsh
mdrun -s Usage README.md
```

you will find that the following are *NOT* updated, because they belong
to a subsection of the "Usage" section rather than belonging directly
to the "Usage" section itself:

```zsh

```

### We can do it recursively too, if you'd like (as long as you're careful)

TODO: Describe

### Handling of multiple sections which share the same name

Below we have two sections which are both named "What happens if you try to
run `mdrun` on a section that does not have a unique name?"

We then attempt to run `mdrun` specifying that we want it to run on a section
with this name.

```zsh
mdrun -s "What happens if you try to run `mdrun` on a section that does not have a unique name?" README.md
```

In this case, `mdrun` will report that the section name is ambiguous because
multiple sections share the same name. Therefore, the commands will not be executed,
the output will remain unmodified, and `mdrun` will exit with a non-zero exit code.

#### What happens if you try to run `mdrun` on a section that does not have a unique name?

This is an example of a section that does not have a unique name.
The section that follows this one shares the same name.

```zsh
echo hello
```

```text

```

#### What happens if you try to run `mdrun` on a section that does not have a unique name?

This is an example of a section that does not have a unique name.
The section that precedes this one shares the same name.

```zsh
echo world
```

```text

```

### A config-file for use in projects?

I am debating making it so that `mdrun` will look for a file named `.mdrunconf`
or some-such in the current working directory, but I am leaning towards leaving
it up to each project to create a short shell script that invokes `mdrun` with
the sections it should run instead of doing that.

### Handling of multiple sections which share the same name (cont.)

The above applies also even when the sections in questions are not direct
siblings. `mdrun` was made to behave this way for safety, to ensure that one
does not accidentally run the commands belonging to the wrong section.

For this same reason, `mdrun` does not provide numeric indexing as a way of
disambiguating, because additional sections sharing the same name could have
been inserted into the document between.

(Of course, sections could also have been renamed, but this is less likely
to happen by accident/without noticing in our opinion, so we take it as a
prerequisite for use of the tool to assume that the specificed section names
correspond to what the user intended as long as we deny running commands
on specified sections where the section name is non-unique.)

The primary and recommended way of dealing with multiple sections that
share the same name is to give unique names to each section in your document.

The secondary and non-recommended way of dealing with multiple sections
that share the same name is to run `mdrun` recursively on a common ancestor,
or on the nearest non-ambigous ancestor of each of the sections in question.

## Installation

### Option 1: Via the package manager for your operating system

This option is not yet available, but we hope to see this tool available
in the following package managers eventually:

- Homebrew for macOS
- Official apt repositories for Ubuntu, KDE Neon and Debian
- Arch Linux AUR
- FreeBSD ports
- and more...

### Option 2: Download precompiled binaries via the GitHub releases page

Not yet available but will be soonish.

### Option 3: Via crates.io using the Rust development environment tool `cargo`

1. Install the Rust development environment, if you haven't done so already:

   ```zsh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install mdrun:

   ```zsh
   cargo install mdrun
   ```

## Acknowledgements

This tool would not be possible without the awesome CommonMark Markdown parser
[comrak](https://github.com/kivikakk/comrak).

## Contributing

If you have a feature that you'd like to implement, or a bug that you would
like to fix, please file an issue so that we can discuss it and if what you
have in mind is in line with the goals and scope of this project then I will
give you the go-ahead signal and you can then code it up and submit a
pull-request that we can discuss further.

## ⭐ Star, share, like, subscribe ;) ⭐

If you enjoy this project please remember to star it on GitHub.

And feel free to tell some friends, colleagues or your family about it too :)