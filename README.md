# WEBFUSE

A tool to fuse your html file with its dependencies. This will **not** bundle or minify your code. This tools only bundles index.html and its direct dependencies into one file.

## Usage

```
webfuse index.html --to findex.html
```

this command will generate `findex.html` with `script` src and `link` style tags replaced with respective file contents.

`webfuse --help` output:

```
USAGE:
    webfuse.exe [OPTIONS] <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --to <TO>    Name of generated file (is fused_index.html) by default

ARGS:
    <FILE>    Html file with dependencies
```

## Why

This tool was made because [`webview`](https://github.com/Boscop/web-view) does not support multiple files as sources.
