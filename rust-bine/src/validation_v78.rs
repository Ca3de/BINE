/*!
BINE v7.8 Validation - Input Whitening Fix
===========================================

Previous: v7.7 had 0.5000 max bias
Root cause: ChaCha20 can't handle regular patterns like [42,42,...,42]

v7.8 Fix: 3-pass input whitening
- Pass 1: Position-dependent XOR + S-box
- Pass 2: Neighborhood mixing
- Pass 3: Long-range diffusion

Target: Differential < 0.01
*/

use std::time::Instant;

mod lib_v77;
mod lib_v78;

use lib_v77::BineHasherV77 as BineV77;
use lib_v77::SecurityMode as SecurityModeV77;
use lib_v78::BineHasherV78 as BineV78;
use lib_v78::SecurityMode as SecurityModeV78;

fn main() {
    println!("{}", "=".repeat(80));
    println!("BINE v7.8 - INPUT WHITENING FIX VALIDATION");
    println!("{}", "=".repeat(80));
    println!();

    println!("Problem: ChaCha20 can't handle patterns like [42,42,42,...,42]");
    println!("Solution: 3-pass input whitening breaks patterns BEFORE hashing");
    println!();
    println!("Goal: Differential < 0.01");
    println!();

    test_differential();
    test_avalanche();
    test_speed();
}

fn test_differential() {
    println!("{}", "=".repeat(80));
    println!("[DIFFERENTIAL CRYPTANALYSIS TEST]");
    println!("{}", "=".repeat(80));
    println!();

    let salt = b"differential_test_salt_32!!!!!";
    let test_count = 256;

    println!("Testing {} input pairs with controlled differences...", test_count);
    println!();

    // v7.7 baseline
    let mut v77_hasher = BineV77::new(256, SecurityModeV77::Balanced);
    let mut v77_max_bias = 0.0f64;
    let mut v77_avg_bias = 0.0f64;
    let mut v77_bias_count = 0usize;

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
            v77_avg_bias += bias;
            v77_bias_count += 1;
        }
    }
    v77_avg_bias /= v77_bias_count as f64;

    // v7.8 with input whitening
    let mut v78_hasher = BineV78::new(256, SecurityModeV78::Balanced);
    let mut v78_max_bias = 0.0f64;
    let mut v78_avg_bias = 0.0f64;
    let mut v78_bias_count = 0usize;

    for delta in 1..=test_count {
        let mut bit_differences = [0u32; 256];
        let samples = 100;

        for sample in 0..samples {
            let input1 = vec![sample as u8; 32];
            let mut input2 = input1.clone();
            input2[0] ^= delta as u8;

            let hash1 = v78_hasher.hash(&input1, salt);
            let hash2 = v78_hasher.hash(&input2, salt);

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
            if bias > v78_max_bias {
                v78_max_bias = bias;
            }
            v78_avg_bias += bias;
            v78_bias_count += 1;
        }
    }
    v78_avg_bias /= v78_bias_count as f64;

    println!("{:<15} | {:<15} | {:<15} | {:<15}",
             "Version", "Max Bias", "Avg Bias", "Status");
    println!("{}", "-".repeat(70));

    let v77_status = if v77_max_bias < 0.01 {
        "✓ EXCELLENT"
    } else if v77_max_bias < 0.02 {
        "✓ GOOD"
    } else if v77_max_bias < 0.05 {
        "⚠ ACCEPTABLE"
    } else {
        "✗ BAD"
    };

    let v78_status = if v78_max_bias < 0.005 {
        "✓✓✓ AMAZING"
    } else if v78_max_bias < 0.01 {
        "✓✓ EXCELLENT"
    } else if v78_max_bias < 0.02 {
        "✓ GOOD"
    } else if v78_max_bias < 0.05 {
        "⚠ IMPROVED"
    } else {
        "✗ BAD"
    };

    println!("{:<15} | {:<15.4} | {:<15.4} | {:<15}",
             "v7.7 (broken)", v77_max_bias, v77_avg_bias, v77_status);
    println!("{:<15} | {:<15.4} | {:<15.4} | {:<15}",
             "v7.8 (whitened)", v78_max_bias, v78_avg_bias, v78_status);
    println!();

    if v78_max_bias < v77_max_bias {
        let improvement_pct = ((v77_max_bias - v78_max_bias) / v77_max_bias) * 100.0;
        println!("Improvement: {:.1}% reduction in max bias", improvement_pct);
        println!("  {:.4} → {:.4}", v77_max_bias, v78_max_bias);
        println!();
    }

    if v78_max_bias < 0.005 {
        println!("✓✓✓ AMAZING SUCCESS! Differential < 0.005!");
        println!("    This is exceptional cryptographic strength!");
        println!("    Input whitening completely fixed the problem!");
    } else if v78_max_bias < 0.01 {
        println!("✓✓ EXCELLENT! Differential < 0.01!");
        println!("   TARGET ACHIEVED! Strong differential resistance!");
        println!("   Input whitening works perfectly!");
    } else if v78_max_bias < 0.02 {
        println!("✓ GOOD! Differential < 0.02!");
        println!("  Almost at target - very strong improvement!");
    } else if v78_max_bias < v77_max_bias {
        println!("⚠ IMPROVED but not yet at target (<0.01)");
        println!("  Need more aggressive whitening");
    } else {
        println!("✗ No improvement - need different strategy");
    }
}

fn test_avalanche() {
    println!("\n{}", "=".repeat(80));
    println!("[AVALANCHE EFFECT - Ensure Not Broken]");
    println!("{}", "=".repeat(80));
    println!();

    let salt = b"avalanche_test";
    let test_count = 1000;

    let mut v78_hasher = BineV78::new(256, SecurityModeV78::Balanced);
    let mut v78_total_flipped = 0u64;
    let mut v78_total_bits = 0u64;

    for i in 0..test_count {
        let mut input1 = vec![0u8; 32];
        input1[i % 32] = (i / 32) as u8;

        let mut input2 = input1.clone();
        input2[i % 32] ^= 1u8 << (i % 8);

        let hash1 = v78_hasher.hash(&input1, salt);
        let hash2 = v78_hasher.hash(&input2, salt);

        for j in 0..hash1.len().min(hash2.len()) {
            let diff = hash1[j] ^ hash2[j];
            v78_total_flipped += diff.count_ones() as u64;
            v78_total_bits += 8;
        }
    }

    let v78_avalanche = (v78_total_flipped as f64 / v78_total_bits as f64) * 100.0;

    let status = if v78_avalanche >= 48.0 && v78_avalanche <= 52.0 {
        "✓ PERFECT"
    } else if v78_avalanche >= 45.0 && v78_avalanche <= 55.0 {
        "✓ GOOD"
    } else {
        "✗ BROKEN"
    };

    println!("v7.8 Avalanche: {:.2}% (target: 48-52%)", v78_avalanche);
    println!("Status: {}", status);
    println!();

    if v78_avalanche >= 48.0 && v78_avalanche <= 52.0 {
        println!("✓ Avalanche effect still perfect!");
        println!("  Input whitening didn't break avalanche!");
    } else if v78_avalanche >= 45.0 && v78_avalanche <= 55.0 {
        println!("✓ Avalanche still good");
    } else {
        println!("⚠ WARNING: Avalanche degraded by input whitening");
    }
}

fn test_speed() {
    println!("\n{}", "=".repeat(80));
    println!("[SPEED CHECK]");
    println!("{}", "=".repeat(80));
    println!();

    println!("Input whitening adds 3 full passes over data");
    println!("Expected: Some slowdown, but acceptable\n");

    let test_size = 1024 * 1024;
    let data = vec![0x42u8; test_size];
    let salt = b"speed";

    let mut v77_hasher = BineV77::new(256, SecurityModeV77::Balanced);
    let v77_start = Instant::now();
    for _ in 0..10 {
        let _ = v77_hasher.hash(&data, salt);
    }
    let v77_duration = v77_start.elapsed();
    let v77_throughput = (test_size as f64 * 10.0) / v77_duration.as_secs_f64() / 1_000_000.0;

    let mut v78_hasher = BineV78::new(256, SecurityModeV78::Balanced);
    let v78_start = Instant::now();
    for _ in 0..10 {
        let _ = v78_hasher.hash(&data, salt);
    }
    let v78_duration = v78_start.elapsed();
    let v78_throughput = (test_size as f64 * 10.0) / v78_duration.as_secs_f64() / 1_000_000.0;

    println!("{:<15} | {:<20}",
             "Version", "Throughput");
    println!("{}", "-".repeat(40));
    println!("{:<15} | {:<20.2} MB/s",
             "v7.7 (no whiten)", v77_throughput);
    println!("{:<15} | {:<20.2} MB/s",
             "v7.8 (whitened)", v78_throughput);
    println!();

    let diff_pct = ((v77_throughput - v78_throughput) / v77_throughput) * 100.0;

    if diff_pct > 0.0 {
        println!("Speed regression: {:.1}%", diff_pct);
        if diff_pct < 20.0 {
            println!("✓ ACCEPTABLE - Security improvement worth the cost");
        } else if diff_pct < 40.0 {
            println!("⚠ MODERATE - Significant slowdown");
        } else {
            println!("⚠ SIGNIFICANT - May need optimization");
        }
    } else {
        println!("Speed improvement: {:.1}%!", diff_pct.abs());
    }
}
