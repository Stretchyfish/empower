
![empower_logo](media/empower_logo.png)

# Introduction

Empower is a general purpose, node graph based, game-engine inspired visual programming system.

![empower_example](media/empower_usecase_example.png)

# Getting started

## Prerequisite
Make sure to have the rust toolchain running on your system

https://rust-lang.org/tools/install/

### Linux
#### Ubuntu

Make sure the system has the following packages installed.

~~~
sudo apt-get install -y libclang-dev libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev
~~~

## Building

The project is build using rust cargo.

Clone repository in your system
~~~
git clone <git_url> && cd empower
~~~

Build using cargo
~~~
cargo build
~~~

Open empower studio (the visual editor) with
~~~
cargo run -p empower-studio
~~~
