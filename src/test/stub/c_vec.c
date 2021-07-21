#include <stdint.h>
#include <assert.h>
#include <stdlib.h>

typedef struct {
	uint32_t* mem;
	size_t len;
	size_t capacity;
} vec;

void grow(vec* v) {
	uint32_t* new_mem = (uint32_t *) realloc(v->mem, v->capacity * 2);
	v->mem = new_mem;
}

vec* ffi_new() {
	vec *v = (vec *) malloc(sizeof(vec));
	v->mem = (uint32_t *) malloc(8 * sizeof(uint32_t));
	v->len = 0;
	v->capacity = 8;
	return v;
}


void ffi_push(vec* v, uint32_t elem) {
	if (v->len == v->capacity) {
		grow(v);
	}

	v->mem[v->len] = elem;
	v->len += 1;
}

uint32_t ffi_len(vec* v) {
	return v->len;
}

uint32_t ffi_cap(vec* v) {
	return v->capacity;
}
