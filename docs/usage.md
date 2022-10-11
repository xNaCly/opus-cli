# Usage

## Add a task

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

## List tasks

-   opus hides finished tasks from the `opus ls` command

```bash
# list all tasks
opus list
# list all task with the tag #work
opus list "#work"
# list all tasks with the priority ,,,
opus list ",,,"
# list the task with the id 1
opus list 1
# same as above
opus l 1
```

## Mark a task as finished

-   opus hides finished tasks from the `opus ls` command
-   contrary to `opus ls`, `opus fin` does only accept a tasks id as the argument

```bash
# mark the task with id=1 as finished
opus finish 1
# same as above
opus f 1
```

## Remove all tasks

```bash
# this clears the whole database
opus clear
```

## Export Tasks

-   opus exports all tasks to a specified file

```bash
# exports all tasks in the data.json file
opus export json data
# export all tasks in the mycsvfile.csv file
opus export csv mycsvfile
# data.tsv
opus export tsv data
```

## Delete a task

-   just like `opus finish`, `opus delete` only accepts a tasks id as the argument

```bash
# delete the task with id=1
opus delete 1
opus d 1
```
