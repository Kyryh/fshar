**F**ile **shar**ing via TCP

# Usage
```
fshar [OPTIONS] <mode> [server-address]
```


# Arguments
```
<mode>            File sharing mode
                  • server-sender: Send all files in folder to the client
                  • server-receiver: Receive all files in client's folder
                  • client: Send/receive files to/from server, depending on server's mode
[server-address]  Server's address, required only when `mode` is `client`
```

# Options
```
-p, --server-port <server-port>      Server: port to listen on
                                      Client: port to connect to
                                      [default: 4931]
-i, --input-folder <input-folder>    Folder to use when sending files, in case:
                                      • `mode` is `server-sender`
                                      • `mode` is `client` with a `server-receiver` server
                                      [default: ./in]
-o, --output-folder <output-folder>  Folder to use when receiving files, in case:
                                      • `mode` is `server-receiver`
                                      • `mode` is `client` with a `server-sender` server
                                      [default: ./out]
-k, --keep-listening                 If the server should keep listening
                                      after sending files to the client
-r, --retry <retry>                  How many times a client should retry
                                      connecting to the server after an error
                                      -1 means it will retry indefinitely
                                      [default: 0]
-v, --overwrite                      If files should be overwritten when receiving
                                       Useful for receiving updated copies of a file
                                       but it makes it unable to complete partially
                                       downloaded files
-h, --help                           Print help
-V, --version                        Print version
```

# Examples
## Server
- Send contents of ./in folder
  ```
  $ fshar server-sender
  ```
- Receive files and store in ./my-files 
  ```
  $ fshar server-receiver -o ./my-files
  ```
- Send files using port 5050 and continue listening for more clients 
  ```
  $ fshar server-receiver -p 5050 --keep-listening
  ```
## Client

- Receive files from local `server-sender` and store them in ./out
  ```
  $ fshar client 127.0.0.1
  ```

- Send files stored in ./my-files to remote `server-receiver`
  ```
  $ fshar client 8.8.8.8 -i ./my-files 
  ```

- Receive files local `server-sender` running on port 5050 retry 5 times if it fails 
  ```
  $ fshar client 127.0.0.1 --port 5050 -r 5
  ```

