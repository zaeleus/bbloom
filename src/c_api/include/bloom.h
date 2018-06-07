#ifndef __BLOOM_H__
#define __BLOOM_H__

#include <stdbool.h>
#include <stddef.h>

struct BloomFilter;

struct BloomFilter *bb_bloom_filter_from_fpp(double p, size_t n);
bool bb_bloom_filter_insert(struct BloomFilter *ctx, const char *key);
bool bb_bloom_filter_contains(struct BloomFilter *ctx, const char *key);
void bb_bloom_filter_free(struct BloomFilter *ctx);

#endif
