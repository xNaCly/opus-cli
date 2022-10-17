# opusCli

_simple to-do manager for power users_

-   [Roadmap](https://github.com/xNaCly/opusCli/milestone/2)

## Installation

### Binary

`opus` can be installed via the provided binary files in the release section,
[here](https://github.com/xNaCly/opusCli/releases).

Select the latest release, add the `opus` executable to a directory which is registered in the path variable and you're
good to go.

### From source

> Make sure [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) and
> [rustup](https://www.rust-lang.org/tools/install) are installed

```bash
git clone https://github.com/xnacly/opusCli
cargo build --release
./target/release/opus
```

Cargo will now build `opus` for the target you're currently using as a operating system. The resulting executable can be
found in the `target` directory: `target/release/opus`

## Usage

### Add a task

-   title is the only required value
-   opus replaces `@tomorrow` and `@today` with the corresponding dates in `YYYY-mm-DD` notation

```bash
# add a new task with the following properties:
# title: review and merge pr 5
# due: @tomorrow
# tag: #github
# priority: 3
opus add "review and merge pr 5 @tomorrow #github ,,,"
# add a new task with the given title
opus a "review and merge pr 5"
```

### List tasks

-   this command hides finished tasks from the `opus ls` command

```bash
# list all tasks
opus list
# list all task + finished tasks
opus list --finished
# list all task with the tag #work
opus list "#work"
# list all tasks with the priority ,,,
opus list ",,,"
# list the task with the id 1
opus list 1
# same as above
opus l 1
```

### Mark a task as finished

-   opus hides finished tasks from the `opus ls` command
-   contrary to `opus ls`, `opus fin` does only accept a tasks id as the argument

```bash
# mark the task with id=1 as finished
opus finish 1
# same as above
opus f 1
```

### Remove all tasks

```bash
# this clears the whole database
opus clear
```

### Export Tasks

-   opus exports all tasks to a specified file

```bash
# exports all tasks in the data.json file
opus export json data
# export all tasks in the mycsvfile.csv file
opus export csv mycsvfile
# data.tsv
opus export tsv data
```

### Delete a task

-   just like `opus finish`, `opus delete` only accepts a tasks id as the argument

```bash
# delete the task with id=1
opus delete 1
opus d 1
```

## Configuration

### The `OPUS_PATH` env variable

By default `opus` decides where to store its database based on your operating system. Currently opus supports
automatically figuring out where to store the database on windows, linux and macos.

-   Linux and Macos: `$HOME/opus/opus.db` or `$XDG_CONFIG_HOME/opus/opus.db`
-   Windows: `%LOCALAPPDATA%/opus/opus.db`

For use cases which force the use of a different location, opus honors the `OPUS_PATH` environment variable. Set this
variable and override the above paths.:

-   Linux: _add to .bashrc (or your shells config file)_

```
set OPUS_PATH=~/.config # opus will create and use ~/.config/opus/opus.db
```

-   Windows: Add to your system environment variables
    _[guide](https://geekflare.com/system-environment-variables-in-windows/)_.

> Opus will create the `opus/opus.db` file and directory in the directory specified in the `OPUS_PATH` variable
