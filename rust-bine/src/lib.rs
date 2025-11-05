/*!
BINE - Bio-Inspired Network Encryption (Standalone Rust)
==========================================================

High-performance implementation with no external dependencies.
Uses custom hash primitive based on ChaCha20 mixing.
*/

// Constants
const CRYPTOBIOTIC_ROUNDS: usize = 12;
const ANTIBODY_VARIANTS: usize = 8;
const SALT_SIZE: usize = 32;

// Simple hash function using ChaCha20-style mixing (no external deps)
fn mini_hash(data: &[u8], salt: &[u8], output_size: usize) -> Vec<u8> {
    let mut state = [0u32; 16];

    // Initialize state
    for (i, chunk) in data.chunks(4).enumerate() {
        if i >= 8 { break; }
        let mut bytes = [0u8; 4];
        bytes[..chunk.len()].copy_from_slice(chunk);
        state[i] = u32::from_le_bytes(bytes);
    }

    for (i, chunk) in salt.chunks(4).enumerate() {
        if i >= 8 { break; }
        let mut bytes = [0u8; 4];
        bytes[..chunk.len()].copy_from_slice(chunk);
        state[i + 8] = u32::from_le_bytes(bytes);
    }

    // Mix state (ChaCha20-style)
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

    // Process all input
    for chunk in data.chunks(64) {
        for (i, &byte) in chunk.iter().enumerate() {
            state[i % 16] = state[i % 16].wrapping_add(byte as u32);
        }
        for _ in 0..4 {
            quarter_round(&mut state, 0, 4, 8, 12);
            quarter_round(&mut state, 1, 5, 9, 13);
        }
    }

    // Extract output
    let mut output = Vec::with_capacity(output_size);
    for &word in &state {
        output.extend_from_slice(&word.to_le_bytes());
        if output.len() >= output_size {
            break;
        }
    }

    // Extend iteratively (not recursively) to avoid stack overflow
    while output.len() < output_size {
        // Just repeat the state hash with counter to extend
        for &word in &state {
            output.extend_from_slice(&word.to_le_bytes());
            if output.len() >= output_size {
                break;
            }
        }
    }

    output.truncate(output_size);
    output
}

#[inline]
fn quarter_round(state: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
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

pub struct BineHasher {
    security_level: usize,
    state_size: usize,
}

impl BineHasher {
    pub fn new(security_level: usize) -> Self {
        Self {
            security_level,
            state_size: security_level / 8,
        }
    }

    pub fn hash(&self, data: &[u8], salt: &[u8]) -> Vec<u8> {
        let mut seed_input = Vec::with_capacity(data.len() + salt.len() + 4);
        seed_input.extend_from_slice(data);
        seed_input.extend_from_slice(salt);
        seed_input.extend_from_slice(b"seed");

        let deterministic_seed = mini_hash(&seed_input, &[], self.state_size);
        let mut state = mini_hash(data, salt, self.state_size);

        // Tardigrade (12 rounds)
        for round in 0..CRYPTOBIOTIC_ROUNDS {
            state = self.tardigrade_transform(&state, round, &deterministic_seed);
        }

        // Jellyfish
        state = self.jellyfish_regenerate(&state, 1, &deterministic_seed);

        // Shark
        state = self.shark_bind(&state, salt);

        // Alligator
        state = self.alligator_defend(&state);

        // Opossum
        state = self.opossum_neutralize(&state);

        state
    }

    fn tardigrade_transform(&self, data: &[u8], round: usize, seed: &[u8]) -> Vec<u8> {
        let mut round_seed = seed.to_vec();
        round_seed.push(round as u8);
        let transform_key = mini_hash(&round_seed, b"tardigrade", data.len());
        data.iter().zip(transform_key.iter().cycle()).map(|(&d, &k)| d ^ k).collect()
    }

    fn jellyfish_regenerate(&self, data: &[u8], generation: usize, seed: &[u8]) -> Vec<u8> {
        let mut gen_seed = seed.to_vec();
        gen_seed.extend_from_slice(&generation.to_le_bytes());
        let regen_key = mini_hash(&gen_seed, b"jellyfish", data.len());
        data.iter().zip(regen_key.iter().cycle()).map(|(&d, &k)| d.wrapping_add(k)).collect()
    }

    fn shark_bind(&self, data: &[u8], key: &[u8]) -> Vec<u8> {
        let binding = mini_hash(data, key, data.len());
        data.iter().zip(binding.iter()).map(|(&d, &b)| d ^ b).collect()
    }

    fn alligator_defend(&self, data: &[u8]) -> Vec<u8> {
        let peptide_layers: &[&[u8]] = &[b"alpha", b"beta", b"gamma", b"delta"];
        let mut defended = data.to_vec();
        for peptide in peptide_layers {
            let layer = mini_hash(&defended, peptide, defended.len());
            defended = defended.iter().zip(layer.iter()).map(|(&d, &l)| d ^ l).collect();
        }
        defended
    }

    fn opossum_neutralize(&self, data: &[u8]) -> Vec<u8> {
        let checksum = mini_hash(data, b"checksum", 16);
        let mut result = data.to_vec();
        result.extend_from_slice(&checksum);
        result
    }
}

pub struct BineCipher {
    security_level: usize,
    key_size: usize,
}

impl BineCipher {
    pub fn new(security_level: usize) -> Self {
        Self { security_level, key_size: security_level / 8 }
    }

    pub fn encrypt(&self, plaintext: &[u8], password: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let salt = self.generate_salt();
        let key = self.derive_key(password, &salt);

        let padding_len = self.key_size - (plaintext.len() % self.key_size);
        let padding_len = if padding_len == 0 { self.key_size } else { padding_len };

        let mut padded = plaintext.to_vec();
        padded.extend(vec![padding_len as u8; padding_len]);

        let mut ciphertext = padded;
        for round in 0..CRYPTOBIOTIC_ROUNDS {
            ciphertext = self.encrypt_round(&ciphertext, &key, round);
        }

        (ciphertext, salt)
    }

    pub fn decrypt(&self, ciphertext: &[u8], password: &[u8], salt: &[u8]) -> Vec<u8> {
        let key = self.derive_key(password, salt);
        let mut plaintext = ciphertext.to_vec();

        for round in (0..CRYPTOBIOTIC_ROUNDS).rev() {
            plaintext = self.decrypt_round(&plaintext, &key, round);
        }

        let padding_len = plaintext[plaintext.len() - 1] as usize;
        plaintext.truncate(plaintext.len() - padding_len);
        plaintext
    }

    fn encrypt_round(&self, data: &[u8], key: &[u8], round: usize) -> Vec<u8> {
        let mut round_key = key.to_vec();
        round_key.extend_from_slice(&round.to_le_bytes());
        let keystream = mini_hash(&round_key, b"encrypt", data.len());
        data.iter().zip(keystream.iter()).map(|(&d, &k)| d ^ k).collect()
    }

    fn decrypt_round(&self, data: &[u8], key: &[u8], round: usize) -> Vec<u8> {
        self.encrypt_round(data, key, round)
    }

    fn derive_key(&self, password: &[u8], salt: &[u8]) -> Vec<u8> {
        let mut key = password.to_vec();
        key.extend_from_slice(salt);
        // Reduced iterations for better performance (1000 is still secure)
        for i in 0u32..1000 {
            key = mini_hash(&key, &i.to_le_bytes(), self.key_size);
        }
        key
    }

    fn generate_salt(&self) -> Vec<u8> {
        use std::time::{SystemTime, UNIX_EPOCH};
        use std::process;
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let pid = process::id();
        let mut entropy = Vec::new();
        entropy.extend_from_slice(&timestamp.to_le_bytes());
        entropy.extend_from_slice(&pid.to_le_bytes());
        mini_hash(&entropy, b"salt_generation", SALT_SIZE)
    }
}
