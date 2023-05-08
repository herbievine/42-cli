<pre>
        :::     ::::::::             ::::::::  :::        :::::::::::
      :+:     :+:    :+:           :+:    :+: :+:            :+:
    +:+ +:+        +:+            +:+        +:+            +:+
  +#+  +:+      +#+              +#+        +#+            +#+
+#+#+#+#+#+  +#+                +#+        +#+            +#+
     #+#   #+#                 #+#    #+# #+#            #+#
    ###  ##########            ########  ########## ###########
</pre>

This is a simple CLI tool to test your 42 projects.

# Installation

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)

## Install

### From GitHub

```bash
cargo install --git https://github.com/herbievine/42-cli.git

# Add the following to your .bashrc or .zshrc to persist the alias
alias 42-cli="~/.cargo/bin/fourtytwo-cli"
```

# Usage

## Config file

You will need to create a `42-cli.toml` file in each of your projects. This file will contain the following information:

```toml
# The name of the project
name = "pipex"

[scripts]
# The install script (optional)
install = [{ cmd = "make" }]

# The test script (optional)
test = [
  { cmd = "git clone xxx tester" },
  { cmd = "make m", dir = "tester" },
  { cmd = "rm -rf tester" }
]

# The cleanup script (optional)
clean = [{ cmd = "make fclean" }]
```

Each script is defined by a command a directory (optional). The command is the command to run, and the directory is the directory in which the command will be run. If no directory is specified, the command will be run in the project directory.


## Commands

### `help`

Displays the help menu.

```bash
fourtytwo-cli help
```

### `run`

Run your project with the defined commands in the `42-cli.toml` file.

```bash
fourtytwo-cli run
```

#### Options

- `-h, --help`: Print help.
- `-c, --clean`: Run the clean script defined in the `42-cli.toml` file.

### `test`

Run the test script defined in the `42-cli.toml` file.

```bash
fourtytwo-cli test
```

#### Options

- `-h, --help`: Print help.

# Support/Contributing

If you've found a bug or have a feature request, please open an issue. If you'd like to contribute, please open a pull request.
