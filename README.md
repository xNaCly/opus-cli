# opusCli

cli todo tool - add, edit, tag and delete tasks

## Showcase

## Installation

## Syntax

### Add a task

```
opus add "update excelsheet @work !!!"
          ^^^^^^^^^^^^^^^^^ ^^^^^ ^^^
          |                 |     |
          |                 |     |_ priority = amount of '!'
          |                 |
          |                 |_ tag prefixed with '@'
          |
          |_ title of the task
```

### List tasks

```bash
opus list
opus list @work   # List all tasks with tag work (@work):
opus list !!      # List all tasks with priority !! (2):
opus list .2      # List task with id 2
```

## Contributing
