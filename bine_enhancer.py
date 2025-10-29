"""
BINE Cryptographic Enhancer
============================

Strengthen existing encryption by wrapping it with BINE's bio-inspired layers.

Concept: "Defense in Depth"
- Start with proven encryption (AES-256, etc.)
- Add BINE's bio-inspired resilience layers
- Create composite encryption stronger than either alone

Like reinforcing a building:
1. Keep the solid foundation (AES)
2. Add bio-inspired armor (BINE layers)
3. Result: Multi-layer protection

Use Cases:
- Legacy system hardening
- Future-proofing against quantum attacks
- High-security data protection
- Critical infrastructure defense
"""

import hashlib
from typing import Tuple, Optional
from dataclasses import dataclass
import secrets

from bine_core import BINECore
from bine_encrypt import BINECipher, BINEKey


@dataclass
class EnhancedEncryption:
    """
    Represents data encrypted with both standard and BINE layers.

    Structure:
    [BINE Header][BINE-encrypted[Original Algorithm Data]]

    Like a fortified building:
    - Inner layer: Original encryption (AES, etc.)
    - Outer layers: BINE bio-inspired protection
    """
    algorithm: str  # Original algorithm used (e.g., "AES-256-GCM")
    bine_version: int
    inner_ciphertext: bytes  # Original encrypted data
    outer_ciphertext: bytes  # BINE-wrapped result
    metadata: dict


class CryptoEnhancer:
    """
    Enhance existing encryption with BINE's bio-inspired layers.

    This is DEFENSIVE security - making existing systems stronger,
    not breaking them!
    """

    def __init__(self, security_level: int = 256):
        """
        Initialize the crypto enhancer.

        Args:
            security_level: BINE security level (128, 256, 512)
        """
        self.security_level = security_level
        self.bine_cipher = BINECipher(security_level)
        self.version = 1

    def enhance_encrypted_data(
        self,
        existing_ciphertext: bytes,
        algorithm_name: str,
        bine_password: bytes,
        metadata: Optional[dict] = None
    ) -> Tuple[bytes, BINEKey]:
        """
        Wrap existing encrypted data with BINE layers.

        This creates "defense in depth" - multiple independent layers of protection.

        Args:
            existing_ciphertext: Already encrypted data (e.g., from AES)
            algorithm_name: Name of original algorithm (for tracking)
            bine_password: Password for BINE layer
            metadata: Optional metadata about the original encryption

        Returns:
            (enhanced_ciphertext, bine_key) tuple

        Example:
            # You have AES-encrypted data
            aes_ciphertext = encrypt_with_aes(data, aes_key)

            # Enhance it with BINE layers
            enhanced, bine_key = enhancer.enhance_encrypted_data(
                aes_ciphertext,
                "AES-256-GCM",
                b"bine_password"
            )

            # Now you have AES + BINE protection!
        """
        metadata = metadata or {}

        # Generate BINE key
        bine_key = self.bine_cipher.generate_key(bine_password)

        # Build header with metadata
        header = self._build_header(algorithm_name, metadata)

        # Combine header + existing ciphertext
        combined = header + existing_ciphertext

        # Wrap everything in BINE encryption (multiple bio-layers)
        enhanced_ciphertext = self.bine_cipher.encrypt(combined, bine_key, include_ecc=True)

        return enhanced_ciphertext, bine_key

    def unwrap_enhanced_data(
        self,
        enhanced_ciphertext: bytes,
        bine_key: BINEKey
    ) -> Tuple[bytes, str, dict]:
        """
        Remove BINE layers to get back the original encrypted data.

        This removes the outer BINE protection, revealing the inner
        layer (which is still encrypted with the original algorithm).

        Args:
            enhanced_ciphertext: BINE-enhanced data
            bine_key: BINE decryption key

        Returns:
            (original_ciphertext, algorithm_name, metadata) tuple

        Example:
            # Unwrap BINE layers
            aes_ciphertext, algo, meta = enhancer.unwrap_enhanced_data(
                enhanced,
                bine_key
            )

            # Now decrypt with original algorithm
            data = decrypt_with_aes(aes_ciphertext, aes_key)
        """
        # Decrypt BINE layers
        combined = self.bine_cipher.decrypt(enhanced_ciphertext, bine_key)

        # Parse header
        algorithm_name, metadata, header_size = self._parse_header(combined)

        # Extract original ciphertext
        original_ciphertext = combined[header_size:]

        return original_ciphertext, algorithm_name, metadata

    def strengthen_migration(
        self,
        plaintext: bytes,
        original_encrypt_func,
        original_key,
        bine_password: bytes,
        algorithm_name: str
    ) -> Tuple[bytes, BINEKey, any]:
        """
        Migrate data from single encryption to enhanced multi-layer.

        Process:
        1. Encrypt with original algorithm (e.g., AES)
        2. Wrap with BINE layers
        3. Return both keys for future decryption

        Args:
            plaintext: Original data
            original_encrypt_func: Function to encrypt with original algorithm
            original_key: Key for original algorithm
            bine_password: Password for BINE layer
            algorithm_name: Name of original algorithm

        Returns:
            (enhanced_ciphertext, bine_key, original_key) tuple

        Example:
            # Migrate from AES-only to AES+BINE
            from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes

            def aes_encrypt(data, key):
                # ... AES encryption logic
                return ciphertext

            enhanced, bine_key, aes_key = enhancer.strengthen_migration(
                my_data,
                aes_encrypt,
                aes_key,
                b"bine_password",
                "AES-256-GCM"
            )
        """
        # Step 1: Encrypt with original algorithm
        inner_ciphertext = original_encrypt_func(plaintext, original_key)

        # Step 2: Enhance with BINE
        enhanced_ciphertext, bine_key = self.enhance_encrypted_data(
            inner_ciphertext,
            algorithm_name,
            bine_password,
            metadata={
                "migration_date": str(secrets.randbits(64)),
                "original_algorithm": algorithm_name
            }
        )

        return enhanced_ciphertext, bine_key, original_key

    def full_decrypt(
        self,
        enhanced_ciphertext: bytes,
        bine_key: BINEKey,
        original_decrypt_func,
        original_key
    ) -> bytes:
        """
        Decrypt through all layers to get original plaintext.

        Process:
        1. Unwrap BINE layers
        2. Decrypt with original algorithm
        3. Return plaintext

        Args:
            enhanced_ciphertext: Multi-layer encrypted data
            bine_key: BINE decryption key
            original_decrypt_func: Function to decrypt original layer
            original_key: Key for original algorithm

        Returns:
            Original plaintext
        """
        # Remove BINE layers
        inner_ciphertext, algorithm, metadata = self.unwrap_enhanced_data(
            enhanced_ciphertext,
            bine_key
        )

        # Decrypt original layer
        plaintext = original_decrypt_func(inner_ciphertext, original_key)

        return plaintext

    def _build_header(self, algorithm_name: str, metadata: dict) -> bytes:
        """Build header with version and metadata"""
        import json

        header_data = {
            "version": self.version,
            "algorithm": algorithm_name,
            "metadata": metadata
        }

        header_json = json.dumps(header_data).encode('utf-8')
        header_size = len(header_json)

        # [4 bytes: size][header_json]
        return len(header_json).to_bytes(4, 'big') + header_json

    def _parse_header(self, data: bytes) -> Tuple[str, dict, int]:
        """Parse header from combined data"""
        import json

        # Read header size
        header_size = int.from_bytes(data[0:4], 'big')

        # Read header JSON
        header_json = data[4:4+header_size]
        header_data = json.loads(header_json.decode('utf-8'))

        algorithm = header_data['algorithm']
        metadata = header_data.get('metadata', {})
        total_header_size = 4 + header_size

        return algorithm, metadata, total_header_size


class BINEArmorPlating:
    """
    Add BINE "armor plating" to existing systems.

    Concept: Like adding armor to a tank
    - Tank still works (original encryption)
    - Armor adds protection (BINE layers)
    - Both work together for maximum defense
    """

    def __init__(self):
        self.enhancer = CryptoEnhancer(256)
        self.bine_core = BINECore(256)

    def armor_plate_hash(self, existing_hash: bytes, salt: bytes) -> bytes:
        """
        Add BINE protection to an existing hash.

        Use case: Strengthen password hashes from legacy systems

        Example:
            # Legacy SHA-256 hash
            sha256_hash = hashlib.sha256(password).digest()

            # Add BINE armor
            armored = armor.armor_plate_hash(sha256_hash, salt)

            # Now protected by SHA-256 + BINE layers
        """
        # Run existing hash through BINE's bio-transformations
        armored = self.bine_core.bine_hash(existing_hash, salt)
        return armored

    def verify_armored_hash(
        self,
        data: bytes,
        armored_hash: bytes,
        salt: bytes,
        original_hash_func
    ) -> bool:
        """
        Verify data against armored hash.

        Args:
            data: Original data
            armored_hash: BINE-armored hash
            salt: Salt used in armoring
            original_hash_func: Function to compute original hash

        Returns:
            True if data matches armored hash
        """
        # Compute original hash
        original_hash = original_hash_func(data)

        # Compute armored version
        test_armored = self.armor_plate_hash(original_hash, salt)

        # Constant-time comparison
        return secrets.compare_digest(test_armored, armored_hash)

    def create_defense_in_depth(
        self,
        data: bytes,
        layers: list
    ) -> Tuple[bytes, list]:
        """
        Create multiple independent encryption layers.

        Concept: "Defense in Depth"
        - Even if one layer is broken, others protect the data
        - Each layer uses different algorithm/key
        - BINE provides bio-inspired resilience

        Args:
            data: Plaintext to protect
            layers: List of (encrypt_func, key, name) tuples

        Returns:
            (multi_layer_ciphertext, decryption_info) tuple

        Example:
            layers = [
                (aes_encrypt, aes_key, "AES-256"),
                (rsa_encrypt, rsa_key, "RSA-2048"),
            ]

            # Create onion of encryption layers
            protected, info = armor.create_defense_in_depth(data, layers)

            # Data is now protected by: AES → RSA → BINE
        """
        current_data = data
        decryption_info = []

        # Apply each layer
        for encrypt_func, key, name in layers:
            current_data = encrypt_func(current_data, key)
            decryption_info.append({
                "algorithm": name,
                "key_type": type(key).__name__
            })

        # Final BINE armor layer
        bine_password = secrets.token_bytes(32)
        enhanced, bine_key = self.enhancer.enhance_encrypted_data(
            current_data,
            "Multi-Layer-Defense",
            bine_password
        )

        decryption_info.append({
            "algorithm": "BINE-256",
            "bine_key": bine_key,
            "bine_password": bine_password
        })

        return enhanced, decryption_info


# Convenience functions
def quick_enhance(ciphertext: bytes, algorithm: str, password: bytes) -> Tuple[bytes, BINEKey]:
    """
    Quickly enhance existing encryption with BINE.

    Example:
        aes_encrypted = my_aes_function(data, aes_key)
        enhanced, bine_key = quick_enhance(aes_encrypted, "AES-256", b"password")
    """
    enhancer = CryptoEnhancer(256)
    return enhancer.enhance_encrypted_data(ciphertext, algorithm, password)


def quick_unwrap(enhanced: bytes, bine_key: BINEKey) -> bytes:
    """
    Quickly remove BINE layers.

    Example:
        original_aes = quick_unwrap(enhanced, bine_key)
        data = my_aes_decrypt(original_aes, aes_key)
    """
    enhancer = CryptoEnhancer(256)
    original, _, _ = enhancer.unwrap_enhanced_data(enhanced, bine_key)
    return original
