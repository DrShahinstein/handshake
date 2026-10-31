# CLI Infrastructure

PeerA  
> public_ip=82.14.99.42  
> private_ip=192.168.1.3  
> preferred_local_port=6969  
> nat_endpoint_turns_out=58293  

PeerB  
> public_ip=48.21.7.10  
> private_ip=192.168.1.7  
> preferred_local_port=4545  
> nat_endpoint_turns_out=44012  

### Help

```
Usage: handshake punch [OPTIONS] [PEER]

Arguments:
  [PEER]  base64-link or IP:PORT

Options:
  -p, --port <PORT>  Local UDP port to bind [default: 6969]
  -h, --help         Print help information
  -V, --version      Print version information
```

### Punch

Both peers open a socket,  
find out how the outside world sees them,  
swap that information,  
begin punching holes toward each other,  
start a session.  

PeerA
```
$ handshake punch
> bound to 0.0.0.0:6969
  * loopback: 127.0.0.1:6969
  * lan: 192.168.1.3:6969
> discovered public nat endpoint: 82.14.99.42:58293
> share this link with your peer:
  handshake://<base64-link-information>
> paste your peer's link:
  handshake://<base64-link-information>

...punching lan 192.168.1.7:4545
...punching public 48.21.7.10:44012
...peer answered from 48.21.7.10:44012
...connected to peer-b
```

PeerB
```
$ handshake punch -p 4545 <peerA_base64>
> bound to 0.0.0.0:4545
  * loopback: 127.0.0.1:4545
  * lan: 192.168.1.7:4545
> discovered public nat endpoint: 48.21.7.10:44012
> share this link with your peer:
  handshake://<base64-link-information>

...punching lan 192.168.1.3:6969
...punching public 82.14.99.42:58293
...peer answered from 82.14.99.42:58293
...connected to peer-a
```

Peers exchange their base64 links. This can be done through any kind of communication. Using Telegram/WhatsApp etc. or even talking to each other physically, why not.  

### Base64 Link & Candidates

`<base64-link-information>` could look like:
```
{
  "candidates": [
    "192.168.1.3:6969",
    "82.14.99.42:58293"
  ],
  "name": "peer-a",
  "version": 1
}
```

Corresponding to:  
```
handshake://ewogICJjYW5kaWRhdGVzIjogWwogICAgIjE5Mi4xNjguMS4zOjY5NjkiLAogICAgIjgyLjE0Ljk5LjQyOjU4MjkzIgogIF0sCiAgIm5hbWUiOiAicGVlci1hIiwKICAidmVyc2lvbiI6IDEKfQ==
```

Both peers probe every address in the list, and the first one that answers wins:

```
Thread 1 (lan):      Probing 192.168.1.3:6969
Thread 2 (public):   Probing 82.14.99.42:58293
...
```

Whichever wins the race, the connection is held upon it.  
This approach would also make room for potential IPv6 fallback implementation in the future.  
And, LAN is always the fastest connection if it is really available. That also fits finely.  
So, it is preferable in many ways for its flexibility.  

Refer to [P2P Comm. Across NAT, section3.2](https://pdos.csail.mit.edu/papers/p2pnat.pdf)

> ...The order and timing of these messages are not critical as long as they are asynchronous.

> ...Nevertheless, hole punching works automatically in all of these scenarios without the application having to know the specific network organization.

# Protocol

### Ascii Diagram

```
Frame header: 13 bytes total

+------------------+------------------+------------------+----------------------+--------------------+--------------------+
| Magic Bytes (3B) | Frame Type (1B)  | Status Code (1B) | Sequence/Req ID (4B) | Payload Length(4B) | Payload (Variable) |
+------------------+------------------+------------------+----------------------+--------------------+--------------------+
```

### Table View

| Field | Size | Type | Description |
| :--- | :--- | :--- | :--- |
| **Magic Bytes** | 3 Bytes | `[u8; 3]` | Must be `0x48 0x53 0x48` ("HSH"). Used for protocol identification |
| **Frame Type** | 1 Byte | `u8` | Identifies the intent of the packet. Will determine what to expect from the payload |
| **Status Code** | 1 Byte | `u8` | Execution status (`200`=Success, `201`=Error) |
| **Sequence ID** | 4 Bytes | `u32` | Packet indexing |
| **Payload Length**| 4 Bytes | `u32` | Used to know where to stop reading the incoming payload |
| **Payload** | Variable | `Vec<u8>` | Transmitted data as raw bytes. Can be anything depending on our purpose |

# Manual Handshake Model

It's a mirrored SYN/ACK exchange, implemented upon UDP packets manually.  

```
X) PeerA --SYN--> PeerB
Y) PeerA <-SYN--- PeerB

X) PeerA <--ACK-- PeerB
Y) PeerA ---ACK-> PeerB
```

