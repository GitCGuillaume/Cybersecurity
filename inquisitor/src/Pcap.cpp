#include <Pcap.hpp>

Pcap::Pcap(const char *ip_src, const char *mac_src,
	  const char *ip_target, const char *mac_target) : _ip_src(ip_src), _mac_src(mac_src),
		_ip_target(ip_target), _mac_target(mac_target) {
	std::cout << "Pcap Created" <<std::endl;
	this->_ip_select = NULL;
	this->_mac_select = NULL;
	this->_pcap_list = NULL;
	this->_device_select = NULL;
}

/*
 * Clear pcap allocation
 */
Pcap::~Pcap() {
	if (this->_pcap_list)
		pcap_freealldevs(this->_pcap_list);
	//pcap_close(this->pcap);
	std::cout << "Destroyed" <<std::endl;
}

struct pcap_if * Pcap::GetDevice() const {
	return this->_device_select;
}

/*
 * Check the selected pcap_addr structure
 * if mac or ip address found, return the selected structure,
 * otherwise return NULL
 */
static pcap_addr *search_address(struct pcap_addr *elem,
		std::string &ip_src, const struct ether_addr *mac_src) {
	struct  sockaddr_in *addr = (sockaddr_in *)elem->addr;
	struct sockaddr_ll *macc = (sockaddr_ll *)elem->addr;
	char *ip = NULL;


	switch (addr->sin_family) {
		case AF_INET:
			ip = inet_ntoa(addr->sin_addr);
			if (ip && ip_src  == std::string(ip)) {
			       return elem;	
			}
			break ;
		case AF_PACKET:
			if (macc->sll_halen == 6) {
				int i = 0;

				for (; i < macc->sll_halen; i++) {
					if (mac_src->ether_addr_octet[i] != macc->sll_addr[i]) {
						return NULL;
					}
				}
				return elem;
			}
	}
	return NULL;
}

/*
 * Search for AF_INET type address
 */
static struct sockaddr_in *get_afinet(pcap_addr *addr_find) {
	while (addr_find) {
		if (addr_find->addr->sa_family == AF_INET) {
			return (sockaddr_in *)addr_find->addr;
		}
		addr_find = addr_find->next;
	}
	return NULL;
}

/*
 * Search for AF_PACKET type address
 */
static struct sockaddr_ll *get_afpacket(pcap_addr *addr_find) {
	while (addr_find) {
		if (addr_find->addr->sa_family == AF_PACKET) {
			return (sockaddr_ll *)addr_find->addr;
		}
		addr_find = addr_find->next;
	}
	return NULL;
}

bool Pcap::SetPcapList(void) {
	if (!this->_ip_src.length() || !this->_mac_src.length())
		return false;
	char errbuf[PCAP_ERRBUF_SIZE] = {0};
	const struct ether_addr *mac_src = ether_aton(this->_mac_src.c_str());
	int res = pcap_findalldevs(&this->_pcap_list, errbuf);

	if (res)
		return false;
	pcap_if_t *list_search = this->_pcap_list;
	//pcap_if_t *list_compare = this->pcap_list;
	pcap_addr *addr_list = NULL;
	pcap_addr *addr_find = NULL;
	
	while (list_search) {
		addr_list = list_search->addresses;
		while (addr_list) {
			addr_find = search_address(addr_list, this->_ip_src, mac_src);
			if (addr_find)
				break ;
			addr_list = addr_list->next;
		}
		list_search = list_search->next;
	}
	bool find_ip = false;
	bool find_mac = false;

	if (addr_find) {
		const struct  sockaddr_in *addr = get_afinet(addr_find);
		const struct sockaddr_ll *macc = get_afpacket(addr_find);

		if (addr) {
			const char *ip = inet_ntoa(addr->sin_addr);

			if (ip && this->_ip_src  == std::string(ip)) {
				this->_ip_select = &this->_ip_src;
				find_ip = true;
			}
		}
		if (macc) {
			if (macc->sll_halen == 6) {
				int i = 0;

				for (; i < macc->sll_halen; i++) {
					if (mac_src->ether_addr_octet[i] != macc->sll_addr[i]) {
						break ;
					}
				}
				if (i == macc->sll_halen) {
					find_mac = true;
					this->_mac_select = &this->_mac_src;
				}
			}
		}
	}
	if (!find_mac || !find_ip)
		return false;
	if (this->_ip_select && this->_mac_select
		&& *this->_ip_select == this->_ip_src
		&& *this->_mac_select == this->_mac_src) {
		std::cout << "return true" << std::endl;
		std::cout <<
		"i_sel: " << *this->_ip_select << "\n"
		<< "m_sel: " << *this->_mac_select << std::endl;
		this->_device_select = list_search;
		return true;
	}
	return false;
}

void	Pcap::SetDeviceCapture(pcap_if_t *src) {
	char errbuf[PCAP_ERRBUF_SIZE] = {0};
	// if src empty then throw
	if (!src)
		throw std::invalid_argument("Couldn't find a device to start capture.");
	this->_device_capture = pcap_create(src->name, errbuf);
	if (!this->_device_capture)
		throw std::runtime_error("Failed to start capture.");
	//if null throw errbuf val
}
