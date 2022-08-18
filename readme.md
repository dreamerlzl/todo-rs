# ov
```
todo

USAGE:
    todo [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -h, --help                 Print help information
    -t, --task-id <TASK_ID>
    -v, --verbose

SUBCOMMANDS:
    add
    del
    fin
    help       Print this message or the help of the given subcommand(s)
    history
    list
    note
    tidy
    update
```

# usage
- configure a env variable `TODO_DB` to specify the path of sqlite
- the sql migrations are embedded so no need to run them manually
```
# add a todo
todo add <desc>

# add a todo, with an optional link
todo add -l <link> <desc>

# list all the todos with id, sentence and the optional link
todo list

# list todos whose descs contain the pattern
todo list <pattern>

# update the desc of a todo with id 2 (use vi)
todo update 2

# mark a todo as finished
todo fin <id>

# directly add a finished todo into history
todo note "something already finished"

# list the last 10 finished todos
todo history -n 10

# list the finished todos within the date range
todo history -s 2022-01-01 -e 2022-03-01

# by default the end date is now
todo history -s 2022-03-01

# by default the start date is 1970-01-01
todo history -e 2022-04-01
```

## subtask
```
# list the subtasks of a todo
todo -t <id> list

# add a subtask to a todo (by using id)
todo -t <task-id> add [-l link] <string>

# finish a subtask
todo -t <task-id> fin <order>
```

# FAQ
- how to sync my todo.db to other devices?
  - check [syncthing](https://syncthing.net/)
- why don't you make an interactive cli?
  - for easier pipelining with other existing shell utilities, like `fzf`, `rg`, etc.

# migrations
- `diesel migration generate <name>`
- `diesel migration <subcommand>`
  - run
  - revert
  - redo
