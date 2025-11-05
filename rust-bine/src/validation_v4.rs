/*! BINE v4.0 Validation Tests
==========================

Testing the properly calibrated fixes:
1. Learning: 5x multiplier (10-40 extra rounds, was 0-8)
2. Differential: BLAKE2-style mixing (actual crypto, not weak mini_hash)
3. Speed: Removed expensive operations (should recover to 228+ MB/s)
4. Avalanche: Kept v3.0's successful fix (49.80% ✅)

v3.0 Results (what we're validating against):
- Learning: 1.07x (insufficient) → Target: 1.1-1.5x
- Differential: 0.0885 (worse!) → Target: <0.02 (0.01 may be unrealistic)
- Speed: 196 MB/s (regression) → Target: 300-500 MB/s
- Avalanche: 49.80% ✅ → Target: 48-52%
*/

use std::time::Instant;
use std::collections::HashSet;

mod lib_v4;
use lib_v4::{BineHasherV4, SecurityMode};

fn main() {
    println!("{}", "=".repeat(70));
    println!("BINE v4.0 VALIDATION - Properly Calibrated Fixes");
    println!("{}", "=".repeat(70));
    println!();

    println!("v3.0 Results (baseline):");
    println!("  Learning:     1.07x (insufficient)");
    println!("  Differential: 0.0885 (worse than v2.0)");
    println!("  Speed:        196 MB/s (regression)");
    println!("  Avalanche:    49.80% ✅ (perfect)");
    println!();
    println!("v4.0 Targets:");
    println!("  Learning:     1.1-1.5x slowdown");
    println!("  Differential: <0.02 (realistic target)");
    println!("  Speed:        300-500 MB/s");
    println!("  Avalanche:    48-52% (maintain fix)");
    println!();

    // Test 1: Learning (5x stronger multiplier)
    test_learning_fix();

    // Test 2: Avalanche (should maintain v3.0's success)
    test_avalanche_maintained();

    // Test 3: Differential (BLAKE2-style mixing)
    test_differential_improvement();

    // Test 4: Speed (removed expensive ops)
    test_speed_recovery();

    // Test 5: Security metrics (baseline)
    test_security_metrics();

    // Final verdict
    print_final_verdict();
}

fn test_learning_fix() {
    println!("\n{}", "=".repeat(70));
    println!("[1] LEARNING FIX VALIDATION (5x Multiplier)");
    println!("{}", "=".repeat(70));

    let mut hasher = BineHasherV4::new(256, SecurityMode::Balanced);
    let attack_pattern = vec![0xFF; 100];
    let salt = b"test_salt_32_bytes_learning_!!!";

    let mut attempt_times = Vec::new();

    println!("\nSimulating 100 repeated attack attempts...");
    println!("(Reporting threats every 5 attempts)");

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

    // Check extra rounds being applied
    let extra_rounds = hasher.immune_memory.extra_defense_rounds();
    println!("  Extra rounds applied: {} (was 0-8 in v3.0, now 10-40)", extra_rounds);
    println!();

    let learning_works = early_to_late >= 1.1 && early_to_late <= 1.5;
    let improved = early_to_late > 1.07; // Better than v3.0

    if learning_works {
        println!("  ✓ LEARNING FIXED! Slowdown {:.2}x (ideal 1.1-1.5x)", early_to_late);
        println!("  ✓ Difficulty increased by {:.0}% after 100 attacks", (early_to_late - 1.0) * 100.0);
    } else if improved {
        println!("  ⚠ IMPROVED from v3.0 ({:.2}x vs 1.07x) but not ideal", early_to_late);
        println!("  ⚠ Target is 1.1-1.5x, may need more calibration");
    } else {
        println!("  ✗ Learning still broken or insufficient ({:.2}x)", early_to_late);
    }
}

fn test_avalanche_maintained() {
    println!("\n{}", "=".repeat(70));
    println!("[2] AVALANCHE EFFECT (Maintain v3.0 Success)");
    println!("{}", "=".repeat(70));

    let mut hasher = BineHasherV4::new(256, SecurityMode::Balanced);
    let salt = b"avalanche_test_salt_32bytes!!!!";

    // Test many single-bit differences
    let mut total_diff_percent = 0.0;
    let samples = 100;

    println!("\nTesting {} single-bit input differences...", samples);

    for i in 0..samples {
        let data1 = vec![0u8; 32];
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
    println!("  v3.0 result: 49.80% ✅");
    println!();

    let maintained = avg_avalanche >= 48.0 && avg_avalanche <= 52.0;

    if maintained {
        println!("  ✓ AVALANCHE MAINTAINED! Still in ideal range ({:.2}%)", avg_avalanche);
        println!("  ✓ v3.0 fix preserved successfully");
    } else if avg_avalanche > 45.0 && avg_avalanche < 55.0 {
        println!("  ⚠ SLIGHTLY DEGRADED but acceptable ({:.2}%)", avg_avalanche);
    } else {
        println!("  ✗ REGRESSION - avalanche degraded ({:.2}%)", avg_avalanche);
    }
}

fn test_differential_improvement() {
    println!("\n{}", "=".repeat(70));
    println!("[3] DIFFERENTIAL RESISTANCE (BLAKE2-Style Mixing)");
    println!("{}", "=".repeat(70));

    let mut hasher = BineHasherV4::new(256, SecurityMode::Balanced);
    let salt = b"differential_test_salt_32bytes!";

    let mut max_bias = 0.0;
    let test_positions = 256;

    println!("\nTesting {} single-bit differences for differential bias...", test_positions);
    println!("Using BLAKE2-style compression instead of mini_hash...");

    for bit_pos in 0..test_positions {
        let data1 = vec![0u8; 32];
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
    println!("  Ideal target: <0.0001 (SHA-256 level)");
    println!("  Realistic target: <0.02 (good cryptographic strength)");
    println!("  v2.0 result: 0.0833");
    println!("  v3.0 result: 0.0885 (worse!)");
    println!();

    let excellent = max_bias < 0.01;
    let good = max_bias < 0.02;
    let improved = max_bias < 0.0833;

    if excellent {
        println!("  ✓ DIFFERENTIAL EXCELLENT! Now <1% bias ({:.4})", max_bias);
        println!("  ✓ BLAKE2-style mixing worked!");
    } else if good {
        println!("  ✓ DIFFERENTIAL GOOD! <2% bias ({:.4})", max_bias);
        println!("  ✓ Significantly improved from v3.0 (0.0885)");
    } else if improved {
        println!("  ⚠ IMPROVED but not cryptographic strength ({:.4}, was 0.0885)", max_bias);
    } else {
        println!("  ✗ Not improved ({:.4})", max_bias);
    }
}

fn test_speed_recovery() {
    println!("\n{}", "=".repeat(70));
    println!("[4] SPEED RECOVERY (Removed Expensive Operations)");
    println!("{}", "=".repeat(70));

    let mut hasher = BineHasherV4::new(256, SecurityMode::Balanced);
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
    println!("{:<10} | {:<15} | {:<15} | {:<15} | {:<15}",
             "Size", "Speed (MB/s)", "Time (ms)", "vs v2.0", "vs v3.0");

    let dash = "-".repeat(78);
    println!("{}", dash);

    let v2_speeds = [47.53, 204.20, 200.45, 230.80, 228.0];
    let v3_speeds = [41.12, 176.54, 173.41, 200.13, 196.0]; // Regression

    for (idx, (size, label)) in test_sizes.iter().enumerate() {
        let data = vec![0xAA; *size];

        let start = Instant::now();
        let _hash = hasher.hash(&data, salt);
        let duration = start.elapsed().as_secs_f64();

        let throughput = (*size as f64 / duration) / 1_048_576.0;
        let v2_speed = v2_speeds[idx];
        let v3_speed = v3_speeds[idx];
        let vs_v2 = throughput / v2_speed;
        let vs_v3 = throughput / v3_speed;

        println!("{:<10} | {:<15.2} | {:<15.4} | {:<15.2}x | {:<15.2}x",
                 label,
                 throughput,
                 duration * 1000.0,
                 vs_v2,
                 vs_v3);
    }

    println!();
    println!("Performance Targets:");
    println!("  v2.0 Balanced: 228 MB/s (10 MB)");
    println!("  v3.0 Balanced: 196 MB/s (regression ❌)");
    println!("  v4.0 Target:   300-500 MB/s");
    println!();

    // Get 10MB speed for final verdict
    let data_10mb = vec![0xAA; 10 * 1024 * 1024];
    let start = Instant::now();
    let _hash = hasher.hash(&data_10mb, salt);
    let duration = start.elapsed().as_secs_f64();
    let final_speed = (10.0 * 1024.0 * 1024.0 / duration) / 1_048_576.0;

    if final_speed >= 300.0 {
        println!("  ✓ SPEED TARGET MET! {:.2} MB/s (target: 300-500 MB/s)", final_speed);
    } else if final_speed >= 228.0 {
        println!("  ⚠ RECOVERED to v2.0 level ({:.2} MB/s) but below target", final_speed);
    } else if final_speed > 196.0 {
        println!("  ⚠ IMPROVED from v3.0 ({:.2} MB/s vs 196 MB/s) but below v2.0", final_speed);
    } else {
        println!("  ✗ STILL SLOW ({:.2} MB/s)", final_speed);
    }
}

fn test_security_metrics() {
    println!("\n{}", "=".repeat(70));
    println!("[5] SECURITY METRICS (Baseline)");
    println!("{}", "=".repeat(70));

    let mut hasher = BineHasherV4::new(256, SecurityMode::Balanced);
    let salt = b"security_metrics_salt_32bytes!!!";

    // Collision resistance
    println!("\nCollision Resistance (10,000 hashes):");
    let mut hashes = HashSet::new();
    let start = Instant::now();
    for i in 0..10000 {
        let data = format!("test{}", i);
        let hash = hasher.hash(data.as_bytes(), salt);
        hashes.insert(hash);
    }
    let duration = start.elapsed().as_secs_f64();

    let collisions = 10000 - hashes.len();
    println!("  Collisions: {}", collisions);
    println!("  Time: {:.4}s", duration);
    println!("  Result: {}", if collisions == 0 { "✓ Perfect" } else { "✗ Failed" });

    // Pre-image resistance (limited test)
    println!("\nPre-image Resistance (1M attempts):");
    let target = hasher.hash(b"target", salt);
    let mut found = 0;
    let start = Instant::now();
    for i in 0..1_000_000 {
        let data = format!("attempt{}", i);
        let hash = hasher.hash(data.as_bytes(), salt);
        if hash == target {
            found += 1;
        }
    }
    let duration = start.elapsed().as_secs_f64();
    println!("  Pre-images found: {}", found);
    println!("  Time: {:.2}s", duration);
    println!("  Result: {}", if found == 0 { "✓ Perfect" } else { "✗ Failed" });
}

fn print_final_verdict() {
    println!("\n{}", "=".repeat(70));
    println!("V4.0 VALIDATION SUMMARY");
    println!("{}", "=".repeat(70));

    println!("\nv4.0 Improvements vs v3.0:");
    println!("  1. Learning multiplier:    1x → 5x (0-8 rounds → 10-40 rounds)");
    println!("  2. Differential mixing:    mini_hash → BLAKE2-style");
    println!("  3. Speed optimizations:    Removed expensive extra operations");
    println!("  4. Avalanche preservation: Kept v3.0's successful fix");
    println!();

    println!("Expected Results:");
    println!("  Learning:     1.1-1.5x slowdown (was 1.07x in v3.0)");
    println!("  Differential: <0.02 bias (was 0.0885 in v3.0)");
    println!("  Speed:        300-500 MB/s (was 196 MB/s in v3.0)");
    println!("  Avalanche:    48-52% (maintain 49.80% from v3.0)");
    println!();

    println!("{}", "=".repeat(70));
    println!("Run the tests above to see actual v4.0 results!");
    println!("{}", "=".repeat(70));
}
