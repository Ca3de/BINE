/*!
BINE v7.8 - INPUT WHITENING Fix for Differential
=================================================

Root cause of 0.5000 bias:
ChaCha20-based mini_hash can't handle regular patterns like [42,42,...,42]

v7.8 Solution: AGGRESSIVE INPUT WHITENING
- Break regular patterns BEFORE they enter mini_hash
- XOR with position-dependent values
- Apply S-box to break linearity
- Mix with neighbors for diffusion
- Then feed to mini_hash

Target: Differential < 0.01 (ideally < 0.005)
*/

use std::collections::HashMap;

pub const ANTIBODY_VARIANTS: usize = 8;
pub const PEPTIDE_LAYERS: usize = 4;
const LEARNING_ROUNDS_MULTIPLIER: usize = 10;
const FEATURE_DIM: usize = 64;

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

#[derive(Debug, Clone, Copy)]
pub enum ThreatLevel {
    Clean,
    Low,
    Medium,
    High,
    Critical,
}

pub struct CompromiseReport {
    pub is_compromised: bool,
    pub corruption_rate: f64,
    pub threat_level: ThreatLevel,
}

pub struct NeuralLearning {
    weights: Vec<f64>,
    bias: f64,
    learning_rate: f64,
    training_count: u64,
}

impl NeuralLearning {
    pub fn new() -> Self {
        let mut weights = vec![0.0; FEATURE_DIM];
        for (i, w) in weights.iter_mut().enumerate() {
            *w = ((i * 7919) % 1000) as f64 / 1000.0 - 0.5;
        }

        Self {
            weights,
            bias: 0.0,
            learning_rate: 0.01,
            training_count: 0,
        }
    }

    pub fn extract_features(&self, data: &[u8]) -> Vec<f64> {
        let mut features = vec![0.0; FEATURE_DIM];
        if data.is_empty() {
            return features;
        }

        for &byte in data.iter().take(256) {
            let bin = (byte as usize) / 16;
            if bin < 16 {
                features[bin] += 1.0;
            }
        }
        let total = data.len().min(256) as f64;
        for f in features[..16].iter_mut() {
            *f /= total;
        }

        for i in 0..data.len().saturating_sub(1).min(16) {
            let pattern = (data[i] ^ data[i + 1]) as usize;
            features[16 + (pattern % 8)] += 1.0;
        }

        let sum: u32 = data.iter().take(32).map(|&b| b as u32).sum();
        features[24] = (sum as f64) / (data.len().min(32) as f64 * 255.0);

        let mut xor = 0u8;
        for &b in data.iter().take(32) {
            xor ^= b;
        }
        features[25] = (xor as f64) / 255.0;

        let mut repeats = 0;
        for i in 0..data.len().saturating_sub(1).min(32) {
            if data[i] == data[i + 1] {
                repeats += 1;
            }
        }
        features[26] = (repeats as f64) / 32.0;

        for i in 0..data.len().saturating_sub(1).min(32) {
            let bigram = ((data[i] as usize) + (data[i + 1] as usize)) % 8;
            features[32 + bigram] += 1.0;
        }

        for i in 0..data.len().min(8) {
            features[40 + i] = (data[i] as f64) / 255.0;
        }

        let mut byte_counts = [0u32; 256];
        for &byte in data.iter().take(64) {
            byte_counts[byte as usize] += 1;
        }
        let mut entropy_bins = [0.0; 8];
        for (i, &count) in byte_counts.iter().enumerate() {
            if count > 0 {
                entropy_bins[i % 8] += count as f64;
            }
        }
        for i in 0..8 {
            features[48 + i] = entropy_bins[i] / 64.0;
        }

        for i in 0..data.len().saturating_sub(2).min(8) {
            let is_ascending = data[i] < data[i + 1] && data[i + 1] < data[i + 2];
            let is_descending = data[i] > data[i + 1] && data[i + 1] > data[i + 2];
            if is_ascending {
                features[56 + i] = 1.0;
            } else if is_descending {
                features[56 + i] = -1.0;
            }
        }

        features
    }

    pub fn predict(&self, features: &[f64]) -> f64 {
        let mut activation = self.bias;
        for (w, &f) in self.weights.iter().zip(features.iter()) {
            activation += w * f;
        }
        1.0 / (1.0 + (-activation).exp())
    }

    pub fn train(&mut self, attack: &[u8], is_threat: bool) {
        let features = self.extract_features(attack);
        let prediction = self.predict(&features);
        let target = if is_threat { 1.0 } else { 0.0 };
        let error = target - prediction;

        for (w, &f) in self.weights.iter_mut().zip(&features) {
            *w += self.learning_rate * error * f;
        }
        self.bias += self.learning_rate * error;
        self.training_count += 1;

        if self.training_count % 100 == 0 {
            self.learning_rate *= 0.99;
        }
    }

    pub fn is_learned_threat(&self, data: &[u8]) -> bool {
        let features = self.extract_features(data);
        let score = self.predict(&features);
        score > 0.5
    }

    pub fn threat_confidence(&self, data: &[u8]) -> f64 {
        let features = self.extract_features(data);
        self.predict(&features)
    }
}

pub struct ImmuneSystem {
    integrity_checksum: Vec<u8>,
    backup_state: Option<Vec<u8>>,
    redundant_fragments: Vec<Vec<u8>>,
    neural_learning: NeuralLearning,
    pub known_attacks: HashMap<Vec<u8>, u32>,
    pub compromises_detected: u64,
    pub successful_repairs: u64,
}

impl ImmuneSystem {
    pub fn new() -> Self {
        Self {
            integrity_checksum: Vec::new(),
            backup_state: None,
            redundant_fragments: Vec::new(),
            neural_learning: NeuralLearning::new(),
            known_attacks: HashMap::new(),
            compromises_detected: 0,
            successful_repairs: 0,
        }
    }

    pub fn store_clean_state(&mut self, data: &[u8]) {
        self.backup_state = Some(data.to_vec());
        self.integrity_checksum = self.compute_checksum(data);
        self.create_redundant_fragments(data);
    }

    fn compute_checksum(&self, data: &[u8]) -> Vec<u8> {
        let mut checksum = vec![0u8; 32];
        for (i, &byte) in data.iter().enumerate() {
            checksum[i % 32] ^= byte.wrapping_add((i % 256) as u8);
        }
        checksum
    }

    fn create_redundant_fragments(&mut self, data: &[u8]) {
        self.redundant_fragments.clear();
        let chunk_size = (data.len() / 4).max(1);

        for i in 0..4 {
            let start = i * chunk_size;
            let end = ((i + 1) * chunk_size).min(data.len());
            if start < data.len() {
                self.redundant_fragments.push(data[start..end].to_vec());
            }
        }
    }

    pub fn detect_compromise(&mut self, data: &[u8]) -> CompromiseReport {
        if self.integrity_checksum.is_empty() {
            self.store_clean_state(data);
            return CompromiseReport {
                is_compromised: false,
                corruption_rate: 0.0,
                threat_level: ThreatLevel::Clean,
            };
        }

        let current_checksum = self.compute_checksum(data);
        let is_corrupted = current_checksum != self.integrity_checksum;

        if is_corrupted {
            self.compromises_detected += 1;
            let corruption_rate = self.calculate_corruption_rate(data);

            let threat_level = match corruption_rate {
                r if r >= 0.5 => ThreatLevel::Critical,
                r if r >= 0.2 => ThreatLevel::High,
                r if r >= 0.1 => ThreatLevel::Medium,
                r if r > 0.0 => ThreatLevel::Low,
                _ => ThreatLevel::Clean,
            };

            CompromiseReport {
                is_compromised: true,
                corruption_rate,
                threat_level,
            }
        } else {
            CompromiseReport {
                is_compromised: false,
                corruption_rate: 0.0,
                threat_level: ThreatLevel::Clean,
            }
        }
    }

    fn calculate_corruption_rate(&self, data: &[u8]) -> f64 {
        if let Some(ref backup) = self.backup_state {
            let mut corrupted_bytes = 0;
            let len = data.len().min(backup.len());

            for i in 0..len {
                if data[i] != backup[i] {
                    corrupted_bytes += 1;
                }
            }

            (corrupted_bytes as f64) / (len as f64)
        } else {
            0.0
        }
    }

    pub fn activate_immune_response(&mut self, corrupted_data: &[u8], report: CompromiseReport) -> Vec<u8> {
        match report.threat_level {
            ThreatLevel::Critical => self.full_system_restore(),
            ThreatLevel::High | ThreatLevel::Medium => self.reconstruct_from_fragments(corrupted_data),
            ThreatLevel::Low => self.error_correction_repair(corrupted_data),
            ThreatLevel::Clean => corrupted_data.to_vec(),
        }
    }

    fn full_system_restore(&mut self) -> Vec<u8> {
        if let Some(ref backup) = self.backup_state {
            self.successful_repairs += 1;
            backup.clone()
        } else {
            Vec::new()
        }
    }

    fn reconstruct_from_fragments(&mut self, _corrupted: &[u8]) -> Vec<u8> {
        if self.redundant_fragments.is_empty() {
            return Vec::new();
        }

        let mut repaired = Vec::new();
        for fragment in &self.redundant_fragments {
            repaired.extend_from_slice(fragment);
        }

        self.successful_repairs += 1;
        repaired
    }

    fn error_correction_repair(&mut self, corrupted: &[u8]) -> Vec<u8> {
        if let Some(ref backup) = self.backup_state {
            let mut repaired = corrupted.to_vec();

            for i in 0..repaired.len().min(backup.len()) {
                if repaired[i] != backup[i] {
                    repaired[i] = backup[i];
                }
            }

            self.successful_repairs += 1;
            repaired
        } else {
            corrupted.to_vec()
        }
    }

    pub fn build_immunity(&mut self, attack_pattern: &[u8], is_malicious: bool) {
        self.neural_learning.train(attack_pattern, is_malicious);

        if is_malicious {
            let key = attack_pattern[..attack_pattern.len().min(32)].to_vec();
            *self.known_attacks.entry(key).or_insert(0) += 1;
        }
    }

    pub fn is_recognized_threat(&self, pattern: &[u8]) -> bool {
        if self.neural_learning.is_learned_threat(pattern) {
            return true;
        }

        if pattern.len() >= 32 {
            if self.known_attacks.contains_key(&pattern[..32]) {
                return true;
            }
        }

        false
    }

    pub fn threat_score(&self, pattern: &[u8]) -> f64 {
        self.neural_learning.threat_confidence(pattern)
    }
}

pub struct BineHasherV78 {
    state_size: usize,
    mode: SecurityMode,
    pub immune_system: ImmuneSystem,
}

impl BineHasherV78 {
    pub fn new(bits: usize, mode: SecurityMode) -> Self {
        Self {
            state_size: bits / 8,
            mode,
            immune_system: ImmuneSystem::new(),
        }
    }

    pub fn report_attack(&mut self, pattern: &[u8], is_malicious: bool) {
        self.immune_system.build_immunity(pattern, is_malicious);
    }

    pub fn hash(&mut self, data: &[u8], salt: &[u8]) -> Vec<u8> {
        let salted = [data, salt].concat();

        // v7.8: AGGRESSIVE INPUT WHITENING before mini_hash
        let whitened = self.whiten_input(&salted);

        let mut seed_data = whitened.clone();
        seed_data.extend_from_slice(b"seed");
        let deterministic_seed = mini_hash(&seed_data, self.state_size);

        let mut state = mini_hash(&whitened, self.state_size);

        let base_rounds = self.mode.rounds();
        let extra_rounds = if self.mode.use_learning() {
            self.compute_adaptive_rounds(data)
        } else {
            0
        };
        let total_rounds = base_rounds + extra_rounds;

        for round in 0..total_rounds {
            state = self.tardigrade_transform(&state, round, &deterministic_seed);
            state = self.jellyfish_regenerate(&state, round, &deterministic_seed);
            state = self.cockroach_distribute(&state, round);
        }

        state = self.ostrich_diversify(&state);
        state = self.alligator_defend(&state);
        state = self.enhanced_diffusion(&state);

        self.final_nonlinear_output(&state)
    }

    // v7.8: CRITICAL FIX - Input whitening to break regular patterns
    fn whiten_input(&self, data: &[u8]) -> Vec<u8> {
        if data.is_empty() {
            return Vec::new();
        }

        let len = data.len();
        let mut whitened = Vec::with_capacity(len);

        // Pass 1: Position-dependent XOR + S-box
        for i in 0..len {
            let byte = data[i];

            // XOR with position-dependent constants
            let pos_key1 = ((i * 251) % 256) as u8;
            let pos_key2 = ((i * 179) % 256) as u8;
            let pos_key3 = ((i.wrapping_mul(i) + 17) % 256) as u8;

            let xored = byte ^ pos_key1 ^ pos_key2 ^ pos_key3;

            // Apply S-box
            let sboxed = SBOX[xored as usize];

            whitened.push(sboxed);
        }

        // Pass 2: Neighborhood mixing
        let mut mixed = vec![0u8; len];
        for i in 0..len {
            let curr = whitened[i];
            let prev = whitened[if i > 0 { i - 1 } else { len - 1 }];
            let next = whitened[if i < len - 1 { i + 1 } else { 0 }];
            let far = whitened[(i + len / 2) % len];

            // Non-linear mixing
            let temp1 = curr.wrapping_add(prev).wrapping_mul(251);
            let temp2 = SBOX[temp1 as usize];
            let temp3 = temp2 ^ next ^ far;
            let temp4 = SBOX[temp3 as usize];

            mixed[i] = temp4.rotate_left((i % 8) as u32);
        }

        // Pass 3: Long-range diffusion
        let mut diffused = vec![0u8; len];
        for i in 0..len {
            let mut acc = mixed[i];

            // Mix with 4 distant positions
            let pos1 = (i + len / 4) % len;
            let pos2 = (i + len / 2) % len;
            let pos3 = (i + 3 * len / 4) % len;
            let pos4 = (i.wrapping_mul(7) + 13) % len;

            acc = acc.wrapping_add(mixed[pos1]).wrapping_mul(251);
            acc = SBOX[acc as usize];
            acc ^= mixed[pos2];
            acc = acc.wrapping_add(mixed[pos3]).wrapping_mul(179);
            acc = SBOX[acc as usize];
            acc ^= mixed[pos4];

            diffused[i] = acc;
        }

        diffused
    }

    fn compute_adaptive_rounds(&self, data: &[u8]) -> usize {
        let threat_score = self.immune_system.threat_score(data);
        let exact_count = if data.len() >= 32 {
            self.immune_system.known_attacks.get(&data[..32].to_vec())
                .copied().unwrap_or(0)
        } else {
            0
        };

        let neural_rounds = (threat_score * 100.0) as usize * LEARNING_ROUNDS_MULTIPLIER;
        let exact_rounds = exact_count as usize * LEARNING_ROUNDS_MULTIPLIER;

        neural_rounds + exact_rounds
    }

    fn tardigrade_transform(&self, state: &[u8], round: usize, seed: &[u8]) -> Vec<u8> {
        let round_key = self.derive_round_key(seed, round);
        let mut transformed = Vec::with_capacity(state.len());

        for i in 0..state.len() {
            let s = state[i];
            let k = round_key[i % round_key.len()];
            let prev = if i > 0 { state[i - 1] } else { state[state.len() - 1] };

            let mixed = s.wrapping_add(k).wrapping_mul(251);
            let sboxed = SBOX[mixed as usize];
            let val = sboxed.rotate_left(3) ^ prev.rotate_right(5);

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
                *byte = byte.wrapping_add(variant as u8).rotate_left((variant % 8) as u32);
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

    fn enhanced_diffusion(&self, data: &[u8]) -> Vec<u8> {
        let mut diffused = data.to_vec();

        for pass in 0..5 {
            for i in 0..diffused.len() {
                let prev = if i > 0 { diffused[i - 1] } else { diffused[diffused.len() - 1] };
                let next = if i < diffused.len() - 1 { diffused[i + 1] } else { diffused[0] };
                let far = diffused[(i + diffused.len() / 2) % diffused.len()];
                let very_far = diffused[(i + diffused.len() / 4) % diffused.len()];

                let mix1 = prev.wrapping_add(next);
                let mix2 = far.rotate_left(pass as u32);
                let mix3 = very_far.rotate_right(pass as u32);

                diffused[i] ^= mix1 ^ mix2 ^ mix3;
                diffused[i] = diffused[i].wrapping_mul(251);
                diffused[i] = diffused[i].rotate_left((pass % 8) as u32);
            }
        }

        diffused
    }

    fn final_nonlinear_output(&self, state: &[u8]) -> Vec<u8> {
        let checksum_data = [state, b"final"].concat();
        let checksum = mini_hash(&checksum_data, 16);

        let mut combined = state.to_vec();
        combined.extend_from_slice(&checksum);

        for byte in combined.iter_mut() {
            *byte = SBOX[*byte as usize];
        }

        let len = combined.len();
        let mut output = vec![0u8; len];
        for i in 0..len {
            let curr = combined[i];
            let prev = combined[if i > 0 { i - 1 } else { len - 1 }];
            let next = combined[if i < len - 1 { i + 1 } else { 0 }];

            let temp1 = SBOX[curr as usize];
            let temp2 = SBOX[(temp1 ^ prev) as usize];
            let temp3 = SBOX[(temp2 ^ next) as usize];

            output[i] = temp3.rotate_left((i % 8) as u32);
        }

        output
    }

    fn derive_round_key(&self, seed: &[u8], round: usize) -> Vec<u8> {
        let round_data = [seed, &round.to_le_bytes()].concat();
        mini_hash(&round_data, self.state_size)
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
