#ifndef FT_PCAP_HPP
# define FT_PCAP_HPP

#include <map>
#include <iostream>
#include <pcap/pcap.h>
#include <Pcap.hpp>
#include <arpa/inet.h>
#include <netinet/ether.h>
#include <netinet/ip.h>
#include <netinet/tcp.h>
#include <net/ethernet.h>
#include <net/if_arp.h>
#include <sys/socket.h>
#include <linux/if_packet.h>
#include <stdio.h>
#include <iomanip>
#include <exception>
#include <cstdlib>
#include <pcap/pcap.h>
#include <iostream>
#include <cstring>
#include "ft_macros.hpp"

#define BUFFER_SIZE \
	sizeof(struct ether_header) + sizeof(struct ether_arp)

class Pcap {
	private:
		const std::string _ip_src;
		const std::string _mac_src;
		const std::string _ip_target;
		const std::string _mac_target;
		const std::string _interface;
		//std::string	_self_mac;
		//std::string *_ip_select;
		//std::string *_mac_select;
		//pcap_if_t *_device_select;
		pcap_if_t *_pcap_list;
		pcap_t	*_device_capture;
		struct bpf_program *_fp;
		bpf_u_int32	_netmask;
		bpf_u_int32	_network;
		char _buf[BUFFER_SIZE];
		unsigned char  _sll_halen;
		unsigned char  *_sll_addr;
		//arp

		Pcap();
		void	forgePacketReply(const u_char *bytes, bpf_u_int32 len);

	public:
		Pcap(const char *ip_src, const char *mac_src,
				const char *ip_target, const char *mac_target,
				std::string &interface);
		~Pcap(); //<< also clear arp?
		const std::string & getInterface() const;
		//struct	pcap_if * GetDevice() const;
		pcap_t	*GetDeviceCapture() const;
		struct bpf_program *getBpf() const;
		//bool	SetPcapList(void);
		void	SetDeviceCapture(const std::string &interface);
		int	setTimeout(pcap_t *src, int to_ms) const;
		int	setNonBlock(char *errbuf, int val);
		int	setSelfMac();
		int	activateCapture(pcap_t *src) const;
		int	compileFilterArp(pcap_t *src);
		int	setFilter(pcap_t *src, struct bpf_program *fp) const;
		int	loopPcap(pcap_t *src);
		int	sendPacket() const;
		void	forgePacketRequest(bool restore);
		void	forgePacketRequestSrc(bool restore);
		
		//init arp
		//clear arp
};

#endif
