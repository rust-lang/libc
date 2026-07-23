#include <linux/netlink.h>
#include <stddef.h>

// Since the NLMSG_* helpers are macros instead of functions, they aren't
// available to FFI. libc must reimplement them, which is error-prone. This
// file provides FFI access to the actual macros so they can be tested against
// the Rust reimplementation.

int nlmsg_align_ffi(size_t size) {
	return NLMSG_ALIGN(size);
}

int nlmsg_length_ffi(size_t size) {
	return NLMSG_LENGTH(size);
}

int nlmsg_space_ffi(size_t size) {
	return NLMSG_SPACE(size);
}

void *nlmsg_data_ffi(struct nlmsghdr *nlh) {
	return NLMSG_DATA(nlh);
}

struct nlmsghdr *nlmsg_next_ffi(struct nlmsghdr *nlh, int *size) {
	return NLMSG_NEXT(nlh, *size);
}

int nlmsg_ok_ffi(struct nlmsghdr *nlh, int size) {
	return NLMSG_OK(nlh, size);
}

int nlmsg_payload_ffi(struct nlmsghdr *nlh, int size) {
	return NLMSG_PAYLOAD(nlh, size);
}
