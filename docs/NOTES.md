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
So, inbound connection is not allowed. And, we don't even have dedicated public IPs. Eventually, a direct P2P connection is not possible between two machines.

### Preliminary Definitions

1. **IP Accessibility**
  * Public IP  -> Globally unique and directly reachable from anywhere. Example: `140.82.121.3`
  * Private IP -> Used only within LAN. Example: `192.168.1.4`

2. **IP Persistency**
  * Static -> Never changing fixed IP address dedicated for only you. Companies and real servers use it. If you want one, you can contact your ISP and request to buy one.
  * Dynamic -> Temporarily dedicated for you. Resets after restarting modem.

3. **Traffic Direction**
  * Outbound -> You ---> Target | Example: Open youtube.com
  * Inbound  -> Target --> You  | Example: Your machine is tried to be connected with SSH

4. **Public Endpoint:** `<IP>:<PORT>`

5. **NAT Network:** Stands for network address translation. Multiple devices on the same network are exposed to the internet via the same IP address.

6. **Core Problem at a Glance**
  * Classic Internet Era -> Residential users had **dynamic public IPs** like today we have. However, with a difference, even if your IP changed when restarting the modem, it was **dedicated** to you at that moment. Your computer was directly exposed to the global internet, allowing inbound connection and making direct P2P connection possible.
  * Modern Internet Era -> Residential users no longer get a dedicated public IP. Instead, ISPs assign a **private IP** locally, routing thousands of users through a single, **shared public IP** (Carrier-Grade NAT). Inbound connection attempts are dropped at the ISP level before ever reaching your router.

### The Story

<img width="1864" height="643" alt="Image" src="https://github.com/user-attachments/assets/a0556933-f13d-478c-92d1-321d4f3be1e9" />

In the classic era of the internet (up through the early 2000s), internet users connected to the internet using a unique public IP address assigned solely to them. In other words, if you hosted an `index.html` file on your computer, a friend in another country could view that HTML file by going to the address `156.212.36.14:8000` (assuming that’s your modem’s IP address and the port your machine is hosting on). You could send requests to their computer, and they could send requests to yours, making information exchange possible (P2P). Consequently, the public IP address you owned was, in the truest sense, a dedicated address belonging solely to you, and you could freely establish connections both outbound and inbound. If your IP address were to leak somewhere, it might even make it possible for someone to launch an attack against you.

An IPv4 address is a number consisting of four 8-bit segments: [8-bit, 8-bit, 8-bit, 8-bit]. Therefore, it is 32 bits long. With this capacity, there are a total of 2^32 ≈ 4.3 billion possible addresses. In the early 1990s, it was obvious that IPv4 addresses would exhaust at some point. For this reason, the development of technologies such as NAT and IPv6 had already begun back then.

In the 2000s, NAT network technology came into play as a first step. In short, suppose you have three devices at home; previously, separate public IP addresses would have been assigned to each of them, and they would access the internet using those addresses. Now, all three would access the internet through a single IP address assigned to the home modem. This would moderate IPv4 exhaustion to an extent.

Over time, in the 2011-2012 years, NAT had evolved to CGNAT, standing for Carrier-Grade NAT. To put it simply: Just as all three devices in your home would now access the internet through the same address, so too would your neighbors, users in the building across the street, and perhaps even an entire district. There are a certain number of static IP addresses in the CGNAT pool: A, B, C, D, E... Each of these might be used by 500 users, just like you. If one of the users on D gets IP-banned from a forum, the other users connected to D might also see themselves as banned from that IP address.

The way the situation evolved with CGNAT wasn’t exactly ideal. While most devices connecting to the internet used to be personal computers, the majority has now shifted to servers, IoT devices, kitchen robots, and etc. This is where IPv6 addresses were introduced into the system and began to gain widespread adoption. These addresses consist of 128 bits, and for this reason, with a massive number like 2^128, there is no concern about them running out. However, this transition is not that easy. Currently, many people—myself included—still do not have an IPv6 address. Nevertheless, when both the sender and the receiver have IPv6 addresses, data communication has once again become as easy as it used to be.

To conclude: This project is based on P2P connections. The main challenge here is to bypass CGNAT, access the internet using an IPv4 address, and establish a handshake with a peer. Once this connection is established, we will work on implementing the functional features that allow users to share files with one another.

**Comparison Table:**

| #                          | Pre-2000s (The Classic Internet)                                              | Modern Era (NAT & CGNAT)                                                                                                                 |
| ------------------------   | ----------------------------------------------------------------------------  | --------------------------------------------------------------------------------------------------------------------------------------   |
| **IP Allocation**          | You have dedicated public ip                                                  | You have shared public ip with many others. The massive NAT device provided by your ISP would assign a temporary public endpoint (ip+port) for you to connect to the internet |
| **Inbound Connections**    | Open. Anyone with your IP could connect directly to your machine.             | Blocked. Routers and ISPs drop unsolicited incoming traffic by default.                                                                  |
| **Hosting (`index.html`)** | You could host a website directly from your personal computer to the world.   | You would contact your ISP and buy a static IP for yourself to host index.html from your machine. This way you would skip the whole CGNAT pool having your own special address in the internet.                                       |
| **P2P Communication**      | Direct `Machine A <-> Machine B` connection was the natural default behavior. | Requires routing traffic through intermediary middleman servers. The solution section in this writing is exactly related to this.        |
| **Address Exhaustion**     | 4.3 billion addresses seemed sufficient for early internet adoption.          | IPv4 pool depleted, making massive-scale IP sharing (CGNAT) mandatory. IPv6 is also something, but even I don't have it.                 |

<br/>

## Solution Approach

<img width="1581" height="783" alt="Image" src="https://github.com/user-attachments/assets/521aadd1-7cbf-4493-8612-9772467449e3" />

Before establishing a peer-to-peer connection behind CGNAT, we face two fundamental roadblocks:
1. **Lack of self-awareness:** A machine behind CGNAT has no native way of knowing which temporary public endpoint (`IP:Port`) it uses to communicate with the outside world.
2. **Blocked inbound traffic:** Any unsolicited incoming connection attempt sent to our public endpoint is immediately dropped by the ISP's CGNAT firewall.

### 1. STUN Servers

To overcome the first roadblock, we need an outside entity to act as a mirror: enter **STUN** (*Session Traversal Utilities for NAT*).

* **What STUN servers are NOT:**
  * They are **not middleman or relay servers** for your actual file transfers.
  * They do **not host, proxy, or store any data**, keeping your P2P bandwidth completely private and direct.

* **What STUN servers DO:**
  * In a way, you are asking for your own identity to police before leaving your town.
  * A STUN server simply receives a lightweight UDP packet from your machine and replies: *"Hey, I received your message from this public IP and Port!"*
  * By reflecting this information back, it reveals your **Public Endpoint (NAT Binding)** to you. Once both peers discover their respective endpoints, they are ready for the next step.

### 2. Hole-Punching

Now comes the second roadblock: CGNAT drops all incoming traffic because it appears unrequested. Since traditional inbound connections are blocked, we use a technique referred to as **hole-punching**.

Instead of relying on a traditional inbound connection, we exploit how NAT firewalls handle outbound traffic:

* **The outbound mechanism:** We are freely allowed with outbound connection. Since it is the most basic traffic direction, it isn't restricted. The other way around, internet would have no practical use already. So, when you send a packet out, your firewall temporary opens a state entry saying: *"I sent a packet to Address X, so allow any response coming back from Address X."*
* **Simultaneous punching:** Once PeerA and PeerB exchange their STUN-discovered public endpoints, both peers send outbound packets to each other **at the exact same moment**.
* **Establishing the link:** When PeerA sends a packet to PeerB, PeerA's firewall opens a path for PeerB. Simultaneously, PeerB's firewall opens a path for PeerA. 

By punching a hole from the inside out simultaneously, both firewalls are convinced that the incoming packets are legitimate responses to their own outbound requests—turning two blocked connections into a direct, bidirectional P2P tunnel.

