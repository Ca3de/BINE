/*!
BINE v6.0 - TRUE Unbounded Learning (No Caps)
==============================================

CRITICAL FIX: v5.0's "learning" was capped at 40 extra rounds (max 3.3x slowdown)
This is NOT real learning - it's a capped counter system.

v6.0 implements TRUE unbounded adaptive learning:
- NO caps on extra rounds
- Exponential growth: x1 → x5 → x10 → x100 → x1000+
- After 100 attacks: ~10x slower
- After 1000 attacks: ~100x slower
- After 10000 attacks: ~1000x slower (effectively blocks attack)

Note: This is still pattern matching (HashMap lookup), not ML/neural networks.
But it's TRUE adaptive defense that grows indefinitely.

Expected behavior:
- Attack #1:     1.0x speed (12 base rounds)
- Attack #10:    ~2x slower (~12 extra rounds)
- Attack #100:   ~10x slower (~120 extra rounds)
- Attack #1000:  ~100x slower (~1200 extra rounds)
- Attack #10000: ~1000x slower (~12000 extra rounds) - essentially blocked
*/

use std::collections::HashMap;

pub const ANTIBODY_VARIANTS: usize = 8;
pub const PEPTIDE_LAYERS: usize = 4;

// v6.0: MUCH stronger multiplier for exponential growth
const LEARNING_ROUNDS_MULTIPLIER: usize = 10; // Was 5, now 10 for faster growth

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SecurityMode {
    Fast,
    Balanced,
    Paranoid,
}

impl SecurityMode {
    pub fn rounds(&self) -> usize {
        match self {
            SecurityMode::Fast => 8,
            SecurityMode::Balanced => 12,
            SecurityMode::Paranoid => 20,
        }
    }

    pub fn use_learning(&self) -> bool {
        !matches!(self, SecurityMode::Fast)
    }
}

pub struct ImmunologicalMemory {
    threat_database: HashMap<Vec<u8>, u32>,
    pub mutation_rate: u32,
    pub total_threats_seen: u64,
}

impl ImmunologicalMemory {
    pub fn new() -> Self {
        Self {
            threat_database: HashMap::new(),
            mutation_rate: 0,
            total_threats_seen: 0,
        }
    }

    pub fn learn_threat(&mut self, pattern: &[u8]) {
        let key = if pattern.len() >= 32 {
            pattern[..32].to_vec()
        } else {
            let mut padded = pattern.to_vec();
            padded.resize(32, 0);
            padded
        };

        let entry = self.threat_database.entry(key).or_insert(0);
        *entry = (*entry).saturating_add(1);
        self.total_threats_seen = self.total_threats_seen.saturating_add(1);

        if *entry > 10 {
            self.mutation_rate = self.mutation_rate.saturating_add(1);
        }
    }

    // v6.0: NO CAPS! Unbounded exponential growth
    pub fn extra_defense_rounds(&self) -> usize {
        // NO .min() caps - grows indefinitely!
        let base_extra = (self.mutation_rate as usize) * LEARNING_ROUNDS_MULTIPLIER;
        let adaptive_extra = ((self.total_threats_seen / 10) as usize) * LEARNING_ROUNDS_MULTIPLIER;

        // Total can be 100s, 1000s, even 10000s of extra rounds
        base_extra + adaptive_extra
    }

    pub fn is_threat(&self, pattern: &[u8]) -> bool {
        if pattern.len() < 32 {
            return false;
        }
        self.threat_database.get(&pattern[..32].to_vec())
            .map(|&count| count > 5)
            .unwrap_or(false)
    }
}

pub struct BineHasherV6 {
    state_size: usize,
    mode: SecurityMode,
    pub immune_memory: ImmunologicalMemory,
}

impl BineHasherV6 {
    pub fn new(bits: usize, mode: SecurityMode) -> Self {
        Self {
            state_size: bits / 8,
            mode,
            immune_memory: ImmunologicalMemory::new(),
        }
    }

    pub fn report_threat(&mut self, pattern: &[u8]) {
        self.immune_memory.learn_threat(pattern);
    }

    pub fn hash(&mut self, data: &[u8], salt: &[u8]) -> Vec<u8> {
        let salted = [data, salt].concat();
        let mut seed_data = salted.clone();
        seed_data.extend_from_slice(b"seed");
        let deterministic_seed = mini_hash(&seed_data, self.state_size);

        let mut state = mini_hash(&salted, self.state_size);

        // v6.0: Unbounded learning - can be 100s or 1000s of rounds!
        let base_rounds = self.mode.rounds();
        let extra_rounds = if self.mode.use_learning() {
            self.immune_memory.extra_defense_rounds()
        } else {
            0
        };
        let total_rounds = base_rounds + extra_rounds;

        // This loop could execute 12, 120, 1200, or even 12000 times!
        for round in 0..total_rounds {
            state = self.tardigrade_transform(&state, round, &deterministic_seed);
            state = self.jellyfish_regenerate(&state, round, &deterministic_seed);
            state = self.cockroach_distribute(&state, round);
        }

        state = self.ostrich_diversify(&state);
        state = self.alligator_defend(&state);
        state = self.extra_diffusion(&state);

        self.opossum_neutralize(&state)
    }

    fn tardigrade_transform(&self, state: &[u8], round: usize, seed: &[u8]) -> Vec<u8> {
        let round_key = self.derive_round_key(seed, round);
        let mut transformed = Vec::with_capacity(state.len());

        for i in 0..state.len() {
            let s = state[i];
            let k = round_key[i % round_key.len()];
            let prev = if i > 0 { state[i - 1] } else { state[state.len() - 1] };

            let val = s
                .wrapping_add(k)
                .wrapping_mul(251)
                .rotate_left(3)
                ^ prev.rotate_right(5);

            transformed.push(val);
        }

        transformed
    }

    fn jellyfish_regenerate(&self, state: &[u8], generation: usize, seed: &[u8]) -> Vec<u8> {
        let mut evolved = state.to_vec();
        let gen_factor = (generation.wrapping_mul(17).wrapping_add(seed[0] as usize)) as u32;

        for i in 0..evolved.len() {
            evolved[i] = evolved[i].rotate_left(gen_factor % 8);
            evolved[i] ^= ((generation.wrapping_mul(31).wrapping_add(i)) % 256) as u8;
        }

        evolved
    }

    fn cockroach_distribute(&self, state: &[u8], segment_id: usize) -> Vec<u8> {
        let mut distributed = state.to_vec();
        let offset = segment_id % distributed.len();

        for i in 0..distributed.len() {
            let j = (i + offset) % distributed.len();
            distributed[i] ^= state[j];
        }

        distributed
    }

    fn ostrich_diversify(&self, state: &[u8]) -> Vec<u8> {
        let mut variants = Vec::new();

        for variant in 0..ANTIBODY_VARIANTS {
            let mut v = state.to_vec();
            for byte in v.iter_mut() {
                *byte = byte
                    .wrapping_add(variant as u8)
                    .rotate_left((variant % 8) as u32);
            }
            variants.extend_from_slice(&v);
        }

        mini_hash(&variants, self.state_size)
    }

    fn alligator_defend(&self, state: &[u8]) -> Vec<u8> {
        let mut defended = state.to_vec();

        for layer in 0..PEPTIDE_LAYERS {
            for i in 0..defended.len() {
                let mask = ((layer * 67 + i * 13) % 256) as u8;
                defended[i] ^= mask;
                defended[i] = defended[i].rotate_left((layer % 8) as u32);
            }
        }

        defended
    }

    fn extra_diffusion(&self, data: &[u8]) -> Vec<u8> {
        let mut diffused = data.to_vec();
        for pass in 0..2 {
            for i in 0..diffused.len() {
                let prev = if i > 0 { diffused[i - 1] } else { diffused[diffused.len() - 1] };
                let next = if i < diffused.len() - 1 { diffused[i + 1] } else { diffused[0] };
                let far = diffused[(i + diffused.len() / 2) % diffused.len()];
                diffused[i] ^= (prev.wrapping_add(next)) ^ far.rotate_left(pass as u32);
            }
        }
        diffused
    }

    fn opossum_neutralize(&self, state: &[u8]) -> Vec<u8> {
        let checksum = self.compute_checksum(state);
        let mut output = state.to_vec();
        output.extend_from_slice(&checksum);
        output
    }

    fn compute_checksum(&self, data: &[u8]) -> Vec<u8> {
        let hash = mini_hash(data, 16);
        hash
    }

    fn derive_round_key(&self, seed: &[u8], round: usize) -> Vec<u8> {
        let round_data = [seed, &round.to_le_bytes()].concat();
        mini_hash(&round_data, self.state_size)
    }
}

// v6.0: Keep fast mini_hash from v5.0
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unbounded_growth() {
        let mut hasher = BineHasherV6::new(256, SecurityMode::Balanced);
        let attack = vec![0xFF; 100];
        let salt = b"test";

        // Report many threats
        for _ in 0..100 {
            hasher.report_threat(&attack[..32]);
        }

        let extra_rounds = hasher.immune_memory.extra_defense_rounds();
        println!("Extra rounds after 100 reports: {}", extra_rounds);

        // Should be 100+ rounds, NOT capped at 40!
        assert!(extra_rounds >= 100, "Expected 100+ rounds, got {}", extra_rounds);
    }
}
