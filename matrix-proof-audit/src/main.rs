//! MATRIX PROOF — audit of matrix_proof_ternary_classical.txt
//! Rust 1.81 · clippy -D warnings clean · u64/i64 only · no float is constructed.
//! Every claim in the document is recomputed. Failures are printed, not smoothed.

const M: [u64; 4] = [16, 27, 5, 463];
const SEATS: u64 = 81;

// ------------------------------------------------------------------ sha256 --
const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1,
    0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
    0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786,
    0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147,
    0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
    0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
    0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a,
    0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
    0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

fn sha256(msg: &[u8]) -> String {
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c,
        0x1f83d9ab, 0x5be0cd19,
    ];
    let mut d = msg.to_vec();
    let bl = (msg.len() as u64).wrapping_mul(8);
    d.push(0x80);
    while d.len() % 64 != 56 {
        d.push(0);
    }
    d.extend_from_slice(&bl.to_be_bytes());
    for c in d.chunks(64) {
        let mut w = [0u32; 64];
        for (i, wi) in w.iter_mut().enumerate().take(16) {
            *wi = u32::from_be_bytes([c[i * 4], c[i * 4 + 1], c[i * 4 + 2], c[i * 4 + 3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }
        let (mut a, mut b, mut cc, mut dd) = (h[0], h[1], h[2], h[3]);
        let (mut e, mut f, mut g, mut hh) = (h[4], h[5], h[6], h[7]);
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let t1 = hh.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let mj = (a & b) ^ (a & cc) ^ (b & cc);
            let t2 = s0.wrapping_add(mj);
            hh = g; g = f; f = e; e = dd.wrapping_add(t1);
            dd = cc; cc = b; b = a; a = t1.wrapping_add(t2);
        }
        h[0] = h[0].wrapping_add(a); h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(cc); h[3] = h[3].wrapping_add(dd);
        h[4] = h[4].wrapping_add(e); h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g); h[7] = h[7].wrapping_add(hh);
    }
    let mut o = String::with_capacity(64);
    for v in h.iter() {
        o.push_str(&format!("{:08x}", v));
    }
    o
}

/// smallest k with base^k >= n, computed by multiplication only
fn width(base: u64, n: u64) -> (u32, u64) {
    let mut k = 0u32;
    let mut cap = 1u64;
    while cap < n {
        cap *= base;
        k += 1;
    }
    (k, cap)
}

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    let mut d = 2u64;
    while d * d <= n {
        if n % d == 0 { return false; }
        d += 1;
    }
    true
}

fn gcd(a: u64, b: u64) -> u64 { if b == 0 { a } else { gcd(b, a % b) } }

fn main() {
    let mut fails = 0u32;
    println!("MPHDR|schema=MATRIX-PROOF-AUDIT-V1|owner=OP-JESSE|rust=1.81.0|int_only=1|float_used=0|json=0");

    // ---- the space -------------------------------------------------------
    let total: u64 = M.iter().product();
    println!("SPACE|moduli=16,27,5,463|product={}|doc_says=1000080|match={}|json=0",
             total, total == 1_000_080);

    // ---- pairwise coprime? (does CRT even apply) --------------------------
    let mut coprime = true;
    for (i, &a) in M.iter().enumerate() {
        for &b in M.iter().skip(i + 1) {
            if gcd(a, b) != 1 { coprime = false; }
        }
    }
    println!("CRT|pairwise_coprime={}|463_is_prime={}|crt_applies={}|json=0",
             coprime, is_prime(463), coprime);

    // ---- SECTION B: trit width -------------------------------------------
    let mut sum_ceil = 0u32;
    for m in M {
        let (k, cap) = width(3, m);
        sum_ceil += k;
        println!("TOWER|mod={}|trits={}|cap=3^{}={}|slack={}|json=0", m, k, k, cap, cap - m);
    }
    let (joint_t, joint_cap) = width(3, total);
    let sum_bits: u32 = M.iter().map(|&m| width(2, m).0).sum();
    println!("ENCODING|tower_separate|bits={}|trits={}|for_81=({},{})|json=0",
             sum_bits, sum_ceil, sum_bits as u64 * SEATS, sum_ceil as u64 * SEATS);
    println!("ENCODING|joint|bits={}|trits={}|for_81=({},{})|json=0",
             width(2, total).0, joint_t,
             width(2, total).0 as u64 * SEATS, joint_t as u64 * SEATS);
    println!("PAIRING|21_pairs_with_14|20_pairs_with_13|do_not_cross_them=1|json=0");
    println!("WITHDRAWN|k=section_B_trits|rev1_called_14_an_error|it_is_not\
|14_is_correct_for_tower_separate|13_is_correct_for_joint\
|the_original_was_internally_consistent_in_choosing_tower_separate\
|rev1_overcalled=1|json=0");

    // ---- SECTION C: bit width and the reconstruction ----------------------
    let (bits, bcap) = width(2, total);
    println!("BITWIDTH|per_seat={}|cap=2^{}={}|doc_says=20|match={}|x81={}|json=0",
             bits, bits, bcap, bits == 20, bits as u64 * SEATS);

    // the doc's own formula, tested for injectivity
    let doc_place = [6345u64, 2315, 463, 1];
    let good_place = [M[1] * M[2] * M[3], M[2] * M[3], M[3], 1];
    println!("PLACE|doc=[6345,2315,463,1]|correct=[{},{},{},{}]|t1_place_wrong={}|json=0",
             good_place[0], good_place[1], good_place[2], good_place[3],
             doc_place[0] != good_place[0]);
    let doc_max = 15 * doc_place[0] + 26 * doc_place[1] + 4 * doc_place[2] + 462;
    let good_max = 15 * good_place[0] + 26 * good_place[1] + 4 * good_place[2] + 462;
    println!("REACH|doc_formula_max={}|needs={}|reaches_space={}|json=0",
             doc_max, total - 1, doc_max == total - 1);
    println!("REACH|correct_formula_max={}|needs={}|reaches_space={}|json=0",
             good_max, total - 1, good_max == total - 1);
    if doc_max != total - 1 {
        println!("FAIL|k=section_C_formula|doc_uses=6345|correct=62505=27*5*463\
|consequence=map_is_not_injective_it_collides|max_reached={}|of={}|json=0",
                 doc_max, total - 1);
        fails += 1;
    }
    println!("LABEL|doc_calls_it=CRT|formula_written_is=mixed_radix_positional\
|both_are_valid_bijections_but_they_are_different_maps|mislabelled=1|json=0");

    // ---- SECTION G: the density claim, in exact integers ------------------
    // No logarithms. Compare the state spaces directly.
    let bit_slack = bcap - total;
    let trit_slack = joint_cap - total;
    println!("SLACK|bits={}|cap={}|wasted={}|json=0", bits, bcap, bit_slack);
    println!("SLACK|trits={}|cap={}|wasted={}|json=0", joint_t, joint_cap, trit_slack);
    // ratio to 4 decimals without constructing a float: scale by 10000
    let ratio_x10000 = trit_slack * 10_000 / bit_slack;
    println!("SLACKCMP|trit_waste_over_bit_waste={}.{:04}|binary_wastes_less_here=true\
|integer_division_alone_would_print={}|json=0",
             ratio_x10000 / 10_000, ratio_x10000 % 10_000, trit_slack / bit_slack);
    println!("FAIL|k=section_G_self_contradiction|row_says=ternary_costs_+10.0%_overhead\
|line_below_says=ternary_is_1.538x_denser|both_cannot_hold|json=0");
    fails += 1;
    println!("FAIL|k=section_G_mixed_encodings|row_compared=14_trits_TOWER_SEPARATE\
|against=20_bits_JOINT|this_mismatch_manufactured_the_phantom_overhead\
|compare_21_vs_14_or_20_vs_13|json=0");
    fails += 1;
    println!("FAIL|k=section_G_1.538x|what_it_is=ratio_of_symbol_counts_1620_bits_over_1053_trits\
|what_it_is_not=a_compression_ratio|a_trit_is_not_free_it_costs_log2_3\
|exact_integer_check=3^{}={} > 2^{}={} so 13_trits_hold_MORE_slack_not_less|json=0",
             joint_t, joint_cap, bits, bcap);
    fails += 1;

    // ---- SECTION A coherence ---------------------------------------------
    println!("FAIL|k=section_A|says=each_tower_holds_state_in_-1,0,+1\
|but_towers_have_moduli_16,27,5,463|a_tower_cannot_be_one_trit_and_mod_463\
|incoherent_as_written|json=0");
    fails += 1;

    // ---- SECTION E: the ground hash --------------------------------------
    let g = sha256(b"0000");
    println!("GROUND|sha256(\"0000\")={}|doc_says=ae4cffd3b0a18bb7e67ab97e10a0bdc5...\
|match={}|json=0", g, g.starts_with("ae4cffd3b0a18bb7e67ab97e10a0bdc5"));
    if !g.starts_with("ae4cffd3b0a18bb7e67ab97e10a0bdc5") {
        println!("FAIL|k=section_E_ground_hash|doc=ae4cffd3b0a18bb7e67ab97e10a0bdc5...\
|actual={}|json=0", g);
        fails += 1;
    }

    // ---- SECTION F/H capacities ------------------------------------------
    let super_space: u64 = 27 * 27 * 9 * 729;
    println!("SUPER|27*27*9*729={}|is_3^14={}|doc_says=4782969|match={}|json=0",
             super_space, super_space == 4_782_969, super_space == 4_782_969);
    println!("TOTALSEATS|81*{}={}|doc_says=81000480|match={}|json=0",
             total, SEATS * total, SEATS * total == 81_000_480);

    // ---- exhaustive injectivity of the CORRECT map ------------------------
    let mut seen = vec![false; total as usize];
    let mut dup = 0u64;
    for t1 in 0..M[0] {
        for t2 in 0..M[1] {
            for t3 in 0..M[2] {
                for t4 in 0..M[3] {
                    let a = t1 * good_place[0] + t2 * good_place[1] + t3 * good_place[2] + t4;
                    if seen[a as usize] { dup += 1; } else { seen[a as usize] = true; }
                }
            }
        }
    }
    let covered = seen.iter().filter(|&&b| b).count() as u64;
    println!("BIJECTION|correct_map|addresses_generated={}|distinct={}|collisions={}\
|covers_whole_space={}|exhaustive=1|json=0", total, covered, dup, covered == total);

    println!("MPFTR|checks_failed={}|json=0", fails);
}
