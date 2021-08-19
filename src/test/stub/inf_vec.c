// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// NOTE: EXPERIMENTAL CODE
// This file tries to use the idea of unbounded arrays to implement
// a Vector abstraction

#include <stdint.h>
#include <assert.h>
#include <stdlib.h>

typedef struct {
	size_t len;
	size_t capacity;
	uint32_t* mem;
} vec;

vec* ffi_new() {
	vec* v = (vec *) malloc(sizeof(vec));
	v->len = 0;
	v->capacity = __CPROVER_constant_infinity_uint;
	v->mem = (uint32_t *) malloc(__CPROVER_constant_infinity_uint);
	return v;
}

void ffi_push(vec* v, uint32_t elem) {
	v->mem[v->len] = elem;
	v->len += 1;
}

vec* ffi_with_capacity(size_t capacity) {
	vec *v = (vec *) malloc(sizeof(vec));
	v->len = 0;
	v->capacity = capacity;
	return v;
}

uint32_t ffi_pop(vec* v) {
	assert(v->len > 0);
	v->len -= 1;
	return v->mem[v->len];
}

void ffi_append(vec* v1, vec* v2) {
}

void ffi_insert(vec* v, size_t index, uint32_t elem) {
}

uint32_t ffi_len(vec* v) {
}

uint32_t ffi_cap(vec* v) {
}

// int main() {
// 	vec* v = ffi_new();
// 	ffi_push(v, 5);
// 	ffi_push(v, 5);
// 	ffi_push(v, 5);
// 	ffi_push(v, 5);
// 	ffi_push(v, 5);
// 	assert(v->len == 5);
// 	vec* n = ffi_new();
// 	__CPROVER_assume(!__CPROVER_same_object(v, n));
// }
