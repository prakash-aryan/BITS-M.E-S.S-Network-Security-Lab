# Network Security Course Assignments

## Introduction

This repository contains assignments and lab work for the CS G513 NETWORK SECURITY course, part of the Masters of Engineering in Software Systems program at Birla Institute of Technology and Science, Pilani- Dubai Campus.

Student: Prakash Aryan

## Assignments

### [Week 1: Lab One - Network Configuration and ACLs](./LabOne)

In this lab, we implemented a network topology with three separate networks connected by a router. The main objectives were:

1. Configure a network using Class B IP addresses
2. Implement Access Control Lists (ACLs) to manage traffic between networks
3. Test and verify the ACL rules using Cisco Packet Tracer

Key files:
- [LabOne.pkt](./LabOne/LabOne.pkt): Cisco Packet Tracer simulation file
- [README.md](./LabOne/README.md): Detailed lab report and instructions
- [PacketTracer.png](./LabOne/PacketTracer.png): Screenshot of the network topology

The lab demonstrates the practical application of ACLs in controlling network traffic and securing communication between different network segments.

### [Week 2: Lab Two - Advanced ACL Implementation](./LabTwo)

In this lab, we expanded on the concepts from Lab One by implementing a more complex network topology with four separate networks. The main objectives were:

1. Configure a network using Class A and Class B IP addresses
2. Implement extended Access Control Lists (ACLs) to manage traffic between multiple networks
3. Configure ACLs on multiple router interfaces
4. Test and verify the ACL rules using Cisco Packet Tracer

Key files:
- [labTwo.pkt](./LabTwo/labTwo.pkt): Cisco Packet Tracer simulation file
- [README.md](./LabTwo/README.md): Detailed lab report, ACL configurations, and test results
- [packetTracerConfig.png](./LabTwo/packetTracerConfig.png): Screenshot of the network topology

This lab demonstrates advanced ACL implementation techniques, including selective blocking of specific hosts and managing traffic flow across multiple network segments.

### [Week 3: Lab Three - Cipher Application](./LabThree)

In this lab, we developed a Rust-based application that implements two cipher algorithms: Additive Cipher (Caesar Cipher) and Rail Fence Cipher. The main objectives were:

1. Implement the Additive Cipher algorithm
2. Implement the Rail Fence Cipher algorithm
3. Create a graphical user interface for the cipher application
4. Provide encryption functionality for both ciphers
5. Visualize the Rail Fence Cipher matrix

Key files:
- [src/main.rs](./LabThree/src/main.rs): Main Rust source code for the cipher application
- [cipher_app.exe](./LabThree/cipher_app.exe): Executable file for Windows
- [README.md](./LabThree/README.md): Detailed explanation of the cipher algorithms and application usage
- [app_icon.ico](./LabThree/app_icon.ico): Application icon

This lab demonstrates the implementation of cryptographic algorithms and the development of a user-friendly interface for encryption tasks. The application provides a practical tool for understanding basic encryption techniques.

### [Week 4: Lab Four - Configuring Server Firewalls](./LabFour)

In this lab, we focused on configuring firewall rules for servers in a network environment. The main objectives were:

1. Configure Server1 to deny ICMP traffic but allow HTTP traffic
2. Configure Server0 to allow ICMP traffic but deny HTTP traffic
3. Implement firewall rules using the GUI in Cisco Packet Tracer
4. Test and verify the firewall configurations using ping and web browser access

Key files:
- [Lab4_Server_Firewall_Configuration.pkt](./LabFour/Lab4_Server_Firewall_Configuration.pkt): Cisco Packet Tracer simulation file
- [README.md](./LabFour/README.md): Detailed lab report, firewall configurations, and test results

This lab demonstrates the implementation of server-level firewall rules to control specific types of network traffic, enhancing network security and service availability control.


Certainly! I'll update the README to include information about LabFive. Here's the revised version with the new section added:

# Network Security Course Assignments

## Introduction

This repository contains assignments and lab work for the CS G513 NETWORK SECURITY course, part of the Masters of Engineering in Software Systems program at Birla Institute of Technology and Science, Pilani- Dubai Campus.

Student: Prakash Aryan

## Assignments

[Previous lab descriptions remain unchanged]

### [Week 5: Lab Five - VLAN Configuration and Inter-VLAN Routing](./LabFive)

In this lab, we implemented a complex network topology with VLANs. The main objectives were:

1. Configure a network using the IP address range 193.175.12.0/26
2. Create 4 subnets with specific VLAN configurations:
   - 1st subnet: 3 VLANs with 9 PCs total
   - 2nd subnet: 2 VLANs with 8 PCs total
   - 3rd subnet: 2 VLANs with 8 PCs total
   - 4th subnet: 3 VLANs with 9 PCs total
3. Test and verify VLAN segmentation and inter-VLAN communication

Key files:
- [Lab5_VLAN_Configuration.pkt](./LabFive/Lab5_VLAN_Configuration.pkt): Cisco Packet Tracer simulation file
- [README.md](./LabFive/README.md): Detailed lab report, VLAN configurations, IP addressing scheme, and test results
- [network_topology.png](./LabFive/network_topology.png): Screenshot of the network topology

This lab demonstrates advanced network segmentation techniques using VLANs, proper IP subnetting. 


## License

This project is licensed under the terms included in the [LICENSE](./LICENSE) file.
