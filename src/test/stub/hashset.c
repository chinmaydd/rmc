// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#include <stdint.h>
#include <assert.h>
#include <stdlib.h>

/*
 * NOTE: Experimental Code
 * This file tries to use the idea of uninterpreted functions to implement the 
 * idea of HashSet

uint32_t __CPROVER_uninterpreted_f(uint32_t);

struct set {
	int counter;
};

struct set g_s = { 0 };

uint32_t ffi_insert(uint32_t value) {
	__CPROVER_assume(__CPROVER_uninterpreted_f(g_s.counter) == value);
	g_s.counter += 1;

	return 1;
}

uint32_t ffi_contains(uint32_t value) {
	__CPROVER_bool condition = 0;
	for (int i = 0; i < g_s.counter; i++) {
		condition = condition || (__CPROVER_uninterpreted_f(i) == value);
	}

	return condition;
}

uint32_t ffi_remove(uint32_t value) {
	// TODO: Implement this
	return 1;
}
*/

// The SENTINEL value is chosen to be outside the range of the hash function
// For this HashSet, the domain is uint32_t and the range is uint32_t. The 
// SENTINEL value is -1 which is in the range int32_t.
const SENTINEL = -1;

// This function needs to be reimplemented depending on the value being 
// stored in the hashset.
uint32_t hasher(uint32_t value) {
	return value;
}

// Here the domain is in the range of uint32_t but we store the hash value range
// as part of a larger domain.
typedef struct {
	int32_t* domain;
	// int32_t domain[UINT32_MAX];
	uint32_t counter;
} hashset;

hashset* ffi_new() {
	hashset* set = (hashset *) malloc(sizeof(hashset));
	/*
	 *  Ideally, we should be able to work with arrays of arbitrary size but
	 *  1. CBMC does not currently handle unbounded arrays correctly.
	 * 	  Issue: https://github.com/diffblue/cbmc/issues/6261
	 * 
	 *  2. With UINT32_MAX we get error "array too large for flattening"
	 * 
	 *  Currently working with 4096 as the size. NOTE: This is unsound.
	 * 
	 *  set->domain = calloc(UINT32_MAX, sizeof(int32_t));
	 */
	set->domain = calloc(4096, sizeof(int32_t));

	set->domain[0] = SENTINEL;

	return set;
}

uint32_t ffi_insert(hashset* s, uint32_t value) {
	uint32_t hash = hasher(value);

	if ((hash == 0 && s->domain[hash] != SENTINEL) || s->domain[hash] != 0) {
		return 0;
	}

	s->domain[hash] = value;
	return 1;
}

uint32_t ffi_contains(hashset* s, uint32_t value) {
	uint32_t hash = hasher(value);

	if ((hash == 0 && s->domain[hash] != SENTINEL) || s->domain[hash] != 0) {
		return 1;
	}

	return 0;
}

uint32_t ffi_remove(hashset* s, uint32_t value) {
	uint32_t hash = hasher(value);

	if ((hash == 0 && s->domain[hash] == SENTINEL) || s->domain[hash] == 0) {
		return 0;
	}

	if (hash == 0) {
		s->domain[hash] = SENTINEL;
	} else {
		s->domain[hash] = 0;
	}

	return 1;
}

// int main() {
// 	hashset* s = ffi_new();
// 	ffi_insert(s, 10);
// 	assert(ffi_contains(s, 10));
// 	ffi_remove(s, 10);
// 	assert(!ffi_contains(s, 10));
// }
