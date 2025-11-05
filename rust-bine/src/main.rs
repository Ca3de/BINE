use bine::{BineHasher, BineCipher};
use std::time::Instant;

fn main() {
    println!("{}", "=".repeat(70));
    println!("BINE Rust Benchmark - Standalone Implementation");
    println!("{}", "=".repeat(70));
    println!();

    // Test hash
    println!("[1] Hash Correctness Test");
    let hasher = BineHasher::new(256);
    let data = b"test data";
    let salt = b"test salt 32 bytes long here";

    let hash1 = hasher.hash(data, salt);
    let hash2 = hasher.hash(data, salt);
    assert_eq!(hash1, hash2);
    println!("  ✓ Hash is deterministic");

    let hash3 = hasher.hash(b"different data", salt);
    assert_ne!(hash1, hash3);
    println!("  ✓ Different inputs produce different hashes");
    println!();

    // Test encryption
    println!("[2] Encryption Correctness Test");
    let cipher = BineCipher::new(256);
    let plaintext = b"Hello, BINE!";
    let password = b"secret_password";

    let (ciphertext, salt) = cipher.encrypt(plaintext, password);
    let decrypted = cipher.decrypt(&ciphertext, password, &salt);

    assert_eq!(&decrypted, plaintext);
    println!("  ✓ Encryption/decryption works correctly");
    println!();

    // Benchmark hashing
    println!("[3] Hash Performance");
    let sizes = vec![
        (1024, "1 KB"),
        (10 * 1024, "10 KB"),
        (100 * 1024, "100 KB"),
        (1024 * 1024, "1 MB"),
    ];

    for (size, label) in sizes.iter() {
        let data = vec![0xAA; *size];
        let salt = b"benchmark_salt_32_bytes_here";

        let start = Instant::now();
        let _hash = hasher.hash(&data, salt);
        let duration = start.elapsed();

        let throughput = (*size as f64 / duration.as_secs_f64()) / 1_048_576.0;

        println!("  {:>7}: {:>10.4}ms ({:>8.2} MB/s)",
                 label,
                 duration.as_secs_f64() * 1000.0,
                 throughput);
    }
    println!();

    // Benchmark encryption
    println!("[4] Encryption Performance");
    let sizes = vec![
        (1024, "1 KB"),
        (10 * 1024, "10 KB"),
        (100 * 1024, "100 KB"),
        (1024 * 1024, "1 MB"),
        (10 * 1024 * 1024, "10 MB"),
    ];

    for (size, label) in sizes.iter() {
        let data = vec![0xBB; *size];
        let password = b"benchmark_password";

        let start = Instant::now();
        let (ciphertext, salt) = cipher.encrypt(&data, password);
        let enc_duration = start.elapsed();

        let start = Instant::now();
        let plaintext = cipher.decrypt(&ciphertext, password, &salt);
        let dec_duration = start.elapsed();

        assert_eq!(plaintext, data, "Decryption failed for {}", label);

        let enc_throughput = (*size as f64 / enc_duration.as_secs_f64()) / 1_048_576.0;
        let dec_throughput = (*size as f64 / dec_duration.as_secs_f64()) / 1_048_576.0;

        println!("  {:>7}:", label);
        println!("    Encrypt: {:>10.4}ms ({:>8.2} MB/s)",
                 enc_duration.as_secs_f64() * 1000.0,
                 enc_throughput);
        println!("    Decrypt: {:>10.4}ms ({:>8.2} MB/s)",
                 dec_duration.as_secs_f64() * 1000.0,
                 dec_throughput);
    }
    println!();

    // Collision resistance test
    println!("[5] Collision Resistance (10,000 hashes)");
    use std::collections::HashSet;
    let mut hashes = HashSet::new();
    let salt = b"collision_test_salt_32_bytes";

    let start = Instant::now();
    for i in 0..10000 {
        let data = format!("test_data_{}", i);
        let hash = hasher.hash(data.as_bytes(), salt);
        hashes.insert(hash);
    }
    let duration = start.elapsed();

    println!("  Unique hashes: {}/10000", hashes.len());
    println!("  Time: {:.4}s", duration.as_secs_f64());
    println!("  Collisions: {}", 10000 - hashes.len());
    println!();

    println!("{}", "=".repeat(70));
    println!("All tests passed! ✓");
    println!("{}", "=".repeat(70));
}
