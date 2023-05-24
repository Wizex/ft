fs
------------
**fs** is a command-line tool for file transferring.
It works by transferring files between server and client. The project contains two modules for a server and a client.   
After installation, you will have two binaries 'ft' and 'ftd' for a client and a server respectively.

# Installation
Run the following command to install:
```shell
cargo install --git https://github.com/Wizex/ft
```
Try to run the command:
```shell
ft --help 
```

# Usage
## Client
```shell
Usage: ft <TRANSFER_TYPE> <HOST> <LOCAL_PATH> <REMOTE_PATH>

Arguments:
  <TRANSFER_TYPE>
          File transfer type

          Possible values:
          - to:   Send a file to a server
          - from: Download a file from a server

  <HOST>
          Remote server address

  <LOCAL_PATH>
          Local path

  <REMOTE_PATH>
          Remote path

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
## Server
The server accepts only 1 argument.
```shell
ftd <ADDRESS>
```
## Examples
### Copy a file to a server
```shell
$ ft to 0.0.0.0:8080 ./penguin.jpg /home/user/Pictures/penguin.jpg 
```
