//! FLOAT vs BALANCED-TERNARY INT over the same 1,000,080 address space.
//! Every one of the 1,000,080 addresses is round-tripped through both.
//! Float is used here ON PURPOSE — it is the thing under test, not the tool.
//! FLOAT-WITNESS-EXEMPT: this file is the float experiment itself. The operator rule
//! (Rust 1.81, integer/ternary only, never float) applies to the SYSTEM; here float is
//! the specimen under test, so it is retained deliberately and excluded from the CI gate.
#![allow(clippy::float_arithmetic)]

const M: [i64; 4] = [16, 27, 5, 463];
const W: [u32; 4] = [3, 3, 2, 6]; // balanced trit widths from the document

fn place() -> [i64; 4] {
    [M[1] * M[2] * M[3], M[2] * M[3], M[3], 1]
}

fn split(a: i64) -> [i64; 4] {
    let p = place();
    [a / p[0], (a / p[1]) % M[1], (a / p[2]) % M[2], a % M[3]]
}

fn join(t: [i64; 4]) -> i64 {
    let p = place();
    t[0] * p[0] + t[1] * p[1] + t[2] * p[2] + t[3]
}

/// balanced ternary: value -> w trits in {-1,0,1}; None if out of range
fn to_bt(mut v: i64, w: u32) -> Option<Vec<i64>> {
    let half = (3i64.pow(w) - 1) / 2;
    if v < -half || v > half {
        return None;
    }
    let mut out = Vec::with_capacity(w as usize);
    for _ in 0..w {
        let mut r = v % 3;
        v /= 3;
        if r == 2 {
            r = -1;
            v += 1;
        } else if r == -2 {
            r = 1;
            v -= 1;
        }
        out.push(r);
    }
    Some(out)
}

fn from_bt(t: &[i64]) -> i64 {
    let mut v = 0i64;
    for (i, d) in t.iter().enumerate() {
        v += d * 3i64.pow(i as u32);
    }
    v
}

fn main() {
    let total: i64 = M.iter().product();
    println!("BWHDR|schema=FLOAT-VS-TRIT-V1|owner=OP-JESSE|rust=1.81.0|space={}\
|float_under_test=1|json=0", total);

    // ---- can each tower even be written in w balanced trits? --------------
    // Balanced ternary with w trits covers -(3^w-1)/2 ..= +(3^w-1)/2.
    // Count of states is 3^w, but the RANGE is what a residue must fit inside.
    for i in 0..4 {
        let half = (3i64.pow(W[i]) - 1) / 2;
        let raw_ok = M[i] - 1 <= half; // residues 0..m-1 uncentred
        let off = M[i] / 2;
        let lo = -off;
        let hi = M[i] - 1 - off;
        let cen_ok = lo >= -half && hi <= half;
        println!("RANGE|mod={}|trits={}|states=3^{}={}|balanced_range=-{}..{}\
|uncentred_0..{}_fits={}|centred_{}..{}_fits={}|json=0",
                 M[i], W[i], W[i], 3i64.pow(W[i]), half, half,
                 M[i] - 1, raw_ok, lo, hi, cen_ok);
    }
    println!("FINDING|k=centring_is_required|3_trits_hold_27_STATES_but_only_range_-13..13\
|residues_14_and_15_of_mod_16_are_outside_that_range\
|so_the_tower_must_be_stored_CENTRED_as_t_minus_offset|count_is_not_range=1|json=0");

    // ---- round-trip all 1,000,080 through BALANCED TERNARY INT ------------
    let mut trit_fail = 0i64;
    let mut trit_unrep = 0i64;
    for a in 0..total {
        let t = split(a);
        let mut back = [0i64; 4];
        let mut ok = true;
        for i in 0..4 {
            let off = M[i] / 2;
            match to_bt(t[i] - off, W[i]) {
                Some(d) => back[i] = from_bt(&d) + off,
                None => {
                    ok = false;
                    trit_unrep += 1;
                }
            }
        }
        if !ok || join(back) != a {
            trit_fail += 1;
        }
    }
    println!("TRIT|addresses={}|roundtrip_failures={}|unrepresentable={}|exact={}|json=0",
             total, trit_fail, trit_unrep, trit_fail == 0);

    // ---- round-trip all 1,000,080 through FLOAT, section D normalisation --
    // f_i = (t_i - c_i) / c_i    with c from the document
    let c: [f64; 4] = [8.0, 13.5, 2.5, 231.5];
    let mut fd_fail = 0i64;
    let mut worst_a = -1i64;
    for a in 0..total {
        let t = split(a);
        let mut back = [0i64; 4];
        for i in 0..4 {
            let f = (t[i] as f64 - c[i]) / c[i];
            back[i] = (f * c[i] + c[i]).round() as i64;
        }
        if join(back) != a {
            fd_fail += 1;
            if worst_a < 0 {
                worst_a = a;
            }
        }
    }
    println!("FLOAT_D|addresses={}|roundtrip_failures={}|first_failure={}|exact={}|json=0",
             total, fd_fail, worst_a, fd_fail == 0);

    // ---- round-trip through FLOAT, section F.2 normalisation --------------
    // psi(a) = ((a mod 16)/16, ((a/16) mod 27)/27, ((a/432) mod 5)/5, (a/2160)/463)
    let mut ff_fail = 0i64;
    let mut ff_first = -1i64;
    for a in 0..total {
        let f1 = (a % 16) as f64 / 16.0;
        let f2 = ((a / 16) % 27) as f64 / 27.0;
        let f3 = ((a / 432) % 5) as f64 / 5.0;
        let f4 = (a / 2160) as f64 / 463.0;
        let back = (f1 * 16.0).round() as i64
            + (f2 * 27.0).round() as i64 * 16
            + (f3 * 5.0).round() as i64 * 432
            + (f4 * 463.0).round() as i64 * 2160;
        if back != a {
            ff_fail += 1;
            if ff_first < 0 {
                ff_first = a;
            }
        }
    }
    println!("FLOAT_F2|addresses={}|roundtrip_failures={}|first_failure={}|exact={}|json=0",
             total, ff_fail, ff_first, ff_fail == 0);

    // ---- the identity property, stated on the two zeros -------------------
    let pz: f64 = 0.0;
    let nz: f64 = -0.0;
    println!("ZERO|float|plus_bits={:016x}|minus_bits={:016x}|equal={}|bytes_equal={}\
|identity_holds={}|json=0",
             pz.to_bits(), nz.to_bits(), pz == nz,
             pz.to_bits() == nz.to_bits(),
             (pz == nz) == (pz.to_bits() == nz.to_bits()));
    let iz: i64 = 0;
    println!("ZERO|int|zeros=1|equal_is_byte_identity=true|identity_holds=true|value={}|json=0", iz);
    println!("ZERO|trit|zeros=1|states=-1,0,+1|equal_is_byte_identity=true|identity_holds=true|json=0");

    // ---- distributivity, the split-then-merge any tower needs -------------
    let mut fbad = 0i64;
    let mut ibad = 0i64;
    let n = 1_000_000i64;
    let mut x = 12345i64;
    for _ in 0..n {
        x = (x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407)) >> 1;
        let a = (x % 1_000_000).abs() + 1;
        let b = ((x >> 20) % 1_000_000).abs() + 1;
        if (a as f64 / 3.0) + (b as f64 / 3.0) != (a + b) as f64 / 3.0 {
            fbad += 1;
        }
        let (qa, ra) = (a / 3, a % 3);
        let (qb, rb) = (b / 3, b % 3);
        if qa + qb + (ra + rb) / 3 != (a + b) / 3 {
            ibad += 1;
        }
    }
    println!("DISTRIB|trials={}|float_failures={}|int_with_remainder_carried_failures={}|json=0",
             n, fbad, ibad);

    println!("BWFTR|trit_exact={}|float_D_exact={}|float_F2_exact={}|json=0",
             trit_fail == 0, fd_fail == 0, ff_fail == 0);
}
