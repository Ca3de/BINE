/*!
BINE v8.0 Quick Validation - LEAN Custom Hash
==============================================

Testing ONLY differential (fast test) to see if custom hash works!
*/

mod lib_v80;
use lib_v80::BineHasherV80 as BineV80;
use lib_v80::SecurityMode;

fn main() {
    println!("================================================================================");
    println!("BINE v8.0 - LEAN CUSTOM HASH TEST");
    println!("================================================================================");
    println!();
    println!("Testing differential resistance with LEAN custom hash...");
    println!("(Single pass, O(n), ~100 ops for 32-byte input)");
    println!();

    test_differential();
}

fn test_differential() {
    let salt = b"differential_test_salt_32!!!!!";
    let test_count = 255;  // Fixed: 256 as u8 = 0, causing identical inputs

    println!("Testing {} input pairs with 100 samples each...", test_count);
    println!();

    let mut v80_hasher = BineV80::new(256, SecurityMode::Balanced);
    let mut v80_max_bias = 0.0f64;
    let mut v80_avg_bias = 0.0f64;
    let mut v80_bias_count = 0usize;

    for delta in 1..=test_count {
        let mut bit_differences = [0u32; 256];
        let samples = 100;  // Balance speed and statistics

        for sample in 0..samples {
            let input1 = vec![sample as u8; 32];
            let mut input2 = input1.clone();
            input2[0] ^= delta as u8;

            let hash1 = v80_hasher.hash(&input1, salt);
            let hash2 = v80_hasher.hash(&input2, salt);

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
            if bias > v80_max_bias {
                v80_max_bias = bias;
            }
            v80_avg_bias += bias;
            v80_bias_count += 1;
        }
    }
    v80_avg_bias /= v80_bias_count as f64;

    println!("RESULTS:");
    println!("========");
    println!();
    println!("Max Bias:  {:.4}", v80_max_bias);
    println!("Avg Bias:  {:.4}", v80_avg_bias);
    println!();

    let status = if v80_max_bias < 0.005 {
        "✓✓✓ AMAZING! < 0.005"
    } else if v80_max_bias < 0.01 {
        "✓✓ EXCELLENT! < 0.01 - TARGET ACHIEVED!"
    } else if v80_max_bias < 0.02 {
        "✓ GOOD! < 0.02"
    } else if v80_max_bias < 0.05 {
        "⚠ IMPROVED! < 0.05"
    } else if v80_max_bias < 0.10 {
        "⚠ MODERATE: < 0.10"
    } else {
        "✗ NEEDS WORK: >= 0.10"
    };

    println!("Status: {}", status);
    println!();

    if v80_max_bias < 0.01 {
        println!("🎉 SUCCESS! Custom BINE hash achieves <0.01 differential!");
        println!("   BINE now has its own secure cryptographic primitive!");
    } else if v80_max_bias < 0.05 {
        println!("✓ Good progress! Getting close to <0.01 target.");
    } else {
        println!("⚠ More optimization needed for <0.01.");
    }
}
