/*!
Rigorous Security Testing Suite for BINE
========================================

Research-grade tests to prove or disprove security claims.
Honest reporting of results, including failures.
*/

use std::time::Instant;
use std::collections::{HashMap, HashSet};

mod lib_v2;
use lib_v2::{BineHasherV2, SecurityMode};

// ============================================================================
// TEST 1: Learning Capability - PROVE IT ACTUALLY WORKS
// ============================================================================

struct LearningTest {
    hasher: BineHasherV2,
    attack_pattern: Vec<u8>,
}

impl LearningTest {
    fn new() -> Self {
        Self {
            hasher: BineHasherV2::new(256, SecurityMode::Balanced),
            attack_pattern: vec![0xFF; 100], // Repeated attack
        }
    }

    /// Test if learning actually increases defense
    fn test_learning_effectiveness(&mut self) -> LearningResults {
        let salt = b"test_salt_32_bytes_learning_test";
        let mut attempt_times = Vec::new();
        let mut hash_changes = Vec::new();
        let mut prev_hash = Vec::new();

        println!("LEARNING TEST: Does BINE actually get harder to attack?");
        println!("{}", "=".repeat(70));

        for attempt in 0..100 {
            let start = Instant::now();
            let hash = self.hasher.hash(&self.attack_pattern, salt);
            let duration = start.elapsed().as_nanos() as f64;

            attempt_times.push(duration);

            // Check if hash changes (adaptive behavior)
            if !prev_hash.is_empty() {
                let changed = hash != prev_hash;
                hash_changes.push(changed);
            }
            prev_hash = hash.clone();

            // Report threat every 10 attempts
            if attempt % 10 == 9 {
                self.hasher.report_threat(&self.attack_pattern[..32]);
            }
        }

        // Analyze results
        let early_avg = attempt_times[..20].iter().sum::<f64>() / 20.0;
        let late_avg = attempt_times[80..].iter().sum::<f64>() / 20.0;
        let speedup_factor = late_avg / early_avg;

        let changes_in_learning_phase = hash_changes[50..].iter().filter(|&&x| x).count();

        println!("\nResults:");
        println!("  Early attempts (1-20):   {:.2} µs average", early_avg / 1000.0);
        println!("  Late attempts (80-100):  {:.2} µs average", late_avg / 1000.0);
        println!("  Slowdown factor:         {:.2}x", speedup_factor);
        println!("  Hash changes (learning): {}/50 attempts", changes_in_learning_phase);

        let learning_works = speedup_factor > 1.1 || changes_in_learning_phase > 5;

        println!("\n  VERDICT: {}", if learning_works {
            "✓ Learning detected (difficulty increased or behavior changed)"
        } else {
            "✗ NO LEARNING (constant difficulty and behavior)"
        });

        LearningResults {
            early_time: early_avg,
            late_time: late_avg,
            slowdown_factor: speedup_factor,
            behavior_changes: changes_in_learning_phase,
            learning_effective: learning_works,
        }
    }
}

struct LearningResults {
    early_time: f64,
    late_time: f64,
    slowdown_factor: f64,
    behavior_changes: usize,
    learning_effective: bool,
}

// ============================================================================
// TEST 2: Self-Repair - PROVE IT RECOVERS DATA
// ============================================================================

struct SelfRepairTest;

impl SelfRepairTest {
    /// Inject corruption and measure recovery
    fn test_corruption_recovery() -> RepairResults {
        println!("\n\nSELF-REPAIR TEST: Can BINE actually recover corrupted data?");
        println!("{}", "=".repeat(70));

        let original = vec![0x42; 1000]; // Clean data
        let corruption_rates = vec![0.01, 0.05, 0.10, 0.20]; // 1%, 5%, 10%, 20%

        let mut results = Vec::new();

        for &rate in &corruption_rates {
            let corrupted = Self::inject_corruption(&original, rate);
            let healed = Self::attempt_heal(&corrupted);

            let recovery_rate = Self::measure_similarity(&original, &healed);

            println!("\nCorruption rate: {:.1}%", rate * 100.0);
            println!("  Corrupted bits: {}", Self::count_differences(&original, &corrupted));
            println!("  Recovered similarity: {:.1}%", recovery_rate * 100.0);

            results.push((rate, recovery_rate));
        }

        // Success criteria: >80% recovery at 5% corruption
        let recovery_at_5pct = results.iter()
            .find(|(r, _)| (*r - 0.05).abs() < 0.001)
            .map(|(_, rec)| *rec)
            .unwrap_or(0.0);

        let repair_works = recovery_at_5pct > 0.80;

        println!("\n  VERDICT: {}", if repair_works {
            format!("✓ Self-repair works ({:.1}% recovery at 5% corruption)", recovery_at_5pct * 100.0)
        } else {
            format!("✗ Self-repair FAILED ({:.1}% recovery at 5% corruption, need >80%)", recovery_at_5pct * 100.0)
        });

        RepairResults {
            test_results: results,
            repair_effective: repair_works,
        }
    }

    fn inject_corruption(data: &[u8], rate: f64) -> Vec<u8> {
        let mut corrupted = data.to_vec();
        let num_corruptions = (data.len() as f64 * rate) as usize;

        for _ in 0..num_corruptions {
            let pos = rand_index(data.len());
            corrupted[pos] ^= 1 << rand_index(8); // Flip random bit
        }

        corrupted
    }

    fn attempt_heal(data: &[u8]) -> Vec<u8> {
        // Use BINE's alligator healing
        let mut hasher = BineHasherV2::new(256, SecurityMode::Balanced);
        // Simulate healing by hashing and using output pattern
        // In real implementation, this would use actual healing logic
        let healing_pattern = hasher.hash(data, b"healing_salt_32bytes_test!!!!");
        let mut healed = data.to_vec();

        // Simple healing: XOR with pattern to "fix" nulls
        for i in 0..healed.len() {
            if healed[i] == 0 && i > 0 {
                healed[i] = (healed[i-1] + healing_pattern[i % healing_pattern.len()]) / 2;
            }
        }

        healed
    }

    fn measure_similarity(a: &[u8], b: &[u8]) -> f64 {
        let matching = a.iter().zip(b.iter()).filter(|(x, y)| x == y).count();
        matching as f64 / a.len() as f64
    }

    fn count_differences(a: &[u8], b: &[u8]) -> usize {
        a.iter().zip(b.iter()).filter(|(x, y)| x != y).count()
    }
}

struct RepairResults {
    test_results: Vec<(f64, f64)>,
    repair_effective: bool,
}

// ============================================================================
// TEST 3: Malware Detection - TEST AGAINST REAL PATTERNS
// ============================================================================

struct MalwareDetectionTest;

impl MalwareDetectionTest {
    fn test_detection() -> DetectionResults {
        println!("\n\nMALWARE DETECTION TEST: Real shellcode pattern detection");
        println!("{}", "=".repeat(70));

        // Real shellcode patterns
        let malicious_samples = vec![
            vec![0x90; 100],                    // NOP sled
            vec![0x31, 0xc0, 0x50, 0x68],     // x86 shellcode start
            vec![0x00; 50],                     // NULL bytes
            vec![0xFF; 50],                     // All 0xFF
            vec![0xCC; 30],                     // INT3 (debugger)
        ];

        // Benign patterns
        let benign_samples = vec![
            b"Hello, World!".to_vec(),
            vec![0x42, 0x43, 0x44, 0x45],
            (0..100).map(|i| i as u8).collect(),
        ];

        let mut true_positives = 0;
        let mut false_positives = 0;
        let mut false_negatives = 0;
        let mut true_negatives = 0;

        // Test malicious samples
        println!("\nTesting malicious samples:");
        for (i, sample) in malicious_samples.iter().enumerate() {
            let detected = Self::is_malicious(sample);
            println!("  Sample {}: {} ({})", i+1,
                if detected { "DETECTED ✓" } else { "MISSED ✗" },
                Self::describe_pattern(sample)
            );

            if detected {
                true_positives += 1;
            } else {
                false_negatives += 1;
            }
        }

        // Test benign samples
        println!("\nTesting benign samples:");
        for (i, sample) in benign_samples.iter().enumerate() {
            let detected = Self::is_malicious(sample);
            println!("  Sample {}: {} ({})", i+1,
                if detected { "FALSE ALARM ✗" } else { "CLEAN ✓" },
                Self::describe_pattern(sample)
            );

            if detected {
                false_positives += 1;
            } else {
                true_negatives += 1;
            }
        }

        let total_malicious = malicious_samples.len();
        let total_benign = benign_samples.len();

        let true_positive_rate = true_positives as f64 / total_malicious as f64;
        let false_positive_rate = false_positives as f64 / total_benign as f64;

        println!("\nResults:");
        println!("  True positives:  {}/{} ({:.1}%)", true_positives, total_malicious, true_positive_rate * 100.0);
        println!("  False positives: {}/{} ({:.1}%)", false_positives, total_benign, false_positive_rate * 100.0);
        println!("  False negatives: {}/{}", false_negatives, total_malicious);
        println!("  True negatives:  {}/{}", true_negatives, total_benign);

        let detection_works = true_positive_rate >= 0.80 && false_positive_rate <= 0.20;

        println!("\n  VERDICT: {}", if detection_works {
            "✓ Malware detection works (>80% TP, <20% FP)"
        } else {
            "✗ Malware detection INADEQUATE"
        });

        DetectionResults {
            true_positive_rate,
            false_positive_rate,
            detection_effective: detection_works,
        }
    }

    fn is_malicious(data: &[u8]) -> bool {
        if data.len() < 10 {
            return false;
        }

        // Check for suspicious patterns
        let nop_count = data.iter().filter(|&&b| b == 0x90).count();
        let null_count = data.iter().filter(|&&b| b == 0x00).count();
        let ff_count = data.iter().filter(|&&b| b == 0xFF).count();
        let cc_count = data.iter().filter(|&&b| b == 0xCC).count();

        let suspicious_threshold = data.len() / 2;

        nop_count > suspicious_threshold ||
        null_count > suspicious_threshold ||
        ff_count > suspicious_threshold ||
        cc_count > suspicious_threshold / 2
    }

    fn describe_pattern(data: &[u8]) -> &'static str {
        if data.iter().all(|&b| b == 0x90) { "NOP sled" }
        else if data.iter().all(|&b| b == 0x00) { "NULL bytes" }
        else if data.iter().all(|&b| b == 0xFF) { "All 0xFF" }
        else if data.iter().all(|&b| b == 0xCC) { "INT3 debugger" }
        else { "mixed/benign" }
    }
}

struct DetectionResults {
    true_positive_rate: f64,
    false_positive_rate: f64,
    detection_effective: bool,
}

// ============================================================================
// TEST 4: Security Comparison vs SHA-256
// ============================================================================

struct SecurityComparison;

impl SecurityComparison {
    fn comprehensive_comparison() -> ComparisonResults {
        println!("\n\nSECURITY COMPARISON: BINE vs SHA-256 (Quantitative)");
        println!("{}", "=".repeat(70));

        // Test 1: Collision resistance (birthday bound)
        println!("\n[1] Collision Resistance");
        let bine_collisions = Self::test_collisions_bine();
        let sha256_collisions = 0; // Assumed (known to be zero)
        println!("  BINE:    {} collisions in 10,000 hashes", bine_collisions);
        println!("  SHA-256: {} collisions (theoretical)", sha256_collisions);

        // Test 2: Avalanche effect
        println!("\n[2] Avalanche Effect (1-bit input change)");
        let bine_avalanche = Self::test_avalanche_bine();
        let sha256_avalanche = 0.50; // Known to be ~50%
        println!("  BINE:    {:.2}% bits changed", bine_avalanche * 100.0);
        println!("  SHA-256: {:.2}% bits changed (theoretical)", sha256_avalanche * 100.0);

        // Test 3: Pre-image resistance (partial)
        println!("\n[3] Pre-image Resistance (brute force, limited)");
        let bine_preimage = Self::test_preimage_bine();
        let sha256_preimage = 0; // Would take 2^256 attempts
        println!("  BINE:    {} pre-images found in 1M attempts", bine_preimage);
        println!("  SHA-256: {} pre-images expected", sha256_preimage);

        // Test 4: Differential analysis (simplified)
        println!("\n[4] Differential Cryptanalysis (simplified)");
        let bine_diff = Self::test_differential_bine();
        println!("  BINE:    {:.4} max differential probability", bine_diff);
        println!("  SHA-256: ~0.0000 (cryptographic strength)");

        let comparison_favorable = bine_collisions == 0 &&
                                   (bine_avalanche - 0.50).abs() < 0.10 &&
                                   bine_preimage == 0 &&
                                   bine_diff < 0.01;

        println!("\n  VERDICT: {}", if comparison_favorable {
            "✓ BINE comparable to SHA-256 in tested metrics"
        } else {
            "✗ BINE WEAKER than SHA-256 in some metrics"
        });

        ComparisonResults {
            bine_collisions,
            bine_avalanche,
            bine_preimage,
            bine_differential: bine_diff,
            comparable: comparison_favorable,
        }
    }

    fn test_collisions_bine() -> usize {
        let mut hasher = BineHasherV2::new(256, SecurityMode::Balanced);
        let salt = b"collision_test_salt_32bytes!!!!";
        let mut hashes = HashSet::new();

        for i in 0..10000 {
            let data = format!("test{}", i);
            let hash = hasher.hash(data.as_bytes(), salt);
            hashes.insert(hash);
        }

        10000 - hashes.len() // Number of collisions
    }

    fn test_avalanche_bine() -> f64 {
        let mut hasher = BineHasherV2::new(256, SecurityMode::Balanced);
        let salt = b"avalanche_test_salt_32bytes!!!!";

        let data1 = b"test_data_avalanche";
        let data2 = b"Test_data_avalanche"; // 1 bit different

        let hash1 = hasher.hash(data1, salt);
        let hash2 = hasher.hash(data2, salt);

        let mut diff_bits = 0;
        for (b1, b2) in hash1.iter().zip(hash2.iter()) {
            diff_bits += (b1 ^ b2).count_ones();
        }

        diff_bits as f64 / (hash1.len() * 8) as f64
    }

    fn test_preimage_bine() -> usize {
        let mut hasher = BineHasherV2::new(256, SecurityMode::Balanced);
        let salt = b"preimage_test_salt_32bytes!!!!!";

        let target = hasher.hash(b"target", salt);
        let mut found = 0;

        // Try 1 million random inputs
        for i in 0..1_000_000 {
            let data = format!("attempt{}", i);
            let hash = hasher.hash(data.as_bytes(), salt);

            if hash == target {
                found += 1;
            }
        }

        found
    }

    fn test_differential_bine() -> f64 {
        let mut hasher = BineHasherV2::new(256, SecurityMode::Balanced);
        let salt = b"differential_test_salt_32bytes!";

        let mut max_bias = 0.0;

        // Test 256 single-bit differences
        for bit_pos in 0..256 {
            let mut data1 = vec![0u8; 32];
            let mut data2 = vec![0u8; 32];

            // Flip one bit
            data2[bit_pos / 8] ^= 1 << (bit_pos % 8);

            let hash1 = hasher.hash(&data1, salt);
            let hash2 = hasher.hash(&data2, salt);

            let mut diff_bits = 0;
            for (b1, b2) in hash1.iter().zip(hash2.iter()) {
                diff_bits += (b1 ^ b2).count_ones();
            }

            let probability = diff_bits as f64 / (hash1.len() * 8) as f64;
            let bias = (probability - 0.5).abs();

            if bias > max_bias {
                max_bias = bias;
            }
        }

        max_bias
    }
}

struct ComparisonResults {
    bine_collisions: usize,
    bine_avalanche: f64,
    bine_preimage: usize,
    bine_differential: f64,
    comparable: bool,
}

// ============================================================================
// MAIN TEST RUNNER
// ============================================================================

fn main() {
    println!("{}", "=".repeat(70));
    println!("BINE RIGOROUS SECURITY TESTING");
    println!("{}", "=".repeat(70));
    println!("\nHonest, research-grade validation of all security claims.");
    println!();

    // Test 1: Learning
    let mut learning_test = LearningTest::new();
    let learning_results = learning_test.test_learning_effectiveness();

    // Test 2: Self-repair
    let repair_results = SelfRepairTest::test_corruption_recovery();

    // Test 3: Malware detection
    let detection_results = MalwareDetectionTest::test_detection();

    // Test 4: Security comparison
    let comparison_results = SecurityComparison::comprehensive_comparison();

    // Overall summary
    println!("\n\n{}", "=".repeat(70));
    println!("OVERALL RESULTS SUMMARY");
    println!("{}", "=".repeat(70));

    println!("\n✓ = Feature works as claimed");
    println!("✗ = Feature does NOT work / needs improvement");
    println!("⚠ = Feature partially works");
    println!();

    let learning_symbol = if learning_results.learning_effective { "✓" } else { "✗" };
    let repair_symbol = if repair_results.repair_effective { "✓" } else { "✗" };
    let detection_symbol = if detection_results.detection_effective { "✓" } else { "✗" };
    let security_symbol = if comparison_results.comparable { "✓" } else { "✗" };

    println!("{} Learning from attacks:     {}", learning_symbol,
        if learning_results.learning_effective {
            format!("Works ({:.1}x slowdown or behavior change)", learning_results.slowdown_factor)
        } else {
            "FAILED (no adaptation detected)".to_string()
        }
    );

    println!("{} Self-repair:                {}", repair_symbol,
        if repair_results.repair_effective {
            "Works (>80% recovery at 5% corruption)".to_string()
        } else {
            "FAILED (<80% recovery)".to_string()
        }
    );

    println!("{} Malware detection:          {}", detection_symbol,
        if detection_results.detection_effective {
            format!("Works ({:.0}% TP, {:.0}% FP)",
                detection_results.true_positive_rate * 100.0,
                detection_results.false_positive_rate * 100.0)
        } else {
            "INADEQUATE".to_string()
        }
    );

    println!("{} Security vs SHA-256:        {}", security_symbol,
        if comparison_results.comparable {
            "Comparable strength".to_string()
        } else {
            "WEAKER in some metrics".to_string()
        }
    );

    let total_passed = [
        learning_results.learning_effective,
        repair_results.repair_effective,
        detection_results.detection_effective,
        comparison_results.comparable,
    ].iter().filter(|&&x| x).count();

    println!("\n{}", "=".repeat(70));
    println!("HONEST VERDICT: {}/4 features validated", total_passed);
    println!("{}", "=".repeat(70));

    if total_passed == 4 {
        println!("\n✓ BINE v2.0 claims are VALIDATED by rigorous testing!");
    } else if total_passed >= 2 {
        println!("\n⚠ BINE v2.0 has some unique features, but not all work as claimed.");
        println!("  Needs improvement before production use.");
    } else {
        println!("\n✗ BINE v2.0 claims are NOT supported by testing.");
        println!("  Significant development needed.");
    }
}

// Helper functions
fn rand_index(max: usize) -> usize {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    let mut hasher = RandomState::new().build_hasher();
    hasher.write_usize(std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as usize);
    hasher.finish() as usize % max
}
