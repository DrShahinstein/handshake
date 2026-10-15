# CLI Infrastructure

> public_ip=82.14.99.42  
> private_ip=192.168.1.69  
> preferred_local_port=6969  
> nat_endpoint_turns_out=58293 (via STUN)  

### Listen

PeerA
```bash
$ handshake listen 6969
> listening on 0.0.0.0:6969
  * loopback: 127.0.0.1:6969
  * lan: 192.168.1.69:6969
> discovered public nat endpoint: 82.14.99.42:58293
> share this link with your peer: handshake://<base64-link-information>
```

### Connect

PeerB (loopback)
```bash
$ handshake connect 127.0.0.1:6969
# shortcut: handshake connect loopback:6969
# shortcut: handshake connect localhost:6969
```

PeerB (LAN)
```bash
$ handshake connect 192.168.1.69:6969
```

PeerB (internet)
```bash
$ handshake connect handshake://<base64-link-information>
```

### Base64 Link & Candidates

\<base64-link-information\> could look like:
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

Candidates logic:  
```
Thread1: Expects successful connection with 192.168.1.69:6969
Thread2: Expects successful connection with 82.14.99.42:58293
...
```

Whichever wins the race, the connection is held upon it.  
This approach would also make room for potential IPv6 fallback implementation in the future.  
And, LAN is always the fastest connection if it is really available. That also fits finely.  
Additionally, you can also give race advantage to some certain threads.  
So, it is preferrable in many ways for its flexibility.

### Questions

**Q1:** Can peerB accidentally connect to another service already running on port 6969 in their own LAN while trying the candidate 192.168.1.69:6969 address?

Yes, this is possible. To prevent false connections, we'll rely on the special sign of our framing protocol. It is actually just some magic bytes. Check out the protocol section below.

So, it would roughly look like a communication like this:  
\+: Do you know what "HSH" means?  
\-:  
\+: Yeah you don't. I think you are my television by the way.    

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


