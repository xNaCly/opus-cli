## v1.0.0-rc.1
This release breaks compatibility with [v0.2.0-alpha](https://github.com/xNaCly/opusCli/releases/tag/v0.2.0-alpha) and [v0.1.0-alpha](https://github.com/xNaCly/opusCli/releases/tag/v0.1.0-alpha).

### Breaking Changes:
- the notation for adding a priority has changed from:
  ```bash
  opus add "title ,,,"
  ```
  to: 
  ```bash
  opus add "title .5"
  ```
- reworked `opus export` interface:
  - before
  
    ```bash
    # exports all tasks in the data.json file
    opus export json data
    # export all tasks in the mycsvfile.csv file
    opus export csv mycsvfile
    # data.tsv
    opus export tsv data
    ```
  - after
    ```bash
     # exports all tasks in the data.json file
    opus export --format="json" --output="data.json"
    # export all tasks in the data.csv file
    opus export --format="csv" --output="data.csv"
    # export all tasks in the data.tsv file
    opus export --format="tsv" --output="data.csv"
    ```


### New Features:
- help page for all subcommands and toplevel:
  ```
  Manage tasks with ease
  
  Usage: opus <COMMAND>
  
  Commands:
    add     create a new task [aliases: a]
    delete  delete a task with the given id [aliases: del, d]
    clear   remove all tasks from the database
    finish  mark the task with the given id as finished [aliases: fin, f]
    list    list tasks matching the given query [aliases: ls, l]
    export  export all tasks
    help    Print this message or the help of the given subcommand(s)
  
  Options:
    -h, --help     Print help information
    -V, --version  Print version information
  ```
- list tasks marked as finished using `opus list --finished`
- sort the `opus list` output using `--sort-by` and `--sort-order`, read more [here](https://github.com/xNaCly/opusCli#list-tasks)
- improved displaying tasks:
  - finished tasks are now marked with `FINISHED`
  - priority changed for `[,,,]` to `.3`
  - before:
    ```text
    [2]: 'update excel sheet' (2022-10-13) #work [,,,]
     ```
  - after:
    ```
    [2]: 'update excel sheet' (2022-10-13) #work .3 FINISHED
    ```

### Pull Requests
* test: use sqlite in-memory db when running tests by @daniel0611 in https://github.com/xNaCly/opusCli/pull/15
* Feat/migrate to clap by @xNaCly in https://github.com/xNaCly/opusCli/pull/17
* test: add missing asserts to task.compare_content() calls by @daniel0611 in https://github.com/xNaCly/opusCli/pull/18
* Feat/list sort by @xNaCly in https://github.com/xNaCly/opusCli/pull/19
* fix: get add task content as owned string by @daniel0611 in https://github.com/xNaCly/opusCli/pull/20

### New Contributors
* @daniel0611 made their first contribution in https://github.com/xNaCly/opusCli/pull/15

**Full Changelog**: https://github.com/xNaCly/opusCli/compare/v0.2.0-alpha...v1.0.0-rc.1
