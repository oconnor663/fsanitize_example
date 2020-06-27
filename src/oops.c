#include "stdint.h"

const uint8_t* woops_local_array() {
    uint8_t local_array[6] = {3, 1, 4, 1, 5, 9};
    return local_array;
}
