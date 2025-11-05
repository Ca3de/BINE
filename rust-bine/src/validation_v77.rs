/*!
BINE v7.7 Validation - Root Cause Fix
======================================

v7.6 failed with 0.5000 max bias (no improvement)
Root cause identified: Untransformed bytes in output!

v7.7 Fix:
- Triple S-box on EVERY output byte
- Final mixing pass ensuring all bytes are non-linear
- No untransformed checksum/padding bytes

Target: Differential < 0.01
*/

use std::time::Instant;

mod lib_v76;
mod lib_v77;

use lib_v76::BineHasherV76 as BineV76;
use lib_v76::SecurityMode as SecurityModeV76;
use lib_v77::BineHasherV77 as BineV77;
use lib_v77::SecurityMode as SecurityModeV77;

fn main() {
    println!("{}", "=".repeat(80));
    println!("BINE v7.7 - ROOT CAUSE FIX VALIDATION");
    println!("{}", "=".repeat(80));
    println!();

    println!("v7.6 failed: Max bias still 0.5000");
    println!("Root cause: Untransformed bytes in output");
    println!();
    println!("v7.7 fix: Triple S-box on EVERY output byte");
    println!();

    test_differential();
    test_avalanche();
    test_speed();
}

fn test_differential() {
    println!("{}", "=".repeat(80));
    println!("[DIFFERENTIAL TEST]");
    println!("{}", "=".repeat(80));
    println!();

    let salt = b"differential_test_salt_32!!!!!";
    let test_count = 256;

    println!("Testing {} input pairs...\n", test_count);

    // v7.6
    let mut v76_hasher = BineV76::new(256, SecurityModeV76::Balanced);
    let mut v76_max_bias = 0.0f64;

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
        }
    }

    // v7.7
    let mut v77_hasher = BineV77::new(256, SecurityModeV77::Balanced);
    let mut v77_max_bias = 0.0f64;

    for delta in 1..=test_count {
        let mut bit_differences = [0u32; 256];
        let samples = 100;

        for sample in 0..samples {
            let input1 = vec![sample as u8; 32];
            let mut input2 = input1.clone();
            input2[0] ^= delta as u8;

            let hash1 = v77_hasher.hash(&input1, salt);
            let hash2 = v77_hasher.hash(&input2, salt);

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
            if bias > v77_max_bias {
                v77_max_bias = bias;
            }
        }
    }

    println!("{:<15} | {:<20} | {:<15}",
             "Version", "Max Bias", "Status");
    println!("{}", "-".repeat(55));

    let v76_status = if v76_max_bias < 0.01 {
        "✓ EXCELLENT"
    } else if v76_max_bias < 0.02 {
        "✓ GOOD"
    } else {
        "✗ BAD"
    };

    let v77_status = if v77_max_bias < 0.01 {
        "✓✓✓ AMAZING"
    } else if v77_max_bias < 0.02 {
        "✓✓ EXCELLENT"
    } else if v77_max_bias < 0.05 {
        "✓ IMPROVED"
    } else {
        "✗ BAD"
    };

    println!("{:<15} | {:<20.4} | {:<15}",
             "v7.6 (broken)", v76_max_bias, v76_status);
    println!("{:<15} | {:<20.4} | {:<15}",
             "v7.7 (fixed)", v77_max_bias, v77_status);
    println!();

    if v77_max_bias < 0.01 {
        let improvement_pct = ((v76_max_bias - v77_max_bias) / v76_max_bias) * 100.0;
        println!("✓✓✓ AMAZING! Differential < 0.01!");
        println!("    Improved by {:.1}%!", improvement_pct);
        println!("    Cryptographically excellent differential resistance!");
    } else if v77_max_bias < 0.02 {
        println!("✓✓ EXCELLENT! Differential < 0.02!");
        println!("   Target achieved!");
    } else if v77_max_bias < v76_max_bias {
        let improvement_pct = ((v76_max_bias - v77_max_bias) / v76_max_bias) * 100.0;
        println!("✓ IMPROVED by {:.1}%", improvement_pct);
        println!("  But need to reach <0.01");
    } else {
        println!("✗ Still needs more work");
    }
}

fn test_avalanche() {
    println!("\n{}", "=".repeat(80));
    println!("[AVALANCHE REGRESSION CHECK]");
    println!("{}", "=".repeat(80));
    println!();

    let salt = b"avalanche_test";
    let test_count = 1000;

    let mut v77_hasher = BineV77::new(256, SecurityModeV77::Balanced);
    let mut v77_total_flipped = 0u64;
    let mut v77_total_bits = 0u64;

    for i in 0..test_count {
        let mut input1 = vec![0u8; 32];
        input1[i % 32] = (i / 32) as u8;

        let mut input2 = input1.clone();
        input2[i % 32] ^= 1u8 << (i % 8);

        let hash1 = v77_hasher.hash(&input1, salt);
        let hash2 = v77_hasher.hash(&input2, salt);

        for j in 0..hash1.len().min(hash2.len()) {
            let diff = hash1[j] ^ hash2[j];
            v77_total_flipped += diff.count_ones() as u64;
            v77_total_bits += 8;
        }
    }

    let v77_avalanche = (v77_total_flipped as f64 / v77_total_bits as f64) * 100.0;

    let status = if v77_avalanche >= 48.0 && v77_avalanche <= 52.0 {
        "✓ PERFECT"
    } else if v77_avalanche >= 45.0 && v77_avalanche <= 55.0 {
        "✓ GOOD"
    } else {
        "✗ BROKEN"
    };

    println!("v7.7 Avalanche: {:.2}% - {}", v77_avalanche, status);

    if v77_avalanche >= 48.0 && v77_avalanche <= 52.0 {
        println!("✓ Avalanche maintained!");
    }
}

fn test_speed() {
    println!("\n{}", "=".repeat(80));
    println!("[SPEED CHECK]");
    println!("{}", "=".repeat(80));
    println!();

    let test_size = 1024 * 1024;
    let data = vec![0x42u8; test_size];
    let salt = b"speed";

    let mut v76_hasher = BineV76::new(256, SecurityModeV76::Balanced);
    let v76_start = Instant::now();
    for _ in 0..10 {
        let _ = v76_hasher.hash(&data, salt);
    }
    let v76_duration = v76_start.elapsed();
    let v76_throughput = (test_size as f64 * 10.0) / v76_duration.as_secs_f64() / 1_000_000.0;

    let mut v77_hasher = BineV77::new(256, SecurityModeV77::Balanced);
    let v77_start = Instant::now();
    for _ in 0..10 {
        let _ = v77_hasher.hash(&data, salt);
    }
    let v77_duration = v77_start.elapsed();
    let v77_throughput = (test_size as f64 * 10.0) / v77_duration.as_secs_f64() / 1_000_000.0;

    println!("v7.6: {:.2} MB/s", v76_throughput);
    println!("v7.7: {:.2} MB/s", v77_throughput);

    let diff_pct = ((v76_throughput - v77_throughput) / v76_throughput) * 100.0;
    if diff_pct.abs() < 10.0 {
        println!("✓ Speed difference: {:.1}%", diff_pct);
    }
}
