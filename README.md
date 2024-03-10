# My start with Rust development
<img src="https://rustacean.net/assets/rustacean-orig-noshadow.png" width="200" />

<p>
This repository contains my first mini-projects developed in Rust with the aim of
practicing and learning more about the language and its tools.
</p>

## 1. hello-rust - 08/03/2024
<p>
My first ever rust project created with cargo. Just a "hello world" with the "ferris-says" message.
</p>

[dependencies]
ferris-says = "0.3.1"

### Reference(s):
<a href="https://www.rust-lang.org/pt-BR/learn/get-started">rust-lang.org - Get started</a>

## 2. csvtutor - 08/03/2024
<p>
A simple playground to read/write CSV files with <b>csv</b> package.
</p>

[dependencies]
csv = "1.1"
encoding_rs = "0.8.13"
encoding_rs_io = "0.1.3"

### Reference(s):
<a href="https://docs.rs/csv/latest/csv/index.html">docs.rs - csv package</a>
<a href="https://docs.rs/csv/latest/csv/tutorial/index.html#setup">docs.rs - csv tutorial setup</a>
<a href="https://stackoverflow.com/questions/53826986/how-to-read-a-non-utf8-encoded-csv-file">Reading non-UTF-8 encoded CSV</a>