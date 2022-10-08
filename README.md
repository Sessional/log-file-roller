# log-file-roller

Just pipe your app to this and log files will role!

```
run-your-app.exe | log-file-roller -n 5 -e json -o output -s 8MiB
```

```
log-file-roller 0.1
Handles rolling of log files with by piping the stdout of a process to it

USAGE:
    log-file-roller [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --file-count <COUNT>            The number of files to maintain when rolling files. The current file will have
                                        no number and older ones will have increasing numbers
    -e, --file-extension <EXTENSION>    Sets the extension for each log file. File will be named (<output-
                                        file>.#.<extension>)
    -s, --file-size <SIZE>              Used to specify what size file will trigger a roll to a new file. This is not
                                        the cap. (Default: 2MiB)
    -o, --output-file <NAME>            Sets the name prefix of the output file. File will be named (<output-file>.#)
```