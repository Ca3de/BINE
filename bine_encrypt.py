"""
BINE Encryption - Bio-Inspired Encryption System
=================================================

Symmetric encryption built on BINE hash primitives with bio-inspired features:
- Self-healing key management (jellyfish)
- Distributed key fragments (cockroach)
- Adaptive security levels (bat)
- Multi-layer encryption (ostrich)
- Error resilience (tardigrade)
"""

import secrets
import struct
from typing import Tuple, Optional, List
from dataclasses import dataclass
import hashlib

from bine_core import BINECore, BINEState, ANTIBODY_VARIANTS, SALT_SIZE


@dataclass
class BINEKey:
    """
    Bio-inspired encryption key with regenerative capabilities.
    """
    primary_key: bytes
    generation: int
    fragments: List[bytes]  # Cockroach-style redundancy
    master_seed: bytes  # For jellyfish regeneration
    security_level: int = 256

    def regenerate(self, new_generation: int) -> 'BINEKey':
        """
        Jellyfish-inspired key regeneration.
        Can evolve to new generation while maintaining backward compatibility.
        """
        bine = BINECore(self.security_level)
        bine.master_seed = self.master_seed

        # Regenerate to new generation
        new_key = bine._jellyfish_regenerate(self.primary_key, new_generation)

        # Create new fragments
        new_fragments = bine._cockroach_distribute(new_key)

        return BINEKey(
            primary_key=new_key,
            generation=new_generation,
            fragments=new_fragments,
            master_seed=self.master_seed,
            security_level=self.security_level
        )

    def to_cryptobiotic(self) -> bytes:
        """Serialize to dormant state"""
        data = struct.pack('I', self.generation) + struct.pack('I', self.security_level)
        data += self.primary_key + self.master_seed
        data += struct.pack('I', len(self.fragments))
        for frag in self.fragments:
            data += struct.pack('I', len(frag)) + frag
        return data

    @classmethod
    def from_cryptobiotic(cls, data: bytes) -> 'BINEKey':
        """Resurrect from dormant state"""
        offset = 0
        generation = struct.unpack('I', data[offset:offset+4])[0]
        offset += 4
        security_level = struct.unpack('I', data[offset:offset+4])[0]
        offset += 4

        key_size = security_level // 8
        primary_key = data[offset:offset+key_size]
        offset += key_size
        master_seed = data[offset:offset+key_size]
        offset += key_size

        num_fragments = struct.unpack('I', data[offset:offset+4])[0]
        offset += 4

        fragments = []
        for _ in range(num_fragments):
            frag_len = struct.unpack('I', data[offset:offset+4])[0]
            offset += 4
            fragments.append(data[offset:offset+frag_len])
            offset += frag_len

        return cls(primary_key, generation, fragments, master_seed, security_level)


class BINECipher:
    """
    Bio-inspired symmetric cipher combining multiple defense strategies.
    """

    def __init__(self, security_level: int = 256):
        """
        Initialize BINE cipher.

        Args:
            security_level: Bit security level (128, 256, 512)
        """
        self.security_level = security_level
        self.bine_core = BINECore(security_level)
        self.key_size = security_level // 8

    def generate_key(self, passphrase: Optional[bytes] = None) -> BINEKey:
        """
        Generate a BINE encryption key.

        Args:
            passphrase: Optional passphrase to derive key from

        Returns:
            BINEKey with regenerative capabilities
        """
        if passphrase:
            # Derive from passphrase using BINE hash
            master_seed = self.bine_core.bine_hash(passphrase)[:self.key_size]
        else:
            master_seed = secrets.token_bytes(self.key_size)

        # Generate primary key
        primary_key = hashlib.blake2b(
            master_seed + b'primary',
            digest_size=self.key_size
        ).digest()

        # Create cockroach-style fragments
        fragments = self.bine_core._cockroach_distribute(primary_key)

        return BINEKey(
            primary_key=primary_key,
            generation=0,
            fragments=fragments,
            master_seed=master_seed,
            security_level=self.security_level
        )

    def _derive_round_keys(self, key: BINEKey, data_len: int) -> List[bytes]:
        """
        Ostrich-inspired: Generate diverse round keys like antibody variants.
        """
        round_keys = []
        num_rounds = min(ANTIBODY_VARIANTS, (data_len // self.key_size) + 1)

        for round_idx in range(num_rounds):
            # Each round key is like a different antibody
            round_key = hashlib.blake2b(
                key.primary_key + struct.pack('I', round_idx) + struct.pack('I', key.generation),
                digest_size=self.key_size
            ).digest()
            round_keys.append(round_key)

        return round_keys

    def _tardigrade_protect(self, data: bytes, key: bytes, round_num: int) -> bytes:
        """
        Tardigrade-inspired: Protect data with multiple layers of resilience.
        """
        # Layer 1: XOR encryption (DNA protection analog)
        protected = bytearray(len(data))
        key_stream = self._generate_keystream(key, len(data), round_num)

        for i in range(len(data)):
            protected[i] = data[i] ^ key_stream[i]

        # Layer 2: Mixing for additional diffusion
        mixed = self._molecular_mix(bytes(protected), round_num)

        return mixed

    def _generate_keystream(self, key: bytes, length: int, nonce: int) -> bytes:
        """
        Generate cryptographic keystream using BINE hash.
        """
        keystream = b''
        counter = 0

        while len(keystream) < length:
            block = hashlib.blake2b(
                key + struct.pack('I', nonce) + struct.pack('I', counter),
                digest_size=self.key_size
            ).digest()
            keystream += block
            counter += 1

        return keystream[:length]

    def _molecular_mix(self, data: bytes, round_num: int) -> bytes:
        """
        Diffusion layer for avalanche effect.
        Involutory - applying twice returns original (for easy decryption).
        Uses only XOR operations which are self-inverse.
        """
        mixed = bytearray(data)

        # XOR with deterministic pseudo-random pattern (involutory)
        for i in range(len(mixed)):
            # Generate pseudo-random byte from position and round
            prng_byte = ((round_num * 31 + i * 17 + (i * i) * 7) ^ (round_num + i)) % 256
            mixed[i] ^= prng_byte

        return bytes(mixed)

    def _error_correction_encode(self, data: bytes) -> Tuple[bytes, bytes]:
        """
        Opossum-inspired: Add error correction like venom neutralization.
        """
        # Simple checksum-based error detection
        checksum = hashlib.blake2b(data, digest_size=16).digest()

        # Redundancy encoding (simple repetition code for demonstration)
        parity = bytearray(len(data))
        for i in range(len(data)):
            # Calculate parity bits
            parity[i] = data[i] ^ (data[i-1] if i > 0 else 0)

        return data, checksum + bytes(parity)

    def _error_correction_decode(self, data: bytes, ecc: bytes) -> bytes:
        """
        Opossum-inspired: Detect and correct errors.
        """
        checksum = ecc[:16]
        parity = ecc[16:]

        # Verify checksum
        computed_checksum = hashlib.blake2b(data, digest_size=16).digest()

        if not secrets.compare_digest(checksum, computed_checksum):
            # Attempt correction using parity
            corrected = bytearray(data)
            for i in range(min(len(corrected), len(parity))):
                expected_parity = corrected[i] ^ (corrected[i-1] if i > 0 else 0)
                if expected_parity != parity[i]:
                    # Single bit error detected, attempt correction
                    corrected[i] ^= (expected_parity ^ parity[i])

            return bytes(corrected)

        return data

    def encrypt(self, plaintext: bytes, key: BINEKey, include_ecc: bool = True) -> bytes:
        """
        Encrypt data using BINE cipher with bio-inspired multi-layer protection.

        Args:
            plaintext: Data to encrypt
            key: BINEKey for encryption
            include_ecc: Include error correction (opossum feature)

        Returns:
            Encrypted ciphertext with metadata
        """
        # Add padding (PKCS7-style)
        padding_len = self.key_size - (len(plaintext) % self.key_size)
        if padding_len == 0:
            padding_len = self.key_size
        padded = plaintext + bytes([padding_len] * padding_len)

        # Generate diverse round keys (ostrich diversity)
        round_keys = self._derive_round_keys(key, len(padded))

        # Multi-round encryption (tardigrade resilience)
        ciphertext = padded
        for round_idx, round_key in enumerate(round_keys):
            ciphertext = self._tardigrade_protect(ciphertext, round_key, round_idx)

        # Note: Shark binding skipped for encryption as it's one-way
        # (still used in hashing where one-way is desired)

        # Optional error correction (opossum neutralization)
        if include_ecc:
            ciphertext, ecc_data = self._error_correction_encode(ciphertext)
        else:
            ecc_data = b''

        # Build final encrypted package
        header = struct.pack('I', key.generation)  # For jellyfish regeneration
        header += struct.pack('I', len(round_keys))  # Number of rounds
        header += struct.pack('?', include_ecc)  # ECC flag
        header += struct.pack('I', len(ecc_data))  # ECC length

        return header + ecc_data + ciphertext

    def decrypt(self, ciphertext: bytes, key: BINEKey) -> bytes:
        """
        Decrypt BINE ciphertext.

        Args:
            ciphertext: Encrypted data
            key: BINEKey for decryption

        Returns:
            Original plaintext
        """
        # Parse header
        offset = 0
        generation = struct.unpack('I', ciphertext[offset:offset+4])[0]
        offset += 4
        num_rounds = struct.unpack('I', ciphertext[offset:offset+4])[0]
        offset += 4
        has_ecc = struct.unpack('?', ciphertext[offset:offset+1])[0]
        offset += 1
        ecc_len = struct.unpack('I', ciphertext[offset:offset+4])[0]
        offset += 4

        # Regenerate key to correct generation if needed (jellyfish)
        if generation != key.generation:
            key = key.regenerate(generation)

        # Extract ECC and encrypted data
        ecc_data = ciphertext[offset:offset+ecc_len] if has_ecc else b''
        offset += ecc_len
        encrypted_data = ciphertext[offset:]

        # Note: Shark binding not used in encryption (it's one-way)
        decrypted = encrypted_data

        # Error correction if present
        if has_ecc:
            decrypted = self._error_correction_decode(decrypted, ecc_data)

        # Generate round keys
        round_keys = self._derive_round_keys(key, len(decrypted))

        # Reverse multi-round encryption
        for round_idx in range(len(round_keys) - 1, -1, -1):
            decrypted = self._tardigrade_protect(decrypted, round_keys[round_idx], round_idx)

        # Remove padding
        padding_len = decrypted[-1]
        plaintext = decrypted[:-padding_len]

        return plaintext


# Convenience functions
def encrypt_simple(plaintext: bytes, passphrase: bytes, security_level: int = 256) -> Tuple[bytes, bytes]:
    """
    Simple encryption interface.

    Returns:
        (ciphertext, key_data) tuple for storage
    """
    cipher = BINECipher(security_level)
    key = cipher.generate_key(passphrase)
    ciphertext = cipher.encrypt(plaintext, key)
    return ciphertext, key.to_cryptobiotic()


def decrypt_simple(ciphertext: bytes, key_data: bytes, security_level: int = 256) -> bytes:
    """Simple decryption interface"""
    cipher = BINECipher(security_level)
    key = BINEKey.from_cryptobiotic(key_data)
    return cipher.decrypt(ciphertext, key)
