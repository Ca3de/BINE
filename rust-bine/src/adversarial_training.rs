/*!
BINE v7.0+ - Continuous Adversarial Training & Sandbox Learning
================================================================

User's reinforcement strategy:
1. "Keep bashing and smashing with variations of attacks to strengthen it,
    everyday, different variations, multivariations."

2. Options for learning:
   A. Test in sandbox, allow if safe
   B. Allow everything, rebuild if destroyed, learn
   C. Both (hybrid)

This implements ACTIVE LEARNING - learning by doing, like biological immune systems!

Key concepts:
- Adversarial training: Generate attack variations daily
- Sandbox execution: Test commands in isolated environment
- Damage-based learning: Learn from what actually causes harm
- Continuous reinforcement: System gets stronger through exposure
*/

use std::time::{SystemTime, Duration, UNIX_EPOCH};
use std::collections::HashMap;

// Simple pseudo-random generator (no external dependencies)
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn from_entropy() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        Self { state: seed }
    }

    fn gen_range(&mut self, min: usize, max: usize) -> usize {
        if max <= min {
            return min;
        }
        let range = (max - min) as u64;
        let val = self.next_u64() % range;
        min + val as usize
    }

    fn gen(&mut self) -> u8 {
        (self.next_u64() & 0xFF) as u8
    }

    fn next_u64(&mut self) -> u64 {
        // Linear Congruential Generator
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.state
    }
}

/// Generates attack variations for continuous training
pub struct AdversarialTrainer {
    base_attacks: Vec<Vec<u8>>,
    variation_strategies: Vec<VariationStrategy>,
    training_history: Vec<TrainingSession>,
    pub total_variations_generated: u64,
}

#[derive(Clone)]
pub enum VariationStrategy {
    ByteFlip,           // Flip random bytes
    ByteInsert,         // Insert random bytes
    ByteDelete,         // Delete random bytes
    ByteSwap,           // Swap byte positions
    PatternMutation,    // Mutate patterns
    Concatenation,      // Combine multiple attacks
    Encoding,           // Different encodings (hex, base64, etc)
}

pub struct TrainingSession {
    pub date: SystemTime,
    pub variations_tested: usize,
    pub threats_learned: usize,
}

impl AdversarialTrainer {
    pub fn new() -> Self {
        Self {
            base_attacks: Vec::new(),
            variation_strategies: vec![
                VariationStrategy::ByteFlip,
                VariationStrategy::ByteInsert,
                VariationStrategy::ByteDelete,
                VariationStrategy::ByteSwap,
                VariationStrategy::PatternMutation,
                VariationStrategy::Concatenation,
                VariationStrategy::Encoding,
            ],
            training_history: Vec::new(),
            total_variations_generated: 0,
        }
    }

    /// Load base attacks from known attack database
    pub fn load_base_attacks(&mut self, attacks: Vec<Vec<u8>>) {
        self.base_attacks = attacks;
    }

    /// Generate variations of attacks for continuous training
    pub fn generate_daily_variations(&mut self, count: usize) -> Vec<Vec<u8>> {
        let mut variations = Vec::new();
        let mut rng = SimpleRng::from_entropy();

        for _ in 0..count {
            if self.base_attacks.is_empty() {
                break;
            }

            // Pick random base attack
            let base_idx = rng.gen_range(0, self.base_attacks.len());
            let base_attack = &self.base_attacks[base_idx];

            // Pick random variation strategy
            let strategy_idx = rng.gen_range(0, self.variation_strategies.len());
            let strategy = &self.variation_strategies[strategy_idx];

            // Generate variation
            let variation = self.apply_variation(base_attack, strategy, &mut rng);
            variations.push(variation);
        }

        self.total_variations_generated += count as u64;
        variations
    }

    fn apply_variation(&self, base: &[u8], strategy: &VariationStrategy, rng: &mut SimpleRng) -> Vec<u8> {
        match strategy {
            VariationStrategy::ByteFlip => {
                let mut mutated = base.to_vec();
                if !mutated.is_empty() {
                    let pos = rng.gen_range(0, mutated.len());
                    mutated[pos] ^= 0xFF;
                }
                mutated
            }
            VariationStrategy::ByteInsert => {
                let mut mutated = base.to_vec();
                if !mutated.is_empty() {
                    let pos = rng.gen_range(0, mutated.len());
                    let byte = rng.gen();
                    mutated.insert(pos, byte);
                }
                mutated
            }
            VariationStrategy::ByteDelete => {
                let mut mutated = base.to_vec();
                if mutated.len() > 1 {
                    let pos = rng.gen_range(0, mutated.len());
                    mutated.remove(pos);
                }
                mutated
            }
            VariationStrategy::ByteSwap => {
                let mut mutated = base.to_vec();
                if mutated.len() > 1 {
                    let pos1 = rng.gen_range(0, mutated.len());
                    let pos2 = rng.gen_range(0, mutated.len());
                    mutated.swap(pos1, pos2);
                }
                mutated
            }
            VariationStrategy::PatternMutation => {
                // Replace patterns (e.g., OR -> AND, UNION -> JOIN)
                let mut mutated = base.to_vec();
                let replacements: Vec<(&[u8], &[u8])> = vec![
                    (b"OR" as &[u8], b"AND" as &[u8]),
                    (b"UNION" as &[u8], b"JOIN" as &[u8]),
                    (b"SELECT" as &[u8], b"INSERT" as &[u8]),
                    (b"'" as &[u8], b"\"" as &[u8]),
                    (b"--" as &[u8], b"#" as &[u8]),
                ];

                for (from, to) in replacements {
                    if let Some(pos) = mutated.windows(from.len()).position(|w| w == from) {
                        for (i, &byte) in to.iter().enumerate() {
                            if pos + i < mutated.len() {
                                mutated[pos + i] = byte;
                            }
                        }
                        break;
                    }
                }
                mutated
            }
            VariationStrategy::Concatenation => {
                // Combine with another random attack
                if self.base_attacks.len() > 1 {
                    let other_idx = rng.gen_range(0, self.base_attacks.len());
                    let mut combined = base.to_vec();
                    combined.extend_from_slice(&self.base_attacks[other_idx]);
                    combined
                } else {
                    base.to_vec()
                }
            }
            VariationStrategy::Encoding => {
                // Simple encoding variations (hex encoding as example)
                let mut encoded = Vec::new();
                for &byte in base {
                    let hex = format!("{:02x}", byte);
                    encoded.extend_from_slice(hex.as_bytes());
                }
                encoded
            }
        }
    }

    /// Record training session
    pub fn record_session(&mut self, variations_tested: usize, threats_learned: usize) {
        self.training_history.push(TrainingSession {
            date: SystemTime::now(),
            variations_tested,
            threats_learned,
        });
    }
}

/// Sandbox execution environment for learning from real attempts
pub struct SandboxExecutor {
    pub execution_history: Vec<ExecutionResult>,
    pub safe_commands: HashMap<Vec<u8>, u32>,
    pub harmful_commands: HashMap<Vec<u8>, DamageReport>,
}

pub struct ExecutionResult {
    pub command: Vec<u8>,
    pub was_safe: bool,
    pub damage_level: f64,
    pub execution_time: Duration,
}

#[derive(Clone)]
pub struct DamageReport {
    pub corruption_detected: bool,
    pub data_loss: f64,
    pub system_compromise: bool,
}

impl SandboxExecutor {
    pub fn new() -> Self {
        Self {
            execution_history: Vec::new(),
            safe_commands: HashMap::new(),
            harmful_commands: HashMap::new(),
        }
    }

    /// Option A: Test command in sandbox, allow if safe
    pub fn test_and_allow(&mut self, command: &[u8], sandbox_state: &[u8]) -> (bool, DamageReport) {
        let start = SystemTime::now();

        // Simulate execution in isolated sandbox
        let (is_safe, damage) = self.execute_in_sandbox(command, sandbox_state);

        let execution_time = SystemTime::now().duration_since(start).unwrap_or(Duration::from_secs(0));

        // Record result
        self.execution_history.push(ExecutionResult {
            command: command.to_vec(),
            was_safe: is_safe,
            damage_level: damage.data_loss,
            execution_time,
        });

        // Update knowledge
        if is_safe {
            *self.safe_commands.entry(command.to_vec()).or_insert(0) += 1;
        } else {
            self.harmful_commands.insert(command.to_vec(), damage.clone());
        }

        (is_safe, damage)
    }

    fn execute_in_sandbox(&self, command: &[u8], state: &[u8]) -> (bool, DamageReport) {
        // Simulate command execution and damage assessment

        // Check for known attack patterns
        let known_bad_patterns: Vec<&[u8]> = vec![
            b"DROP TABLE" as &[u8],
            b"DELETE FROM" as &[u8],
            b"rm -rf" as &[u8],
            b"FORMAT C:" as &[u8],
            b"../../etc/passwd" as &[u8],
        ];

        let mut damage_detected = false;
        for pattern in known_bad_patterns {
            if command.windows(pattern.len()).any(|w| w == pattern) {
                damage_detected = true;
                break;
            }
        }

        // Simulate state corruption
        let corruption = if damage_detected {
            // High damage
            0.8
        } else {
            // Random low-level noise
            (command.len() % 10) as f64 / 100.0
        };

        let damage = DamageReport {
            corruption_detected: damage_detected,
            data_loss: corruption,
            system_compromise: corruption > 0.5,
        };

        let is_safe = !damage_detected && corruption < 0.1;

        (is_safe, damage)
    }
}

/// Rebuild-and-Learn system (Option B & C)
pub struct RebuildAndLearn {
    pub allow_and_learn: bool,  // Option B: Allow everything
    pub test_first: bool,        // Option A: Test in sandbox first

    rebuild_count: u64,
    learned_from_damage: Vec<LearnedThreat>,
}

#[derive(Clone)]
pub struct LearnedThreat {
    pub command: Vec<u8>,
    pub damage_caused: f64,
    pub recovery_cost: Duration,
    pub threat_signature: Vec<f64>, // Neural features
}

impl RebuildAndLearn {
    /// Option A: Test first
    pub fn new_test_first() -> Self {
        Self {
            allow_and_learn: false,
            test_first: true,
            rebuild_count: 0,
            learned_from_damage: Vec::new(),
        }
    }

    /// Option B: Allow and rebuild
    pub fn new_allow_and_rebuild() -> Self {
        Self {
            allow_and_learn: true,
            test_first: false,
            rebuild_count: 0,
            learned_from_damage: Vec::new(),
        }
    }

    /// Option C: Both (hybrid)
    pub fn new_hybrid() -> Self {
        Self {
            allow_and_learn: true,
            test_first: true,
            rebuild_count: 0,
            learned_from_damage: Vec::new(),
        }
    }

    /// Execute with chosen strategy
    pub fn execute_with_learning<F>(
        &mut self,
        command: &[u8],
        clean_state: &[u8],
        mut execute_fn: F,
        mut rebuild_fn: impl FnMut() -> Vec<u8>,
    ) -> Result<Vec<u8>, LearnedThreat>
    where
        F: FnMut(&[u8]) -> (Vec<u8>, f64), // (result, damage_level)
    {
        let start = SystemTime::now();

        if self.test_first {
            // Option A or C: Test in sandbox first
            // For now, simplified - just execute and check
        }

        // Execute command
        let (result, damage) = execute_fn(command);

        // Check if rebuild needed
        if damage > 0.1 {
            // Damage detected! Learn from it
            println!("⚠️  Damage detected ({:.1}%), activating rebuild...", damage * 100.0);

            self.rebuild_count += 1;

            // Rebuild system
            let rebuilt_state = rebuild_fn();

            let recovery_time = SystemTime::now().duration_since(start).unwrap_or(Duration::from_secs(0));

            // LEARN from this damage
            let learned_threat = LearnedThreat {
                command: command.to_vec(),
                damage_caused: damage,
                recovery_cost: recovery_time,
                threat_signature: self.extract_threat_features(command),
            };

            self.learned_from_damage.push(learned_threat.clone());

            println!("✓ System rebuilt! Learned threat signature.");
            println!("  Rebuild #{}", self.rebuild_count);
            println!("  Recovery time: {:?}", recovery_time);

            Err(learned_threat)
        } else {
            // No damage, safe command
            Ok(result)
        }
    }

    fn extract_threat_features(&self, command: &[u8]) -> Vec<f64> {
        // Extract features for learning (same as neural network)
        let mut features = vec![0.0; 16];

        // Byte frequency
        for &byte in command.iter().take(256) {
            let bin = (byte as usize) / 16;
            if bin < 16 {
                features[bin] += 1.0;
            }
        }

        // Normalize
        let total = command.len() as f64;
        for f in features.iter_mut() {
            *f /= total.max(1.0);
        }

        features
    }

    pub fn get_rebuild_count(&self) -> u64 {
        self.rebuild_count
    }

    pub fn get_learned_threats(&self) -> &[LearnedThreat] {
        &self.learned_from_damage
    }
}

/// Continuous reinforcement training loop
pub fn daily_training_loop<F>(
    adversarial_trainer: &mut AdversarialTrainer,
    mut train_fn: F,
    variations_per_day: usize,
) where
    F: FnMut(&[u8], bool),
{
    println!("\n🏋️ Starting daily adversarial training...");
    println!("   Generating {} attack variations...", variations_per_day);

    // Generate variations
    let variations = adversarial_trainer.generate_daily_variations(variations_per_day);

    // Train on each variation
    let mut threats_learned = 0;
    for variation in &variations {
        // All variations are threats (we generated them from attacks)
        train_fn(variation, true);
        threats_learned += 1;
    }

    // Also train on some clean data (balance)
    let clean_samples: Vec<&[u8]> = vec![
        b"normal_user_input" as &[u8],
        b"email@example.com" as &[u8],
        b"SELECT * FROM users WHERE id = 1" as &[u8],
        b"hello world" as &[u8],
    ];

    for clean in &clean_samples {
        train_fn(clean, false);
    }

    // Record session
    adversarial_trainer.record_session(variations.len() + clean_samples.len(), threats_learned);

    println!("✓ Training complete!");
    println!("   Variations tested: {}", variations.len());
    println!("   Threats learned: {}", threats_learned);
    println!("   Total variations generated: {}", adversarial_trainer.total_variations_generated);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adversarial_variations() {
        let mut trainer = AdversarialTrainer::new();

        // Load base attacks
        trainer.load_base_attacks(vec![
            b"' OR '1'='1".to_vec(),
            b"admin'--".to_vec(),
        ]);

        // Generate variations
        let variations = trainer.generate_daily_variations(10);

        assert_eq!(variations.len(), 10);
        assert!(trainer.total_variations_generated == 10);
    }

    #[test]
    fn test_sandbox_execution() {
        let mut sandbox = SandboxExecutor::new();
        let clean_state = b"CLEAN_DATA";

        // Test safe command
        let (is_safe, _) = sandbox.test_and_allow(b"SELECT * FROM users", clean_state);
        assert!(is_safe);

        // Test harmful command
        let (is_safe, damage) = sandbox.test_and_allow(b"DROP TABLE users", clean_state);
        assert!(!is_safe);
        assert!(damage.corruption_detected);
    }

    #[test]
    fn test_rebuild_and_learn() {
        let mut system = RebuildAndLearn::new_allow_and_rebuild();

        let clean_state = vec![1, 2, 3, 4, 5];
        let mut current_state = clean_state.clone();

        // Execute harmful command
        let result = system.execute_with_learning(
            b"ATTACK",
            &clean_state,
            |cmd| {
                // Simulate damage
                (vec![0xFF; 5], 0.5) // 50% damage
            },
            || {
                // Rebuild
                clean_state.clone()
            },
        );

        // Should have learned from damage
        assert!(result.is_err());
        assert_eq!(system.get_rebuild_count(), 1);
        assert_eq!(system.get_learned_threats().len(), 1);
    }
}
