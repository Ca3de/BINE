#!/usr/bin/env python3
"""
BINE Intensive Testing Suite
=============================

Comprehensive stress testing, edge cases, and security validation.
"""

import unittest
import secrets
import time
import hashlib
from bine_core import BINECore, bine_hash_simple, bine_verify_simple
from bine_encrypt import BINECipher


class TestIntensiveHashing(unittest.TestCase):
    """Intensive hash testing"""

    def test_massive_data(self):
        """Test hashing 10MB of data"""
        print("\n  Testing 10MB hash...")
        data = secrets.token_bytes(10 * 1024 * 1024)
        hash_val, salt = bine_hash_simple(data)
        self.assertIsNotNone(hash_val)
        self.assertTrue(bine_verify_simple(data, hash_val, salt))

    def test_empty_data(self):
        """Test hashing empty data"""
        data = b""
        hash_val, salt = bine_hash_simple(data)
        self.assertIsNotNone(hash_val)
        self.assertTrue(bine_verify_simple(data, hash_val, salt))

    def test_single_byte(self):
        """Test hashing single byte"""
        for byte_val in [0, 1, 127, 255]:
            data = bytes([byte_val])
            hash_val, salt = bine_hash_simple(data)
            self.assertTrue(bine_verify_simple(data, hash_val, salt))

    def test_all_same_bytes(self):
        """Test data with all same bytes"""
        for byte_val in [0, 0xFF, 0x55, 0xAA]:
            data = bytes([byte_val]) * 1000
            hash_val, salt = bine_hash_simple(data)
            self.assertTrue(bine_verify_simple(data, hash_val, salt))

    def test_sequential_bytes(self):
        """Test sequential byte patterns"""
        data = bytes(range(256)) * 100
        hash_val, salt = bine_hash_simple(data)
        self.assertTrue(bine_verify_simple(data, hash_val, salt))

    def test_collision_resistance_intensive(self):
        """Test 10,000 hashes for collisions"""
        print("\n  Testing 10,000 hashes for collisions...")
        hashes = set()
        salt = secrets.token_bytes(32)

        for i in range(10000):
            data = f"test_data_{i}".encode()
            hash_val, _ = bine_hash_simple(data)
            hash_tuple = tuple(hash_val)
            self.assertNotIn(hash_tuple, hashes, f"Collision at iteration {i}")
            hashes.add(hash_tuple)

    def test_avalanche_intensive(self):
        """Test avalanche effect with 1000 samples"""
        print("\n  Testing avalanche effect (1000 samples)...")
        bine = BINECore(256)
        salt = secrets.token_bytes(32)

        differences = []
        for _ in range(1000):
            data1 = secrets.token_bytes(100)
            data2 = bytearray(data1)
            data2[0] ^= 1  # Flip one bit

            hash1 = bine.bine_hash(data1, salt)
            hash2 = bine.bine_hash(bytes(data2), salt)

            diff_count = sum(1 for a, b in zip(hash1, hash2) if a != b)
            diff_pct = (diff_count / len(hash1)) * 100
            differences.append(diff_pct)

        avg_diff = sum(differences) / len(differences)
        print(f"    Average difference: {avg_diff:.2f}%")
        self.assertGreater(avg_diff, 30, "Avalanche effect too weak")
        self.assertLess(avg_diff, 70, "Avalanche effect too strong (suspicious)")

    def test_different_security_levels_comprehensive(self):
        """Test all security levels with various data"""
        data_sizes = [0, 1, 100, 1024, 10240]
        security_levels = [128, 256, 512]

        for sec_level in security_levels:
            for size in data_sizes:
                data = secrets.token_bytes(size)
                hash_val, salt = bine_hash_simple(data, security_level=sec_level)
                self.assertTrue(bine_verify_simple(data, hash_val, salt, sec_level))


class TestIntensiveEncryption(unittest.TestCase):
    """Intensive encryption testing"""

    def test_large_plaintext(self):
        """Test encrypting 10MB"""
        print("\n  Testing 10MB encryption...")
        cipher = BINECipher(256)
        key = cipher.generate_key(b"test")

        plaintext = secrets.token_bytes(10 * 1024 * 1024)
        ciphertext = cipher.encrypt(plaintext, key)
        decrypted = cipher.decrypt(ciphertext, key)

        self.assertEqual(plaintext, decrypted)

    def test_empty_plaintext(self):
        """Test encrypting empty data"""
        cipher = BINECipher(256)
        key = cipher.generate_key(b"test")

        plaintext = b""
        ciphertext = cipher.encrypt(plaintext, key)
        decrypted = cipher.decrypt(ciphertext, key)

        self.assertEqual(plaintext, decrypted)

    def test_all_byte_values(self):
        """Test all possible byte values"""
        cipher = BINECipher(256)
        key = cipher.generate_key(b"test")

        plaintext = bytes(range(256)) * 100
        ciphertext = cipher.encrypt(plaintext, key)
        decrypted = cipher.decrypt(ciphertext, key)

        self.assertEqual(plaintext, decrypted)

    def test_repeated_encryption(self):
        """Test 1000 encryption/decryption cycles"""
        print("\n  Testing 1000 encryption cycles...")
        cipher = BINECipher(256)
        key = cipher.generate_key(b"test")

        for i in range(1000):
            plaintext = f"test_message_{i}".encode()
            ciphertext = cipher.encrypt(plaintext, key)
            decrypted = cipher.decrypt(ciphertext, key)
            self.assertEqual(plaintext, decrypted, f"Failed at iteration {i}")

    def test_wrong_key_never_decrypts(self):
        """Test that wrong keys NEVER accidentally decrypt"""
        print("\n  Testing 1000 wrong keys...")
        cipher = BINECipher(256)
        plaintext = b"Secret message"

        for _ in range(1000):
            key1 = cipher.generate_key(secrets.token_bytes(32))
            key2 = cipher.generate_key(secrets.token_bytes(32))

            ciphertext = cipher.encrypt(plaintext, key1)
            decrypted = cipher.decrypt(ciphertext, key2)

            # Should NEVER decrypt to original with wrong key
            self.assertNotEqual(plaintext, decrypted)

    def test_ciphertext_modification_detection(self):
        """Test that modified ciphertext is detected"""
        cipher = BINECipher(256)
        key = cipher.generate_key(b"test")
        plaintext = b"Important message"

        ciphertext = cipher.encrypt(plaintext, key)

        # Modify ciphertext
        modified = bytearray(ciphertext)
        modified[50] ^= 1  # Flip one bit

        try:
            decrypted = cipher.decrypt(bytes(modified), key)
            # Should not decrypt to original
            self.assertNotEqual(plaintext, decrypted)
        except Exception:
            # Or raise exception (acceptable)
            pass


class TestStressConditions(unittest.TestCase):
    """Test under stress conditions"""

    def test_rapid_fire_hashing(self):
        """Test 10,000 rapid hashes"""
        print("\n  Testing 10,000 rapid hashes...")
        start = time.time()

        for i in range(10000):
            data = f"test_{i}".encode()
            hash_val, salt = bine_hash_simple(data)

        elapsed = time.time() - start
        print(f"    Completed in {elapsed:.2f}s ({10000/elapsed:.0f} hashes/sec)")
        self.assertLess(elapsed, 60, "Too slow for 10,000 hashes")

    def test_concurrent_hashing_simulation(self):
        """Simulate concurrent usage"""
        print("\n  Simulating concurrent usage...")
        results = []

        # Simulate 100 "concurrent" operations
        for i in range(100):
            bine = BINECore(256)
            data = f"user_{i}_data".encode()
            salt = secrets.token_bytes(32)
            hash_val = bine.bine_hash(data, salt)
            results.append((data, hash_val, salt))

        # Verify all
        for data, hash_val, salt in results:
            bine = BINECore(256)
            self.assertTrue(bine.bine_verify(data, hash_val, salt))

    def test_memory_intensive(self):
        """Test with memory-intensive operations"""
        print("\n  Testing memory-intensive operations...")
        bine = BINECore(256)

        # Create 1000 different hashes
        hashes = []
        for i in range(1000):
            data = secrets.token_bytes(1024)
            salt = secrets.token_bytes(32)
            hash_val = bine.bine_hash(data, salt)
            hashes.append((data, hash_val, salt))

        # Verify all still work
        for data, hash_val, salt in hashes[:100]:  # Verify first 100
            self.assertTrue(bine.bine_verify(data, hash_val, salt))


class TestSecurityEdgeCases(unittest.TestCase):
    """Test security edge cases"""

    def test_zero_salt(self):
        """Test with all-zero salt (should still work)"""
        bine = BINECore(256)
        data = b"test data"
        salt = bytes(32)  # All zeros

        hash1 = bine.bine_hash(data, salt)
        hash2 = bine.bine_hash(data, salt)

        self.assertEqual(hash1, hash2)
        self.assertTrue(bine.bine_verify(data, hash1, salt))

    def test_similar_inputs(self):
        """Test very similar inputs produce different hashes"""
        bine = BINECore(256)
        salt = secrets.token_bytes(32)

        test_cases = [
            (b"test", b"test1"),
            (b"password", b"password1"),
            (b"a" * 100, b"a" * 101),
        ]

        for data1, data2 in test_cases:
            hash1 = bine.bine_hash(data1, salt)
            hash2 = bine.bine_hash(data2, salt)

            # Should be completely different
            diff = sum(1 for a, b in zip(hash1, hash2) if a != b)
            self.assertGreater(diff, len(hash1) // 3)

    def test_timing_attack_resistance(self):
        """Test constant-time comparison"""
        print("\n  Testing timing attack resistance...")
        bine = BINECore(256)
        data = b"password"
        salt = secrets.token_bytes(32)
        hash_val = bine.bine_hash(data, salt)

        # Time correct verification
        times_correct = []
        for _ in range(1000):
            start = time.perf_counter()
            bine.bine_verify(data, hash_val, salt)
            times_correct.append(time.perf_counter() - start)

        # Time incorrect verification
        times_wrong = []
        for _ in range(1000):
            wrong_data = b"wrong_password"
            start = time.perf_counter()
            bine.bine_verify(wrong_data, hash_val, salt)
            times_wrong.append(time.perf_counter() - start)

        avg_correct = sum(times_correct) / len(times_correct)
        avg_wrong = sum(times_wrong) / len(times_wrong)

        # Should be similar (constant-time)
        ratio = max(avg_correct, avg_wrong) / min(avg_correct, avg_wrong)
        print(f"    Time ratio (correct/wrong): {ratio:.2f}")
        self.assertLess(ratio, 2.0, "Potential timing attack vulnerability")


class TestComparison(unittest.TestCase):
    """Compare BINE to standard algorithms"""

    def test_vs_sha256(self):
        """Compare BINE hash output characteristics to SHA-256"""
        print("\n  Comparing to SHA-256...")
        bine = BINECore(256)

        test_data = [
            b"",
            b"a",
            b"test",
            b"The quick brown fox jumps over the lazy dog",
            secrets.token_bytes(1024),
        ]

        for data in test_data:
            # BINE hash
            salt = secrets.token_bytes(32)
            bine_hash = bine.bine_hash(data, salt)

            # SHA-256 hash
            sha_hash = hashlib.sha256(data + salt).digest()

            # Both should produce appropriate length outputs
            self.assertGreaterEqual(len(bine_hash), 32)
            self.assertEqual(len(sha_hash), 32)

            # Both should be deterministic
            bine_hash2 = bine.bine_hash(data, salt)
            sha_hash2 = hashlib.sha256(data + salt).digest()

            self.assertEqual(bine_hash, bine_hash2)
            self.assertEqual(sha_hash, sha_hash2)

    def test_entropy(self):
        """Test output entropy"""
        print("\n  Testing output entropy...")
        bine = BINECore(256)

        # Generate 1000 hashes
        hashes = []
        for i in range(1000):
            data = f"test_{i}".encode()
            salt = secrets.token_bytes(32)
            hash_val = bine.bine_hash(data, salt)
            hashes.append(hash_val)

        # Calculate byte distribution
        byte_counts = [0] * 256
        total_bytes = 0

        for hash_val in hashes:
            for byte in hash_val:
                byte_counts[byte] += 1
                total_bytes += 1

        # Check distribution is relatively uniform
        expected_per_byte = total_bytes / 256
        max_deviation = max(abs(count - expected_per_byte) / expected_per_byte
                          for count in byte_counts)

        print(f"    Max deviation from uniform: {max_deviation*100:.1f}%")
        self.assertLess(max_deviation, 0.15, "Output entropy too low")


def run_intensive_tests():
    """Run all intensive tests"""
    loader = unittest.TestLoader()
    suite = unittest.TestSuite()

    # Add all test classes
    test_classes = [
        TestIntensiveHashing,
        TestIntensiveEncryption,
        TestStressConditions,
        TestSecurityEdgeCases,
        TestComparison,
    ]

    for test_class in test_classes:
        tests = loader.loadTestsFromTestCase(test_class)
        suite.addTests(tests)

    # Run with verbose output
    print("\n" + "="*70)
    print("BINE INTENSIVE TEST SUITE")
    print("="*70)
    runner = unittest.TextTestRunner(verbosity=2)
    result = runner.run(suite)

    # Print summary
    print("\n" + "="*70)
    print("INTENSIVE TEST SUMMARY")
    print("="*70)
    print(f"Tests run: {result.testsRun}")
    print(f"Successes: {result.testsRun - len(result.failures) - len(result.errors)}")
    print(f"Failures: {len(result.failures)}")
    print(f"Errors: {len(result.errors)}")

    if result.wasSuccessful():
        print("\n✓ ALL INTENSIVE TESTS PASSED!")
    else:
        print("\n✗ SOME TESTS FAILED")

    print("="*70)

    return result.wasSuccessful()


if __name__ == '__main__':
    success = run_intensive_tests()
    exit(0 if success else 1)
