### Hello Flappy in Mozilla's Rust language

First game I've coded in rust.
This is trivial Flappy Bird game written in Mozilla's Rust language.
I'm following the steps of Hands-on Rust (English Edition) book.


## Setup

Tested on `Ubuntu 18.04.2 LTS` (formerly Ubuntu 16.04.1 LTS)

Install these packages on Ubuntu:

```bash
sudo apt-get install git rustc cargo
```
Clone this project using:

```bash
mkdir ~/projects
cd ~/projects
git clone https://github.com/pepoon/HelloFlappy.git
cd HelloFlappy
```

Updating from Ubuntu 16 to Ubuntu 18:
Issue this command to update to recent dependencies:
```bash
cargo update
```

## Build

Invoke for this project directory:

```bash
cargo build
```

## Run

Invoke this command from project directory:

```bash
cargo run
```

