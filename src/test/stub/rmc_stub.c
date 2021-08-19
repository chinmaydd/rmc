// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#include <stdint.h>
#include <assert.h>
#include <stdlib.h>

// PoC code which implements a Vector stub
// Functions in this module are exported as FFI to work with c_vec.rs which 
// acts like the Rust frontend.

// A Vector has a pointer to a memory allocation, length and capacity.
typedef struct {
	uint32_t* mem;
	size_t len;
	size_t capacity;
} vec;

// The grow operation resizes the vector and copies its original contents into 
// a new allocation. This implementation doubles the capacity, but in theory
// it could be implemented as a sized_grow() which takes in a new size.
void grow(vec* v) {
	uint32_t* new_mem = (uint32_t *) realloc(v->mem, v->capacity * 2 * sizeof(uint32_t));
	v->mem = new_mem;
	v->capacity = v->capacity * 2;
}

vec* ffi_new() {
	vec *v = (vec *) malloc(sizeof(vec));
	// Default size is 8
	v->mem = (uint32_t *) malloc(8 * sizeof(uint32_t));
	v->len = 0;
	v->capacity = 8;
	return v;
}

vec* ffi_with_capacity(size_t capacity) {
	vec *v = (vec *) malloc(sizeof(vec));
	v->mem = (uint32_t *) malloc(capacity * sizeof(uint32_t));
	v->len = 0;
	v->capacity = capacity;
	return v;
}

void ffi_push(vec* v, uint32_t elem) {
	// If we have already reached capacity, resize the vector before
	// pushing in new elements
	if (v->len == v->capacity) {
		grow(v);
	}

	v->mem[v->len] = elem;
	v->len += 1;
}

uint32_t ffi_pop(vec* v) {
	assert(v->len > 0);
	v->len -= 1;
	return v->mem[v->len];
}

// Takes in a new vector and appends its elements to the current one.
void ffi_append(vec* v1, vec* v2) {
	size_t i = 0;
	for (i = 0; i < v2->len; i++) {
		ffi_push(v1, v2->mem[i]);
	}
	v1->len = v1->len + v2->len;
}

void ffi_insert(vec* v, size_t index, uint32_t elem) {
	// TODO
}

uint32_t ffi_len(vec* v) {
	return v->len;
}

uint32_t ffi_cap(vec* v) {
	return v->capacity;
}
