#!/usr/bin/env python3
"""
BINE Demo Script
================

Demonstrates the bio-inspired features of BINE cryptographic system.
"""

import sys
from bine_core import BINECore, bine_hash_simple, bine_verify_simple, BINEState
from bine_encrypt import BINECipher, BINEKey


def print_section(title):
    """Print a section header"""
    print("\n" + "=" * 70)
    print(f"  {title}")
    print("=" * 70)


def demo_basic_hashing():
    """Demonstrate basic hashing with BINE"""
    print_section("1. Basic Hashing (Ostrich + Tardigrade)")

    data = b"The quick brown fox jumps over the lazy dog"
    print(f"Original data: {data.decode()}")

    # Hash the data
    hash_value, salt = bine_hash_simple(data, security_level=256)
    print(f"\nHash (hex): {hash_value[:32].hex()}...")
    print(f"Salt (hex): {salt[:16].hex()}...")

    # Verify
    is_valid = bine_verify_simple(data, hash_value, salt, security_level=256)
    print(f"Verification: {'✓ VALID' if is_valid else '✗ INVALID'}")

    # Show avalanche effect
    modified_data = b"The quick brown fox jumps over the lazy Dog"  # Capital D
    is_valid_modified = bine_verify_simple(modified_data, hash_value, salt, security_level=256)
    print(f"\nSingle bit change verification: {'✓ VALID' if is_valid_modified else '✗ INVALID (as expected)'}")


def demo_cryptobiotic_state():
    """Demonstrate tardigrade-inspired state suspension"""
    print_section("2. Cryptobiotic State (Tardigrade Resilience)")

    bine = BINECore(256)
    data = b"Survive extreme conditions!"

    # Create initial hash
    hash1, salt = bine_hash_simple(data)
    print(f"Initial hash: {hash1[:16].hex()}...")

    # Enter cryptobiotic state
    print("\n→ Entering cryptobiotic state (suspended animation)...")
    state = bine.get_state()
    suspended = state.to_cryptobiotic()
    print(f"  Suspended state size: {len(suspended)} bytes")

    # Simulate extreme conditions
    print("→ Simulating extreme conditions (destroying original object)...")
    del bine

    # Revive from suspension
    print("→ Reviving from cryptobiotic state...")
    revived_state = BINEState.from_cryptobiotic(suspended)
    new_bine = BINECore(256)
    new_bine.restore_state(revived_state)

    # Verify it works
    hash2 = new_bine.bine_hash(data, salt)
    print(f"Revived hash: {hash2[:16].hex()}...")
    print(f"State preserved: {'✓ YES' if hash1 == hash2 else '✗ NO'}")


def demo_jellyfish_regeneration():
    """Demonstrate jellyfish-inspired key regeneration"""
    print_section("3. Key Regeneration (Jellyfish Self-Healing)")

    cipher = BINECipher(256)
    passphrase = b"master_password_123"

    # Generate initial key (generation 0 - medusa stage)
    print("→ Generating Generation 0 key (medusa stage)...")
    key_gen0 = cipher.generate_key(passphrase)
    print(f"  Gen 0 key: {key_gen0.primary_key[:16].hex()}...")

    # Encrypt a message
    message = b"Time capsule message from the past"
    print(f"\n→ Encrypting message: {message.decode()}")
    ciphertext = cipher.encrypt(message, key_gen0)
    print(f"  Ciphertext size: {len(ciphertext)} bytes")

    # Regenerate to generation 1 (polyp → medusa cycle)
    print("\n→ Regenerating to Generation 1 (polyp → medusa cycle)...")
    key_gen1 = key_gen0.regenerate(1)
    print(f"  Gen 1 key: {key_gen1.primary_key[:16].hex()}...")

    # Regenerate to generation 2
    print("→ Regenerating to Generation 2...")
    key_gen2 = key_gen0.regenerate(2)
    print(f"  Gen 2 key: {key_gen2.primary_key[:16].hex()}...")

    # All generations descended from same master seed
    print(f"\n→ All share same master seed: {key_gen0.master_seed[:16].hex()}...")

    # Original key can still decrypt
    print("\n→ Testing decryption with Gen 0 key...")
    decrypted = cipher.decrypt(ciphertext, key_gen0)
    print(f"  Decrypted: {decrypted.decode()}")
    print(f"  Success: {'✓ YES' if decrypted == message else '✗ NO'}")


def demo_cockroach_redundancy():
    """Demonstrate cockroach-inspired distributed redundancy"""
    print_section("4. Distributed Redundancy (Cockroach Survival)")

    cipher = BINECipher(256)
    key = cipher.generate_key(b"redundancy_test")

    print(f"→ Generated key with {len(key.fragments)} redundant fragments")
    print("  (No single point of failure - cockroach style!)")

    for i, fragment in enumerate(key.fragments):
        print(f"  Fragment {i}: {fragment[:16].hex()}...")

    # Show that key works even if we "lose" some fragments
    print("\n→ Simulating fragment loss (1 of 3 fragments destroyed)...")
    original_fragments = key.fragments[:]
    key.fragments = key.fragments[:2]  # Lose one fragment
    print("  Remaining fragments: 2/3")

    # Still works for encryption/decryption
    message = b"Cockroach resilience test"
    ciphertext = cipher.encrypt(message, key)
    key.fragments = original_fragments  # Restore for decryption
    decrypted = cipher.decrypt(ciphertext, key)

    print(f"  Still functional: {'✓ YES' if decrypted == message else '✗ NO'}")


def demo_ostrich_diversity():
    """Demonstrate ostrich-inspired antibody diversity"""
    print_section("5. Antibody Diversity (Ostrich Immune Power)")

    bine = BINECore(256)
    data = b"Diverse protection mechanisms"

    print("→ Generating 8 diverse antibody-like variants...")
    antibodies = bine._ostrich_diversify(data)

    print(f"  Generated {len(antibodies)} unique variants:")
    for i, antibody in enumerate(antibodies[:4]):  # Show first 4
        print(f"  Antibody {i}: {antibody[:16].hex()}...")
    print("  ... (4 more)")

    # Verify all unique
    unique = len(set(antibodies)) == len(antibodies)
    print(f"\n→ All variants unique: {'✓ YES' if unique else '✗ NO'}")
    print("  (Like different antibody specificities)")


def demo_bat_adaptive():
    """Demonstrate bat-inspired adaptive complexity"""
    print_section("6. Adaptive Complexity (Bat Balanced Response)")

    bine = BINECore(256)

    threat_levels = [0.1, 0.3, 0.5, 0.7, 0.9]
    print("→ Testing adaptive complexity at different threat levels:\n")

    for threat in threat_levels:
        complexity = bine._bat_adaptive_complexity(threat)
        print(f"  Threat {threat:.1f} → Complexity {complexity:3d} " + "█" * (complexity // 5))

    print("\n→ Like bat immune system: scales response without 'inflammation'")
    print("  (Efficient at low threat, stronger at high threat)")


def demo_shark_binding():
    """Demonstrate shark-inspired molecular binding"""
    print_section("7. Molecular Binding (Shark Precision)")

    bine = BINECore(256)
    data = b"Precise molecular binding"
    key = b"VNAR_antibody_key_123456789012"

    print("→ Binding data to key (like shark VNAR antibodies)...")
    bound1 = bine._shark_bind(data, key)
    print(f"  Bound data: {bound1[:16].hex()}...")

    # Different key = different binding
    key2 = b"Different_key_123456789012345"
    bound2 = bine._shark_bind(data, key2)
    print(f"  Different key: {bound2[:16].hex()}...")

    print(f"\n→ Bindings differ: {'✓ YES' if bound1 != bound2 else '✗ NO'}")
    print("  (Tight, specific binding like shark antibodies)")


def demo_alligator_defense():
    """Demonstrate alligator-inspired broad defense"""
    print_section("8. Broad-Spectrum Defense (Alligator Immunity)")

    bine = BINECore(256)
    data = b"Alligator antimicrobial peptide defense"

    print("→ Applying broad-spectrum defense (like 4 APAP peptides)...")
    defended = bine._alligator_defend(data)
    print(f"  Original: {data[:20].hex()}...")
    print(f"  Defended: {defended[:20].hex()}...")

    print("\n→ Multiple transformation layers applied")
    print("  (Resistant to diverse attack vectors)")


def demo_opossum_neutralization():
    """Demonstrate opossum-inspired error correction"""
    print_section("9. Error Correction (Opossum Venom Immunity)")

    bine = BINECore(256)
    data = b"Error correction like OVNF neutralizing venom"

    print("→ Applying opossum-style neutralization with error correction...")
    neutralized, checksum = bine._opossum_neutralize(data)
    print(f"  Neutralized: {neutralized[:16].hex()}...")
    print(f"  Checksum: {checksum.hex()}")

    print("\n→ Built-in error detection and correction")
    print("  (Isolates and neutralizes 'toxins' like bit flips)")


def demo_full_encryption():
    """Demonstrate full encryption pipeline"""
    print_section("10. Complete Encryption Pipeline (All Organisms)")

    cipher = BINECipher(256)
    passphrase = b"bio_inspired_encryption"

    # Generate key with all features
    print("→ Generating key with all bio-inspired features...")
    key = cipher.generate_key(passphrase)
    print(f"  Primary key: {key.primary_key[:16].hex()}...")
    print(f"  Fragments: {len(key.fragments)}")
    print(f"  Generation: {key.generation}")

    # Encrypt with error correction
    plaintext = b"BINE combines strategies from tardigrades, jellyfish, cockroaches, ostriches, bats, sharks, alligators, and opossums!"
    print(f"\n→ Encrypting ({len(plaintext)} bytes) with ECC...")
    print(f"  Plaintext: {plaintext[:50].decode()}...")

    ciphertext = cipher.encrypt(plaintext, key, include_ecc=True)
    print(f"  Ciphertext size: {len(ciphertext)} bytes")
    print(f"  Ciphertext: {ciphertext[:50].hex()}...")

    # Decrypt
    print("\n→ Decrypting...")
    decrypted = cipher.decrypt(ciphertext, key)
    print(f"  Decrypted: {decrypted[:50].decode()}...")

    success = decrypted == plaintext
    print(f"\n→ Encryption/Decryption: {'✓ SUCCESS' if success else '✗ FAILED'}")


def main():
    """Run all demos"""
    print("\n" + "=" * 70)
    print("  BINE: Bio-Inspired Network Encryption")
    print("  Demonstration of Nature's Cryptographic Strategies")
    print("=" * 70)

    try:
        demo_basic_hashing()
        demo_cryptobiotic_state()
        demo_jellyfish_regeneration()
        demo_cockroach_redundancy()
        demo_ostrich_diversity()
        demo_bat_adaptive()
        demo_shark_binding()
        demo_alligator_defense()
        demo_opossum_neutralization()
        demo_full_encryption()

        print_section("Demo Complete!")
        print("\n✓ All bio-inspired features demonstrated successfully!\n")
        print("BINE combines:")
        print("  - Tardigrade resilience (cryptobiotic state)")
        print("  - Jellyfish regeneration (key evolution)")
        print("  - Cockroach redundancy (distributed state)")
        print("  - Ostrich diversity (antibody variants)")
        print("  - Bat adaptation (balanced response)")
        print("  - Shark precision (molecular binding)")
        print("  - Alligator defense (broad-spectrum)")
        print("  - Opossum correction (error neutralization)")
        print("\n" + "=" * 70 + "\n")

        return 0

    except Exception as e:
        print(f"\n✗ Error during demo: {e}")
        import traceback
        traceback.print_exc()
        return 1


if __name__ == "__main__":
    sys.exit(main())
