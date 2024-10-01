# Network Security Lab 4: Configuring Server Firewalls

## Objective

Configure firewall rules on two servers in a network to achieve the following:

1. Server1: Deny ICMP traffic but allow HTTP/HTTPS traffic
2. Server0: Allow ICMP traffic but deny HTTP/HTTPS traffic

## Network Setup


## Firewall Configurations

### Server1 Firewall Setup

### Server0 Firewall Setup


## Testing

### ICMP Test (Ping)


### HTTP/HTTPS Test


## Cisco Packet Tracer File

The complete network setup and configurations can be found in the attached Packet Tracer file: `Lab4_Server_Firewall_Configuration.pkt`

## Configuration Steps

1. Configure Server1 firewall:
   - Deny ICMP protocol
   - Allow IP protocol (which includes HTTP/HTTPS)

2. Configure Server0 firewall:
   - Allow ICMP protocol
   - Deny IP protocol (which includes HTTP/HTTPS)

3. Test configurations:
   - Use ping command to test ICMP traffic
   - Use web browser to test HTTP/HTTPS access

## Conclusion

This lab demonstrates the successful implementation of firewall rules on servers to control ICMP and HTTP/HTTPS traffic. The configured rules effectively allow or deny traffic as per the specified requirements, enhancing network security and service availability control.