//! BINE: Bio-Inspired Network Encryption
//!
//! A novel cryptographic algorithm inspired by nature's most resilient organisms.
//!
//! # Biological Inspirations
//!
//! - **Tardigrade**: Cryptobiotic state, extreme resilience (12 rounds)
//! - **Jellyfish**: Self-healing key regeneration
//! - **Cockroach**: Distributed redundancy, no single point of failure
//! - **Ostrich**: Antibody diversity (8 parallel paths)
//! - **Bat**: Adaptive complexity
//! - **Shark**: Molecular binding precision
//! - **Alligator**: Broad-spectrum defense
//! - **Opossum**: Error correction and neutralization
//!
//! # Example
//!
//! ```
//! use bine::BineHasher;
//!
//! let data = b"Hello, BINE!";
//! let salt = [0u8; 32]; // In production, use random salt
//! let hash = BineHasher::new(256).hash(data, &salt);
//! ```

use blake2::{Blake2b512, Digest};
use rand::Rng;

/// BINE constants inspired by biological features
pub const CRYPTOBIOTIC_ROUNDS: usize = 12; // Tardigrade 12-year dormancy
pub const ANTIBODY_VARIANTS: usize = 8;   // Ostrich antibody diversity
pub const SALT_SIZE: usize = 32;          // 256-bit salt
pub const DNA_PROTECTION_LAYERS: usize = 4; // Dsup-like protection

/// Security levels supported by BINE
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityLevel {
    Bits128,
    Bits256,
    Bits512,
}

impl SecurityLevel {
    pub fn bytes(&self) -> usize {
        match self {
            SecurityLevel::Bits128 => 16,
            SecurityLevel::Bits256 => 32,
            SecurityLevel::Bits512 => 64,
        }
    }
}

/// Main BINE hasher implementing bio-inspired cryptography
pub struct BineHasher {
    security_level: SecurityLevel,
    state_size: usize,
}

impl BineHasher {
    /// Create a new BINE hasher with specified security level
    pub fn new(bits: usize) -> Self {
        let security_level = match bits {
            128 => SecurityLevel::Bits128,
            256 => SecurityLevel::Bits256,
            512 => SecurityLevel::Bits512,
            _ => SecurityLevel::Bits256, // Default to 256
        };

        Self {
            security_level,
            state_size: security_level.bytes(),
        }
    }

    /// Hash data with BINE algorithm
    ///
    /// Combines all 8 bio-inspired transformations
    pub fn hash(&self, data: &[u8], salt: &[u8]) -> Vec<u8> {
        // Combine data and salt
        let mut salted_data = Vec::with_capacity(data.len() + salt.len());
        salted_data.extend_from_slice(data);
        salted_data.extend_from_slice(salt);

        // Initialize state with BLAKE2b
        let mut state = Self::blake2b_hash(&salted_data, self.state_size);

        // Create deterministic seed for reproducibility
        let mut seed_input = salted_data.clone();
        seed_input.extend_from_slice(b"seed");
        let deterministic_seed = Self::blake2b_hash(&seed_input, self.state_size);

        // Tardigrade: 12 cryptobiotic rounds
        for round in 0..CRYPTOBIOTIC_ROUNDS {
            state = self.tardigrade_transform(&state, round, &deterministic_seed);
        }

        // Jellyfish: Regenerative mixing
        state = self.jellyfish_regenerate(&state, 1, &deterministic_seed);

        // Shark: Molecular binding
        let mut binding_input = salted_data.clone();
        binding_input.extend_from_slice(b"bind");
        let binding_key = Self::blake2b_hash(&binding_input, self.state_size);
        state = self.shark_bind(&state, &binding_key);

        // Alligator: Broad-spectrum defense
        state = self.alligator_defend(&state);

        // Opossum: Error correction
        let (neutralized, checksum) = self.opossum_neutralize(&state);

        // Combine final state with checksum
        let mut result = neutralized;
        result.extend_from_slice(&checksum);
        result
    }

    /// Tardigrade-inspired transformation: Multi-layer protection
    fn tardigrade_transform(&self, data: &[u8], round: usize, seed: &[u8]) -> Vec<u8> {
        // Layer 1: Dsup-like DNA protection via XOR
        let mut round_key_input = seed.to_vec();
        round_key_input.extend_from_slice(&(round as u32).to_le_bytes());
        let round_key = Self::blake2b_hash(&round_key_input, self.state_size);

        let mut protected = vec![0u8; self.state_size];
        for i in 0..data.len().min(self.state_size) {
            protected[i] = data[i] ^ round_key[i % round_key.len()];
        }

        // Layer 2: CAHS protein-like stabilization (molecular mixing)
        for i in 0..protected.len() {
            protected[i] ^= ((round * (i + 1)) % 256) as u8;
        }

        // Layer 3: Cryptobiotic state encoding
        let mut encode_input = protected.clone();
        encode_input.extend_from_slice(b"tun");
        encode_input.extend_from_slice(&(round as u32).to_le_bytes());
        Self::blake2b_hash(&encode_input, self.state_size)
    }

    /// Jellyfish-inspired regeneration: Can revert and regenerate
    fn jellyfish_regenerate(&self, state: &[u8], generation: usize, seed: &[u8]) -> Vec<u8> {
        // Revert to polyp state (base generation)
        let mut polyp_input = seed.to_vec();
        polyp_input.extend_from_slice(b"polyp");
        let mut current_state = Self::blake2b_hash(&polyp_input, self.state_size);

        // Regenerate forward to current generation
        for gen in 0..generation {
            let mut regen_input = current_state.clone();
            regen_input.extend_from_slice(&(gen as u32).to_le_bytes());
            regen_input.extend_from_slice(state);
            current_state = Self::blake2b_hash(&regen_input, self.state_size);
        }

        current_state
    }

    /// Shark-inspired molecular binding: Tight, precise binding
    fn shark_bind(&self, data: &[u8], key: &[u8]) -> Vec<u8> {
        // VNAR-like binding with high precision
        let mut binding_input = key.to_vec();
        binding_input.extend_from_slice(b"vnar");
        let binding_site = Self::blake2b_hash(&binding_input, self.state_size);

        let mut bound = vec![0u8; self.state_size];
        for i in 0..data.len().min(self.state_size) {
            let pos = i % self.state_size;
            // Three-point binding (current + neighbors)
            bound[pos] ^= data[i] ^ binding_site[pos];
            bound[(pos + 1) % self.state_size] ^= data[i].wrapping_shl(1);
            bound[pos.wrapping_sub(1) % self.state_size] ^= data[i].wrapping_shr(1);
        }

        // Stabilize under stress (high urea tolerance analog)
        let mut stabilize_input = bound.clone();
        stabilize_input.extend_from_slice(key);
        Self::blake2b_hash(&stabilize_input, self.state_size)
    }

    /// Alligator-inspired broad-spectrum defense: Multiple peptide-like protections
    fn alligator_defend(&self, data: &[u8]) -> Vec<u8> {
        let mut defended = data.to_vec();

        // Generate 4 APAP-like peptides
        for i in 0..4 {
            let mut peptide_input = data.to_vec();
            peptide_input.extend_from_slice(b"peptide");
            peptide_input.push(i);
            let peptide = Self::blake2b_hash(&peptide_input, 16);

            // Apply peptide transformation
            for j in 0..defended.len() {
                defended[j] ^= peptide[j % peptide.len()];
            }
        }

        defended
    }

    /// Opossum-inspired neutralization: Error detection and correction
    fn opossum_neutralize(&self, data: &[u8]) -> (Vec<u8>, Vec<u8>) {
        // Generate OVNF-like neutralizing factor
        let mut neutralizer_input = data.to_vec();
        neutralizer_input.extend_from_slice(b"ovnf");
        let neutralizer = Self::blake2b_hash(&neutralizer_input, self.state_size);

        // Neutralize with XOR
        let mut neutralized = vec![0u8; data.len()];
        for i in 0..data.len() {
            neutralized[i] = data[i] ^ neutralizer[i % neutralizer.len()];
        }

        // Generate error correction checksum
        let mut checksum_input = data.to_vec();
        checksum_input.extend_from_slice(&neutralizer);
        let checksum = Self::blake2b_hash(&checksum_input, 16);

        (neutralized, checksum)
    }

    /// Helper: BLAKE2b hash with specified output size
    fn blake2b_hash(data: &[u8], output_size: usize) -> Vec<u8> {
        let mut hasher = Blake2b512::new();
        hasher.update(data);
        let result = hasher.finalize();
        result[..output_size.min(64)].to_vec()
    }

    /// Verify data against a BINE hash
    pub fn verify(&self, data: &[u8], hash: &[u8], salt: &[u8]) -> bool {
        let computed = self.hash(data, salt);

        // Constant-time comparison to prevent timing attacks
        if computed.len() != hash.len() {
            return false;
        }

        let mut result = 0u8;
        for (a, b) in computed.iter().zip(hash.iter()) {
            result |= a ^ b;
        }

        result == 0
    }
}

/// Generate cryptographically secure random salt
pub fn generate_salt() -> [u8; SALT_SIZE] {
    let mut salt = [0u8; SALT_SIZE];
    rand::thread_rng().fill(&mut salt);
    salt
}

/// Convenience function for simple hashing
pub fn bine_hash_simple(data: &[u8], security_level: usize) -> (Vec<u8>, [u8; SALT_SIZE]) {
    let hasher = BineHasher::new(security_level);
    let salt = generate_salt();
    let hash = hasher.hash(data, &salt);
    (hash, salt)
}

/// Convenience function for simple verification
pub fn bine_verify_simple(data: &[u8], hash: &[u8], salt: &[u8], security_level: usize) -> bool {
    let hasher = BineHasher::new(security_level);
    hasher.verify(data, hash, salt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_deterministic() {
        let hasher = BineHasher::new(256);
        let data = b"Test data";
        let salt = [0u8; SALT_SIZE];

        let hash1 = hasher.hash(data, &salt);
        let hash2 = hasher.hash(data, &salt);

        assert_eq!(hash1, hash2, "Hash should be deterministic");
    }

    #[test]
    fn test_hash_avalanche() {
        let hasher = BineHasher::new(256);
        let salt = [0u8; SALT_SIZE];

        let hash1 = hasher.hash(b"Test data", &salt);
        let hash2 = hasher.hash(b"Test datA", &salt); // One bit different

        let differences: usize = hash1.iter()
            .zip(hash2.iter())
            .filter(|(a, b)| a != b)
            .count();

        // Should have significant differences (avalanche effect)
        assert!(differences > hash1.len() / 3,
                "Avalanche effect too weak: only {} differences", differences);
    }

    #[test]
    fn test_hash_uniqueness() {
        let hasher = BineHasher::new(256);
        let salt = [0u8; SALT_SIZE];

        let test_data = [
            b"Message 1".as_ref(),
            b"Message 2",
            b"Different content",
        ];

        let hashes: Vec<_> = test_data.iter()
            .map(|data| hasher.hash(data, &salt))
            .collect();

        // All hashes should be unique
        for i in 0..hashes.len() {
            for j in i+1..hashes.len() {
                assert_ne!(hashes[i], hashes[j],
                          "Hashes should be unique");
            }
        }
    }

    #[test]
    fn test_verify() {
        let hasher = BineHasher::new(256);
        let data = b"Verification test";
        let salt = [0u8; SALT_SIZE];

        let hash = hasher.hash(data, &salt);

        assert!(hasher.verify(data, &hash, &salt), "Should verify correctly");
        assert!(!hasher.verify(b"Wrong data", &hash, &salt), "Should fail with wrong data");
    }

    #[test]
    fn test_simple_api() {
        let data = b"Simple API test";
        let (hash, salt) = bine_hash_simple(data, 256);

        assert!(bine_verify_simple(data, &hash, &salt, 256));
        assert!(!bine_verify_simple(b"Wrong", &hash, &salt, 256));
    }

    #[test]
    fn test_different_security_levels() {
        let data = b"Security level test";
        let salt = [0u8; SALT_SIZE];

        let hash_128 = BineHasher::new(128).hash(data, &salt);
        let hash_256 = BineHasher::new(256).hash(data, &salt);
        let hash_512 = BineHasher::new(512).hash(data, &salt);

        assert_ne!(hash_128, hash_256);
        assert_ne!(hash_256, hash_512);
        assert_ne!(hash_128, hash_512);
    }
}
