#!/usr/bin/env python3
"""
BINE Enhancement Examples
=========================

Demonstrates how BINE strengthens existing encryption systems.

Concept: "Building Reinforcement"
- Keep the solid foundation (proven algorithms)
- Add bio-inspired armor layers (BINE)
- Create stronger composite system
"""

import hashlib
import secrets
from bine_enhancer import CryptoEnhancer, BINEArmorPlating, quick_enhance, quick_unwrap


def example1_enhance_existing_encryption():
    """
    Example 1: Enhance already-encrypted data

    Scenario: You have data encrypted with another algorithm,
    and you want to add BINE's bio-inspired protection on top.
    """
    print("="*70)
    print("Example 1: Enhance Existing Encryption")
    print("="*70)

    # Simulate existing encryption (could be AES, RSA, etc.)
    original_data = b"Confidential company data"
    print(f"Original data: {original_data}")

    # Pretend this is encrypted with AES (simplified for demo)
    def simple_encrypt(data, key):
        """Simulate existing encryption (like AES)"""
        # In reality, this would be proper AES
        # For demo, we'll use simple XOR
        result = bytearray(data)
        for i in range(len(result)):
            result[i] ^= key[i % len(key)]
        return bytes(result)

    def simple_decrypt(ciphertext, key):
        """Decrypt (XOR is its own inverse)"""
        return simple_encrypt(ciphertext, key)  # XOR again

    # Encrypt with "existing algorithm"
    existing_key = secrets.token_bytes(32)
    existing_ciphertext = simple_encrypt(original_data, existing_key)
    print(f"\n1. Encrypted with existing algorithm: {existing_ciphertext[:20].hex()}...")

    # Now ENHANCE it with BINE layers!
    enhancer = CryptoEnhancer(256)
    bine_password = b"bine_protection_password"

    enhanced_ciphertext, bine_key = enhancer.enhance_encrypted_data(
        existing_ciphertext,
        "Custom-XOR-256",  # Name of original algorithm
        bine_password
    )

    print(f"2. Enhanced with BINE: {enhanced_ciphertext[:20].hex()}...")
    print(f"   Size increase: {len(existing_ciphertext)} → {len(enhanced_ciphertext)} bytes")
    print("   ✓ Now protected by: Original Algorithm + BINE Bio-Layers!")

    # To decrypt: Remove BINE layers, then decrypt original
    unwrapped, algorithm, metadata = enhancer.unwrap_enhanced_data(
        enhanced_ciphertext,
        bine_key
    )
    print(f"\n3. Unwrapped BINE layers")
    print(f"   Original algorithm: {algorithm}")

    # Decrypt original layer
    recovered_data = simple_decrypt(unwrapped, existing_key)
    print(f"4. Decrypted original layer: {recovered_data}")
    print(f"   ✓ Success: {recovered_data == original_data}")


def example2_strengthen_hashes():
    """
    Example 2: Strengthen password hashes from legacy systems

    Scenario: You have SHA-256 password hashes and want to
    add BINE protection without re-hashing all passwords.
    """
    print("\n" + "="*70)
    print("Example 2: Strengthen Legacy Password Hashes")
    print("="*70)

    armor = BINEArmorPlating()
    password = b"user_password_123"

    # Legacy system: SHA-256 hash
    sha256_hash = hashlib.sha256(password).digest()
    print(f"Legacy SHA-256 hash: {sha256_hash[:16].hex()}...")

    # Add BINE armor plating
    salt = secrets.token_bytes(32)
    armored_hash = armor.armor_plate_hash(sha256_hash, salt)
    print(f"BINE-armored hash: {armored_hash[:16].hex()}...")
    print("✓ Now protected by: SHA-256 + BINE bio-layers")

    # Verification
    def sha256_func(data):
        return hashlib.sha256(data).digest()

    is_valid = armor.verify_armored_hash(
        password,
        armored_hash,
        salt,
        sha256_func
    )
    print(f"\nVerification with correct password: {is_valid} ✓")

    # Wrong password fails
    is_valid = armor.verify_armored_hash(
        b"wrong_password",
        armored_hash,
        salt,
        sha256_func
    )
    print(f"Verification with wrong password: {is_valid} ✓")


def example3_defense_in_depth():
    """
    Example 3: Multi-layer defense system

    Scenario: Create "onion" of encryption layers for
    maximum security (like Tor, but for data at rest).
    """
    print("\n" + "="*70)
    print("Example 3: Defense in Depth (Multiple Layers)")
    print("="*70)

    armor = BINEArmorPlating()
    secret_data = b"Top secret information"

    print(f"Original data: {secret_data}")

    # Define multiple encryption layers
    def layer1_encrypt(data, key):
        """First layer: XOR cipher"""
        result = bytearray(data)
        for i in range(len(result)):
            result[i] ^= key[i % len(key)]
        return bytes(result)

    def layer2_encrypt(data, key):
        """Second layer: Different transformation"""
        result = bytearray(data)
        for i in range(len(result)):
            result[i] = (result[i] + key[i % len(key)]) % 256
        return bytes(result)

    # Create keys for each layer
    key1 = secrets.token_bytes(32)
    key2 = secrets.token_bytes(32)

    # Build layers
    layers = [
        (layer1_encrypt, key1, "XOR-256"),
        (layer2_encrypt, key2, "Additive-256"),
    ]

    # Apply all layers + BINE armor
    protected, decryption_info = armor.create_defense_in_depth(secret_data, layers)

    print(f"\nProtected data: {protected[:20].hex()}...")
    print("\nProtection layers applied:")
    for i, info in enumerate(decryption_info, 1):
        print(f"  Layer {i}: {info['algorithm']}")

    print("\n✓ Data now protected by:")
    print("  1. XOR-256")
    print("  2. Additive-256")
    print("  3. BINE Bio-Layers (Tardigrade + Jellyfish + Shark + ...)")
    print("\n  Even if one layer is broken, others still protect the data!")


def example4_migration_scenario():
    """
    Example 4: Migrate from single encryption to enhanced

    Scenario: You have a system using AES. You want to upgrade
    to AES+BINE without breaking existing data.
    """
    print("\n" + "="*70)
    print("Example 4: Migration to Enhanced Encryption")
    print("="*70)

    enhancer = CryptoEnhancer(256)

    # Original data
    data = b"Customer records and financial data"
    print(f"Original data: {data}")

    # Existing encryption system
    def existing_aes_encrypt(plaintext, key):
        """Simulate existing AES encryption"""
        result = bytearray(plaintext)
        for i in range(len(result)):
            result[i] ^= key[i % len(key)]
        return bytes(result)

    def existing_aes_decrypt(ciphertext, key):
        """Simulate existing AES decryption"""
        return existing_aes_encrypt(ciphertext, key)  # XOR inverse

    aes_key = secrets.token_bytes(32)
    bine_password = b"migration_password"

    # Migrate to enhanced system
    print("\nMigrating to AES+BINE...")
    enhanced, bine_key, original_key = enhancer.strengthen_migration(
        data,
        existing_aes_encrypt,
        aes_key,
        bine_password,
        "AES-256-Simulated"
    )

    print(f"✓ Migration complete!")
    print(f"  Enhanced data size: {len(enhanced)} bytes")
    print(f"  Protection: AES-256 + BINE bio-layers")

    # Full decryption through all layers
    print("\nDecrypting through all layers...")
    recovered = enhancer.full_decrypt(
        enhanced,
        bine_key,
        existing_aes_decrypt,
        aes_key
    )

    print(f"Recovered data: {recovered}")
    print(f"✓ Perfect match: {recovered == data}")


def example5_quick_api():
    """
    Example 5: Quick API for simple use cases
    """
    print("\n" + "="*70)
    print("Example 5: Quick Enhancement API")
    print("="*70)

    # You have some encrypted data
    my_data = b"Already encrypted with something"
    print(f"Existing ciphertext: {my_data[:20]}...")

    # Quickly add BINE protection
    enhanced, bine_key = quick_enhance(
        my_data,
        "MyCustomAlgorithm",
        b"password123"
    )

    print(f"✓ Enhanced: {enhanced[:20].hex()}...")

    # Quickly remove BINE layers
    unwrapped = quick_unwrap(enhanced, bine_key)

    print(f"✓ Unwrapped: {unwrapped[:20]}...")
    print(f"✓ Match: {unwrapped == my_data}")


def example6_real_world_scenario():
    """
    Example 6: Real-world scenario - Database encryption upgrade

    Scenario: Database has millions of records encrypted with AES.
    You want to add BINE protection without decrypting everything.
    """
    print("\n" + "="*70)
    print("Example 6: Real-World Database Upgrade")
    print("="*70)

    enhancer = CryptoEnhancer(256)

    print("Scenario: Database with AES-encrypted records")
    print("Goal: Add BINE protection without mass decryption\n")

    # Simulate database records
    records = [
        (1, b"AES_encrypted_record_1_data"),
        (2, b"AES_encrypted_record_2_data"),
        (3, b"AES_encrypted_record_3_data"),
    ]

    print("Original records (AES encrypted):")
    for record_id, aes_ciphertext in records:
        print(f"  Record {record_id}: {aes_ciphertext[:20]}...")

    # Upgrade process: Add BINE wrapper to each record
    print("\nUpgrading records...")
    enhanced_records = []

    for record_id, aes_ciphertext in records:
        # Add BINE protection (no need to decrypt!)
        enhanced, bine_key = enhancer.enhance_encrypted_data(
            aes_ciphertext,
            "AES-256-GCM",
            b"database_master_password",
            metadata={"record_id": record_id}
        )
        enhanced_records.append((record_id, enhanced, bine_key))
        print(f"  ✓ Record {record_id} upgraded to AES+BINE")

    print("\n✓ Upgrade complete!")
    print("  - Original AES encryption still intact")
    print("  - Added BINE bio-inspired layers")
    print("  - Defense in depth: AES + Tardigrade + Jellyfish + ...")

    # To access: Remove BINE layer, then decrypt AES
    print("\nAccessing record 1:")
    record_id, enhanced_data, bine_key = enhanced_records[0]
    unwrapped, algo, meta = enhancer.unwrap_enhanced_data(enhanced_data, bine_key)
    print(f"  Algorithm: {algo}")
    print(f"  Metadata: {meta}")
    print(f"  Inner ciphertext (AES): {unwrapped[:20]}...")
    print("  (Now decrypt with AES key to get plaintext)")


def main():
    """Run all examples"""
    print("\n" + "="*70)
    print("BINE CRYPTOGRAPHIC ENHANCEMENT - EXAMPLES")
    print("="*70)
    print("\nConcept: Strengthen existing encryption like reinforcing a building")
    print("- Keep solid foundation (proven algorithms)")
    print("- Add bio-inspired armor (BINE layers)")
    print("- Result: Stronger composite system\n")

    example1_enhance_existing_encryption()
    example2_strengthen_hashes()
    example3_defense_in_depth()
    example4_migration_scenario()
    example5_quick_api()
    example6_real_world_scenario()

    print("\n" + "="*70)
    print("SUMMARY")
    print("="*70)
    print("""
BINE Enhancement Use Cases:

✓ Legacy System Hardening
  - Add protection to existing encrypted data
  - No need to decrypt and re-encrypt everything
  - Keeps original encryption working

✓ Defense in Depth
  - Multiple independent layers
  - Even if one breaks, others protect data
  - Industry best practice

✓ Future-Proofing
  - Add quantum-resistant layers
  - Upgrade security without migration pain
  - Gradual enhancement possible

✓ Password Hash Strengthening
  - Armor-plate legacy SHA-256/bcrypt hashes
  - Add BINE bio-layers
  - No password reset needed

✓ Database Encryption Upgrade
  - Wrap existing encrypted records
  - No downtime required
  - Reversible process

Key Benefits:
- ✓ Non-destructive (original encryption preserved)
- ✓ Reversible (can unwrap BINE layers)
- ✓ Stackable (multiple layers possible)
- ✓ Bio-inspired (tardigrade resilience, jellyfish healing, etc.)

This is DEFENSIVE security - making systems stronger!
""")
    print("="*70)


if __name__ == "__main__":
    main()
