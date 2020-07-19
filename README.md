# JPassword Backend

My capstone project for my Bachelor of Science in Computer Science from Truman State University.
Exposes a REST API to encrypt passwords and retreive passwords using a SQLite database. Written in Rust as an oppurtunity to learn Rust. Passwords are encrypted using AEAD AES 256 GCM. This project was written quickly and many design choices coud have been improved such as user handling, application configuration, and database design. Security is not guaranteed.

## Building and Running

Windows environments require Visual C++ from Visual Studio's Build Tools.

With Rust installed on your system run

`cargo build`

Once compiled, the application be executed via

`cargo run`

## Client

An example client is available [here.](https://github.com/amanojeremie/jpassword)