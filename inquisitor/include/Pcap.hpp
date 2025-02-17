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
#include "ft_macros.hpp"

class Pcap {
	private:
		const std::string _ip_src;
		const std::string _mac_src;
		const std::string _ip_target;
		const std::string _mac_target;
		const std::string _interface;
		std::string *_ip_select;
		std::string *_mac_select;
		//pcap_if_t *_pcap_list;
		//pcap_if_t *_device_select;
		pcap_t	*_device_capture;
		struct bpf_program *_fp;
		bpf_u_int32	_netmask;
		//arp

		Pcap();

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
		int	activateCapture(pcap_t *src) const;
		int	compileFilterArp(pcap_t *src);
		int	setFilter(pcap_t *src, struct bpf_program *fp) const;
		int	loopPcap(pcap_t *src, u_char *user);
		//init arp
		//clear arp
};

#endif
