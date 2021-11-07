
#include <stdint.h>

uint32_t arr[8] = { 0, 1, 2, 3, 4, 5, 6, 7 };

uint32_t _start() {
	uint32_t res = 0;
	for (int i = 0; i < 8; i++) {
		res = res + arr[i];
	}
	return res;
}

