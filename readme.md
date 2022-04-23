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
    update     update the description of a todo via vi
    help       Print this message or the help of the given subcommand(s)
    history
    list
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

# mark a todo as finished
todo fin <id>

# list the last 10 todo histories
todo history -n 10

# list the finished todo between the specified dates
todo history -s 2022-01-01 -e 2022-03-01

# by default the end date is now
todo history -s 2022-03-01
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

# migrations
- `diesel migration generate <name>`
- `diesel migration <subcommand>`
  - run
  - revert
  - redo
