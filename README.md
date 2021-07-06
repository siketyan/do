# do
[![Rust](https://github.com/siketyan/do/actions/workflows/rust.yml/badge.svg)](https://github.com/siketyan/do/actions/workflows/rust.yml)

Alternative of make, without making anything.

## 🔧 Prerequisites
- Rust Toolchain v1.52+

## 📦 Installation
```
$ cargo install do-app
```

Then just run:

```
$ do [target]
```

## 💚 Example
Currently only supports `.do.yaml` as filename.
```yaml
tasks:
  test:
    - type: run
      command: cmd
      arguments:
        - "/c"
        - "echo Hello from do command!"
```

## 📄 Licence
This program is licenced under the MIT Licence.
For details, refer the licence description below.

https://s6n.mit-license.org/
