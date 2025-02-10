#include <iostream>
#include <pcap/pcap.h>
#include <Pcap.hpp>
#include <arpa/inet.h>
#include <sys/socket.h>
#include <linux/if_packet.h>
#include <stdio.h>
#include <iomanip>
/*
 * class {
 *
 *  store arp
 *  set arp
 *  get arp
 * }
 */



//./inquisitor ip_src mac_src ip_target mac_target
//get arp table
//
//
//restore arp table after ctrl+c
int main(int argc, char *argv[]) {
	/*if (argc != 5) {
		std::cout << "Please provide ./inquisitor <ip_src>" \
			<< " <MAC_src> <ip_target> <MAC_target> command." << std::endl;
	}*/
	//char errbuf[PCAP_ERRBUF_SIZE];
	//pcap_init(PCAP_CHAR_ENC_LOCAL, &errbuf);
	//if failed display err + exit
	//ppcap_create()
	//pcap_activate()
	//if fail call pcap_close()
	//get dev?
	//pcap_loop()
	//capture packets and filter arp?
	//forge it
	//pcap_sendpacket() to arp poison

	char errbuf[PCAP_ERRBUF_SIZE];
	int res = pcap_init(PCAP_CHAR_ENC_LOCAL, errbuf);

	if (res) {
		if (res == PCAP_ERROR) {
			std::cerr << errbuf << std::endl;
		}
		return 1;
	}
	Pcap c_pcap(argv[1], argv[2], argv[3], argv[4]);
	std::cout << "find: " << c_pcap.SetPcapList() << std::endl;
	try {
		c_pcap.SetDeviceCapture(c_pcap.GetDevice());
		//activate
		//get arp
		//filter
	} catch (std::invalid_argument& err) {
		std::cerr << err.what() << std::endl;
		return 1;
	} catch (std::runtime_error& err) {
		std::cerr << err.what() << std::endl;
		return 1;
	}
	return 0;
}
