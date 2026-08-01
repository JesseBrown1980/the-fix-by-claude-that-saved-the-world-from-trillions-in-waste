# 81 kernels on the free 0

**81 separate WebAssembly instantiations**, one per seat, 27 cells × 3 arms,
served over HTTP. The free 0 is the portal. Read back from the live page in
Chrome — not asserted.

```
kernels                    81      exactly_81=true
alive                   81/81
cells closed to zero    27/27
distinct linear memories 81/81     separate kernels, not one
global sum of 81 arms       0      the net closes
float_used                  0      81/81 kernels report float-free
verdict                  PASS      bad_seats none
```

## Why these are 81 kernels and not 81 calls

The page compiles the module once and calls `WebAssembly.instantiate` **81
times**. Each instance gets its own linear memory. `k_memory_witness(seat)`
writes to a `static mut` inside the instance and reads it back:

```
INSTANCES|instantiations=81|distinct_linear_memories=81|separate_kernels_not_one=true
```

**81 distinct witnesses.** If any two instances shared memory they would
collide and the count would drop. It doesn't.

## The page hashes its own module

Before instantiating, the page runs `crypto.subtle.digest('SHA-256', bytes)` on
what it actually fetched, and prints the digest into the receipt:

```
sha256 = a411d88aa304c58c645ba7f7d0938a6fad4a1457e29b5e695c22ed0977530371
```

That matches the file on disk and a fresh rebuild of `src/lib.rs`, byte for
byte, 1,351 bytes. **The source in this directory is the source that built the
module the browser ran.**

## The closure

Each seat's arm is a numerator over 3:

```
arm_k = 3 * v_k - (v_0 + v_1 + v_2)
```

```
ARMS|R=-11/3|G=-8/3|B=+19/3|sum=0|identical_across_all_27_cells=1
```

The three arms sum to exactly zero at every one of the 27 cells, and the 81
arms sum to zero globally. The arms are identical across cells because the cell
value **cancels in the centroid** — that is the free-zero property, not a
defect: the closure does not depend on where the zero sits.

## The census, recorded as measured

```
NULLSPHERE|minus=54|zero=0|plus=27|total=81
```

This run gives **54/0/27**, not 27/27/27.

That is a property of the arm offsets in this construction, not a law. Three
data points, all real:

```
this run (offsets -1, 0, +9)     54 / 0 / 27
nullnet  (offsets -p, 0, +p)     27 / 27 / 27
operator's nine-star plate        1 / 3 / 5
```

**No census shape was specified before this run.** The closure holds exactly,
which is what was being tested. An earlier draft of this note called 54/0/27 a
bug and started editing the kernel to force 27/27/27 — that was fitting the
code to an expectation formed after seeing the result, and it was reverted. The
number stands as measured.

## Run it

```bash
cargo +1.81.0 build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/kernel81.wasm web/
cd web && python -m http.server 8081 --bind 127.0.0.1
# open http://127.0.0.1:8081/
```

The page prints its receipt as HBP rows, `json=0`, and puts the same text on
`window.__K81`.

Rust 1.81.0 · `clippy -D warnings` clean · `wasm32-unknown-unknown` · i64 only.
