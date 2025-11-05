/*!
Diagnostic Tool - Find the Source of 0.5000 Bias
=================================================

Test each component separately to find where the bias comes from.
*/

fn main() {
    println!("Diagnostic: Finding source of 0.5000 differential bias\n");

    // Test 1: Does mini_hash itself have differential bias?
    test_mini_hash_differential();

    // Test 2: Does the S-box reduce bias?
    test_sbox_effect();
}

fn test_mini_hash_differential() {
    println!("Test 1: Mini-hash differential (ChaCha20-based)");
    println!("================================================\n");

    let salt = b"test_salt";
    let mut max_bias = 0.0f64;

    for delta in 1u8..=255 {
        let mut bit_differences = [0u32; 256];
        let samples = 100;

        for sample in 0..samples {
            let input1 = [vec![sample as u8; 32], salt.to_vec()].concat();
            let mut input2 = input1.clone();
            input2[0] ^= delta;

            let hash1 = mini_hash(&input1, 32);
            let hash2 = mini_hash(&input2, 32);

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

        for &count in &bit_differences {
            let probability = count as f64 / samples as f64;
            let bias = (probability - 0.5).abs();
            if bias > max_bias {
                max_bias = bias;
            }
        }
    }

    println!("Mini-hash max bias: {:.4}", max_bias);

    if max_bias > 0.4 {
        println!("✗ PROBLEM FOUND! Mini-hash has severe differential bias!");
        println!("  ChaCha20-based function may have linear properties\n");
    } else if max_bias > 0.05 {
        println!("⚠ Mini-hash has moderate bias\n");
    } else {
        println!("✓ Mini-hash differential looks OK\n");
    }
}

fn test_sbox_effect() {
    println!("Test 2: Does S-box reduce differential bias?");
    println!("============================================\n");

    // Test with and without S-box
    let test_data = b"test_input_for_sbox_differential";

    let hash_no_sbox = mini_hash_no_sbox(test_data, 32);
    let hash_with_sbox = mini_hash(test_data, 32);

    println!("Created hashes with and without S-box");
    println!("Testing differential with small input change...\n");

    let mut test_data2 = test_data.to_vec();
    test_data2[0] ^= 1;

    let hash_no_sbox2 = mini_hash_no_sbox(&test_data2, 32);
    let hash_with_sbox2 = mini_hash(&test_data2, 32);

    let mut diff_no_sbox = 0;
    let mut diff_with_sbox = 0;

    for i in 0..32 {
        diff_no_sbox += (hash_no_sbox[i] ^ hash_no_sbox2[i]).count_ones();
        diff_with_sbox += (hash_with_sbox[i] ^ hash_with_sbox2[i]).count_ones();
    }

    let avalanche_no_sbox = (diff_no_sbox as f64 / 256.0) * 100.0;
    let avalanche_with_sbox = (diff_with_sbox as f64 / 256.0) * 100.0;

    println!("Without S-box: {:.2}% bits flipped", avalanche_no_sbox);
    println!("With S-box:    {:.2}% bits flipped", avalanche_with_sbox);

    if avalanche_with_sbox > avalanche_no_sbox {
        println!("✓ S-box improves diffusion\n");
    } else {
        println!("⚠ S-box doesn't help much\n");
    }
}

fn mini_hash(data: &[u8], output_size: usize) -> Vec<u8> {
    let mut state = [0u32; 16];
    state[0] = 0x61707865;
    state[1] = 0x3320646e;
    state[2] = 0x79622d32;
    state[3] = 0x6b206574;

    for (i, chunk) in data.chunks(4).enumerate() {
        if i >= 12 { break; }
        let mut val = 0u32;
        for (j, &b) in chunk.iter().enumerate() {
            val |= (b as u32) << (j * 8);
        }
        state[4 + i] ^= val;
    }

    for _ in 0..20 {
        quarter_round(&mut state, 0, 4, 8, 12);
        quarter_round(&mut state, 1, 5, 9, 13);
        quarter_round(&mut state, 2, 6, 10, 14);
        quarter_round(&mut state, 3, 7, 11, 15);
        quarter_round(&mut state, 0, 5, 10, 15);
        quarter_round(&mut state, 1, 6, 11, 12);
        quarter_round(&mut state, 2, 7, 8, 13);
        quarter_round(&mut state, 3, 4, 9, 14);
    }

    let mut output = Vec::with_capacity(output_size);
    for &word in state.iter() {
        for i in 0..4 {
            if output.len() >= output_size {
                break;
            }
            output.push((word >> (i * 8)) as u8);
        }
    }

    output.truncate(output_size);
    apply_sbox(&mut output);
    output
}

fn mini_hash_no_sbox(data: &[u8], output_size: usize) -> Vec<u8> {
    let mut state = [0u32; 16];
    state[0] = 0x61707865;
    state[1] = 0x3320646e;
    state[2] = 0x79622d32;
    state[3] = 0x6b206574;

    for (i, chunk) in data.chunks(4).enumerate() {
        if i >= 12 { break; }
        let mut val = 0u32;
        for (j, &b) in chunk.iter().enumerate() {
            val |= (b as u32) << (j * 8);
        }
        state[4 + i] ^= val;
    }

    for _ in 0..20 {
        quarter_round(&mut state, 0, 4, 8, 12);
        quarter_round(&mut state, 1, 5, 9, 13);
        quarter_round(&mut state, 2, 6, 10, 14);
        quarter_round(&mut state, 3, 7, 11, 15);
        quarter_round(&mut state, 0, 5, 10, 15);
        quarter_round(&mut state, 1, 6, 11, 12);
        quarter_round(&mut state, 2, 7, 8, 13);
        quarter_round(&mut state, 3, 4, 9, 14);
    }

    let mut output = Vec::with_capacity(output_size);
    for &word in state.iter() {
        for i in 0..4 {
            if output.len() >= output_size {
                break;
            }
            output.push((word >> (i * 8)) as u8);
        }
    }

    output.truncate(output_size);
    // NO S-BOX
    output
}

#[inline]
fn quarter_round(state: &mut [u32], a: usize, b: usize, c: usize, d: usize) {
    state[a] = state[a].wrapping_add(state[b]);
    state[d] ^= state[a];
    state[d] = state[d].rotate_left(16);
    state[c] = state[c].wrapping_add(state[d]);
    state[b] ^= state[c];
    state[b] = state[b].rotate_left(12);
    state[a] = state[a].wrapping_add(state[b]);
    state[d] ^= state[a];
    state[d] = state[d].rotate_left(8);
    state[c] = state[c].wrapping_add(state[d]);
    state[b] ^= state[c];
    state[b] = state[b].rotate_left(7);
}

fn apply_sbox(data: &mut [u8]) {
    for byte in data.iter_mut() {
        *byte = SBOX[*byte as usize];
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
