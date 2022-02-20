# usage
```
todo add <desc>

# add a todo, with an optional link
todo add -l <link> <desc>

# list all the todos with id, sentence and the optional link
todo list

# list todos whose descs contain the pattern
todo list <pattern>

# mark a todo as finished
todo end <id -> int>

# list the last 10 done histories
todo history -n 10
```

## subtask
```
# list the subtasks of a todo
todo -t <id> list

# add a subtask to a todo (by using id)
todo -t <task-id> add [-l link] <string>

# finish a subtask
todo -t <task-id> end <order>
```

# migrations
- `diesel migration generate <name>`
- `diesel migration <subcommand>`
  - run
  - revert
  - redo
