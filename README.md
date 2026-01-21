I'm maintaining this fork for myself. Feel free to back-merge or open PRs and issues.

# bin
A paste bin with an SQLite database.

A paste bin that's actually minimalist. No database requirement (because we use SQLite), no commenting functionality, no self-destructing or time bomb messages and no social media integration—just an application to quickly send snippets of text to people.

The only "nice to have" is end-to-end encryption, optionally! For this option, JS on the client-side is required, but the site continues working without JS for all other features.

Allows you to Ctrl+A without selecting anything but the paste.

##### How to build

Compile it from source using `cargo build --release`. Get cargo at https://rustup.rs.

##### How to run

```bash
$ ./bin
```

##### Configuration

```
$ ./bin

Usage: bin [<bind_addr>] [--buffer-size <buffer-size>] [--max-paste-size <max-paste-size>]

a pastebin.

Positional Arguments:
  bind_addr         socket address to bind to (default: 127.0.0.1:8820)

Options:
  --max-paste-size  maximum paste size in bytes (default. 32kB)
  --title           title of the website (default: bin.)
  --db-path         path to the database file (default: ./pastes.db)
  --help, help      display usage information
```

##### is there curl support?

```bash
$ curl -X PUT --data 'hello world' https://<your-domain>
https://<your-domain>/cateettary
$ curl https://<your-domain>/cateettary
hello world
```

##### How does syntax highlighting work?

To get syntax highlighting you need to add the file extension at the end of your paste URL.
