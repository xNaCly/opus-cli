[![cargo-test](https://github.com/xNaCly/opusCli/actions/workflows/ci.yml/badge.svg)](https://github.com/xNaCly/opusCli/actions/workflows/ci.yml)
# opusCli

cli todo tool - add, edit, tag and delete tasks

## Showcase

## Installation

## Syntax

### Add a task

```
opus add "update excelsheet #work @tomorrow ,,,"
          ^^^^^^^^^^^^^^^^^ ^^^^^ ^^^^^^^^^ ^^^
          |                 |     |         |
          |                 |     |         |_ priority
          |                 |     |
          |                 |     |_ due date
          |                 |
          |                 |_ tag
          |_ title
```

-   title is the only required value
-   tags are prefixed with a `#` and should contain `_` instead of spaces.
-   due date is prefixed with a `@` and can either be `@today`, `@tomorrow` or a date (`yyyy-mm-dd`)
-   priority is specified using `,` - highest priority is one `,`

### List tasks

```bash
opus list
opus list "#work"   # List all tasks with tag work (#work):
opus list ",,"      # List all tasks with priority !! (2):
opus list .2        # List task with id 2
```

## Contributing
