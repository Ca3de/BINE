/*!
Test bine_hash_lean DIRECTLY - bypass BINE pipeline
====================================================

CRITICAL TEST: Is the 0.5000 bias in:
A) The hash function itself?
B) The BINE transform pipeline?

This tests ONLY the hash, no transforms!
*/

mod lib_v80;

fn main() {
    println!("Testing bine_hash_lean DIRECTLY (no BINE pipeline)");
    println!("===================================================\n");

    let salt = b"test_salt";
    let test_count = 255;  // Only 1-255, not 256 (which becomes 0)
    let samples = 1000;  // Good balance for testing

    let mut max_bias = 0.0f64;
    let mut max_bias_bit = 0usize;
    let mut max_bias_count = 0u32;
    let mut max_bias_delta = 0usize;
    let mut debug_printed = false;

    for delta in 1..=test_count {
        let mut bit_differences = [0u32; 256];

        for sample in 0..samples {
            let input1 = vec![sample as u8; 32];
            let mut input2 = input1.clone();
            input2[0] ^= delta as u8;

            // Test hash DIRECTLY
            let data1 = [&input1[..], salt].concat();
            let data2 = [&input2[..], salt].concat();

            let hash1 = bine_hash_lean(&data1, 32);
            let hash2 = bine_hash_lean(&data2, 32);

            // Debug first iteration
            if !debug_printed && delta == 1 && sample == 0 {
                println!("DEBUG: First test case");
                println!("  Input1[0]: {}, Input2[0]: {}", input1[0], input2[0]);
                println!("  Hash1[0..4]: {:02x} {:02x} {:02x} {:02x}", hash1[0], hash1[1], hash1[2], hash1[3]);
                println!("  Hash2[0..4]: {:02x} {:02x} {:02x} {:02x}", hash2[0], hash2[1], hash2[2], hash2[3]);
                println!("  Hash1[0] binary: {:08b}", hash1[0]);
                println!("  Hash2[0] binary: {:08b}", hash2[0]);
                println!("  XOR diff binary: {:08b}", hash1[0]^hash2[0]);
                println!("  Bit 0 of output: hash1={}, hash2={}", hash1[0] & 1, hash2[0] & 1);
                debug_printed = true;
            }

            for i in 0..32 {
                let diff = hash1[i] ^ hash2[i];
                for bit in 0..8 {
                    if (diff >> bit) & 1 == 1 {
                        let idx = i * 8 + bit;
                        bit_differences[idx] += 1;
                    }
                }
            }
        }

        for (idx, &count) in bit_differences.iter().enumerate() {
            let probability = count as f64 / samples as f64;
            let bias = (probability - 0.5).abs();
            if bias > max_bias {
                max_bias = bias;
                max_bias_bit = idx;
                max_bias_count = count;
                max_bias_delta = delta;
            }
        }
    }

    println!("Max bias found at:");
    println!("  Delta: {}", max_bias_delta);
    println!("  Bit: {} (byte {}, bit {})", max_bias_bit, max_bias_bit / 8, max_bias_bit % 8);
    println!("  Count: {}/{} (prob: {:.4}, bias: {:.4})",
             max_bias_count, samples,
             max_bias_count as f64 / samples as f64, max_bias);

    println!("DIRECT HASH TEST (no BINE transforms):");
    println!("Max Bias: {:.4}\n", max_bias);

    if max_bias < 0.01 {
        println!("✓✓ HASH IS GOOD! Bias < 0.01");
        println!("   Problem is in BINE PIPELINE transforms!");
    } else if max_bias < 0.05 {
        println!("✓ Hash is decent. Pipeline might amplify it.");
    } else if max_bias >= 0.45 {
        println!("✗ HASH HAS 0.5 BIAS! Hash function itself is broken!");
    } else {
        println!("⚠ Hash has moderate bias: {:.4}", max_bias);
    }
}

// Full-width absorb: All state positions get input, then heavy permutation
fn bine_hash_lean(data: &[u8], output_size: usize) -> Vec<u8> {
    let state_size = output_size.max(32);
    let mut state = vec![0u8; state_size];

    // Initialize with distinct values
    for i in 0..state.len() {
        state[i] = SBOX[(i * 197 + 131) % 256];
    }

    // FULL-WIDTH ABSORB: XOR all input across the full state
    for (idx, &byte) in data.iter().enumerate() {
        let pos = idx % state.len();
        state[pos] ^= byte;
        // Mix after every 8 bytes
        if idx % 8 == 7 {
            permute(&mut state);
            permute(&mut state);
        }
    }

    // Final mixing if needed
    if data.len() % 8 != 0 {
        permute(&mut state);
        permute(&mut state);
    }

    // SQUEEZE PHASE: Heavy permutation before output
    permute(&mut state);
    permute(&mut state);
    permute(&mut state);
    permute(&mut state);

    state.truncate(output_size);
    state
}

// Strong permutation function - the heart of the sponge
fn permute(state: &mut [u8]) {
    let len = state.len();

    // 12 rounds of mixing for maximum diffusion
    for round in 0..12 {
        // Forward pass with S-box and round constant
        for i in 0..len {
            let prev = if i > 0 { state[i - 1] } else { state[len - 1] };
            let round_const = ((round * 37 + i * 13) % 256) as u8;
            state[i] = state[i].wrapping_add(prev).wrapping_add(round_const);
            state[i] = SBOX[state[i] as usize];
        }

        // Backward pass with multiplication and S-box
        for i in (0..len).rev() {
            let next = if i < len - 1 { state[i + 1] } else { state[0] };
            state[i] = state[i].wrapping_mul(251);
            state[i] ^= SBOX[next as usize];
            state[i] = SBOX[state[i] as usize];
        }

        // INPUT-DEPENDENT GLOBAL MIXING: Use state values to determine mix positions
        let mut temp = vec![0u8; len];
        for i in 0..len {
            let mut mix = state[i];

            // Mixing positions depend on current state values (input-dependent!)
            let pos1 = (i + (state[i] as usize % (len / 2 + 1))) % len;
            let pos2 = (i + (state[(i + 1) % len] as usize % (len / 3 + 1))) % len;
            let pos3 = (i + (state[(i + 2) % len] as usize % (len / 4 + 1))) % len;

            // Complex mixing with input-dependent positions
            mix ^= state[pos1];
            mix = mix.wrapping_add(state[pos2]);
            mix = mix.wrapping_mul(179);
            mix ^= SBOX[state[pos3] as usize];
            mix = mix.wrapping_add(state[(i + len / 2) % len]);

            temp[i] = SBOX[mix as usize];
            temp[i] = temp[i].rotate_left(((round * 3 + mix as usize) % 7 + 1) as u32);
            temp[i] = SBOX[temp[i] as usize];
        }
        state.copy_from_slice(&temp);
    }
}

const SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];
