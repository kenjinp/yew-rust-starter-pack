# Rust / Yew Website

This is a simple rust wasm starterkit, [using this example from yew](https://github.com/DenisKolodin/yew/tree/master/examples/counter), as a baseline to get myself started using rust.

It relies on cargo-web for all the things.

## Install

To start off, you should clone this repo.

Next, you'll need rustup. This installs rust, and versions of rust. You can probably install rustup from `homebrew`, or your preferred system package manager. Otherwise you can do this:
`curl https://sh.rustup.rs -sSf | sh`

make sure you install the correct rust toolchain following these exact steps
[from hellorust.com](https://www.hellorust.com/setup/wasm-target/)

we will be targeting the native rust web assembly compiler called, strangely, `wasm32-unknown-unknown`, which is the recommended way to do this (I think). Because of that we need rustup to install the nightly build of the language.

## Develop
run `cargo web start --auto-reload`

## Build
run `cargo web deploy`, which will produce the js and other artifacts in the folder `/target/deploy`