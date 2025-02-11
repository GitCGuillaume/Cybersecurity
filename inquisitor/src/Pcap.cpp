#include <Pcap.hpp>

Pcap::Pcap(const char *ip_src, const char *mac_src,
	  const char *ip_target, const char *mac_target) : _ip_src(ip_src), _mac_src(mac_src),
		_ip_target(ip_target), _mac_target(mac_target) {
	std::cout << "Pcap Created" <<std::endl;
	this->_ip_select = NULL;
	this->_mac_select = NULL;
	this->_pcap_list = NULL;
	this->_device_select = NULL;
	this->_device_capture = NULL;
	this->_fp = NULL;
}

/*
 * Clear pcap allocation
 */
Pcap::~Pcap() {
	if (this->_pcap_list)
		pcap_freealldevs(this->_pcap_list);
	if (this->_device_capture)
		pcap_close(this->_device_capture);
	if (this->_fp) {
		if (this->_fp->bf_insns)
			pcap_freecode(this->_fp);
		delete this->_fp;
	}
	std::cout << "Destroyed" <<std::endl;
}

struct pcap_if * Pcap::GetDevice() const {
	return this->_device_select;
}

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
static pcap_addr *search_address(struct pcap_addr *elem,
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
			return reinterpret_cast<sockaddr_ll *>(addr_find->addr);
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

void	Pcap::SetDeviceCapture(pcap_if_t *src) {
	char errbuf[PCAP_ERRBUF_SIZE] = {0};

	//if (!src)
	//	throw std::invalid_argument("Couldn't find a device to start capture.");
	this->_device_capture = pcap_create(src->name, errbuf);
	if (!this->_device_capture)
		throw std::runtime_error(errbuf);
}

int	Pcap::activateCapture(pcap_t *src) const {
	return pcap_activate(this->_device_capture);
}

int	Pcap::compileFilterArp(pcap_t *src)
{
	if (!src || !this->_device_select)
		return 1;
	struct  sockaddr_in *addr = reinterpret_cast<sockaddr_in *>(this->_device_select);

	if (!addr)
		return 1;
	in_addr_t ipv4 = addr->sin_addr.s_addr;
	if (!this->_fp)
		this->_fp = new struct bpf_program;
	if (!this->_fp)
		return 1;
	return pcap_compile(src, this->_fp, "arp or port ftp", 0, ipv4);
}

int	Pcap::setFilter(pcap_t *src, struct bpf_program *fp) const {
	return pcap_setfilter(src, fp);
}

//
static void handle_arp(){}

/*
 * Get tcp part from packet
 * Get file's name
 * Display file's name
 * Request: STOR | RETR
 */
static void handle_ftp(const u_char *bytes, bpf_u_int32 len){
	if (!bytes)
		return ;
	struct ethhdr *eth = (struct ethhdr *)bytes;
	bpf_u_int32 skip_size = sizeof(*eth);
	printf("sk:%u\n", skip_size);
	printf("B14: %x\n", bytes[skip_size]);
	struct iphdr *ip = (struct iphdr *)(bytes + skip_size);
	skip_size += (ip->ihl * 4);
	printf("sk:%u ihl:%u 0xf:%u 0xf0:%u *4:%u\n", skip_size, ip->ihl, ip->ihl & 0xf, ip->ihl & 0xf0, ip->ihl * 4);
	struct tcphdr *tcp = (struct tcphdr *)(bytes + skip_size);
	skip_size += (tcp->doff * 4);
	printf("sk:%u %u *4:%u\n", skip_size, tcp->doff, tcp->doff * 4);
	u_char *ftp_bytes = (u_char *)(bytes + skip_size);
	//enum pour choisir gogo
	if (ftp_bytes) {
		len -= skip_size;
		for (bpf_u_int32 i = 0; i < len; i++) {
			std::cout << ftp_bytes[i];
		}
		std::cout << std::endl;
	}
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
//	std::cout << std::hex << "*user:" << *user << std::endl;
//	std::cout << std::hex << "*bytes:" << *bytes << std::endl;
	for (bpf_u_int32 i = 0; i < h->len; i++) {
		//std::cout << std::hex << bytes[i] << std::endl;
		printf("%x ", bytes[i]);
	}
	printf("\n");
	const struct ether_header *eth = (const struct ether_header *)bytes;
	std::cout << std::hex << ntohs(eth->ether_type) << std::endl;
	if (eth && ntohs(eth->ether_type) == ETHERTYPE_ARP) { // ARP
	} else if (eth && ntohs(eth->ether_type) == ETHERTYPE_IP) { // FTP
		//get ftp file name
		//sizeof(*eth) + sizeof(iphdr) + sizeof(tcp)
		handle_ftp(bytes, h->len);
	}
	printf("\n");
}

int	Pcap::loopPcap(pcap_t *src, u_char *user) {
	//init arp table
	//comment je vais gérer enregistrer ça?
	int res = pcap_loop(src, INFINITE, handler, user);
	//restore arp table
	return res;
}
