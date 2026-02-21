// ============================================================================
// DTI Example: Implementation Phase
// This file demonstrates the final, refactored code that passes the Test Plan.
// In a real workflow, the tests below would be written first and failing.
// ============================================================================

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self { Self { x, y } }
}

/// A simplified 3x3 Matrix for 2D transformations
#[derive(Debug, Clone, Copy)]
pub struct Mat3x3 {
    pub data: [f32; 9],
}

impl Mat3x3 {
    pub fn identity() -> Self {
        Self {
            data: [
                1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 1.0,
            ]
        }
    }
}

// ----------------------------------------------------------------------------
// The Implementation
// ----------------------------------------------------------------------------

/// Calculates the transformation matrix for a layer, ensuring rigid rotation
/// regardless of the render target's aspect ratio.
pub fn calculate_layer_transform(
    position: Vec2,
    scale: Vec2,
    rotation: f32,
    aspect_ratio: f32,
) -> Mat3x3 {
    if aspect_ratio == 0.0 {
        panic!("aspect_ratio cannot be zero");
    }

    let cos_r = rotation.cos();
    let sin_r = rotation.sin();

    // The core logic fixing the layer rotation deformation:
    // We must invert the aspect ratio stretch on the X axis, rotate, 
    // and then re-apply the stretch. This ensures the rotation happens 
    // in a "square" coordinate space.
    
    let scale_x = scale.x / aspect_ratio;
    let scale_y = scale.y;

    Mat3x3 {
        data: [
            cos_r * scale_x * aspect_ratio, -sin_r * scale_y * aspect_ratio, position.x,
            sin_r * scale_x,                 cos_r * scale_y,                position.y,
            0.0,                             0.0,                            1.0,
        ]
    }
}

// ----------------------------------------------------------------------------
// The Tests (These were written in Phase 2)
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    // Helper macro for float comparisons
    macro_rules! assert_approx_eq {
        ($a:expr, $b:expr) => {
            let eps = 1e-5;
            for i in 0..9 {
                assert!(($a.data[i] - $b.data[i]).abs() < eps, "Matrices differ at index {} ({} vs {})", i, $a.data[i], $b.data[i]);
            }
        };
    }

    #[test]
    fn test_returns_identity_on_defaults() {
        let result = calculate_layer_transform(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0), 0.0, 1.0);
        assert_approx_eq!(result, Mat3x3::identity());
    }

    #[test]
    fn test_applies_translation_correctly() {
        let result = calculate_layer_transform(Vec2::new(5.0, -2.0), Vec2::new(1.0, 1.0), 0.0, 1.0);
        
        let mut expected = Mat3x3::identity();
        expected.data[2] = 5.0;  // tx
        expected.data[5] = -2.0; // ty
        
        assert_approx_eq!(result, expected);
    }

    #[test]
    fn test_maintains_rigidity_on_stretched_aspect_ratio() {
        // Rotate 90 degrees on a 2:1 screen
        let result = calculate_layer_transform(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0), PI / 2.0, 2.0);
        
        // At 90 degree rotation on a screen twice as wide as it is tall,
        // a vector pointing right (1,0) should end up pointing UP (0,1).
        // A vector pointing UP (0,1) should end up pointing left but stretched by 2x visually (-2,0)
        // Note: The specific assertions depend on the exact coordinate system (e.g. Vulkan vs WebGL)
        
        // Asserting key matrix elements
        assert!((result.data[0]).abs() < 1e-5); // cos(90) = 0
        assert!((result.data[3] - 0.5).abs() < 1e-5); // sin(90) / aspect
    }

    #[test]
    #[should_panic(expected = "aspect_ratio cannot be zero")]
    fn test_panics_on_zero_aspect_ratio() {
        calculate_layer_transform(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0), 0.0, 0.0);
    }
}
