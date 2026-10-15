# Implementation Outline

<p>1. Build CLI infrastructure</p>

> public_ip=82.14.99.42  
> private_ip=192.168.1.69  
> preferred_local_port=6969  
> nat_endpoint_turns_out=58293 (via STUN)  

PeerA
```bash
$ handshake listen 6969
> [handshake] listening locally on 192.168.1.69:6969
> [handshake] discovered public nat endpoint: 82.14.99.42:58293
> [handshake] share this link with your peer: handshake://<base64-link-information>
```

PeerB
```bash
$ handshake connect handshake://<base64-link-information>
```

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
```

Whichever wins the race, the connection is held upon it.  
This approach would also make room for potential IPv6 fallback implementation in the future.  
And, LAN is always the fastest connection if it is really available. That also fits finely.  

