#include <sys/types.h>

#include <sys/attr.h>
#include <sys/socket.h>
#include <sys/socketvar.h>
#include <sys/sysctl.h>
#include <stdbool.h>

#include <arpa/inet.h>
#include <netinet/in.h>
#include <netinet/tcp.h>

#include <netinet/in_pcb.h>
#include <netinet/tcp_timer.h>
#include <netinet/tcp_var.h>

#define TCPSTATES
#include <netinet/tcp_fsm.h>

#include <stdio.h>
#include <stddef.h>
#include <stdlib.h>

/* From the private xnu-2782.40.9/bsd/netinet/tcp_var.h lines 622-636: */
/*
 * The rtt measured is in milliseconds as the timestamp granularity is 
 * a millisecond. The smoothed round-trip time and estimated variance
 * are stored as fixed point numbers scaled by the values below.
 * For convenience, these scales are also used in smoothing the average
 * (smoothed = (1/scale)sample + ((scale-1)/scale)smoothed).
 * With these scales, srtt has 5 bits to the right of the binary point,
 * and thus an "ALPHA" of 0.875.  rttvar has 4 bits to the right of the
 * binary point, and is smoothed with an ALPHA of 0.75.
 */
#define TCP_RTT_SCALE           32      /* multiplier for srtt; 3 bits frac. */
#define TCP_RTT_SHIFT           5       /* shift for srtt; 5 bits frac. */
#define TCP_RTTVAR_SCALE        16      /* multiplier for rttvar; 4 bits */
#define TCP_RTTVAR_SHIFT        4       /* shift for rttvar; 4 bits */
#define TCP_DELTA_SHIFT         2       /* see tcp_input.c */


#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>



/*
    
    af_flag 
    -------
    // 0 -> IPv4
    // 1 -> IPv6
    // 2 -> Both
    
    pr_flag 
    -------
    // 0 -> TCP
    // 1 -> UDP
    // 2 -> Both

*/

char* get_connections (uint8_t pr_flag, uint8_t af_flag) {
  
  int rv;
  size_t len;
  struct xtcpcb *pcblist, *p;
  unsigned int buffer_size_per_item = 100;

  bool is_ipv4 = af_flag == 0 || af_flag == 2;
  bool is_tcp  = pr_flag == 0 || pr_flag == 2;


  rv = sysctlbyname("net.inet.tcp.pcblist",
		    NULL, &len, NULL, 0);
  if (rv < 0) {
    perror("sysctl");
  }
  p = pcblist = malloc(len);
  rv = sysctlbyname("net.inet.tcp.pcblist",
		    pcblist, &len, NULL, 0);
  if (rv < 0) {
    perror("sysctl");
  }

  char laddr[25], faddr[25];

  char *ret = (char *)malloc(buffer_size_per_item);
  if (!ret) {
    return NULL;
  }

  unsigned int idx = 0;
  size_t current_length = 0;

  for (;
       (char*)p < (char*)pcblist+len;
       p = (struct xtcpcb*)((char*)p + p->xt_len)) {

        //printf("\nfamily: %d protocol: %d",p->xt_socket.xso_family,
        //p->xt_socket.xso_protocol);
    
    if ( ((is_ipv4 && p->xt_socket.xso_family == AF_INET) ||
          (!is_ipv4 && p->xt_socket.xso_family == AF_INET6)) &&
	    ((is_tcp && p->xt_socket.xso_protocol == IPPROTO_TCP) ||
        (!is_tcp && p->xt_socket.xso_protocol == IPPROTO_UDP))) {

      struct in_addr *lia = &p->xt_inp.inp_laddr;
      struct in_addr *fia = &p->xt_inp.inp_faddr;
	
#define INADDR_LOCALHOST 0x7f000001

      int lip = ntohl(lia->s_addr);

      if (lip == INADDR_LOCALHOST ||
	  lip == INADDR_ANY
	  ) { continue; }


      sprintf(laddr, "%s\t%d", inet_ntoa(*lia), ntohs(p->xt_inp.inp_lport));
      sprintf(faddr,"%s\t%d", inet_ntoa(*fia),  ntohs(p->xt_inp.inp_fport));

      idx += 1;

      char* ret_resized = (char*)realloc(ret, buffer_size_per_item*idx);
      if (!ret_resized) {
          free(ret_resized);
          return NULL; // Return NULL if realloc fails
      }

      ret = ret_resized;

      int written = snprintf(ret + current_length, (buffer_size_per_item*idx)-current_length,
       "%s\t%s\t%s\t",
	     laddr, faddr, 
	     tcpstates[p->xt_tp.t_state]);
      current_length += written;

    }

  }

  return ret;
}

void free_get_connections(char* str) {
    if (str) {
        free(str);
    }
}