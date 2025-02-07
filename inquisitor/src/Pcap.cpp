#include <Pcap.hpp>

Pcap::Pcap(const char *ip_src, const char *mac_src,
	  const char *ip_target, const char *mac_target) {
	std::cout << "Pcap Created" <<std::endl;
	this->ip_src = ip_src;
	this->mac_src = mac_src;
	this->ip_target = ip_target;
	this->mac_target = mac_target;
	this->pcap_list = NULL;
	this->pcap = NULL;
}

/*
 * Clear pcap allocation
 */
Pcap::~Pcap() {
	if (this->pcap_list)
		pcap_freealldevs(this->pcap_list);
	//pcap_close(this->pcap);
	std::cout << "Destroyed" <<std::endl;
}

bool Pcap::setPcapList(void) {
	char errbuf[PCAP_ERRBUF_SIZE] = {0};
	int res = pcap_findalldevs(&this->pcap_list, errbuf);
	if (res)
		return false;
	pcap_if_t *list = this->pcap_list;
	while (list) {
		std::cout << "name: " << list->name << std::endl;
		pcap_addr *addr_list = list->addresses;
		while (addr_list) {
			struct  sockaddr_in *addr = (sockaddr_in *)addr_list->addr;
			struct sockaddr_ll *macc = (sockaddr_ll *)addr_list->addr;
			switch (addr->sin_family) {
				//besoin de get et comparer les ip et mac
				case AF_INET:
					this->ip_select = inet_ntoa(addr->sin_addr);
				case AF_PACKET:
					for (int i = 0; i < macc->sll_halen; i++) {
						std::cout << std::hex << std::setw(2) \
							<< std::setfill('0') \
							<< static_cast<int>(macc->sll_addr[i]);
						if (i != macc->sll_halen - 1)
							std::cout << ":";
					}
					if (0 < macc->sll_halen)
						std::cout << std::endl;
				
			}
			addr_list = addr_list->next;
		}
		std::cout << "next: "  << list->next << std::endl;
		list = list->next;
	}
	return true;	
}
