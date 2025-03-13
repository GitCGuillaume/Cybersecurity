#ifndef FT_MACROS
# define FT_MACROS

#define INFINITE 0
#define TWO_CAPTURE 2
#define FT_IPV4_LEN 4

#include <vector>
#include <thread>

struct s_thread {
	std::thread thread;
	char *cstr;
};

extern int g_free_arp;
extern pcap_t *g_device;

#endif
