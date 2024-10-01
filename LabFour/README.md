# Network Security Lab 4: Configuring Server Firewalls

## Objective

Configure firewall rules on two servers in a network to achieve the following:

1. Server1: Deny ICMP traffic but allow HTTP traffic
2. Server0: Allow ICMP traffic but deny HTTP traffic

## Network Setup


![Screenshot 2024-10-01 195751](https://github.com/user-attachments/assets/9ba5e3f1-d4f7-43d1-8c80-b5fa1d9fa158)

## Firewall Configurations

### Server1 Firewall Setup
![Screenshot 2024-10-01 192239](https://github.com/user-attachments/assets/3d500fff-4695-48ce-9d69-7b16cdd91f23)


### Server0 Firewall Setup

![Screenshot 2024-10-01 201126](https://github.com/user-attachments/assets/b2a41779-060c-44ce-8062-8fdad61f99f7)

## Testing

### ICMP Test (Ping)

![Screenshot 2024-10-01 194529](https://github.com/user-attachments/assets/dbe47a01-75cf-401c-9966-e6b88157f9dc)


![Screenshot 2024-10-01 194841](https://github.com/user-attachments/assets/516e9a9f-6151-40e2-b302-621a9058bb4b)

### HTTP Test

![Screenshot 2024-10-01 194512](https://github.com/user-attachments/assets/8d2cfe99-2bcb-4c0f-9ba0-3b65cda149e7)

![Screenshot 2024-10-01 194827](https://github.com/user-attachments/assets/38fb52d0-b215-4474-8688-838401d527ca)


## Cisco Packet Tracer File

The complete network setup and configurations can be found in the attached Packet Tracer file: `Lab4_Server_Firewall_Configuration.pkt`

## Configuration Steps

1. Configure Server1 firewall:
   - Deny ICMP protocol
   - Allow IP protocol (which includes HTTP)

2. Configure Server0 firewall:
   - Allow ICMP protocol
   - Deny IP protocol (which includes HTTP)

3. Test configurations:
   - Use ping command to test ICMP traffic
   - Use web browser to test HTTP access

## Conclusion

This lab demonstrates the successful implementation of firewall rules on servers to control ICMP and HTTP traffic. The configured rules effectively allow or deny traffic as per the specified requirements, enhancing network security and service availability control.
