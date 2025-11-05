/*!
BINE v7.6 Validation - AGGRESSIVE Differential Fix
===================================================

v7.5 Results:
✓ Avalanche: 50.09% (PERFECT!)
✗ Differential: 0.5000 (WORST POSSIBLE - completely predictable!)

v7.6 Goal:
Reduce differential bias from 0.5000 to <0.01 (ideally <0.005)

This validation tests whether the aggressive anti-differential
transform actually works.
*/

use std::time::Instant;

mod lib_v75;
mod lib_v76;

use lib_v75::BineHasherV75 as BineV75;
use lib_v75::SecurityMode as SecurityModeV75;
use lib_v76::BineHasherV76 as BineV76;
use lib_v76::SecurityMode as SecurityModeV76;

fn main() {
    println!("{}", "=".repeat(80));
    println!("BINE v7.6 - DIFFERENTIAL CRYPTANALYSIS FIX VALIDATION");
    println!("{}", "=".repeat(80));
    println!();

    println!("v7.5 Status:");
    println!("  ✓ Avalanche: 50.09% (PERFECT!)");
    println!("  ✗ Differential: 0.5000 (WORST POSSIBLE!)");
    println!();
    println!("v7.6 Goal: Differential < 0.01");
    println!();

    // Main test: Differential cryptanalysis resistance
    test_differential_detailed();

    // Ensure avalanche wasn't broken
    test_avalanche_regression();

    // Speed check
    test_speed();

    // Final verdict
    print_final_verdict();
}

fn test_differential_detailed() {
    println!("{}", "=".repeat(80));
    println!("[MAIN TEST] DIFFERENTIAL CRYPTANALYSIS RESISTANCE");
    println!("{}", "=".repeat(80));

    println!("\nv7.6 Strategy:");
    println!("  • 4-layer anti-differential transform");
    println!("  • Triple S-box application per round");
    println!("  • Cross-mixing: each byte affects 8 distant positions");
    println!("  • Double S-box feedback loops");
    println!("  • Key-dependent rotations");
    println!();

    let salt = b"differential_test_salt_32!!!!!";
    let test_count = 256;

    println!("Testing {} input pairs with controlled differences...", test_count);
    println!("(This tests resistance to differential cryptanalysis attacks)");
    println!();

    // Test v7.5
    let mut v75_hasher = BineV75::new(256, SecurityModeV75::Balanced);
    let mut v75_max_bias = 0.0f64;
    let mut v75_avg_bias = 0.0f64;
    let mut v75_bias_count = 0usize;

    for delta in 1..=test_count {
        let mut bit_differences = [0u32; 256];
        let samples = 100;

        for sample in 0..samples {
            let input1 = vec![sample as u8; 32];
            let mut input2 = input1.clone();
            input2[0] ^= delta as u8;

            let hash1 = v75_hasher.hash(&input1, salt);
            let hash2 = v75_hasher.hash(&input2, salt);

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
            v75_avg_bias += bias;
            v75_bias_count += 1;
        }
    }
    v75_avg_bias /= v75_bias_count as f64;

    // Test v7.6
    let mut v76_hasher = BineV76::new(256, SecurityModeV76::Balanced);
    let mut v76_max_bias = 0.0f64;
    let mut v76_avg_bias = 0.0f64;
    let mut v76_bias_count = 0usize;

    for delta in 1..=test_count {
        let mut bit_differences = [0u32; 256];
        let samples = 100;

        for sample in 0..samples {
            let input1 = vec![sample as u8; 32];
            let mut input2 = input1.clone();
            input2[0] ^= delta as u8;

            let hash1 = v76_hasher.hash(&input1, salt);
            let hash2 = v76_hasher.hash(&input2, salt);

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
            if bias > v76_max_bias {
                v76_max_bias = bias;
            }
            v76_avg_bias += bias;
            v76_bias_count += 1;
        }
    }
    v76_avg_bias /= v76_bias_count as f64;

    println!("{:<15} | {:<15} | {:<15} | {:<10}",
             "Version", "Max Bias", "Avg Bias", "Status");
    println!("{}", "-".repeat(70));

    let v75_status = if v75_max_bias < 0.02 {
        "✓ EXCELLENT"
    } else if v75_max_bias < 0.05 {
        "⚠ ACCEPTABLE"
    } else if v75_max_bias < 0.10 {
        "⚠ WEAK"
    } else {
        "✗ BROKEN"
    };

    let v76_status = if v76_max_bias < 0.01 {
        "✓ EXCELLENT"
    } else if v76_max_bias < 0.02 {
        "✓ GOOD"
    } else if v76_max_bias < 0.05 {
        "⚠ ACCEPTABLE"
    } else {
        "✗ BAD"
    };

    println!("{:<15} | {:<15.4} | {:<15.4} | {:<10}",
             "v7.5 (broken)", v75_max_bias, v75_avg_bias, v75_status);
    println!("{:<15} | {:<15.4} | {:<15.4} | {:<10}",
             "v7.6 (fixed)", v76_max_bias, v76_avg_bias, v76_status);
    println!();

    let improvement = v75_max_bias - v76_max_bias;
    let improvement_pct = (improvement / v75_max_bias) * 100.0;

    println!("Improvement:");
    println!("  Max bias: {:.4} → {:.4} (reduced by {:.1}%)",
             v75_max_bias, v76_max_bias, improvement_pct);
    println!("  Avg bias: {:.4} → {:.4}",
             v75_avg_bias, v76_avg_bias);
    println!();

    if v76_max_bias < 0.01 {
        println!("✓✓✓ AMAZING! Differential bias < 0.01!");
        println!("    This is cryptographically excellent!");
    } else if v76_max_bias < 0.02 {
        println!("✓✓ EXCELLENT! Differential bias < 0.02!");
        println!("   Strong resistance to differential cryptanalysis");
    } else if v76_max_bias < 0.05 {
        println!("✓ GOOD: Improved but not yet under 0.01");
    } else if improvement > 0.0 {
        println!("⚠ PARTIAL: Some improvement but needs more work");
    } else {
        println!("✗ FAILED: No improvement");
    }
}

fn test_avalanche_regression() {
    println!("\n{}", "=".repeat(80));
    println!("[REGRESSION CHECK] AVALANCHE EFFECT - Ensure It Wasn't Broken");
    println!("{}", "=".repeat(80));

    let salt = b"avalanche_test_salt_32_bytes!!!";
    let test_count = 1000;

    println!("\nTesting {} samples to ensure avalanche still ~50%...", test_count);
    println!();

    let mut v76_hasher = BineV76::new(256, SecurityModeV76::Balanced);
    let mut v76_total_flipped = 0u64;
    let mut v76_total_bits = 0u64;

    for i in 0..test_count {
        let mut input1 = vec![0u8; 32];
        input1[i % 32] = (i / 32) as u8;

        let mut input2 = input1.clone();
        input2[i % 32] ^= 1u8 << (i % 8);

        let hash1 = v76_hasher.hash(&input1, salt);
        let hash2 = v76_hasher.hash(&input2, salt);

        for j in 0..hash1.len().min(hash2.len()) {
            let diff = hash1[j] ^ hash2[j];
            v76_total_flipped += diff.count_ones() as u64;
            v76_total_bits += 8;
        }
    }

    let v76_avalanche = (v76_total_flipped as f64 / v76_total_bits as f64) * 100.0;

    let status = if v76_avalanche >= 48.0 && v76_avalanche <= 52.0 {
        "✓ EXCELLENT"
    } else if v76_avalanche >= 45.0 && v76_avalanche <= 55.0 {
        "✓ GOOD"
    } else {
        "✗ BROKEN"
    };

    println!("v7.6 Avalanche: {:.2}% (target: 48-52%) - {}", v76_avalanche, status);
    println!();

    if v76_avalanche >= 48.0 && v76_avalanche <= 52.0 {
        println!("✓ Avalanche effect maintained - still perfect!");
    } else if v76_avalanche >= 45.0 && v76_avalanche <= 55.0 {
        println!("✓ Avalanche effect still good");
    } else {
        println!("✗ WARNING: Avalanche effect degraded!");
    }
}

fn test_speed() {
    println!("\n{}", "=".repeat(80));
    println!("[PERFORMANCE CHECK] SPEED");
    println!("{}", "=".repeat(80));

    println!("\nv7.6 added significant complexity:");
    println!("  • 4-layer anti-differential transform");
    println!("  • Triple S-box per round");
    println!("  • Cross-mixing with 8 positions");
    println!();

    let test_size = 1024 * 1024;
    let data = vec![0x42u8; test_size];
    let salt = b"speed_test";

    let mut v75_hasher = BineV75::new(256, SecurityModeV75::Balanced);
    let v75_start = Instant::now();
    for _ in 0..10 {
        let _ = v75_hasher.hash(&data, salt);
    }
    let v75_duration = v75_start.elapsed();
    let v75_throughput = (test_size as f64 * 10.0) / v75_duration.as_secs_f64() / 1_000_000.0;

    let mut v76_hasher = BineV76::new(256, SecurityModeV76::Balanced);
    let v76_start = Instant::now();
    for _ in 0..10 {
        let _ = v76_hasher.hash(&data, salt);
    }
    let v76_duration = v76_start.elapsed();
    let v76_throughput = (test_size as f64 * 10.0) / v76_duration.as_secs_f64() / 1_000_000.0;

    println!("{:<15} | {:<20} | {:<15}",
             "Version", "Throughput", "Status");
    println!("{}", "-".repeat(55));

    println!("{:<15} | {:<20.2} MB/s | Baseline",
             "v7.5", v75_throughput);

    let regression_pct = ((v75_throughput - v76_throughput) / v75_throughput) * 100.0;
    let status = if regression_pct < 15.0 {
        "✓ ACCEPTABLE"
    } else if regression_pct < 30.0 {
        "⚠ MODERATE"
    } else {
        "⚠ SIGNIFICANT"
    };

    println!("{:<15} | {:<20.2} MB/s | {:<15}",
             "v7.6", v76_throughput, status);
    println!();

    if regression_pct > 0.0 {
        println!("Speed regression: {:.1}% slower", regression_pct);
        if regression_pct < 30.0 {
            println!("✓ Acceptable trade-off for security improvement");
        } else {
            println!("⚠ Significant slowdown - may need optimization");
        }
    } else {
        println!("Speed improvement: {:.1}% faster!", regression_pct.abs());
    }
}

fn print_final_verdict() {
    println!("\n{}", "=".repeat(80));
    println!("FINAL VERDICT - v7.6 DIFFERENTIAL FIX");
    println!("{}", "=".repeat(80));
    println!();

    println!("Primary Goal: Fix differential bias (0.5000 → <0.01)");
    println!();

    println!("Results: [See detailed results above]");
    println!();

    println!("If v7.6 achieved:");
    println!("  • Differential < 0.01:  AMAZING SUCCESS! 🎉");
    println!("  • Differential < 0.02:  GOOD! Target met");
    println!("  • Differential < 0.05:  Improvement, but needs more");
    println!("  • Differential >= 0.05: Need stronger approach");
    println!();

    println!("{}", "=".repeat(80));
}
