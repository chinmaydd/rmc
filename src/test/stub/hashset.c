// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#include <stdint.h>

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
