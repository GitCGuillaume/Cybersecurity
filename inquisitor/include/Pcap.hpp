#ifndef FT_PCAP_HPP
# define FT_PCAP_HPP

#include <iostream>
#include <pcap/pcap.h>
#include <Pcap.hpp>
#include <arpa/inet.h>
#include <netinet/ether.h>
#include <sys/socket.h>
#include <linux/if_packet.h>
#include <stdio.h>
#include <iomanip>
#include <exception>
#include <cstdlib>
#include <pcap/pcap.h>
#include <iostream>

class Pcap {
	private:
		std::string _ip_src;
		std::string _mac_src;
		std::string _ip_target;
		std::string _mac_target;
		std::string *_ip_select;
		std::string *_mac_select;
		pcap_if_t *_pcap_list;
		pcap_if_t *_device_select;
		pcap_t	*_device_capture;
		//arp

		Pcap();

	public:
		Pcap(const char *ip_src, const char *mac_src,
				const char *ip_target, const char *mac_target);
		~Pcap(); //<< also clear arp?
		struct pcap_if * GetDevice() const;
		pcap_t *GetDeviceCapture() const;
		bool SetPcapList(void);
		void SetDeviceCapture(pcap_if_t *src);
		
		//init arp
		//clear arp
};

#endif
