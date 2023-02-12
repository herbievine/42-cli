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

- NodeJS
- NPM (or Yarn/PNPM)

## Install

```bash
git clone https://github.com/herbievine/42-cli.git
cd 42-cli
npm install # or `yarn` | `pnpm i`
```

# Usage

## Commands

### `help`

Displays the help menu.

```bash
npm run cli help
```

### `push`

Pushes your project to your 42 intranet (or to any git repository).

```bash
npm run cli push [options] <project_directory> <git_repository>
```

#### Options

- `-h, --help`: Displays the help menu.
- `-i, --include <pattern>`: Includes only the files matching the pattern.
- `-n, --norm`: run norminette on your project.

#### Example

```bash
npm run cli push ~/dev/42/libft git@vogsphere.42paris.fr:vogsphere/intra-xxx -i "(c|h|e)$" -n
```

We specify the project directory, the git repository, and we include only the files ending with either a `c`, `h` or `e`. In other words, any C file, header file or Makefile. We also run norminette on the project.

# Support/Contributing

If you've found a bug or have a feature request, please open an issue. If you'd like to contribute, please open a pull request.
