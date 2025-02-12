# CrashDB
A database management system I'm making for school.
It is a nosql database which has a focus on nested key-value tables.

# Building
Make sure to [install rust](https://www.rust-lang.org/tools/install).

From there, CrashDB can be built with the standard command from cargo.
```
$ cd nosql
$ cargo build --release
```

You can also build (or run) each binary individually.
```
$ cargo build --bin server --release
$ cargo build --bin cli --release
```

# Running
To run CrashDB from a [release](https://github.com/EIG520/CrashDB/releases/) build, first run the server.
```
$ ./server-linux
NOSQL SERVER
```
You can also specify a network port and a file to dump data to.
```
$ ./server-linux port 8080 file data.txt
```
There should be only one server running per database.

To communicate with the server and run commands, spawn any number of instances of the cli (set up to the ip and port of the server).
```
$ ./cli-linux ip 127.0.0.1 port 8080
NOSQL CLIENT
```

# Playing with CrashDB

Run the server
```
$ ./server-linux
NOSQL SERVER
```
Run the client
```
$ ./cli-clinux
NOSQL CLIENT
CrashDB> set foo bar
done (435.01µs)
CrashDB> get foo
bar (537.992µs)
CrashDB> 
```
Make a table
```
CrashDB> touch mytable table
done (351.141µs)
CrashDB> open mytable
CrashDB/mytable> set foo notbar
done (562.085µs)
CrashDB/mytable> get foo
notbar (414.081µs)
```
Close a table
```
CrashDB/mytable> close mytable
CrashDB> get foo
bar (596.225µs)
```


