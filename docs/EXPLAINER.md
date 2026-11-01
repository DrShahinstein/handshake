# Main Idea

Aiming to handshake peer-to-peer (P2P) with a friend in the terminal, sharing each other's directories with allowance of mutual downloading facility.

PeerA looking into PeerB's shared directory or vice-versa:
```
--- Shared Directory ---
[ ] file.txt
[x] program.c
[ ] directory/
    [ ] presentation.pptx
    [x] gta6_crack.exe
    [ ] too_secret.torrent       >Select_All   >Download_Selected
```

Relying on IPv6 would make a lot of things easier and direct, sweeping the motivation to start this kind of a project.
However, including me, many people don't have IPv6 available. Check it out using `curl -6 -m 5 https://ifconfig.co`.

For that reason, to maximize usability, peers are going to be assumed to have IPv4 addresses.
This comes with handful of networking problems to resolve with correct strategies which are described in detail here.
These problems are the main motivation to work on this project. To learn, and improve oneself.

## Problem Definition

**Concisely:** We are routed to the internet traffic from behind CGNAT through our IPv4 addresses.
So, inbound connection is not allowed. And, we don't have publicly routable IP anymore like we used to have in the classic internet era. Eventually, a direct P2P connection is not possible without NAT traversal between two machines.

### Preliminary Definitions

1. **IP Accessibility**
  * Public IP  -> Used within WAN. Example: `140.82.121.3`  
  * Private IP -> Used within LAN. Example: `192.168.1.4`  
  * More: In the past, LAN networks weren't the default. The IP you obtained was public by design, globally routable.
  Today, we still have public IPs but it is a lie. Yes, it is used within WAN; no definition trouble. However, nobody can access you from outside. The concept of publicity has changed.  

2. **IP Persistence**
  * Static -> Never changing fixed IP address dedicated for only you. Companies and real servers use it. If you want one, you can contact your ISP and request to buy one.
  * Dynamic -> Temporarily dedicated for you. Resets after restarting modem.

3. **Traffic Direction**
  * Outbound -> You ---> Target | Example: Open youtube.com  
  * Inbound  -> Target --> You  | Example: Make your printer print.  

4. **Public Endpoint:** `<IP>:<PORT>`

5. **NAT Network:** Stands for network address translation. Multiple devices on the same network are exposed to the internet via the same IP address.

6. **CGNAT vs NAT:** A home router performs NAT between LAN and the ISP address; CGNAT is a second NAT layer. Both translate `IP:PORT`; CGNAT is the outer, harder shell.

7. **Core Problem at a Glance**
  * Classic Internet Era -> Residential users had dynamic public IPs. But, even if your IP changed when restarting the modem, it was dedicated to you at that moment. Your computer was directly exposed to the global internet, allowing inbound connections and making direct P2P connections possible.
  * Modern Internet Era -> Residential users still have dynamic public IPs. However, ISPs assign a private IP locally, routing thousands of users through a single, **shared public IP** via Carrier-Grade NAT (CGNAT). Inbound connection attempts are dropped at the ISP level before ever reaching your router. In a way, publicity has lost its meaning compared to the past.

### The Story

<img width="1864" height="643" alt="Image" src="https://github.com/user-attachments/assets/a0556933-f13d-478c-92d1-321d4f3be1e9" />

In the classic era of the internet (up through the early 2000s), internet users connected to the internet using a unique public IP address assigned solely to them. In other words, if you hosted an `index.html` file on your computer, a friend in another country could view that HTML file by going to the address `156.212.36.14:8000` (assuming that’s your modem’s IP address and the port your machine is hosting on). You could send requests to their computer, and they could send requests to yours, making information exchange possible (P2P). Consequently, the public IP address you owned was, in the truest sense, a dedicated address belonging solely to you, and you could freely establish connections both outbound and inbound. If your IP address were to leak somewhere, it might even make it possible for someone to launch an attack against you.

An IPv4 address is a number consisting of four 8-bit segments: [8-bit, 8-bit, 8-bit, 8-bit]. Therefore, it is 32 bits long. With this capacity, there are a total of 2^32 ≈ 4.3 billion possible addresses. In the early 1990s, it was obvious that IPv4 addresses would exhaust at some point. For this reason, the development of technologies such as NAT and IPv6 had already begun back then.

In the 2000s, NAT network technology came into play as a first step. In short, suppose you have three devices at home; previously, separate public IP addresses would have been assigned to each of them, and they would access the internet using those addresses. Now, all three would access the internet through a single IP address assigned to the home modem. This would moderate IPv4 exhaustion to an extent.

Over time, in the 2011-2012 years, NAT had evolved to CGNAT, standing for Carrier-Grade NAT. To put it simply: Just as all three devices in your home would now access the internet through the same address, so too would your neighbors, users in the building across the street, and perhaps even an entire district. There are a certain number of static IP addresses in the CGNAT pool: A, B, C, D, E... Each of these might be used by 500 users, just like you. If one of the users on D gets IP-banned from a forum, the other users connected to D might also see themselves as banned from that IP address.

The way the situation evolved with CGNAT wasn’t exactly ideal. While most devices connecting to the internet used to be personal computers, the majority has now shifted to servers, IoT devices, kitchen robots, and etc. This is where IPv6 addresses were introduced into the system and began to gain widespread adoption. These addresses consist of 128 bits, and for this reason, with a massive number like 2^128, there is no concern about them running out. However, this transition is not that easy. Currently, many people—myself included—still do not have an IPv6 address. Nevertheless, when both the sender and the receiver have IPv6 addresses, data communication has once again become as easy as it used to be.

To conclude: This project is based on P2P connections. The main challenge here is to bypass CGNAT, access the internet using an IPv4 address, and establish a handshake with a peer. Once this connection is established, we will work on implementing the functional features that allow users to share files with one another.

**Comparison Table:**

| #                          | Pre-2000s                                                                     | Modern Era (NAT & CGNAT)                                                                                                                 |
| ------------------------   | ----------------------------------------------------------------------------  | --------------------------------------------------------------------------------------------------------------------------------------   |
| **IP Allocation**          | You have public ip dedicated for you.                                         | You do have public ip, but it's a lie. It isn't dedicated for only you. It's shared by many others same as you.                         |
| **Inbound Connections**    | Open. Anyone with your IP could connect directly to your machine.             | Blocked. Routers and ISPs drop unsolicited incoming traffic by default.                                                                  |
| **Hosting (`index.html`)** | You could host a website directly from your personal computer to the world.   | You would contact your ISP and buy a static IP for yourself to host index.html from your machine, opening yourself to the public realm of the internet. |
| **P2P Communication**      | Direct `Machine A <-> Machine B` connection was the natural default behavior. | Cumbersome. We need to work on NAT traversal. Real-world apps use relay servers to act as guarantors, which comes at cost.     |
| **Address Exhaustion**     | 4.3 billion addresses seemed sufficient for early internet adoption.          | IPv4 pool depleted, making massive-scale IP sharing (CGNAT) mandatory. IPv6 is also something, but even I don't have it.                 |

<br/>

## Solution Approach

<img width="1581" height="783" alt="Image" src="https://github.com/user-attachments/assets/521aadd1-7cbf-4493-8612-9772467449e3" />

Before establishing a peer-to-peer connection behind CGNAT, we face two fundamental obstacles:
1. A machine behind CGNAT has no native way of knowing which temporary public endpoint (`IP:Port`) it uses to communicate with the outside world.
2. Inbound connection is not allowed. Incoming packets sent to our public endpoint are dropped by the ISP's CGNAT firewall.

### 1. STUN Servers

To overcome the first obstacle, we need an outside entity to act as a mirror: **STUN** (*Session Traversal Utilities for NAT*).

Example stun server: `stun.l.google.com:19302`  

* **What STUN servers are NOT:**
  * They are **not middleman or relay servers** for your actual file transfers.
  * They do **not host, proxy, or store any data**, keeping your P2P bandwidth completely private and direct.

* **What STUN servers DO:**
  * In a way, you are asking for your own identity to police before leaving your town.
  * A STUN server simply receives a lightweight UDP packet from your machine and replies: *"Hey, I received your message from this public IP and Port!"*
  * By reflecting this information back, it reveals your **Public Endpoint (NAT Binding)** to you. Once both peers discover their respective endpoints, they are ready for the next step.

There are two things to keep in mind. First, both peers' public endpoints must be known. Second, after this discovery, these peers must exchange these endpoints with each other. This exchange is called **signaling.** Say, peers nudging each other on Telegram "here's mine"; that's also a signaling. A prerequisite for the next step.

### 2. Hole-Punching

Now comes the second obstacle: CGNAT drops all incoming traffic because it appears unrequested.  

We are going to say: "My house isn't open to guests. But I can still go out."  

Instead of relying on a traditional inbound connection, we exploit how NAT firewalls handle outbound traffic:

* **The outbound mechanism:** We are freely allowed with outbound connection. Since it is the most basic traffic direction, it isn't restricted. The other way around, internet would have no practical use anyway. So, when you send a packet out, your firewall temporarily opens a state entry saying: *"I sent a packet to Address X, so allow any response coming back from Address X."*
* **Simultaneous punching:** Once PeerA and PeerB exchange their STUN-discovered public endpoints, both peers send outbound packets to each other at the same moment. They don't have to be accurate to the nanosecond. Retries and a few seconds of skew are fine.
* **Establishing the link:** When PeerA sends a packet to PeerB, PeerA's firewall opens a path for PeerB. Simultaneously, PeerB's firewall opens a path for PeerA. 

By punching a hole from the inside out simultaneously, both firewalls are convinced that the incoming packets are legitimate responses to their own outbound requests—turning two blocked connections into a direct, bidirectional P2P tunnel.

Short story:  
If PeerA simply tries to reach PeerB, the packet gets stuck at the gateway.  
However, if PeerB is also trying to reach PeerA at that moment, they can find each other.  
If both sides shine a light at each other at the same time, they can see each other.  

### 3. UDP

TCP nat traversal is possible for sure, but it has less accuracy and more cumbersome work to add. One of the reasons is because it has a strict, built-in three-way handshake mechanism. CGNAT routers control this system more strictly and rigorously.  

By using UDP, we will have a more flexible option. UDP doesn't have a complex handshake method. It's simply like "throwing data." However, we also need the uninterrupted and clear data flow that TCP provides. To solve this, we will utilize the sequence IDs included in our [protocol](ARCHITECTURE.md#protocol) and index the packets. This way, we'll still be able to communicate without data loss.  

In other words, we'll take advantage of UDP's flexibility while partially adding TCP's reliability ourselves. We will communicate using this combination.

It's also worth noting that STUN servers generally expect UDP requests.  

### 4. Overall Picture

```
1. Discover PeerA's public endpoint. (stun)
2. Discover PeerB's public endpoint. (stun)
3. Let PeerA and PeerB exchange their endpoint information. (signaling)  # could be any chat app, any way of comm.
4. Let peers start sending packets towards each other simultaneously. (udp hole-punching)
5. Shared session begin. Do whatever you want. (connection)
```

## Scope & Boundaries

Common frameworks like WebRTC offer high success rates with P2P connection. However, they rely on a fallback mechanism: Relay servers (commonly known as TURN servers). When direct hole-punching fails, these systems silently route all live traffic through a middleman proxy.

In the scope of this project, relying on a relay server is considered cheating.

A relay server is a real, physical or virtual machine hosted in the public realm of the internet. A relay server actively receives, processes, and re-transmits every single byte of your actual payload. This requires:
* A machine with a dedicated public IP running 24/7.
* Continuous server maintenance and hosting overhead.
* Significant bandwidth consumption.

In short: **Relay servers come with a cost.**

The core motivation of `handshake` is to establish a pure, casual, zero-cost P2P connection using only the resources already available to everyday internet users. If two peers cannot punch a direct hole through their respective firewalls, we gracefully accept the failure rather than masking it behind a middleman.

### What Determines the Success of a P2P Connection?

Not geography, distance, or ISP branding. The NAT devices between the peers decide. Two people in the same city can fail; two people on different continents can succeed.

A NAT does two things: it gives your socket a public mapping, and it filters who can send back through it.

#### -- Cone NAT --

Cone NAT keeps one mapping for every destination:

```
> Hey, google_stun, what is my endpoint?
< 82.14.99.42:51479

to STUN:   192.168.1.10:5000  ->  82.14.99.42:51479
to PeerB:  192.168.1.10:5000  ->  82.14.99.42:51479   same port
```

Straightforward, no trouble. It has three types:

```
full cone          anyone can send in
restricted cone    only IPs you contacted
port-restricted    only exact IP:PORTs you contacted
```

Most home routers are port-restricted cone. That sounds strict, but it is exactly what hole-punching handles.  
So, we are happy with this kind. Even the most restricted option is enough for us. Which is also the most common. We like Cone NAT.

#### -- Symmetric NAT --

Symmetric NAT is the hard one. It gives a different port per destination:

```
to STUN:   your_socket  ->  82.14.99.42:51479
to PeerB:  your_socket  ->  82.14.99.42:61002   different port

your friend aims at 51479
but the path to you is 61002
```

You might be saying, oh if this is the situation then there is no chance.  

Well, we really don't want to encounter symmetric nat. But still, there is a chance we can establish p2p connection.  

Let's say PeerA is symmetric and PeerB is cone:
```
-- Discovery A --
udp_socket --> google_stun --> IP_A:8000
udp_socket --> IP_B:9000   --> fine, I'm knocking on the correct door.

-- Discovery B --
udp_socket --> google_stun --> IP_B:9000
udp_socket --> IP_A:8000   --> {approach}

> approach
Okay, I'm sending packets to port:8000 but we cannot establish connection.
They may have symmetric NAT. The port of 8000 could be the last port that was given to the socket querrying its STUN server.
Let me brute force other possible ports between:
8000-128 <= x <= 8000+128
  7872   <= x <=   8128

Maybe this way I can identify the port that A was assigned while trying to reach me.
And we can perform hole-punching to the correct endpoints.
```

If you would like to read further about here, I recommend you checking out [this.](https://tailscale.com/blog/how-nat-traversal-works#:~:text=rate%2E-,The,manage%2E) They refer to cone/symmetric behaviors as easy-nat/hard-nat.  

So, what if both peers have symmetric nat behavior? We just give up. The rest is relay server stuff.  

#### -- Summary --

Here is the rough picture:

```
public IP + anything           -> works (if the NAT side sends)  # same relationship between your_device-->youtube.com
cone + cone                    -> works
symmetric + full cone          -> often works
symmetric + restricted cone    -> often works
symmetric + port-restricted    -> often works (less)
symmetric + symmetric          -> most probably fails
```

The pattern is what matters: no symmetric NAT, usually fine; one symmetric NAT, questionable; two symmetric NATs, practically no.  

Most home broadband behaves like port-restricted cone, and two of those can punch through each other. So for two friends on ordinary home internet, we would expect successful p2p connection.  

A relay would make every row work, but then your files go through someone else's machine and someone pays for the bandwidth. That is the cheating we keep talking about. We would rather fail on the hard NAT combinations than call a relayed connection peer-to-peer.  

#### -- Testing --

If you wonder what might be your NAT's behavior, `pystun3` is a thing.  

```
$ python3 -m venv anyenv
$ source ./bin/activate
$ pip install pystun3
$ pystun3
```

It gave me this in my case:  
<img width="356" height="179" alt="Image" src="https://github.com/user-attachments/assets/fbfa0b7b-c06f-4282-ae69-345868a2db05" />

We can see from this that I probably have cone NAT as well as many residential users, specifically the most common type port-restricted. 
Which is common and fine to see.  

## Conclusion
In theory, establishing a connection between peers behind Cone NATs should be straightforward. In reality, variables like OS firewalls, aggressive NAT timeouts, timing skews, and unpredictable CGNAT behavior mean a 100% connection guarantee is impossible. Within these everyday constraints, the project tries to maximize P2P connectivity purely using local resources—without taking on any hosting overhead or relay server costs.  

So far I've managed to see p2p:
* is held between my home network and mobile data service giving hotspot to my laptop.  
*  

## References to Note
* https://pdos.csail.mit.edu/papers/p2pnat.pdf (2005)
* https://tailscale.com/blog/how-nat-traversal-works (2020)

