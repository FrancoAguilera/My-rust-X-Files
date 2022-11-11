# The Rust Files

<p>
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=orange" />
</p>
<p align="center">
  <img src="http://rust-lang.org/logos/rust-logo-512x512.png" width="300"/>
</p>

### Let's get Rusty
this repository is made with the idea of being an entry point to the Rust language

## Install 
From the [docs](https://doc.rust-lang.org/stable/book/ch01-01-installation.html).

### Installing rustup on Linux or macOS

If you’re using Linux or macOS, open a terminal and enter the following command:
```bash
$ curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```

The command downloads a script and starts the installation of the rustup tool, which installs the latest stable version of Rust. You might be prompted for your password. If the install is successful, the following line will appear:

```bash
Rust is installed now. Great!
```

You will also need a linker, which is a program that Rust uses to join its compiled outputs into one file. It is likely you already have one. If you get linker errors, you should install a C compiler, which will typically include a linker. A C compiler is also useful because some common Rust packages depend on C code and will need a C compiler.

On macOS, you can get a C compiler by running:

```bash
$ xcode-select --install
```

Linux users should generally install GCC or Clang, according to their distribution’s documentation. For example, if you use Ubuntu, you can install the ```build-essential``` package.

### Installing rustup on Windows

On Windows, go to https://www.rust-lang.org/tools/install and follow the instructions for installing Rust. At some point in the installation, you’ll receive a message explaining that you’ll also need the MSVC build tools for Visual Studio 2013 or later. To acquire the build tools, you’ll need to install Visual Studio 2022. When asked which workloads to install, include:

  * “Desktop Development with C++”
  * The Windows 10 or 11 SDK
  * The English language pack component, along with any other language pack of your * choosing

### Rust Cheatsheet: 
[_link](https://docs.google.com/document/d/1zMS_klTYeSm8ttONNFBmvZ0au_-O9lPRXXfL0WaZ5M4/edit?usp=sharing)
