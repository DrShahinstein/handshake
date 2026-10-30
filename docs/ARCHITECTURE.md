# CLI Infrastructure

> public_ip=82.14.99.42  
> private_ip=192.168.1.69  
> preferred_local_port=6969  
> nat_endpoint_turns_out=58293 (via STUN)  

### Help

```
Usage: handshake <command> [argument]

Commands:
- punch [argument]
- help
- version
```

The argument is interpreted by its shape, not by a name:

```
(nothing)     bind 0.0.0.0:6969,   then wait for the peer's link
<port>        bind 0.0.0.0:<port>, then wait for the peer's link
<address>     bind 0.0.0.0:6969,   then punch this IP:PORT directly
<link>        bind 0.0.0.0:6969,   then punch every candidate in the link
```

### Punch

Both peers open a socket,  
find out how the outside world sees them,  
swap that information,  
begin punching each other's holes,  
start a session.  

```bash
$ handshake punch
> bound to 0.0.0.0:6969
  * loopback: 127.0.0.1:6969
  * lan: 192.168.1.69:6969
> discovered public nat endpoint: 82.14.99.42:58293
> share this link with your peer:
  handshake://<base64-link-information>
> paste your peer's link:
  handshake://<base64-link-information>

...punching lan 192.168.1.55:6969
...punching public 88.21.7.10:44012
...peer answered from 88.21.7.10:44012
...connected to (peer-a / peer-b)
```

Peers will need to exchange their base64 links. This can be done through any kind of communication. Using Telegram/WhatsApp etc. or even talking to each other physically, why not.  

### Base64 Link & Candidates

`<base64-link-information>` could look like:
```
{
  "candidates": [
    "192.168.1.69:6969",
    "82.14.99.42:58293"
  ],
  "name": "peer-a",
  "version": 1
}
```

Corresponding to:  
```
handshake://ewogICJjYW5kaWRhdGVzIjogWwogICAgIjE5Mi4xNjguMS42OTo2OTY5IiwKICAgICI4Mi4xNC45OS40Mjo1ODI5MyIKICBdLAogICJuYW1lIjogInBlZXItYSIsCiAgInZlcnNpb24iOiAxCn0=
```

Both peers probe every address in the list, and the first one that answers wins:

```
Thread 1 (loopback): Probing 127.0.0.1:6969
Thread 2 (lan):      Probing 192.168.1.69:6969
Thread 3 (public):   Probing 82.14.99.42:58293
...
```

Whichever wins the race, the connection is held upon it.
This approach would also make room for potential IPv6 fallback implementation in the future.
And, LAN is always the fastest connection if it is really available. That also fits finely.
So, it is preferable in many ways for its flexibility.

Refer to [P2P Comm. Across NAT, section3.2](https://pdos.csail.mit.edu/papers/p2pnat.pdf)

> ...The order and timing of these messages are not critical as long as they are asynchronous.

> ...Nevertheless, hole punching works automatically in all of these scenarios without the application having to know the specific network organization.

That last quote assumes the NATs allow it. As the <EXPLAINER.md> shows, symmetric
NATs on both sides are where it stops working, and that is accepted.

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
| **Magic Bytes** | 3 Bytes | `[u8; 3]` | Must be `0x48 0x53 0x48` ("HSH"). Used for protocol identification. |
| **Frame Type** | 1 Byte | `u8` | Identifies the intent of the packet. Will determine what to expect from the payload. |
| **Status Code** | 1 Byte | `u8` | Execution status (e.g., `0x00` = Success, `0x01` = Error). |
| **Sequence ID** | 4 Bytes | `u32` | Packet indexing |
| **Payload Length**| 4 Bytes | `u32` | Used to know where to stop reading the incoming payload. |
| **Payload** | Variable | `Vec<u8>` | Transmitted data as raw bytes. Can be anything depending on our purpose. |


