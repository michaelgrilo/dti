# Test Plan: Layer Transform Utility

*Derived from: [01_define.md](./01_define.md) for Layer Transform Utility*

## Coverage Map

| # | Success Criterion | Test Name | Status |
|:--|:------------------|:----------|:-------|
| 1 | Returns identity matrix on defaults | `test_returns_identity_on_defaults` | [ ] |
| 2 | Correctly translates point | `test_applies_translation_correctly` | [ ] |
| 3 | Scales uniformly without skewing | `test_applies_uniform_scaling` | [ ] |
| 4 | Corrects for non-square aspect ratio during rotation | `test_maintains_rigidity_on_stretched_aspect_ratio` | [ ] |
| 5 | Handles negative scaling | `test_handles_negative_scaling_flip` | [ ] |

## Unit Tests

### test_returns_identity_on_defaults
- **Criterion:** Returns an identity matrix when position is (0,0), scale is (1,1), rotation is 0, and aspect ratio is 1.0.
- **Input:** `position=(0,0)`, `scale=(1,1)`, `rotation=0.0`, `aspect_ratio=1.0`
- **Expected output:** A clean 3x3 identity matrix with 1.0s on the diagonal.

### test_applies_translation_correctly
- **Criterion:** Correctly translates a point by (x, y) without scaling or rotating.
- **Input:** `position=(5.0,-2.0)`, `scale=(1,1)`, `rotation=0.0`, `aspect_ratio=1.0`
- **Expected output:** Matrix with translation components set to `(5.0, -2.0)`.

### test_maintains_rigidity_on_stretched_aspect_ratio
- **Criterion:** Corrects for a non-square aspect ratio (e.g., 16:9) by adjusting the X-axis scale proportionally during rotation to maintain a square visual appearance.
- **Setup:** A mock "square" layer local vertex set: `[(1,0), (0,1), (-1,0), (0,-1)]`.
- **Input:** `position=(0,0)`, `scale=(1,1)`, `rotation=PI/2`, `aspect_ratio=2.0`
- **Expected output:** The output matrix applied to the local vertices must yield coordinates that, when mapped to the screen, form a perfect square, NOT a rectangle stretched by 2x width.

## Edge Case Tests

### test_panics_on_zero_aspect_ratio
- **Scenario:** `aspect_ratio` is 0.0
- **Input:** `aspect_ratio=0.0`
- **Expected behavior:** Function safely panics with "aspect_ratio cannot be zero" OR returns a valid fallback identity matrix.

### test_handles_zero_scale_collapse
- **Scenario:** `scale` is exactly (0.0, 0.0)
- **Input:** `scale=(0,0)`, `position=(10, 10)`
- **Expected behavior:** Any input vector multiplied by the result matrix strictly equals `(10, 10)`.

## Test Environment
- **Framework:** `cargo test` (Rust standard testing library)
- **Runner command:** `cargo test transform -- --nocapture`

---

*Once every success criterion has a corresponding test and all tests are written, proceed to the Implement phase.*
