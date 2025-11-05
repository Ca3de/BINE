/*!
BINE v7.5 Validation - Targeted Fixes for Broken Features
==========================================================

Benchmarks showed 3 broken features:
1. Avalanche: 38% (target: 48-52%)
2. Differential: 0.0833 (target: <0.02)
3. Learning: 50% (target: >80%)

This test validates whether v7.5 fixes actually work.
*/

use std::time::Instant;

mod lib_v7;
mod lib_v75;

use lib_v7::BineHasherV7 as BineV7;
use lib_v75::BineHasherV75 as BineV75;
use lib_v75::SecurityMode;

fn main() {
    println!("{}", "=".repeat(80));
    println!("BINE v7.5 - TARGETED FIXES VALIDATION");
    println!("{}", "=".repeat(80));
    println!();

    println!("Benchmarks identified 3 broken features:");
    println!("  ✗ Avalanche: 38% (need 48-52%)");
    println!("  ✗ Differential: 0.0833 (need <0.02)");
    println!("  ⚠ Learning: 50% (need >80%)");
    println!();
    println!("Testing if v7.5 fixes actually work...");
    println!();

    // Fix #1: Avalanche effect
    test_avalanche_fix();

    // Fix #2: Differential cryptanalysis resistance
    test_differential_fix();

    // Fix #3: Learning accuracy
    test_learning_fix();

    // Speed regression check
    test_speed_regression();

    // Final verdict
    print_final_verdict();
}

fn test_avalanche_fix() {
    println!("\n{}", "=".repeat(80));
    println!("[FIX #1] AVALANCHE EFFECT: Enhanced Diffusion (5 passes)");
    println!("{}", "=".repeat(80));

    println!("\nChanges in v7.5:");
    println!("  • 5 diffusion passes (was 2)");
    println!("  • Added very_far mixing (quarter distance)");
    println!("  • Added non-linear multiplication by 251");
    println!();

    let salt = b"avalanche_test_salt_32_bytes!!!";
    let test_count = 1000;

    println!("Testing {} samples with single-bit flips...", test_count);
    println!();

    // Test v7.0
    let mut v7_hasher = BineV7::new(256, lib_v7::SecurityMode::Balanced);
    let mut v7_total_flipped = 0u64;
    let mut v7_total_bits = 0u64;

    for i in 0..test_count {
        let mut input1 = vec![0u8; 32];
        input1[i % 32] = (i / 32) as u8;

        let mut input2 = input1.clone();
        input2[i % 32] ^= 1u8 << (i % 8); // Flip one bit

        let hash1 = v7_hasher.hash(&input1, salt);
        let hash2 = v7_hasher.hash(&input2, salt);

        for j in 0..hash1.len().min(hash2.len()) {
            let diff = hash1[j] ^ hash2[j];
            v7_total_flipped += diff.count_ones() as u64;
            v7_total_bits += 8;
        }
    }

    let v7_avalanche = (v7_total_flipped as f64 / v7_total_bits as f64) * 100.0;

    // Test v7.5
    let mut v75_hasher = BineV75::new(256, SecurityMode::Balanced);
    let mut v75_total_flipped = 0u64;
    let mut v75_total_bits = 0u64;

    for i in 0..test_count {
        let mut input1 = vec![0u8; 32];
        input1[i % 32] = (i / 32) as u8;

        let mut input2 = input1.clone();
        input2[i % 32] ^= 1u8 << (i % 8); // Flip one bit

        let hash1 = v75_hasher.hash(&input1, salt);
        let hash2 = v75_hasher.hash(&input2, salt);

        for j in 0..hash1.len().min(hash2.len()) {
            let diff = hash1[j] ^ hash2[j];
            v75_total_flipped += diff.count_ones() as u64;
            v75_total_bits += 8;
        }
    }

    let v75_avalanche = (v75_total_flipped as f64 / v75_total_bits as f64) * 100.0;

    println!("{:<15} | {:<20} | {:<10}",
             "Version", "Avalanche Effect", "Status");
    println!("{}", "-".repeat(50));

    let v7_status = if v7_avalanche >= 48.0 && v7_avalanche <= 52.0 {
        "✓ GOOD"
    } else if v7_avalanche >= 45.0 && v7_avalanche <= 55.0 {
        "⚠ CLOSE"
    } else {
        "✗ BAD"
    };

    let v75_status = if v75_avalanche >= 48.0 && v75_avalanche <= 52.0 {
        "✓ GOOD"
    } else if v75_avalanche >= 45.0 && v75_avalanche <= 55.0 {
        "⚠ CLOSE"
    } else {
        "✗ BAD"
    };

    println!("{:<15} | {:<20.2}% | {:<10}",
             "v7.0 (baseline)", v7_avalanche, v7_status);
    println!("{:<15} | {:<20.2}% | {:<10}",
             "v7.5 (fixed)", v75_avalanche, v75_status);
    println!();

    let improvement = v75_avalanche - v7_avalanche;
    if improvement > 0.0 {
        println!("Improvement: +{:.2}% (v7.5 better)", improvement);
    } else if improvement < 0.0 {
        println!("Regression: {:.2}% (v7.5 worse!)", improvement);
    } else {
        println!("No change");
    }
    println!();

    if v75_avalanche >= 48.0 && v75_avalanche <= 52.0 {
        println!("✓ FIX #1 SUCCESS! Avalanche now in target range");
    } else if v75_avalanche > v7_avalanche {
        println!("⚠ FIX #1 PARTIAL: Improved but not yet in range");
    } else {
        println!("✗ FIX #1 FAILED: No improvement or regression");
    }
}

fn test_differential_fix() {
    println!("\n{}", "=".repeat(80));
    println!("[FIX #2] DIFFERENTIAL CRYPTANALYSIS: S-box Inside Transform");
    println!("{}", "=".repeat(80));

    println!("\nChanges in v7.5:");
    println!("  • S-box applied DURING tardigrade_transform (not after)");
    println!("  • Non-linear operations on intermediate state");
    println!("  • Renamed: tardigrade_transform_nonlinear()");
    println!();

    let salt = b"differential_test_salt_32!!!!!";
    let test_count = 256;

    println!("Testing {} input pairs with controlled differences...", test_count);
    println!();

    // Test v7.0
    let mut v7_hasher = BineV7::new(256, lib_v7::SecurityMode::Balanced);
    let mut v7_max_bias = 0.0f64;

    for delta in 1..=test_count {
        let mut bit_differences = [0u32; 256];
        let samples = 100;

        for sample in 0..samples {
            let input1 = vec![sample as u8; 32];
            let mut input2 = input1.clone();
            input2[0] ^= delta as u8;

            let hash1 = v7_hasher.hash(&input1, salt);
            let hash2 = v7_hasher.hash(&input2, salt);

            // Only check first 32 bytes (256 bits)
            let check_len = hash1.len().min(hash2.len()).min(32);
            for i in 0..check_len {
                let diff = hash1[i] ^ hash2[i];
                for bit in 0..8 {
                    if (diff >> bit) & 1 == 1 {
                        let idx = i * 8 + bit;
                        if idx < 256 {
                            bit_differences[idx] += 1;
                        }
                    }
                }
            }
        }

        for &count in &bit_differences {
            let probability = count as f64 / samples as f64;
            let bias = (probability - 0.5).abs();
            if bias > v7_max_bias {
                v7_max_bias = bias;
            }
        }
    }

    // Test v7.5
    let mut v75_hasher = BineV75::new(256, SecurityMode::Balanced);
    let mut v75_max_bias = 0.0f64;

    for delta in 1..=test_count {
        let mut bit_differences = [0u32; 256];
        let samples = 100;

        for sample in 0..samples {
            let input1 = vec![sample as u8; 32];
            let mut input2 = input1.clone();
            input2[0] ^= delta as u8;

            let hash1 = v75_hasher.hash(&input1, salt);
            let hash2 = v75_hasher.hash(&input2, salt);

            // Only check first 32 bytes (256 bits)
            let check_len = hash1.len().min(hash2.len()).min(32);
            for i in 0..check_len {
                let diff = hash1[i] ^ hash2[i];
                for bit in 0..8 {
                    if (diff >> bit) & 1 == 1 {
                        let idx = i * 8 + bit;
                        if idx < 256 {
                            bit_differences[idx] += 1;
                        }
                    }
                }
            }
        }

        for &count in &bit_differences {
            let probability = count as f64 / samples as f64;
            let bias = (probability - 0.5).abs();
            if bias > v75_max_bias {
                v75_max_bias = bias;
            }
        }
    }

    println!("{:<15} | {:<20} | {:<10}",
             "Version", "Max Bias", "Status");
    println!("{}", "-".repeat(50));

    let v7_status = if v7_max_bias < 0.02 {
        "✓ GOOD"
    } else if v7_max_bias < 0.05 {
        "⚠ CLOSE"
    } else {
        "✗ BAD"
    };

    let v75_status = if v75_max_bias < 0.02 {
        "✓ GOOD"
    } else if v75_max_bias < 0.05 {
        "⚠ CLOSE"
    } else {
        "✗ BAD"
    };

    println!("{:<15} | {:<20.4} | {:<10}",
             "v7.0 (baseline)", v7_max_bias, v7_status);
    println!("{:<15} | {:<20.4} | {:<10}",
             "v7.5 (fixed)", v75_max_bias, v75_status);
    println!();

    let improvement = v7_max_bias - v75_max_bias;
    if improvement > 0.0 {
        println!("Improvement: -{:.4} (v7.5 better)", improvement);
    } else if improvement < 0.0 {
        println!("Regression: +{:.4} (v7.5 worse!)", improvement.abs());
    } else {
        println!("No change");
    }
    println!();

    if v75_max_bias < 0.02 {
        println!("✓ FIX #2 SUCCESS! Differential bias now < 0.02");
    } else if v75_max_bias < v7_max_bias {
        println!("⚠ FIX #2 PARTIAL: Improved but not yet under 0.02");
    } else {
        println!("✗ FIX #2 FAILED: No improvement or regression");
    }
}

fn test_learning_fix() {
    println!("\n{}", "=".repeat(80));
    println!("[FIX #3] LEARNING ACCURACY: Enhanced Feature Extraction (64 dims)");
    println!("{}", "=".repeat(80));

    println!("\nChanges in v7.5:");
    println!("  • Feature dimensions: 32 → 64");
    println!("  • Added bi-gram frequencies");
    println!("  • Added position-dependent patterns");
    println!("  • Added entropy measures");
    println!("  • Added sequential patterns");
    println!();

    println!("Testing generalization with SQL injection training...");
    println!();

    // Test v7.0
    let mut v7_hasher = BineV7::new(256, lib_v7::SecurityMode::Balanced);

    let sql_attacks: Vec<&[u8]> = vec![
        b"' OR '1'='1",
        b"' OR 1=1--",
        b"admin'--",
        b"' UNION SELECT",
        b"1' AND '1'='1",
    ];

    for attack in &sql_attacks {
        v7_hasher.report_attack(attack, true);
    }

    let test_cases = vec![
        (b"' OR '2'='2" as &[u8], true),
        (b"' OR 2=2--" as &[u8], true),
        (b"admin'#" as &[u8], true),
        (b"' UNION ALL SELECT" as &[u8], true),
        (b"normal_input_123" as &[u8], false),
        (b"user@email.com" as &[u8], false),
    ];

    let mut v7_correct = 0;
    for (input, should_detect) in &test_cases {
        let detected = v7_hasher.immune_system.is_recognized_threat(input);
        if detected == *should_detect {
            v7_correct += 1;
        }
    }

    let v7_accuracy = (v7_correct as f64 / test_cases.len() as f64) * 100.0;

    // Test v7.5
    let mut v75_hasher = BineV75::new(256, SecurityMode::Balanced);

    for attack in &sql_attacks {
        v75_hasher.report_attack(attack, true);
    }

    let mut v75_correct = 0;
    for (input, should_detect) in &test_cases {
        let detected = v75_hasher.immune_system.is_recognized_threat(input);
        if detected == *should_detect {
            v75_correct += 1;
        }
    }

    let v75_accuracy = (v75_correct as f64 / test_cases.len() as f64) * 100.0;

    println!("{:<15} | {:<20} | {:<10}",
             "Version", "Accuracy", "Status");
    println!("{}", "-".repeat(50));

    let v7_status = if v7_accuracy >= 80.0 {
        "✓ GOOD"
    } else if v7_accuracy >= 60.0 {
        "⚠ CLOSE"
    } else {
        "✗ BAD"
    };

    let v75_status = if v75_accuracy >= 80.0 {
        "✓ GOOD"
    } else if v75_accuracy >= 60.0 {
        "⚠ CLOSE"
    } else {
        "✗ BAD"
    };

    println!("{:<15} | {:<20.1}% | {:<10}",
             "v7.0 (baseline)", v7_accuracy, v7_status);
    println!("{:<15} | {:<20.1}% | {:<10}",
             "v7.5 (fixed)", v75_accuracy, v75_status);
    println!();

    let improvement = v75_accuracy - v7_accuracy;
    if improvement > 0.0 {
        println!("Improvement: +{:.1}% (v7.5 better)", improvement);
    } else if improvement < 0.0 {
        println!("Regression: {:.1}% (v7.5 worse!)", improvement);
    } else {
        println!("No change");
    }
    println!();

    if v75_accuracy >= 80.0 {
        println!("✓ FIX #3 SUCCESS! Learning accuracy now ≥80%");
    } else if v75_accuracy > v7_accuracy {
        println!("⚠ FIX #3 PARTIAL: Improved but not yet ≥80%");
    } else {
        println!("✗ FIX #3 FAILED: No improvement or regression");
    }
    println!();
    println!("Note: Limited training data (5 samples). With more training,");
    println!("      accuracy should improve further.");
}

fn test_speed_regression() {
    println!("\n{}", "=".repeat(80));
    println!("[REGRESSION CHECK] SPEED - Ensure Fixes Didn't Slow Down System");
    println!("{}", "=".repeat(80));

    println!("\nAdded complexity in v7.5:");
    println!("  • 5 diffusion passes (was 2) - may slow down");
    println!("  • S-box in transform (was after) - may slow down");
    println!("  • 64 features (was 32) - may slow down learning");
    println!();

    let test_size = 1024 * 1024; // 1 MB
    let data = vec![0x42u8; test_size];
    let salt = b"speed_test";

    // v7.0 speed
    let mut v7_hasher = BineV7::new(256, lib_v7::SecurityMode::Balanced);
    let v7_start = Instant::now();
    for _ in 0..10 {
        let _ = v7_hasher.hash(&data, salt);
    }
    let v7_duration = v7_start.elapsed();
    let v7_throughput = (test_size as f64 * 10.0) / v7_duration.as_secs_f64() / 1_000_000.0;

    // v7.5 speed
    let mut v75_hasher = BineV75::new(256, SecurityMode::Balanced);
    let v75_start = Instant::now();
    for _ in 0..10 {
        let _ = v75_hasher.hash(&data, salt);
    }
    let v75_duration = v75_start.elapsed();
    let v75_throughput = (test_size as f64 * 10.0) / v75_duration.as_secs_f64() / 1_000_000.0;

    println!("{:<15} | {:<20} | {:<10}",
             "Version", "Throughput", "Status");
    println!("{}", "-".repeat(50));

    println!("{:<15} | {:<20.2} MB/s | Baseline",
             "v7.0", v7_throughput);

    let regression_pct = ((v7_throughput - v75_throughput) / v7_throughput) * 100.0;
    let status = if regression_pct < 10.0 {
        "✓ ACCEPTABLE"
    } else if regression_pct < 25.0 {
        "⚠ MODERATE"
    } else {
        "✗ SEVERE"
    };

    println!("{:<15} | {:<20.2} MB/s | {:<10}",
             "v7.5", v75_throughput, status);
    println!();

    if regression_pct > 0.0 {
        println!("Speed regression: {:.1}% slower", regression_pct);
    } else {
        println!("Speed improvement: {:.1}% faster!", regression_pct.abs());
    }
    println!();

    if regression_pct < 10.0 {
        println!("✓ Speed regression acceptable (<10%)");
        println!("  Security improvements worth the cost");
    } else if regression_pct < 25.0 {
        println!("⚠ Moderate speed regression");
        println!("  May need optimization");
    } else {
        println!("✗ Severe speed regression!");
        println!("  Fixes too expensive, need rethinking");
    }
}

fn print_final_verdict() {
    println!("\n{}", "=".repeat(80));
    println!("FINAL VERDICT - v7.5 Targeted Fixes");
    println!("{}", "=".repeat(80));
    println!();

    println!("Benchmarks identified 3 broken features. Did v7.5 fix them?");
    println!();

    println!("The tests above show the HONEST results:");
    println!();
    println!("  Fix #1 (Avalanche):     [See test results above]");
    println!("  Fix #2 (Differential):  [See test results above]");
    println!("  Fix #3 (Learning):      [See test results above]");
    println!("  Speed Regression:       [See test results above]");
    println!();

    println!("{}", "=".repeat(80));
    println!("Run this test to see if the targeted fixes actually worked!");
    println!("{}", "=".repeat(80));
}
