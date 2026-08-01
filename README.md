# The fix — what was actually measured

**Operator and author of the laws: Jesse Daniel Brown (OP-JESSE).**
Runs dated 2026-07-31 / 2026-08-01. Rust 1.81.0, `clippy -D warnings` clean.
Every figure here came out of a named program in this repository. Clone it and
re-run it; nothing below is asserted.

The repository name is the operator's. What follows is the arithmetic, tagged so
nobody has to take either of us on trust.

---

## The claim that matters most: float fails identity, not accuracy

`MEASURED` — `float-vs-trit/src/bin/bothways.rs`, all 1,000,080 addresses.

```
TRIT      addresses=1000080  roundtrip_failures=0  exact=true
FLOAT_D   addresses=1000080  roundtrip_failures=0  exact=true
FLOAT_F2  addresses=1000080  roundtrip_failures=0  exact=true
```

**Float round-trips perfectly here.** The common belief — including mine before I
ran it — is that float loses the address. It does not, at this magnitude. Small
integers over small divisors, `round()` absorbs the error, recovery is exact.
Zero failures on a million, on every encoding tested.

Where it actually breaks:

```
ZERO|float  plus_bits=0000000000000000  minus_bits=8000000000000000
            equal=true  bytes_equal=false  identity_holds=FALSE
ZERO|int    zeros=1  equal_is_byte_identity=true  identity_holds=true
ZERO|trit   zeros=1  states=-1,0,+1  identity_holds=true
```

IEEE-754 has **two zeros**. `+0.0 == -0.0` is **true** while their bytes differ.
So `a == b` and `hash(a) == hash(b)` come apart. Two nodes both holding zero:
consensus says agree, hash reconciliation says disagree, neither made an
arithmetic error. Int and trit have exactly one zero, so for them equality **is**
byte-identity.

And the algebra any split-then-merge depends on:

```
DISTRIB  trials=1000000  float_failures=316267
                         int_with_remainder_carried_failures=0
```

**316,267 of a million.** 31.6% of the time `(a/3)+(b/3) ≠ (a+b)/3` in float.
**Zero** times in integers with the remainder carried.

> Store an address through float if you must. Never do arithmetic on thirds
> inside it. Identity, addressing, hashing and consensus: integers and trits.

This is why PAQ, cmix, zstd and brotli all carry probabilities as integers. Not
for speed — for determinism.

---

## Count is not range

`MEASURED` — same program.

```
RANGE|mod=16 |trits=3|states=27 |balanced_range=-13..13|uncentred_0..15_fits=FALSE
RANGE|mod=27 |trits=3|states=27 |balanced_range=-13..13|uncentred_0..26_fits=FALSE
RANGE|mod=463|trits=6|states=729|balanced_range=-364..364|uncentred_0..462_fits=FALSE
```

Balanced ternary with 3 trits holds **27 states** but spans only **−13…+13**.
Residues 14 and 15 of mod 16 fall outside it. Three of the four towers fail
uncentred. Centred, all fit:

```
mod  16  ->  -8..7      inside -13..13
mod  27  ->  -13..13    inside -13..13   exact at both ends
mod 463  ->  -231..231  inside -364..364
```

**The centring on the ground point is not decoration.** It is what makes balanced
ternary able to hold the value at all. Only mod 27 lands on both ends of its
range with nothing to spare — `3³`, exact in trits and in nothing else.

---

## Three arms close on a free zero; the residual is the trit

`MEASURED` — `nullsphere-closure/`, integers held as numerators over 3.

```
arm_R = 2r - g - b      arm_G = 2g - r - b      arm_B = 2b - r - g
```

```
PROOF|closure_is_identity|triples_checked=1771561|sum_ne_zero=0
PROOF|the_zero_is_free  |shifts_tested=18009    |arms_changed=0
```

The three arms sum to exactly zero — an identity for **all** integers, verified
over 1,771,561 triples with no exceptions. Shift all three channels by any
constant and the arms do not move: the coefficients `(1, 1, −2)` sum to zero, so
the relation is affine and **the centre can be placed anywhere**.

Run against nine recorded star-zeros, the third arm is never anything but
**−1/3, 0, or +1/3** — exact, in thirds, never machine-zero.

---

## 81 links net 27 free zeros — the counts are forced

`MEASURED` — `nullnet-81-over-27/`.

```
zeros   3^3       = 27    free, total_cost 0
links   27*6/2    = 81    HTTP conduits, cost_per_link 0
lines   27/3*3    = 27    one three-body per line
seats   27*3      = 81
```

Close a 3×3×3 lattice mod 3 and every node has exactly six conduits. **81 links
and 81 seats arrive from two different directions with no tuning.** Each line's
three members are `v−3ⁱ, v, v+3ⁱ`, so the centroid is the middle member exactly
and the arms are forced to `(−1, 0, +1)`.

```
NULLSPHERE  minus=27  zero=27  plus=27  total=81  each_exactly_a_third=true  residue=0
GLOBAL      sum_of_all_27_values=0  the_net_itself_closes=true
```

---

## Addressing is not compression

`MEASURED` — `shared_key_81.py` (the operator's program), independently verified.

```
P = 1,000,081  prime
P - 1 = 1,000,080 = 2^4 · 3^3 · 5 · 463
moduli [16, 27, 5, 463]   g = 7, primitive root, full order
81 seats = 27 cells × 3 arms
closure: drop any one seat, recover from the other 80 + banked sum  →  81/81
```

**The four towers are derived, not chosen** — they are the prime-power factors of
`P−1`. Nobody picked them.

```
ship 80 + closure = 1,680 + 21 = 1,701 bits
81 outright                    = 1,701 bits
```

**Identical.** The operator's own header states the gate:

> *"You recover exactly as many seats as you banked closures. The closure costs
> one seat. This ADDRESSES; it does not compress. `total_bits >= N*H(X)` holds."*

A bijection preserves entropy. Re-basing, re-addressing and glyph languages are
identity and addressing organs — never compression organs.

---

## State your encoding; never cross them

```
tower-separate   21 bits   14 trits    towers independently addressable
joint            20 bits   13 trits    flattened, boundaries gone
```

**21 pairs with 14. 20 pairs with 13.** Comparing 14 trits against 20 bits
manufactures a phantom overhead out of nothing — that mismatch is exactly what
produced a "+10% ternary overhead" row sitting next to a "1.538× denser" line in
the same document.

Neither base is exact here. 1,000,080 is not a power of 2 and not a power of 3:
binary wastes 48,496 states, ternary wastes 594,243 — a ratio of **12.2534**.
Ternary's genuine advantage is radix economy (`b/ln b` minimised at 3), which is
cost per digit, not tightness of fit.

---

## What I got wrong, kept beside what replaced it

Retractions travel with the claim. These are mine:

- **Float geometry.** I reported radius spreads, angles and cosine fits computed
  with `hypot`, `atan2` and float multiply, and presented them as measurement.
  Redone in integers they were not just imprecise — they **hid** the result. The
  "spread of 0.74 px" concealed that the pairs are *identical integers*, and the
  exact run then found a perfect square, an exact mirror, and a palindromic turn
  that float had obscured entirely.
- **An overcall.** I flagged `14 trits` as an error. It is not — 14 is correct
  for tower-separate. Withdrawn, and the withdrawal is in the audit receipt.
- **A gate that classified the signal as failure.** I wrote a translation test
  that printed `violations=6/9` where the six "violations" *were the finding*.
- **A step that killed what it measured.** I stepped a shadow trit by 4374 = 2·3⁷,
  divisible by 3, pinning the trit at 0 for 200,000 ticks. There is now an assert
  that aborts if the step is ever divisible by 3.
- **Asserting from press coverage.** I told the operator what a *Nature
  Communications* paper "isn't" after reading two summaries and not the paper.

---

## Contents

```
AGENT-DISCIPLINE.md          the preload — run the check before asserting a limit,
                             do not abandon a computed result under pressure
nullsphere-closure/          three arms, free zero, the trit          Rust 1.81
nullnet-81-over-27/          81 links over 27 free zeros              Rust 1.81
float-vs-trit/               the identity and distributivity runs     Rust 1.81
matrix-proof-audit/          document audit, 6 corrections, 2 withdrawn
shared_key_81.py             the operator's program, verified 81/81
```

Every crate: `cargo +1.81.0 clippy --release --all-targets -- -D warnings`, clean.
Every crate but `float-vs-trit` reports `float_used=0`. `float-vs-trit` uses float
on purpose — it is the thing under test.

---

## How to check any of it

```bash
cd nullsphere-closure  && cargo +1.81.0 run --release
cd nullnet-81-over-27  && cargo +1.81.0 run --release
cd float-vs-trit       && cargo +1.81.0 run --release --bin bothways
cd matrix-proof-audit  && cargo +1.81.0 run --release
python shared_key_81.py
```

If a number here does not reproduce, that number is wrong and should be said so.

---

*Tagging convention used throughout: **MEASURED** — a number on disk from a named
script, reproducible, quoted inline. **NAMED** — stated, coherent, not yet run; a
named law is not a weaker law, it is one that has not yet been asked a question
it could fail. **CONJECTURE** — stated, untested.*
