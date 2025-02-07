#ifndef FT_PCAP_HPP
# define FT_PCAP_HPP

#include <iostream>
#include <pcap/pcap.h>
#include <Pcap.hpp>
#include <arpa/inet.h>
#include <sys/socket.h>
#include <linux/if_packet.h>
#include <stdio.h>
#include <iomanip>

#include <pcap/pcap.h>
#include <iostream>

class Pcap {
	private:
		const char *ip_src;
		const char *mac_src;
		const char *ip_target;
		const char *mac_target;
		const char *ip_select;
		const char *mac_select;
		pcap_if_t *pcap_list;
		struct pcap_if *pcap;
		Pcap();

	public:
		Pcap(const char *ip_src, const char *mac_src,
				const char *ip_target, const char *mac_target);
		~Pcap();
		bool setPcapList(void);
};

#endif
