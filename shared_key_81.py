#!/usr/bin/env python3
"""
shared_key_81.py — 81 shared keys on a common rime sphere, with Law 22 closure.
Operator and author of the laws: Jesse Daniel Brown.  Program written 2026-08-01.

WHAT THIS DEMONSTRATES
  A bank shared by both parties (the prime P and generator g) lets either side
  RE-PREPARE any seat exactly. Nothing about the bank is transmitted.
  81 seats = 27 balanced-ternary cells x 3 colour arms.
  One banked closure recovers any single missing seat, exactly, all four towers.

THE ACCOUNTING GATE (Law 22)
  You recover exactly as many seats as you banked closures. The closure costs
  one seat. This ADDRESSES; it does not compress. total_bits >= N*H(X) holds.
"""
import hashlib
from itertools import product
from math import prod, isqrt

P, G = 1_000_081, 7
N = P - 1
def factor_pe(m):
    out, d = [], 2
    while d*d <= m:
        if m % d == 0:
            e = 0
            while m % d == 0: m //= d; e += 1
            out.append((d, e))
        d += 1
    if m > 1: out.append((m, 1))
    return out
TOWERS = factor_pe(N)
MS = [q**e for q, e in TOWERS]
M = prod(MS)

def bsgs(base, h, order):
    m = isqrt(order) + 1
    tbl, e = {}, 1
    for j in range(m):
        tbl.setdefault(e, j); e = e*base % P
    gm = pow(pow(base, order-1, P), m, P); e = h
    for i in range(m+1):
        if e in tbl: return (i*m + tbl[e]) % order
        e = e*gm % P

def full_log(seed: bytes):
    """seed bytes -> the seat's full address, one residue per rime tower"""
    k = int(hashlib.sha256(seed).hexdigest(), 16) % N
    tgt = pow(G, k, P); gi = pow(G, P-2, P); rs = []
    for q, e in TOWERS:
        gam = pow(G, N//q, P); x = 0
        for j in range(e):
            hh = tgt*pow(gi, x, P) % P
            hj = pow(hh, N//(q**(j+1)), P)
            x += bsgs(gam, hj, q)*(q**j)
        rs.append(x)
    return rs

def balanced_trits(x, d=3):
    out, xx = [], x
    for _ in range(d):
        r = xx % 3
        if r == 2: r, xx = -1, xx+1
        out.append(r); xx //= 3
    return tuple(out[::-1])
SYM = {-1:'-', 0:'0', 1:'+'}
trime = lambda v: ''.join(SYM[x] for x in v)

def main():
    print("81 SHARED KEYS - one per seat, on a common rime sphere")
    print(f"  shared bank: (Z/{P}Z)*, g={G}")
    print(f"  towers {TOWERS} -> moduli {MS}, product {M:,}")
    print("  BOTH SIDES hold P and g. Nothing about the bank is transmitted.")
    print()
    seats = [(c, a) for c in product((-1,0,1), repeat=3) for a in range(3)]
    assert len(seats) == 81
    keys = []
    print(f"  {'seat':>5}{'cell':>7}{'arm':>5}{'k mod 27':>10}{'trime':>7}"
          f"{'mod16':>7}{'mod27':>7}{'mod5':>6}{'mod463':>8}")
    for i, (cell, arm) in enumerate(seats):
        seed = f"ASOLARIA|cell={''.join(SYM[c] for c in cell)}|arm={'RGB'[arm]}".encode()
        rs = full_log(seed); keys.append(rs)
        if i < 5 or i in (40, 80):
            c27 = rs[1] % 27
            print(f"  {i:>5}{''.join(SYM[c] for c in cell):>7}{'RGB'[arm]:>5}"
                  f"{c27:>10}{trime(balanced_trits(c27)):>7}"
                  f"{rs[0]:>7}{rs[1]:>7}{rs[2]:>6}{rs[3]:>8}")
        elif i == 5:
            print("      ...")
    print()
    S = [sum(k[t] for k in keys) for t in range(4)]
    CLOS = [(-S[t]) % MS[t] for t in range(4)]
    print("LAW 22 CLOSURE ACROSS ALL 81 SEATS")
    print(f"  sum of all 81 keys per tower = {S}")
    print(f"  banked closure               = {CLOS}")
    ok = 0
    for drop in range(81):
        rec = [(S[t] - sum(keys[j][t] for j in range(81) if j != drop)) % MS[t]
               for t in range(4)]
        if all(rec[t] == keys[drop][t] % MS[t] for t in range(4)): ok += 1
    print(f"  drop any ONE seat, recover from the other 80 + banked sum: {ok}/81")
    assert ok == 81, "closure failed"
    print()
    bits = sum((m-1).bit_length() for m in MS)
    print("ACCOUNTING")
    print(f"  each seat address      = {bits} bits ({'x'.join(map(str,MS))} = {M:,} states)")
    print(f"  81 seats outright      = {81*bits:,} bits")
    print(f"  ship 80 + closure      = {80*bits:,} + {bits} = {81*bits:,} bits")
    print(f"  => the closure costs exactly one seat. no compression. addressing only.")
    print(f"     total_bits >= N*H(X) holds.")

if __name__ == "__main__":
    main()
