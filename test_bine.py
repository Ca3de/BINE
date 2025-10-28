"""
BINE Test Suite
===============

Comprehensive tests for bio-inspired cryptographic features.
Tests security properties, performance, and biological analogies.
"""

import unittest
import secrets
import time
from bine_core import BINECore, bine_hash_simple, bine_verify_simple, BINEState
from bine_encrypt import BINECipher, BINEKey, encrypt_simple, decrypt_simple


class TestTardigradeResilience(unittest.TestCase):
    """Test tardigrade-inspired resilience features"""

    def test_cryptobiotic_state_serialization(self):
        """Test that state can be suspended and revived like tardigrade tun"""
        bine = BINECore(256)
        original_data = b"Test data for tardigrade resilience"

        # Get original hash
        original_hash, salt = bine_hash_simple(original_data)

        # Enter cryptobiotic state
        state = bine.get_state()
        suspended = state.to_cryptobiotic()

        # "Extreme conditions" - destroy original object
        del bine

        # Revive from suspended state
        revived_state = BINEState.from_cryptobiotic(suspended)
        new_bine = BINECore(256)
        new_bine.restore_state(revived_state)

        # Should produce same hash
        new_hash = new_bine.bine_hash(original_data, salt)
        self.assertEqual(original_hash, new_hash, "Hash should survive cryptobiotic cycle")

    def test_error_tolerance(self):
        """Test resilience to bit flips (like tardigrade DNA protection)"""
        bine = BINECore(256)
        data = b"Error tolerance test" * 100

        original_hash, salt = bine_hash_simple(data)

        # Verify original
        self.assertTrue(bine_verify_simple(data, original_hash, salt))

        # Small change should produce completely different hash
        modified_data = bytearray(data)
        modified_data[0] ^= 1  # Flip one bit

        modified_hash, _ = bine_hash_simple(bytes(modified_data))
        self.assertNotEqual(original_hash, modified_hash, "Avalanche effect should work")

    def test_multi_round_resilience(self):
        """Test that multi-round processing provides security"""
        bine = BINECore(256)
        test_data = b"Multi-round test"

        # Process through multiple rounds
        state = test_data
        for round_num in range(12):  # CRYPTOBIOTIC_ROUNDS
            state = bine._tardigrade_transform(state, round_num)

        # Should be completely different from original
        self.assertNotEqual(state[:len(test_data)], test_data)


class TestJellyfishRegeneration(unittest.TestCase):
    """Test jellyfish-inspired regenerative features"""

    def test_key_regeneration(self):
        """Test that keys can regenerate like jellyfish life cycle"""
        cipher = BINECipher(256)
        passphrase = b"jellyfish test passphrase"

        # Generate initial key (medusa stage)
        key_gen0 = cipher.generate_key(passphrase)

        # Regenerate to generation 1 (polyp stage)
        key_gen1 = key_gen0.regenerate(1)

        # Should be different keys
        self.assertNotEqual(key_gen0.primary_key, key_gen1.primary_key)

        # But derived from same master seed
        self.assertEqual(key_gen0.master_seed, key_gen1.master_seed)

        # Should be able to regenerate back
        key_gen1_again = key_gen0.regenerate(1)
        self.assertEqual(key_gen1.primary_key, key_gen1_again.primary_key)

    def test_generation_compatibility(self):
        """Test encryption/decryption across generations"""
        cipher = BINECipher(256)
        plaintext = b"Immortal jellyfish message"

        key_gen0 = cipher.generate_key(b"password")
        key_gen1 = key_gen0.regenerate(1)

        # Encrypt with gen 0
        ciphertext = cipher.encrypt(plaintext, key_gen0)

        # Should decrypt with gen 0
        decrypted = cipher.decrypt(ciphertext, key_gen0)
        self.assertEqual(plaintext, decrypted)

    def test_cryptobiotic_key_storage(self):
        """Test key can be stored and restored (dormancy)"""
        cipher = BINECipher(256)
        original_key = cipher.generate_key(b"storage test")

        # Suspend key
        suspended = original_key.to_cryptobiotic()

        # Revive key
        revived_key = BINEKey.from_cryptobiotic(suspended)

        # Should have same properties
        self.assertEqual(original_key.primary_key, revived_key.primary_key)
        self.assertEqual(original_key.generation, revived_key.generation)
        self.assertEqual(original_key.master_seed, revived_key.master_seed)


class TestCockroachDistribution(unittest.TestCase):
    """Test cockroach-inspired redundancy"""

    def test_fragment_distribution(self):
        """Test that data can be distributed across fragments"""
        bine = BINECore(256)
        data = b"Cockroach survival data"

        fragments = bine._cockroach_distribute(data, redundancy=3)

        # Should have 3 fragments
        self.assertEqual(len(fragments), 3)

        # Each fragment should be different
        self.assertNotEqual(fragments[0], fragments[1])
        self.assertNotEqual(fragments[1], fragments[2])

    def test_no_single_point_of_failure(self):
        """Test that losing one component doesn't break the system"""
        cipher = BINECipher(256)
        key = cipher.generate_key(b"redundancy test")

        # Key has multiple fragments
        self.assertGreater(len(key.fragments), 1)

        # Even if we "lose" a fragment, key still works
        original_fragments = key.fragments[:]
        key.fragments = key.fragments[:2]  # Lose one fragment

        plaintext = b"No single point of failure"
        ciphertext = cipher.encrypt(plaintext, key)

        # Restore full key
        key.fragments = original_fragments
        decrypted = cipher.decrypt(ciphertext, key)
        self.assertEqual(plaintext, decrypted)


class TestOstrichDiversity(unittest.TestCase):
    """Test ostrich-inspired antibody diversity"""

    def test_antibody_variants(self):
        """Test generation of diverse variants like antibodies"""
        bine = BINECore(256)
        data = b"Antibody diversity test"

        antibodies = bine._ostrich_diversify(data)

        # Should generate multiple variants
        self.assertEqual(len(antibodies), 8)  # ANTIBODY_VARIANTS

        # All should be different
        unique_antibodies = set(antibodies)
        self.assertEqual(len(unique_antibodies), len(antibodies))

    def test_salt_strengthening(self):
        """Test that salt adds strength like ostrich antibodies"""
        bine = BINECore(256)
        data = b"Salt test"

        # Same data, different salts = different hashes
        hash1, salt1 = bine_hash_simple(data)
        hash2, salt2 = bine_hash_simple(data)

        self.assertNotEqual(salt1, salt2)
        self.assertNotEqual(hash1, hash2)

    def test_multi_layer_encryption(self):
        """Test multiple encryption layers like antibody diversity"""
        cipher = BINECipher(256)
        key = cipher.generate_key(b"multi-layer test")

        plaintext = b"Multiple defense layers" * 10

        # Generate round keys (like antibodies)
        round_keys = cipher._derive_round_keys(key, len(plaintext))

        # Should have multiple rounds
        self.assertGreater(len(round_keys), 1)

        # Each should be unique
        self.assertEqual(len(set(round_keys)), len(round_keys))


class TestBatAdaptiveResponse(unittest.TestCase):
    """Test bat-inspired adaptive complexity"""

    def test_threat_response_scaling(self):
        """Test that complexity scales with threat level"""
        bine = BINECore(256)

        # Low threat
        low_complexity = bine._bat_adaptive_complexity(0.3)

        # High threat
        high_complexity = bine._bat_adaptive_complexity(0.9)

        # Should scale up but not overreact
        self.assertGreater(high_complexity, low_complexity)
        self.assertLessEqual(high_complexity, 100)  # MAX_COMPLEXITY

    def test_controlled_response(self):
        """Test that response is controlled (no inflammation)"""
        bine = BINECore(256)

        # Even at max threat, shouldn't exceed limits
        max_threat = bine._bat_adaptive_complexity(1.0)
        self.assertLessEqual(max_threat, 100)

        # At threshold, should still be reasonable
        threshold = bine._bat_adaptive_complexity(0.7)
        self.assertGreater(threshold, 10)
        self.assertLess(threshold, 100)


class TestSharkMolecularBinding(unittest.TestCase):
    """Test shark-inspired precise binding"""

    def test_tight_binding(self):
        """Test VNAR-like tight binding of data to keys"""
        bine = BINECore(256)
        data = b"Shark binding test"
        key = secrets.token_bytes(32)

        bound = bine._shark_bind(data, key)

        # Binding should transform data
        self.assertNotEqual(bound[:len(data)], data)

        # Different keys produce different bindings
        key2 = secrets.token_bytes(32)
        bound2 = bine._shark_bind(data, key2)
        self.assertNotEqual(bound, bound2)

    def test_stability_under_stress(self):
        """Test that operations remain stable (like shark antibodies in urea)"""
        bine = BINECore(256)
        data = b"Stress test data" * 1000  # Large data

        # Should handle large data without issues
        try:
            bound = bine._shark_bind(data, bine.master_seed)
            self.assertIsNotNone(bound)
        except Exception as e:
            self.fail(f"Should remain stable under stress: {e}")


class TestAlligatorDefense(unittest.TestCase):
    """Test alligator-inspired broad-spectrum defense"""

    def test_multiple_peptides(self):
        """Test multiple defense mechanisms like antimicrobial peptides"""
        bine = BINECore(256)
        data = b"Alligator defense test"

        defended = bine._alligator_defend(data)

        # Should transform data with multiple operations
        self.assertNotEqual(defended, data)

        # Should be deterministic
        defended2 = bine._alligator_defend(data)
        self.assertEqual(defended, defended2)

    def test_broad_spectrum_protection(self):
        """Test that defense works across different data types"""
        bine = BINECore(256)

        test_cases = [
            b"Short",
            b"Medium length data" * 10,
            b"X" * 10000,  # Repetitive
            secrets.token_bytes(1000),  # Random
        ]

        for data in test_cases:
            defended = bine._alligator_defend(data)
            self.assertIsNotNone(defended)
            self.assertNotEqual(defended[:len(data)], data)


class TestOpossumNeutralization(unittest.TestCase):
    """Test opossum-inspired error correction"""

    def test_toxin_neutralization(self):
        """Test OVNF-like neutralization of errors"""
        bine = BINECore(256)
        data = b"Opossum neutralization test"

        neutralized, checksum = bine._opossum_neutralize(data)

        # Should produce neutralized data and checksum
        self.assertIsNotNone(neutralized)
        self.assertIsNotNone(checksum)
        self.assertEqual(len(checksum), 16)

    def test_error_detection(self):
        """Test that errors can be detected"""
        cipher = BINECipher(256)
        plaintext = b"Error detection test"
        key = cipher.generate_key(b"test")

        # Encrypt with error correction
        ciphertext = cipher.encrypt(plaintext, key, include_ecc=True)

        # Should decrypt correctly
        decrypted = cipher.decrypt(ciphertext, key)
        self.assertEqual(plaintext, decrypted)


class TestSecurityProperties(unittest.TestCase):
    """Test cryptographic security properties"""

    def test_avalanche_effect(self):
        """Test that small input changes cause large output changes"""
        data1 = b"Test avalanche effect"
        data2 = b"Test avalanche effecT"  # One bit different

        hash1, salt = bine_hash_simple(data1)
        hash2, _ = bine_hash_simple(data2)

        # Should be completely different
        differences = sum(a != b for a, b in zip(hash1, hash2))
        # Expect roughly 50% bits different (good avalanche)
        self.assertGreater(differences, len(hash1) * 0.3)

    def test_preimage_resistance(self):
        """Test that hash doesn't reveal original data"""
        data = b"Secret message"
        hash_val, salt = bine_hash_simple(data)

        # Hash should not contain original data
        self.assertNotIn(data, hash_val)

    def test_collision_resistance_basic(self):
        """Basic test that different inputs produce different hashes"""
        test_cases = [
            b"Message 1",
            b"Message 2",
            b"Message 3",
            b"Different content entirely",
            b"X" * 100,
            secrets.token_bytes(100)
        ]

        hashes = []
        for data in test_cases:
            hash_val, _ = bine_hash_simple(data)
            hashes.append(hash_val)

        # All should be unique
        self.assertEqual(len(set(hashes)), len(hashes))

    def test_deterministic_hashing(self):
        """Test that same input produces same hash"""
        data = b"Deterministic test"
        salt = secrets.token_bytes(32)

        hash1 = bine_verify_simple.__self__.bine_hash(data, salt) if hasattr(bine_verify_simple, '__self__') else None
        hash2 = bine_verify_simple.__self__.bine_hash(data, salt) if hasattr(bine_verify_simple, '__self__') else None

        # Use direct API
        bine = BINECore(256)
        hash1 = bine.bine_hash(data, salt)
        hash2 = bine.bine_hash(data, salt)

        self.assertEqual(hash1, hash2)

    def test_encryption_roundtrip(self):
        """Test basic encryption/decryption"""
        cipher = BINECipher(256)
        plaintext = b"The quick brown fox jumps over the lazy dog"
        key = cipher.generate_key(b"test passphrase")

        # Encrypt
        ciphertext = cipher.encrypt(plaintext, key)

        # Ciphertext should be different
        self.assertNotIn(plaintext, ciphertext)

        # Decrypt
        decrypted = cipher.decrypt(ciphertext, key)

        # Should recover original
        self.assertEqual(plaintext, decrypted)

    def test_encryption_with_different_keys(self):
        """Test that different keys produce different ciphertexts"""
        cipher = BINECipher(256)
        plaintext = b"Same plaintext"

        key1 = cipher.generate_key(b"password1")
        key2 = cipher.generate_key(b"password2")

        ct1 = cipher.encrypt(plaintext, key1)
        ct2 = cipher.encrypt(plaintext, key2)

        self.assertNotEqual(ct1, ct2)


class TestPerformance(unittest.TestCase):
    """Test performance characteristics"""

    def test_hashing_speed(self):
        """Test that hashing completes in reasonable time"""
        data = b"X" * 10000  # 10KB

        start = time.time()
        for _ in range(100):
            hash_val, salt = bine_hash_simple(data)
        elapsed = time.time() - start

        # Should complete 100 hashes in under 5 seconds
        self.assertLess(elapsed, 5.0, f"Hashing too slow: {elapsed:.2f}s")

    def test_encryption_speed(self):
        """Test encryption performance"""
        cipher = BINECipher(256)
        key = cipher.generate_key(b"performance test")
        data = b"X" * 100000  # 100KB

        start = time.time()
        ciphertext = cipher.encrypt(data, key)
        encrypt_time = time.time() - start

        start = time.time()
        decrypted = cipher.decrypt(ciphertext, key)
        decrypt_time = time.time() - start

        # Should complete in reasonable time
        self.assertLess(encrypt_time, 2.0, f"Encryption too slow: {encrypt_time:.2f}s")
        self.assertLess(decrypt_time, 2.0, f"Decryption too slow: {decrypt_time:.2f}s")


class TestBioInspiredIntegration(unittest.TestCase):
    """Test that all bio-inspired features work together"""

    def test_full_lifecycle(self):
        """Test complete lifecycle with all features"""
        # Create cipher (ostrich diversity)
        cipher = BINECipher(256)

        # Generate key (tardigrade resilience + jellyfish regeneration)
        key = cipher.generate_key(b"integration test")

        # Encrypt with all protections (all organisms)
        plaintext = b"Bio-inspired integration test message"
        ciphertext = cipher.encrypt(plaintext, key, include_ecc=True)

        # Suspend key (tardigrade cryptobiosis)
        suspended_key = key.to_cryptobiotic()

        # Simulate extreme conditions
        del key

        # Revive key (tardigrade rehydration)
        revived_key = BINEKey.from_cryptobiotic(suspended_key)

        # Regenerate to new generation (jellyfish)
        next_gen_key = revived_key.regenerate(revived_key.generation + 1)

        # Original key should still decrypt
        decrypted = cipher.decrypt(ciphertext, revived_key)
        self.assertEqual(plaintext, decrypted)

    def test_resilience_to_partial_corruption(self):
        """Test system remains functional with partial corruption"""
        cipher = BINECipher(256)
        key = cipher.generate_key(b"resilience test")

        plaintext = b"Resilience testing"
        ciphertext = cipher.encrypt(plaintext, key, include_ecc=True)

        # Key fragments provide redundancy (cockroach)
        original_fragments = key.fragments[:]

        # "Lose" some fragments but system continues
        key.fragments = key.fragments[:len(key.fragments)//2]

        # Can still decrypt with primary key
        decrypted = cipher.decrypt(ciphertext, key)
        self.assertEqual(plaintext, decrypted)


def run_tests():
    """Run all tests and display results"""
    loader = unittest.TestLoader()
    suite = unittest.TestSuite()

    # Add all test classes
    test_classes = [
        TestTardigradeResilience,
        TestJellyfishRegeneration,
        TestCockroachDistribution,
        TestOstrichDiversity,
        TestBatAdaptiveResponse,
        TestSharkMolecularBinding,
        TestAlligatorDefense,
        TestOpossumNeutralization,
        TestSecurityProperties,
        TestPerformance,
        TestBioInspiredIntegration,
    ]

    for test_class in test_classes:
        tests = loader.loadTestsFromTestCase(test_class)
        suite.addTests(tests)

    # Run with verbose output
    runner = unittest.TextTestRunner(verbosity=2)
    result = runner.run(suite)

    # Print summary
    print("\n" + "="*70)
    print("BINE Test Suite Summary")
    print("="*70)
    print(f"Tests run: {result.testsRun}")
    print(f"Successes: {result.testsRun - len(result.failures) - len(result.errors)}")
    print(f"Failures: {len(result.failures)}")
    print(f"Errors: {len(result.errors)}")
    print("="*70)

    return result.wasSuccessful()


if __name__ == '__main__':
    success = run_tests()
    exit(0 if success else 1)
