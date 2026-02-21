# Definition: Layer Transform Utility

## Summary
A rendering utility function for BlobUI that calculates the final 2D transformation matrix for a layer. This ensures that when a layer is rotated, scaled, and translated over a background image (even if that background is non-square or stretched), the layer rotates rigidly without skewing or deforming visually on screen.

## Inputs

| Input | Type | Required | Description |
|:------|:-----|:---------|:------------|
| `position` | `Vec2<f32>` | Yes | The (x, y) translation of the layer's center |
| `scale` | `Vec2<f32>` | Yes | The local (x, y) scaling factor of the layer |
| `rotation` | `f32` | Yes | The rotation angle in radians |
| `aspect_ratio` | `f32` | Yes | The aspect ratio (width/height) of the render target/background |

## Outputs

| Output | Type | Description |
|:-------|:-----|:------------|
| `transform` | `Mat3x3<f32>` | The final 3x3 transformation matrix to be passed to the WebGPU shader |

## Rules

- **Rigidity:** Rotation must always visually appear as a rigid transformation, regardless of the `aspect_ratio`.
- **Order of Operations:** The transform must be applied in the order: Scale -> Rotate -> Translate.
- **Normalization:** The resulting matrix must map the layer's local coordinates into the correct normalized device coordinates (NDC) space considering the target aspect ratio.

## Success Criteria

- [ ] Returns an identity matrix when position is (0,0), scale is (1,1), rotation is 0, and aspect ratio is 1.0.
- [ ] Correctly translates a point by (x, y) without scaling or rotating.
- [ ] Scales uniformly when `scale.x == scale.y` without skewing.
- [ ] Corrects for a non-square aspect ratio (e.g., 16:9) by adjusting the X-axis scale proportionally during rotation to maintain a square visual appearance.
- [ ] Handles negative scaling values (flipping) correctly.

## Edge Cases

| Scenario | Expected Behavior |
|:---------|:------------------|
| `aspect_ratio` is 0.0 | Should panic or return a fallback identity matrix to avoid division by zero. |
| `scale` is exactly (0.0, 0.0) | Matrix should result in all points collapsing to the `position` origin. |
| Extremely large rotation angles (e.g., > 2PI) | Should normalize visually identical to `rotation % 2PI`. |

## Usage Examples

```rust
// Calculate transform for a layer rotated by 45 degrees on a 16:9 screen
let transform = calculate_layer_transform(
    Vec2::new(100.0, 50.0), // position
    Vec2::new(1.0, 1.0),    // scale
    std::f32::consts::FRAC_PI_4, // 45 degrees
    16.0 / 9.0              // aspect ratio
);
```
