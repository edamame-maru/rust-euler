# rust-euler
Welcome to rust-euler!

![GitHub commit activity](https://img.shields.io/github/commit-activity/t/edamame-maru/rust-euler)
![GitHub last commit](https://img.shields.io/github/last-commit/edamame-maru/rust-euler)

## Execution/Testing

1. Download the zip file or clone this repository.
2. The source code is in the ```.rs``` file, usually with the path ```<projectName>/src/main.rs```. The pre-built executable included in ```<projectName>/target/debug/<executable>```. On linux, ```cd``` to the directory with the executable and run the code with:

```
$ ./<executable name>
```
For example, in the project ```hello-world```, ```main.rs``` compiled with ```rustc``` (managed by cargo) will output a file called ```hello-world``` which is the executable. These are already included in the repository. You would then:

```
$ cd ./path/to/rust-euler/euler1/
$ ls
Cargo.lock Cargo.toml src *target*
$ cd ./target/
$ ls
CACHEDIR.TAG *debug*
$ cd ./debug/
$ ls
build deps examples *hello-world* hello-world.d incremental
$ ./hello-world
```
to run the code. Alternatively, if you know how to, ```cargo run``` should do the job if you clone and setup the repository correctly.
Note the space is included at the start of the command.

## Roadmap
