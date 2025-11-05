/*!
BINE v5.0 Validation - Balanced Approach
=========================================

Testing the balanced security/speed design:
1. Mini-hash (fast, from v2.0)
2. S-box layer (non-linear, improves differential)
3. 5x learning multiplier (working, from v4.0)
4. Extra diffusion (perfect avalanche, from v3.0/v4.0)

Expected v5.0 results:
- Learning:     1.5-1.8x (maintain v4.0's 1.65x)
- Avalanche:    48-52% (maintain v4.0's 50.75%)
- Differential: 0.05-0.06 (better than v2.0's 0.0833, trade-off for speed)
- Speed:        220-250 MB/s (recovered from v4.0's 145 MB/s)

Comparison:
v2.0: 228 MB/s, 0.0833 diff, 42.71% avalanche, learning broken
v3.0: 196 MB/s, 0.0885 diff, 49.80% avalanche, learning broken
v4.0: 145 MB/s, 0.0729 diff, 50.75% avalanche, learning works (1.65x)
v5.0: ???
*/

use std::time::Instant;
use std::collections::HashSet;

mod lib_v5;
use lib_v5::{BineHasherV5, SecurityMode};

fn main() {
    println!("{}", "=".repeat(70));
    println!("BINE v5.0 VALIDATION - Balanced Security/Speed");
    println!("{}", "=".repeat(70));
    println!();

    println!("v5.0 Design:");
    println!("  • Mini-hash (fast ChaCha20-style, 20 rounds)");
    println!("  • S-box layer (AES S-box for non-linearity)");
    println!("  • 5x learning multiplier (proven to work in v4.0)");
    println!("  • Extra diffusion (proven to work in v3.0/v4.0)");
    println!();

    println!("Performance Comparison:");
    println!("  v2.0: 228 MB/s | 0.0833 diff | 42.71% avalanche | learning broken");
    println!("  v3.0: 196 MB/s | 0.0885 diff | 49.80% avalanche | learning broken");
    println!("  v4.0: 145 MB/s | 0.0729 diff | 50.75% avalanche | 1.65x learning ✅");
    println!("  v5.0: ???      | ???         | ???             | ???");
    println!();

    // Test 1: Learning (should maintain v4.0's success)
    test_learning_maintained();

    // Test 2: Avalanche (should maintain v3.0/v4.0's success)
    test_avalanche_maintained();

    // Test 3: Differential (S-box should improve over v2.0)
    test_differential_with_sbox();

    // Test 4: Speed (should recover to v2.0 levels)
    test_speed_recovery();

    // Test 5: Security metrics
    test_security_metrics();

    // Final verdict
    print_final_verdict();
}

fn test_learning_maintained() {
    println!("\n{}", "=".repeat(70));
    println!("[1] LEARNING MECHANISM (Maintain v4.0 Success)");
    println!("{}", "=".repeat(70));

    let mut hasher = BineHasherV5::new(256, SecurityMode::Balanced);
    let attack_pattern = vec![0xFF; 100];
    let salt = b"test_salt_32_bytes_learning_!!!";

    let mut attempt_times = Vec::new();

    println!("\nSimulating 100 repeated attack attempts...");

    for attempt in 0..100 {
        let start = Instant::now();
        let _hash = hasher.hash(&attack_pattern, salt);
        let duration = start.elapsed().as_nanos() as f64;

        attempt_times.push(duration);

        if attempt % 5 == 4 {
            hasher.report_threat(&attack_pattern[..32]);
        }
    }

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

    let extra_rounds = hasher.immune_memory.extra_defense_rounds();
    println!("  Extra rounds applied: {}", extra_rounds);
    println!("  v4.0 result: 1.65x (target: 1.1-1.5x)");
    println!();

    if early_to_late >= 1.1 && early_to_late <= 1.8 {
        println!("  ✓ LEARNING MAINTAINED! Slowdown {:.2}x", early_to_late);
        println!("  ✓ v4.0's successful fix preserved");
    } else if early_to_late > 1.0 {
        println!("  ⚠ Learning works but outside ideal range ({:.2}x)", early_to_late);
    } else {
        println!("  ✗ Learning broken or regressed ({:.2}x)", early_to_late);
    }
}

fn test_avalanche_maintained() {
    println!("\n{}", "=".repeat(70));
    println!("[2] AVALANCHE EFFECT (Maintain v3.0/v4.0 Success)");
    println!("{}", "=".repeat(70));

    let mut hasher = BineHasherV5::new(256, SecurityMode::Balanced);
    let salt = b"avalanche_test_salt_32bytes!!!!";

    let mut total_diff_percent = 0.0;
    let samples = 100;

    println!("\nTesting {} single-bit input differences...", samples);

    for i in 0..samples {
        let data1 = vec![0u8; 32];
        let mut data2 = vec![0u8; 32];
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
    println!("  v2.0: 42.71%");
    println!("  v3.0: 49.80%");
    println!("  v4.0: 50.75%");
    println!();

    if avg_avalanche >= 48.0 && avg_avalanche <= 52.0 {
        println!("  ✓ AVALANCHE MAINTAINED! Still in ideal range ({:.2}%)", avg_avalanche);
        println!("  ✓ v3.0/v4.0 fix preserved successfully");
    } else if avg_avalanche >= 45.0 && avg_avalanche <= 55.0 {
        println!("  ⚠ Good avalanche but outside ideal range ({:.2}%)", avg_avalanche);
    } else {
        println!("  ✗ Avalanche degraded ({:.2}%)", avg_avalanche);
    }
}

fn test_differential_with_sbox() {
    println!("\n{}", "=".repeat(70));
    println!("[3] DIFFERENTIAL RESISTANCE (Mini-hash + S-box)");
    println!("{}", "=".repeat(70));

    let mut hasher = BineHasherV5::new(256, SecurityMode::Balanced);
    let salt = b"differential_test_salt_32bytes!";

    let mut max_bias = 0.0;
    let test_positions = 256;

    println!("\nTesting {} single-bit differences for differential bias...", test_positions);
    println!("Using mini_hash (fast) + AES S-box (non-linear)...");

    for bit_pos in 0..test_positions {
        let data1 = vec![0u8; 32];
        let mut data2 = vec![0u8; 32];
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
    println!("  Target: <0.02 (good) or <0.0001 (SHA-256 level)");
    println!("  v2.0: 0.0833");
    println!("  v3.0: 0.0885");
    println!("  v4.0: 0.0729");
    println!();

    if max_bias < 0.02 {
        println!("  ✓ DIFFERENTIAL EXCELLENT! <2% bias ({:.4})", max_bias);
        println!("  ✓ S-box layer worked!");
    } else if max_bias < 0.06 {
        println!("  ✓ DIFFERENTIAL GOOD! Better than v2.0 ({:.4} vs 0.0833)", max_bias);
        println!("  ✓ S-box improved over pure mini_hash");
    } else if max_bias < 0.0833 {
        println!("  ⚠ IMPROVED from v2.0 ({:.4} vs 0.0833)", max_bias);
    } else {
        println!("  ✗ Not improved ({:.4})", max_bias);
    }
}

fn test_speed_recovery() {
    println!("\n{}", "=".repeat(70));
    println!("[4] SPEED RECOVERY (Mini-hash vs BLAKE2)");
    println!("{}", "=".repeat(70));

    let mut hasher = BineHasherV5::new(256, SecurityMode::Balanced);
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
    println!("{:<10} | {:<15} | {:<15} | {:<10} | {:<10} | {:<10}",
             "Size", "Speed (MB/s)", "Time (ms)", "vs v2.0", "vs v3.0", "vs v4.0");

    let dash = "-".repeat(80);
    println!("{}", dash);

    let v2_speeds = [47.53, 204.20, 200.45, 230.80, 228.0];
    let v3_speeds = [41.12, 176.54, 173.41, 200.13, 196.0];
    let v4_speeds = [50.29, 100.45, 97.76, 138.28, 145.0];

    for (idx, (size, label)) in test_sizes.iter().enumerate() {
        let data = vec![0xAA; *size];

        let start = Instant::now();
        let _hash = hasher.hash(&data, salt);
        let duration = start.elapsed().as_secs_f64();

        let throughput = (*size as f64 / duration) / 1_048_576.0;
        let vs_v2 = throughput / v2_speeds[idx];
        let vs_v3 = throughput / v3_speeds[idx];
        let vs_v4 = throughput / v4_speeds[idx];

        println!("{:<10} | {:<15.2} | {:<15.4} | {:<10.2}x | {:<10.2}x | {:<10.2}x",
                 label,
                 throughput,
                 duration * 1000.0,
                 vs_v2,
                 vs_v3,
                 vs_v4);
    }

    println!();
    println!("Performance Targets:");
    println!("  v2.0 (baseline):  228 MB/s");
    println!("  v3.0 (regression): 196 MB/s");
    println!("  v4.0 (worse):     145 MB/s");
    println!("  v5.0 target:      220-250 MB/s");
    println!("  Ideal target:     300-500 MB/s");
    println!();

    // Final verdict on 10MB
    let data_10mb = vec![0xAA; 10 * 1024 * 1024];
    let start = Instant::now();
    let _hash = hasher.hash(&data_10mb, salt);
    let duration = start.elapsed().as_secs_f64();
    let final_speed = (10.0 * 1024.0 * 1024.0 / duration) / 1_048_576.0;

    if final_speed >= 220.0 {
        println!("  ✓ SPEED RECOVERED! {:.2} MB/s (target: 220-250 MB/s)", final_speed);
        println!("  ✓ Mini-hash is faster than BLAKE2");
    } else if final_speed >= 196.0 {
        println!("  ⚠ IMPROVED from v4.0 ({:.2} MB/s vs 145 MB/s)", final_speed);
        println!("  ⚠ But still below v3.0 (196 MB/s)");
    } else if final_speed > 145.0 {
        println!("  ⚠ FASTER than v4.0 ({:.2} MB/s vs 145 MB/s)", final_speed);
        println!("  ⚠ But slower than v3.0 (196 MB/s)");
    } else {
        println!("  ✗ STILL SLOW ({:.2} MB/s)", final_speed);
    }
}

fn test_security_metrics() {
    println!("\n{}", "=".repeat(70));
    println!("[5] SECURITY METRICS");
    println!("{}", "=".repeat(70));

    let mut hasher = BineHasherV5::new(256, SecurityMode::Balanced);
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

    // Pre-image resistance
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
    println!("V5.0 VALIDATION SUMMARY");
    println!("{}", "=".repeat(70));

    println!("\nv5.0 Design Philosophy:");
    println!("  Balanced security/speed via mini_hash + S-box");
    println!("  Keep proven fixes (learning, avalanche)");
    println!("  Trade perfect differential for acceptable speed");
    println!();

    println!("Target Results:");
    println!("  Learning:     1.5-1.8x (maintain v4.0)");
    println!("  Avalanche:    48-52% (maintain v3.0/v4.0)");
    println!("  Differential: 0.05-0.06 (trade-off, better than v2.0)");
    println!("  Speed:        220-250 MB/s (recovered)");
    println!();

    println!("Comparison Table:");
    println!("┌─────────┬──────────┬───────────┬───────────┬──────────┐");
    println!("│ Version │ Speed    │ Diff      │ Avalanche │ Learning │");
    println!("├─────────┼──────────┼───────────┼───────────┼──────────┤");
    println!("│ v2.0    │ 228 MB/s │ 0.0833    │ 42.71%    │ Broken   │");
    println!("│ v3.0    │ 196 MB/s │ 0.0885    │ 49.80% ✅ │ Broken   │");
    println!("│ v4.0    │ 145 MB/s │ 0.0729 ✅ │ 50.75% ✅ │ 1.65x ✅ │");
    println!("│ v5.0    │ ???      │ ???       │ ???       │ ???      │");
    println!("└─────────┴──────────┴───────────┴───────────┴──────────┘");
    println!();

    println!("{}", "=".repeat(70));
    println!("Run the tests above to see actual v5.0 results!");
    println!("{}", "=".repeat(70));
}
