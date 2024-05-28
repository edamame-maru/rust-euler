# rust-euler
Welcome to rust-euler!

![GitHub commit activity](https://img.shields.io/github/commit-activity/t/edamame-maru/rust-euler)
![GitHub last commit](https://img.shields.io/github/last-commit/edamame-maru/rust-euler)
![GitHub Repo stars](https://img.shields.io/github/stars/edamame-maru/rust-euler)

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Vim](https://img.shields.io/badge/VIM-%2311AB00.svg?style=for-the-badge&logo=vim&logoColor=white)
![Manjaro](https://img.shields.io/badge/Manjaro-35BF5C?style=for-the-badge&logo=Manjaro&logoColor=white)

## Project layout
<pre>.
├── <font color="#12488B"><b>hello-world</b></font>
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── <font color="#12488B"><b>src</b></font>
│   │   └── main.rs
│   └── <font color="#12488B"><b>target</b></font>
│       ├── CACHEDIR.TAG
│       └── <font color="#12488B"><b>debug</b></font>
│           ├── <font color="#12488B"><b>build</b></font>
│           ├── <font color="#12488B"><b>deps</b></font>
│           │   ├── <font color="#26A269"><b>hello_world-99ed12927bc23265</b></font>
│           │   └── hello_world-99ed12927bc23265.d
│           ├── <font color="#12488B"><b>examples</b></font>
│           ├── <font color="#26A269"><b>hello-world</b></font>
│           ├── hello-world.d
│           └── <font color="#12488B"><b>incremental</b></font>
│               └── <font color="#12488B"><b>hello_world-356t5tm55t73o</b></font>
│                   ├── <font color="#12488B"><b>s-gwdpsicqq0-7i3zl2-224kkz2zegjy1l2z5c6uqy6yz</b></font>
│                   │   ├── 1f68qjuhijjmc5i7.o
│                   │   ├── 3sbtbb5h8ru55pik.o
│                   │   ├── 3wqjr936igavuvx1.o
│                   │   ├── 3xa1dh1eeps7pkl3.o
│                   │   ├── 4rwjz7g0puotc3i.o
│                   │   ├── 541blf7fzejf93ta.o
│                   │   ├── dep-graph.bin
│                   │   ├── query-cache.bin
│                   │   └── work-products.bin
│                   └── s-gwdpsicqq0-7i3zl2.lock
└── README.md
</pre>



## Execution/Testing

1. Download the zip file or clone this repository.
2. The source code is in the ```.rs``` file, usually with the path ```<projectName>/src/main.rs```. The pre-built executable included in ```<projectName>/target/debug/<executable>```. On linux, ```cd``` to the directory with the executable and run the code with:

```
$ ./<executable name>
```
For example, in the project ```hello-world```, ```main.rs``` compiled with ```rustc``` (managed by cargo) will output a file called ```hello-world``` which is the executable. These are already included in the repository. You would then:

<pre>
```
$ cd ./path/to/rust-euler/euler1/
$ ls
Cargo.lock Cargo.toml src <b>target</b>
$ cd ./target/
$ ls
CACHEDIR.TAG <b>debug</b>
$ cd ./debug/
$ ls
build deps examples <b>hello-world</b> hello-world.d incremental
$ ./hello-world
```
</pre>

to run the code. Alternatively, if you know how to, ```cargo run``` should do the job if you clone and setup the repository correctly.
Note the space is included at the start of the command.

