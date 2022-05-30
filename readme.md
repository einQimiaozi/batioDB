# a persistence no-sql(kv) db,based on rust

batioDB is a a persistence no-sql db being developed.

if you are interested in this project, you can contact me.

mail: ocelotbuck@gmail.com

## QuickStart

### step1: clone this respositry in your terminal

```
git clone https://github.com/einQimiaozi/batioDB.git
```

### step2: run server

```
cd batioDB
cargo run

baby turn it on!
bind: yourIpAddress:Port
db in: test.data
```

You can modify config.yaml file configures your server address and port.

The default address is 127.0.0.1 and the default port is 8765.

### step3: run client

Open another terminal

```
cd batioDB/batioDB_client
cargo run

connection: PollEvented {
    io: Some(
        TcpStream {
            addr: clientIpAddress:clientPort,
            peer: serverIpAddress:serverPort,
            fd: yourFD,
        },
    ),
}
baby turn it on!!!
```

Similarly, you can modify the address and port of the server that the current client wants to connect to in client_config.yaml

### step4: turn it on!!！

Batiodb supports four kinds of instructions: 

set [key] [value]
```
set name Qimiaozi            
value: "success"
```

get [key]
```
get name
value: "Qimiaozi"
```

update [key] [value]
```
update name einQimiaozi
value: "success"
```

delete [key]
```
delete name
value: "success"
```

If you enter an invalid instruction
```
set age 
value: "command invalid"
```

finally，Enter ctrl+z to close the client


