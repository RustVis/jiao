// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use crate::sksl::glsl::GLSLGeneration;
use crate::sksl::version::Version;

/// Indicates how GLSL must interact with advanced blend equations.
///
/// The KHR extension requires special layout qualifiers in the fragment shader.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum AdvBlendEqInteraction {
    /// No _blend_equation_advanced extension
    NotSupported,

    /// No interaction required
    Automatic,

    /// layout(blend_support_all_equations) out
    GeneralEnable,
}

#[derive(Debug)]
pub struct ShaderCaps {
    glsl_generation: GLSLGeneration,

    dual_source_blending_support: bool,
    shader_derivative_support: bool,

    /// Enables sampleGrad and sampleLod functions that don't rely on implicit derivatives
    explicit_texture_lod_support: bool,
    /// Indicates true 32-bit integer support, with unsigned types and bitwise operations
    integer_support: bool,
    non_square_matrix_support: bool,
    /// asinh(), acosh(), atanh()
    inverse_hyperbolic_support: bool,
    fb_fetch_support: bool,
    fb_fetch_needs_custom_output: bool,
    uses_precision_modifiers: bool,
    flat_interpolation_support: bool,
    no_perspective_interpolation_support: bool,
    sample_mask_support: bool,
    external_texture_support: bool,
    float_is_32_bits: bool,

    // isinf() is defined, and floating point infinities are handled according to IEEE standards.
    infinity_support: bool,

    // Used by SkSL to know when to generate polyfills.
    builtin_fma_support: bool,
    builtin_determinant_support: bool,

    // Used for specific driver bug work arounds
    can_use_void_in_sequence_expressions: bool,
    can_use_min_and_abs_together: bool,
    can_use_fract_for_negative_values: bool,
    must_force_negated_atan_param_to_float: bool,
    must_force_negated_ldexp_param_to_multiply: bool,
    // Returns whether a device incorrectly implements atan(y,x) as atan(y/x)
    atan2_implemented_as_atan_y_over_x: bool,
    // If this returns true some operation (could be a no op) must be called between floor and abs
    // to make sure the driver compiler doesn't inline them together which can cause a driver bug in
    // the shader.
    must_do_op_between_floor_and_abs: bool,
    // The D3D shader compiler, when targeting PS 3.0 (ie within ANGLE) fails to compile certain
    // constructs. See detailed comments in GrGLCaps.cpp.
    must_guard_division_even_after_explicit_zero_check: bool,
    // If false, SkSL uses a workaround so that sk_FragCoord doesn't actually query gl_FragCoord
    can_use_frag_coord: bool,
    // If true, then conditions in for loops need "&& true" to work around driver bugs.
    add_and_true_to_loop_condition: bool,
    // If true, then expressions such as "x && y" or "x || y" are rewritten as ternary to work
    // around driver bugs.
    unfold_short_circuit_as_ternary: bool,
    emulated_abs_int_function: bool,
    rewrite_do_while_loops: bool,
    rewrite_switch_statements: bool,
    remove_pow_with_constatnt_exponent: bool,
    // The Android emulator claims samplerExternalOES is an unknown type if a default precision
    // statement is made for the type.
    no_default_precision_for_external_samplers: bool,
    // ARM GPUs calculate `matrix * vector` in SPIR-V at full precision, even when the inputs are
    // RelaxedPrecision. Rewriting the multiply as a sum of vector*scalar fixes this. (skia:11769)
    rewrite_matrix_vector_mulitply: bool,
    // Rewrites matrix equality comparisons to avoid an Adreno driver bug. (skia:11308)
    rewrite_matrix_comparisons: bool,
    // Strips const from function parameters in the GLSL code generator. (skia:13858)
    remove_const_from_function_parameters: bool,
    // On some Android devices colors aren't accurate enough for the double lookup in the
    // Perlin noise shader. This workaround aggressively snaps colors to multiples of 1/255.
    perlin_noise_rounding_fix: bool,
    // Vulkan requires certain builtin variables be present, even if they're unused. At one time,
    // validation errors would result if sk_Clockwise was missing. Now, it's just (Adreno) driver
    // bugs that drop or corrupt draws if they're missing.
    must_declare_fragment_front_facing: bool,

    version_decl_string: String,

    shader_derivative_extension_string: Option<String>,
    external_texture_extension_string: Option<String>,
    second_external_texture_extension_string: Option<String>,
    fb_fetch_color_name: Option<String>,

    adv_blend_eq_interaction: AdvBlendEqInteraction,
}

impl Default for ShaderCaps {
    fn default() -> Self {
        Self {
            glsl_generation: GLSLGeneration::V330,

            dual_source_blending_support: false,
            shader_derivative_support: false,

            explicit_texture_lod_support: false,
            integer_support: false,
            non_square_matrix_support: false,
            inverse_hyperbolic_support: false,
            fb_fetch_support: false,
            fb_fetch_needs_custom_output: false,
            uses_precision_modifiers: false,
            flat_interpolation_support: false,
            no_perspective_interpolation_support: false,
            sample_mask_support: false,
            external_texture_support: false,
            float_is_32_bits: true,
            infinity_support: false,
            builtin_fma_support: true,
            builtin_determinant_support: true,

            can_use_void_in_sequence_expressions: true,
            can_use_min_and_abs_together: true,
            can_use_fract_for_negative_values: true,
            must_force_negated_atan_param_to_float: false,
            must_force_negated_ldexp_param_to_multiply: false,
            atan2_implemented_as_atan_y_over_x: false,
            must_do_op_between_floor_and_abs: false,
            must_guard_division_even_after_explicit_zero_check: false,
            can_use_frag_coord: true,
            add_and_true_to_loop_condition: false,
            unfold_short_circuit_as_ternary: false,
            emulated_abs_int_function: false,
            rewrite_do_while_loops: false,
            rewrite_switch_statements: false,
            remove_pow_with_constatnt_exponent: false,
            no_default_precision_for_external_samplers: false,
            rewrite_matrix_vector_mulitply: false,
            rewrite_matrix_comparisons: false,
            remove_const_from_function_parameters: false,
            perlin_noise_rounding_fix: false,
            must_declare_fragment_front_facing: false,

            version_decl_string: String::new(),

            shader_derivative_extension_string: None,
            external_texture_extension_string: None,
            second_external_texture_extension_string: None,
            fb_fetch_color_name: None,

            adv_blend_eq_interaction: AdvBlendEqInteraction::NotSupported,
        }
    }
}

impl ShaderCaps {
    #[must_use]
    #[inline]
    pub fn must_enable_adv_blend_eqs(&self) -> bool {
        self.adv_blend_eq_interaction >= AdvBlendEqInteraction::GeneralEnable
    }

    #[must_use]
    #[inline]
    pub fn must_declare_fragment_shader_output(&self) -> bool {
        self.glsl_generation > GLSLGeneration::V110
    }

    /// Returns the string of an extension that must be enabled in the shader to support derivatives.
    ///
    /// If nullptr is returned then no extension needs to be enabled. Before calling
    /// this function, the caller should check that shaderDerivativeSupport exists.
    #[must_use]
    #[inline]
    pub fn shader_derivative_extension_string(&self) -> Option<String> {
        debug_assert!(self.shader_derivative_support);
        self.shader_derivative_extension_string.clone()
    }

    // This returns the name of an extension that must be enabled in the shader to support external
    // textures. In some cases, two extensions must be enabled - the second extension is returned
    // by secondExternalTextureExtensionString(). If that function returns nullptr, then only one
    // extension is required.
    #[must_use]
    #[inline]
    pub fn external_texture_extension_string(&self) -> Option<String> {
        debug_assert!(self.external_texture_support);
        self.external_texture_extension_string.clone()
    }

    #[must_use]
    #[inline]
    pub fn second_external_texture_extension_string(&self) -> Option<String> {
        debug_assert!(self.external_texture_support);
        self.second_external_texture_extension_string.clone()
    }

    /// `SkSL` 300 requires support for derivatives, nonsquare matrices and bitwise integer operations.
    #[must_use]
    #[inline]
    pub fn supported_sksl_verion(&self) -> Version {
        if self.shader_derivative_support
            && self.non_square_matrix_support
            && self.integer_support
            && self.glsl_generation >= GLSLGeneration::V330
        {
            Version::V300
        } else {
            Version::V100
        }
    }

    #[must_use]
    #[inline]
    pub fn supports_distance_field_text(&self) -> bool {
        self.shader_derivative_support
    }
}
