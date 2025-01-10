


## Install RUST

https://doc.rust-lang.org/book/ch01-01-installation.html

```bash 
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

## clone repository

```bash 
$ git clone https://github.com/weratyr/fwupdate.git
```

Build the project by executing: 

```bash 
$ cargo build
```


## cross compiling with docker 

```bash 
$ cross build -j 5 --target aarch64-unknown-linux-gnu
```

## fwupdate

# logger 
```bash 
$ MY_LOG_LEVEL=info,debug MY_LOG_STYLE=auto ./fwupdate
```
 
# custom port 
```bash 
$ MY_LOG_LEVEL=info,debug MY_LOG_STYLE=auto ./fwupdate -p 8080
```