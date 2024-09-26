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

## License

This project is licensed under the terms included in the [LICENSE](./LICENSE) file.