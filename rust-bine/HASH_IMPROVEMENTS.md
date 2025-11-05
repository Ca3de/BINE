# BINE Custom Hash Function - Improvement Progress

## Problem
The original hash function had a differential cryptanalysis bias of 0.5000 (worst possible), making it completely predictable for certain input differences.

## Target
Achieve differential bias < 0.01 (ideally even less), as requested.

## Progress Timeline

### Initial State (v7.5-v8.0)
- **Bias: 0.5000** (BROKEN)
- Root cause: Test bug (delta=256 → 0) masked real performance
- Also discovered: Original hash design had fundamental weaknesses

### Phase 1: Test Fix & Initial Improvements
- Fixed test bug (delta range 1-255, not 256)
- **Bias: 0.21** → First major improvement!
- Increased sample size 100→1000 to reduce statistical noise
- **Bias: 0.147** → Confirmed real improvement

### Phase 2: Sponge Construction
Implemented proper sponge-based hash design:
- Absorb/squeeze phases
- Permutation after each absorption
- AES S-box for non-linearity
- **Result: Foundation for further improvements**

### Phase 3: Permutation Strengthening
- Increased rounds: 5 → 8 → 12
- Added round constants to break symmetry
- Forward + backward + cross-mixing passes
- Multiple S-box applications per round
- **Bias: 0.147 → 0.138**

### Phase 4: Global Mixing
- Each byte mixed with 5 positions across state
- Added arithmetic operations (mul, add) alongside XOR
- Position-dependent rotations
- **Bias: 0.138 → 0.134**

### Phase 5: Input-Dependent Mixing
- Mixing positions determined by state values
- Creates different diffusion patterns for different inputs
- **Bias: 0.134 → 0.138** (slight regression)

### Phase 6: Full-Width Absorption
- All state positions directly receive input (not just first 8)
- Heavy permutation (quad permute before output)
- **Bias: 0.131** (Current best)

## Current Status

### Achieved
- ✓ **74% reduction** in differential bias (0.5→0.131)
- ✓ Strong sponge construction with proven primitives
- ✓ Input-dependent mixing for variable diffusion
- ✓ Full state utilization for even input influence
- ✓ 12-round permutation with multiple S-box layers

### Current Performance
- **Differential Bias: 0.131** (Target: <0.01)
- **Gap to Target: ~13x improvement still needed**

### Known Issues
- Delta 163, Bit 200: Probability 0.631 (should be ~0.5)
- Some input differences still create predictable output patterns
- Permutation may have weak dependencies for specific deltas

## Technical Details

### Hash Structure
```rust
1. Initialize state[32] with S-box-transformed position values
2. For each input byte:
   - XOR into state[idx % 32]
   - Permute (double) every 8 bytes
3. Quad-permute before output
4. Truncate to desired length
```

### Permutation (12 rounds)
```rust
Each round:
  1. Forward pass: Add prev + round_const, S-box
  2. Backward pass: Mul 251, XOR S-box(next), S-box again
  3. Global mix: XOR/Add from 5 positions, S-box, rotate, S-box
```

### Key Features
- AES S-box for non-linearity
- Wrapping arithmetic (add, mul 179/251)
- Input-dependent mixing positions
- Position-dependent rotations
- Multiple S-box layers

## Next Steps

### To Reach <0.01 Target
1. **Option A: More aggressive permutation**
   - Increase to 16-20 rounds
   - Add matrix-based mixing (MDS)
   - May impact performance

2. **Option B: Proven cryptographic primitive**
   - Adapt ChaCha20 quarter-round (well-studied diffusion)
   - Use as permutation function
   - User concern: "not using other algorithms"

3. **Option C: Statistical analysis**
   - Verify if 0.131 includes expected statistical noise
   - With 65,280 tests, some outliers expected
   - May need different bias metric

4. **Option D: Accept current performance**
   - 0.131 is significantly better than 0.5
   - May be acceptable for BINE's use case
   - Focus on other security features

## Files Modified
- `test_hash_direct.rs` - Direct hash testing (no BINE pipeline)
- `lib_v80.rs` - Needs update with improved hash
- Multiple validation files (v76-v80)

## Recommendations
1. Test current hash (0.131 bias) in full BINE pipeline
2. Measure end-to-end differential resistance
3. Benchmark performance impact
4. Decide if further hash improvements worth the complexity
5. Document final decision and rationale
