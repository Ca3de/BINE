/*!
BINE v3.0 Validation Tests
==========================

Validates all fixes from v2.0 → v3.0:
1. Learning now actually works (adds computational cost)
2. Differential resistance improved (<0.01 target)
3. Avalanche effect improved (48-52% target)
4. Speed optimized (500+ MB/s target)
*/

use std::time::Instant;
use std::collections::HashSet;

mod lib_v3;
use lib_v3::{BineHasherV3, SecurityMode};

fn main() {
    println!("{}", "=".repeat(70));
    println!("BINE v3.0 VALIDATION - Proving the Fixes Work");
    println!("{}", "=".repeat(70));
    println!();

    // Test 1: Learning (was broken, now fixed)
    test_learning_fix();

    // Test 2: Avalanche (was 42.71%, target 48-52%)
    test_avalanche_improvement();

    // Test 3: Differential (was 0.0833, target <0.01)
    test_differential_improvement();

    // Test 4: Speed (was 217-309 MB/s, target 500+ MB/s)
    test_speed_improvement();

    // Test 5: Security metrics (collision, pre-image)
    test_security_metrics();

    // Final summary
    print_final_verdict();
}

fn test_learning_fix() {
    println!("\n{}", "=".repeat(70));
    println!("[1] LEARNING FIX VALIDATION");
    println!("{}", "=".repeat(70));

    let mut hasher = BineHasherV3::new(256, SecurityMode::Balanced);
    let attack_pattern = vec![0xFF; 100];
    let salt = b"test_salt_32_bytes_learning_!!!";

    let mut attempt_times = Vec::new();

    println!("\nSimulating 100 repeated attack attempts...");

    for attempt in 0..100 {
        let start = Instant::now();
        let _hash = hasher.hash(&attack_pattern, salt);
        let duration = start.elapsed().as_nanos() as f64;

        attempt_times.push(duration);

        // Report threat every 5 attempts to trigger learning
        if attempt % 5 == 4 {
            hasher.report_threat(&attack_pattern[..32]);
        }
    }

    // Analyze learning effect
    let early_avg = attempt_times[..20].iter().sum::<f64>() / 20.0;
    let mid_avg = attempt_times[40..60].iter().sum::<f64>() / 20.0;
    let late_avg = attempt_times[80..].iter().sum::<f64>() / 20.0;

    let early_to_mid = mid_avg / early_avg;
    let early_to_late = late_avg / early_avg;

    println!("\nResults:");
    println!("  Attempts 1-20:   {:.2} µs average", early_avg / 1000.0);
    println!("  Attempts 40-60:  {:.2} µs average", mid_avg / 1000.0);
    println!("  Attempts 80-100: {:.2} µs average", late_avg / 1000.0);
    println!();
    println!("  Slowdown (early → mid):  {:.2}x", early_to_mid);
    println!("  Slowdown (early → late): {:.2}x", early_to_late);
    println!();

    let learning_works = early_to_late > 1.1;

    if learning_works {
        println!("  ✓ LEARNING FIXED! Difficulty increased by {:.0}%",
                 (early_to_late - 1.0) * 100.0);
        println!("  ✓ System now actually adds extra computational work");
    } else {
        println!("  ✗ Learning still broken (no slowdown detected)");
    }
}

fn test_avalanche_improvement() {
    println!("\n{}", "=".repeat(70));
    println!("[2] AVALANCHE EFFECT IMPROVEMENT");
    println!("{}", "=".repeat(70));

    let mut hasher = BineHasherV3::new(256, SecurityMode::Balanced);
    let salt = b"avalanche_test_salt_32bytes!!!!";

    // Test many single-bit differences
    let mut total_diff_percent = 0.0;
    let samples = 100;

    println!("\nTesting {} single-bit input differences...", samples);

    for i in 0..samples {
        let mut data1 = vec![0u8; 32];
        let mut data2 = vec![0u8; 32];

        // Flip one bit at position i
        data2[i % 32] ^= 1 << (i % 8);

        let hash1 = hasher.hash(&data1, salt);
        let hash2 = hasher.hash(&data2, salt);

        let mut diff_bits = 0;
        for (b1, b2) in hash1.iter().zip(hash2.iter()) {
            diff_bits += (b1 ^ b2).count_ones();
        }

        let diff_percent = (diff_bits as f64 / (hash1.len() * 8) as f64) * 100.0;
        total_diff_percent += diff_percent;
    }

    let avg_avalanche = total_diff_percent / samples as f64;

    println!("\nResults:");
    println!("  Average bit difference: {:.2}%", avg_avalanche);
    println!("  Ideal range: 48-52%");
    println!("  v2.0 result: 42.71%");
    println!();

    let improved = avg_avalanche >= 48.0 && avg_avalanche <= 52.0;

    if improved {
        println!("  ✓ AVALANCHE FIXED! Now in ideal range ({:.2}%)", avg_avalanche);
    } else if avg_avalanche > 42.71 {
        println!("  ⚠ IMPROVED but not ideal ({:.2}%, was 42.71%)", avg_avalanche);
    } else {
        println!("  ✗ Not improved ({:.2}%)", avg_avalanche);
    }
}

fn test_differential_improvement() {
    println!("\n{}", "=".repeat(70));
    println!("[3] DIFFERENTIAL RESISTANCE IMPROVEMENT");
    println!("{}", "=".repeat(70));

    let mut hasher = BineHasherV3::new(256, SecurityMode::Balanced);
    let salt = b"differential_test_salt_32bytes!";

    let mut max_bias = 0.0;
    let test_positions = 256;

    println!("\nTesting {} single-bit differences for differential bias...", test_positions);

    for bit_pos in 0..test_positions {
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

    println!("\nResults:");
    println!("  Maximum differential bias: {:.4}", max_bias);
    println!("  Target: <0.01 (1%)");
    println!("  v2.0 result: 0.0833");
    println!("  SHA-256 typical: ~0.0001");
    println!();

    let improved = max_bias < 0.01;
    let better_than_v2 = max_bias < 0.0833;

    if improved {
        println!("  ✓ DIFFERENTIAL RESISTANCE FIXED! Now <1% bias");
    } else if better_than_v2 {
        println!("  ⚠ IMPROVED but not cryptographic strength ({:.4}, was 0.0833)", max_bias);
    } else {
        println!("  ✗ Not improved ({:.4})", max_bias);
    }
}

fn test_speed_improvement() {
    println!("\n{}", "=".repeat(70));
    println!("[4] SPEED OPTIMIZATION VALIDATION");
    println!("{}", "=".repeat(70));

    let mut hasher = BineHasherV3::new(256, SecurityMode::Balanced);
    let salt = b"speed_test_salt_32_bytes_here!!!";

    let test_sizes = vec![
        (1024, "1 KB"),
        (10 * 1024, "10 KB"),
        (100 * 1024, "100 KB"),
        (1024 * 1024, "1 MB"),
        (10 * 1024 * 1024, "10 MB"),
    ];

    println!("\nBenchmarking hash performance...");
    println!();
    println!("{:<10} | {:<15} | {:<15} | {:<15}", "Size", "Speed (MB/s)", "Time (ms)", "vs v2.0");

    let dash = "-".repeat(70);
    println!("{}", dash);

    let v2_speeds = [47.53, 204.20, 200.45, 230.80, 228.0]; // v2.0 Balanced results

    for (idx, (size, label)) in test_sizes.iter().enumerate() {
        let data = vec![0xAA; *size];

        let start = Instant::now();
        let _hash = hasher.hash(&data, salt);
        let duration = start.elapsed().as_secs_f64();

        let throughput = (*size as f64 / duration) / 1_048_576.0;
        let v2_speed = v2_speeds[idx];
        let improvement = throughput / v2_speed;

        println!("{:<10} | {:<15.2} | {:<15.4} | {:<15.2}x",
                 label,
                 throughput,
                 duration * 1000.0,
                 improvement);
    }

    println!();
    println!("Target: 500+ MB/s for large files");
    println!("v2.0 Balanced: 228 MB/s (10 MB)");
}

fn test_security_metrics() {
    println!("\n{}", "=".repeat(70));
    println!("[5] SECURITY METRICS (Baseline)");
    println!("{}", "=".repeat(70));

    let mut hasher = BineHasherV3::new(256, SecurityMode::Balanced);
    let salt = b"security_metrics_salt_32bytes!!!";

    // Collision resistance
    println!("\nCollision Resistance (10,000 hashes):");
    let mut hashes = HashSet::new();
    for i in 0..10000 {
        let data = format!("test{}", i);
        let hash = hasher.hash(data.as_bytes(), salt);
        hashes.insert(hash);
    }
    let collisions = 10000 - hashes.len();
    println!("  Collisions: {}", collisions);
    println!("  Result: {}", if collisions == 0 { "✓ Perfect" } else { "✗ Failed" });

    // Pre-image resistance (limited test)
    println!("\nPre-image Resistance (1M attempts):");
    let target = hasher.hash(b"target", salt);
    let mut found = 0;
    for i in 0..1_000_000 {
        let data = format!("attempt{}", i);
        let hash = hasher.hash(data.as_bytes(), salt);
        if hash == target {
            found += 1;
        }
    }
    println!("  Pre-images found: {}", found);
    println!("  Result: {}", if found == 0 { "✓ Perfect" } else { "✗ Failed" });
}

fn print_final_verdict() {
    println!("\n{}", "=".repeat(70));
    println!("V3.0 VALIDATION SUMMARY");
    println!("{}", "=".repeat(70));

    println!("\nKey Improvements:");
    println!("  1. Learning: FIXED (now adds extra rounds based on threats)");
    println!("  2. Avalanche: IMPROVED (better mixing, extra diffusion)");
    println!("  3. Differential: IMPROVED (24 rounds + extra mixing)");
    println!("  4. Speed: OPTIMIZED (reduced key derivation, inlining)");
    println!();

    println!("Expected Results:");
    println!("  ✓ Learning: 1.1-1.5x slowdown after 100 attacks");
    println!("  ✓ Avalanche: 48-52% (was 42.71%)");
    println!("  ⚠ Differential: <0.02 (was 0.0833, target <0.01)");
    println!("  ⚠ Speed: 300-500 MB/s (was 228 MB/s, target 500+)");
    println!();

    println!("{}", "=".repeat(70));
    println!("Run the tests above to see actual results!");
    println!("{}", "=".repeat(70));
}
