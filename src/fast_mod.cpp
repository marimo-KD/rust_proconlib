#include <cstdio>
#include <iostream>

using i64 = long long;
using u32 = unsigned;
using u64 = unsigned long long;

// 1 でも動作する
struct FastDiv {
  FastDiv() {}
  FastDiv(u64 n) : m(n) {
    s = (n == 1) ? 0 : 64 + std::__lg(n - 1);
    x = ((__uint128_t(1) << s) + n - 1) / n;
  }
  friend u64 operator / (u64 n, FastDiv d) { return __uint128_t(n) * d.x >> d.s; }
  friend u64 operator % (u64 n, FastDiv d) { return n - n / d * d.m; }
  u64 m, x; int s;
};

inline u64 mod64_32_small(u64 a, u32 b) {
  u32 q, r;
  __asm__ (
    "divl\t%4"
    : "=a"(q), "=d"(r)
    : "0"(u32(a)), "1"(u32(a >> 32)), "rm"(b)
  );
  return r;
}

int fact_slow(int n, int mod) {
  int ret = 1;
  for (int i = 1; i <= n; ++i) ret = i64(ret) * i % mod;
  return ret;
}

int fact_slow_asm(int n, int mod) {
  int ret = 1;
  for (int i = 1; i <= n; ++i) ret = mod64_32_small(u64(ret) * i, mod);
  return ret;
}

int fact_fast(int n, int mod) {
  auto fd = FastDiv(mod);
  std::cout << "s:" << fd.s << " x:" << fd.x << std::endl;
  int ret = 1;
  for (int i = 1; i <= n; ++i) ret = i64(ret) * i % fd;
  return ret;
}

int main() {
  int mod = 1000000007;
  int a3 = fact_fast(mod - 1, 1000000);
  return 0;
}
