#include <time.h>
#include <linux/errqueue.h>

// SO_EE_OFFENDER is defined as a macro in linux/errqueue.h.  This file wraps
// that macro in a function so we can test the reimplementation in this package
// is equivalent.

struct sockaddr *so_ee_offender(struct sock_extended_err *ee) {
  return SO_EE_OFFENDER(ee);
}
