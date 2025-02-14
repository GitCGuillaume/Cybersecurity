#include <iostream>
#include <pcap/pcap.h>
#include <Pcap.hpp>
#include <arpa/inet.h>
#include <sys/socket.h>
#include <linux/if_packet.h>
#include <stdio.h>
#include <iomanip>
#include <csignal>
#include "ft_macros.hpp"

//a delete
//fin delete

/*
 * class {
 *
 *  store arp
 *  set arp
 *  get arp
 * }
 */

/*
 * To free ARP table
 */
static u_char g_free_arp = 0;
/*
 * Mandatory global
 */
static pcap_t	*g_pcap = NULL;

void	signal_handler(int sig) {
	if (sig == SIGINT) {
		std::cout<<"signal"<<std::endl;
		g_free_arp = 1;
		if (g_pcap) {
			pcap_breakloop(g_pcap);
		}
	}
}

int loop_filter(Pcap &c_pcap) {
	std::signal(SIGINT, signal_handler);
	pcap_t *device = c_pcap.GetDeviceCapture();
	if (!device)
		return 1;
	g_pcap = device;
	c_pcap.loopPcap(device, &g_free_arp);
	//loop {
	//
	//}
	//if g_free_arp == 1 > free arp after pcap_breakloop;
	return 0;
}

int start_capture(Pcap &c_pcap) {
	pcap_if_t *device;
	pcap_t	*device_capture;
	int	error;
	struct bpf_program *fp;

	try {
		//get arp
		device = c_pcap.GetDevice();
		if (!device) {
			std::cerr << "Couldn't get device" << std::endl;
			return 1;
		}
		c_pcap.SetDeviceCapture(device);
		device_capture = c_pcap.GetDeviceCapture();
		if (!device_capture) {
			std::cerr << "Couldn't get device for capture." << std::endl;
			return 1;
		}
		error = c_pcap.activateCapture(device_capture);
		if (error != 0) {
			std::cerr << pcap_statustostr(error) << std::endl;
			return 1;
		}
		//filter ask activate before
		error = c_pcap.compileFilterArp(device_capture);
		if (error != 0) {
			pcap_perror(c_pcap.GetDeviceCapture(),
				"Compile filter failed");
			return 1;
		}
		fp = c_pcap.getBpf();
		if (!fp) {
			std::cerr << "Couldn't get Bpf." << std::endl;
			return 1;
		}
		error = c_pcap.setFilter(device_capture, fp);
		if (error != 0) {
			pcap_perror(c_pcap.GetDeviceCapture(),
				"Set filter failed");
			return 1;
		}
		return loop_filter(c_pcap);
	} catch (std::runtime_error& err) {
		std::cerr << err.what() << std::endl;
		return 1;
	} catch (std::bad_alloc& err) {
		std::cerr << err.what() << std::endl;
		return 1;
	} catch (std::exception& err) {
		std::cerr << err.what() << std::endl;
		return 1;
	}
}

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
	return start_capture(c_pcap);
}
