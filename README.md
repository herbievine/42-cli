<pre>
        :::     ::::::::             ::::::::  :::        :::::::::::
      :+:     :+:    :+:           :+:    :+: :+:            :+:
    +:+ +:+        +:+            +:+        +:+            +:+
  +#+  +:+      +#+              +#+        +#+            +#+
+#+#+#+#+#+  +#+                +#+        +#+            +#+
     #+#   #+#                 #+#    #+# #+#            #+#
    ###  ##########            ########  ########## ###########
</pre>

# Introduction

This is a simple CLI tool to manage your 42 projects, and make sure you don't push an empty project or forget to norminette your code...

We've all been there...

# Installation

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [Git](https://git-scm.com/downloads)
- [Norminette](https://github.com/42School/norminette)

## Install

### From GitHub

```bash
cargo install --git https://github.com/herbievine/42-cli.git
alias 42-cli="~/.cargo/bin/fourtytwo-cli" # optional, but recommended
```

# Usage

## Commands

### `help`

Displays the help menu.

```bash
fourtytwo-cli help
```

### `push`

Pushes your project to your 42 intranet (or to any git repository).

```bash
fourtytwo-cli push [options] <project_directory> <git_repository>
```

#### Options

- `-h, --help`: Print help.
- `-i, --include <pattern>`: Includes only the files matching the pattern.
- `-n, --no-norm`: Disables the norminette check.

#### Example

```bash
fourtytwo-cli push ~/dev/42/libft git@vogsphere.42paris.fr:vogsphere/intra-xxx -i "(c|h|e)$"
```

We specify the project directory, the git repository, and we include only the files ending with either a `c`, `h` or `e`. In other words, any C file, header file or Makefile.

### `update`

Clone your existing project from your 42 intranet, and override it with your local project.

```bash
fourtytwo-cli update [options] <project_directory> <git_repository>
```

#### Options

- `-h, --help`: Print help.
- `-i, --include <pattern>`: Includes only the files matching the pattern.
- `-n, --no-norm`: Disables the norminette check.

#### Example

```bash
fourtytwo-cli update ~/dev/42/libft git@vogsphere.42paris.fr:vogsphere/intra-xxx -i "h$"
```

We specify the project directory, the git repository, and here, we update only the header files.

# Support/Contributing

If you've found a bug or have a feature request, please open an issue. If you'd like to contribute, please open a pull request.
