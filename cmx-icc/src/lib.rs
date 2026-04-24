// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

//! WebAssembly bindings for the [cmx](https://crates.io/crates/cmx) ICC color
//! profile library, published to npm as
//! [`cmx-icc`](https://www.npmjs.com/package/cmx-icc).
//!
//! CMX is an advanced spectral color management system built on the
//! [Rust Colorimetry library](https://crates.io/crates/colorimetry), which
//! uses spectral representations of light to model color with physical
//! accuracy — going beyond the tristimulus approximations used in traditional
//! ICC workflows.  This package exposes the ICC profile layer of CMX: parse,
//! inspect, and build ICC color profiles entirely in the browser or Node.js,
//! with no native dependencies required.
//!
//! ## Installation
//!
//! ```sh
//! npm install cmx-icc
//! ```
//!
//! ## Quick start
//!
//! ```js
//! import init, { Profile, DisplayProfile, RenderingIntent } from 'cmx-icc';
//!
//! await init(); // load the .wasm binary once
//!
//! // ── Parse an existing profile ─────────────────────────────────────────────
//! // iccBytes is a Uint8Array, e.g. from fetch() or FileReader
//! const profile = Profile.fromBytes(iccBytes);
//! const intent  = profile.renderingIntent(); // RenderingIntent enum value
//! const bytes   = profile.toBytes();         // Uint8Array — byte-identical round-trip
//!
//! // ── Use a built-in preset ─────────────────────────────────────────────────
//! const srgb  = DisplayProfile.srgb(RenderingIntent.RelativeColorimetric);
//! const p3    = DisplayProfile.displayP3(RenderingIntent.RelativeColorimetric);
//! const adobe = DisplayProfile.adobeRgb(RenderingIntent.RelativeColorimetric);
//!
//! // ── Build a custom matrix display profile ────────────────────────────────
//! const custom = new DisplayProfile();
//! custom.setRenderingIntent(RenderingIntent.RelativeColorimetric);
//! custom.setProfileDescription("My Custom Profile");
//! custom.setCopyright("CC0 1.0");
//! custom.setWhitePoint(0.950455, 1.0, 1.08905);          // D50 XYZ
//! custom.setRedMatrixColumn(0.436066, 0.222488, 0.013916);
//! custom.setGreenMatrixColumn(0.385147, 0.716873, 0.097076);
//! custom.setBlueMatrixColumn(0.143066, 0.060608, 0.714096);
//! custom.setRedTrcGamma(2.2);
//! custom.setGreenTrcGamma(2.2);
//! custom.setBlueTrcGamma(2.2);
//! custom.finalize();                                      // embeds MD5 profile ID
//! const customBytes = custom.toBytes();                   // Uint8Array
//! ```
//!
//! ## TypeScript
//!
//! Full `.d.ts` declarations are included in the package.  Every class and
//! method is documented inline, so your IDE will show JSDoc on hover.
//!
//! ## Bundler support
//!
//! The package is built with `--target bundler` (Webpack, Vite, Rollup).
//! The `.wasm` binary is a separate file that your bundler will handle via the
//! standard WebAssembly asset pipeline.
//!
//! ## API summary
//!
//! | Class | Purpose |
//! |---|---|
//! | `Profile` | Parse an existing ICC profile from a `Uint8Array` |
//! | `DisplayProfile` | Build a display-class ICC profile from scratch |
//! | `RenderingIntent` | Enum: `Perceptual`, `RelativeColorimetric`, `Saturation`, `AbsoluteColorimetric` |
//!
//! ### `Profile`
//!
//! | Method | Description |
//! |---|---|
//! | `Profile.fromBytes(data)` | Parse a `Uint8Array`; throws on invalid data |
//! | `profile.toBytes()` | Serialize back to `Uint8Array` (byte-identical round-trip) |
//! | `profile.renderingIntent()` | Read the rendering intent from the header |
//!
//! ### `DisplayProfile`
//!
//! | Method | Description |
//! |---|---|
//! | `new DisplayProfile()` | Empty profile — set all tags manually |
//! | `DisplayProfile.srgb(intent)` | sRGB preset |
//! | `DisplayProfile.displayP3(intent)` | Display P3 preset |
//! | `DisplayProfile.adobeRgb(intent)` | Adobe RGB preset |
//! | `setRenderingIntent(intent)` | Header rendering intent |
//! | `setProfileDescription(text)` | ASCII description tag |
//! | `setProfileDescriptionMluc(lang, country, text)` | v4 multi-language description |
//! | `setCopyright(text)` | Copyright tag |
//! | `setWhitePoint(x, y, z)` | Media white point (XYZ) |
//! | `setRedMatrixColumn(x, y, z)` | Red primary (XYZ) |
//! | `setGreenMatrixColumn(x, y, z)` | Green primary (XYZ) |
//! | `setBlueMatrixColumn(x, y, z)` | Blue primary (XYZ) |
//! | `setChromaticAdaptation(Float64Array[9])` | Bradford matrix, row-major |
//! | `setRedTrcGamma(gamma)` | Red TRC — simple power curve |
//! | `setGreenTrcGamma(gamma)` | Green TRC — simple power curve |
//! | `setBlueTrcGamma(gamma)` | Blue TRC — simple power curve |
//! | `setRedTrcParametric(Float64Array)` | Red TRC — ICC parametric curve (1/3/4/5/7 params) |
//! | `setGreenTrcParametric(Float64Array)` | Green TRC — ICC parametric curve |
//! | `setBlueTrcParametric(Float64Array)` | Blue TRC — ICC parametric curve |
//! | `finalize()` | Compute and embed the MD5 profile ID checksum |
//! | `toBytes()` | Serialize to `Uint8Array`; consumes the object |
//!
//! ## Related
//!
//! - [`cmx`](https://crates.io/crates/cmx) — the underlying Rust crate
//! - [ICC specification](https://www.color.org/specification/ICC.1-2022-05.pdf)

use wasm_bindgen::prelude::*;

use cmx::{
    profile::DisplayProfile,
    tag::{
        tags::{
            BlueMatrixColumnTag, BlueTRCTag, ChromaticAdaptationTag, CopyrightTag,
            GreenMatrixColumnTag, GreenTRCTag, MediaWhitePointTag, ProfileDescriptionTag,
            RedMatrixColumnTag, RedTRCTag,
        },
        RenderingIntent,
    },
};

// ---------------------------------------------------------------------------
// RenderingIntent
// ---------------------------------------------------------------------------

/// ICC rendering intent, controlling how out-of-gamut colors are handled
/// when converting between color spaces.
#[wasm_bindgen(js_name = "RenderingIntent")]
#[derive(Clone, Copy)]
pub enum WasmRenderingIntent {
    Perceptual = 0,
    RelativeColorimetric = 1,
    Saturation = 2,
    AbsoluteColorimetric = 3,
}

impl From<WasmRenderingIntent> for RenderingIntent {
    fn from(i: WasmRenderingIntent) -> Self {
        match i {
            WasmRenderingIntent::Perceptual => RenderingIntent::Perceptual,
            WasmRenderingIntent::RelativeColorimetric => RenderingIntent::RelativeColorimetric,
            WasmRenderingIntent::Saturation => RenderingIntent::Saturation,
            WasmRenderingIntent::AbsoluteColorimetric => RenderingIntent::AbsoluteColorimetric,
        }
    }
}

impl From<RenderingIntent> for WasmRenderingIntent {
    fn from(i: RenderingIntent) -> Self {
        match i {
            RenderingIntent::Perceptual => WasmRenderingIntent::Perceptual,
            RenderingIntent::RelativeColorimetric => WasmRenderingIntent::RelativeColorimetric,
            RenderingIntent::Saturation => WasmRenderingIntent::Saturation,
            RenderingIntent::AbsoluteColorimetric => WasmRenderingIntent::AbsoluteColorimetric,
        }
    }
}

// ---------------------------------------------------------------------------
// WasmProfile — parse / round-trip existing profiles
// ---------------------------------------------------------------------------

/// A parsed ICC color profile.
///
/// Use [`WasmProfile.fromBytes`] to parse an existing ICC file that you have
/// loaded as a `Uint8Array`, and [`WasmProfile.toBytes`] to serialize it back.
#[wasm_bindgen(js_name = "Profile")]
pub struct WasmProfile {
    inner: cmx::profile::Profile,
}

#[wasm_bindgen]
impl WasmProfile {
    /// Parse an ICC profile from a `Uint8Array`.
    ///
    /// Throws a `TypeError` if the bytes do not contain a valid ICC profile.
    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(data: &[u8]) -> Result<WasmProfile, JsError> {
        let inner = cmx::profile::Profile::from_bytes(data)
            .map_err(|e| JsError::new(&e.to_string()))?;
        Ok(WasmProfile { inner })
    }

    /// Serialize the profile back to a `Uint8Array`.
    ///
    /// Throws if serialization fails (should not happen for a successfully
    /// parsed profile).
    #[wasm_bindgen(js_name = toBytes)]
    pub fn to_bytes(self) -> Result<Vec<u8>, JsError> {
        self.inner
            .into_bytes()
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// Returns the rendering intent stored in the profile header.
    #[wasm_bindgen(js_name = renderingIntent)]
    pub fn rendering_intent(&self) -> WasmRenderingIntent {
        self.inner.rendering_intent().into()
    }
}

// ---------------------------------------------------------------------------
// WasmDisplayProfile — build display profiles from scratch
// ---------------------------------------------------------------------------

/// A display-class ICC color profile builder.
///
/// Create a blank profile with `new WasmDisplayProfile()`, set tags via the
/// setter methods, then call `finalize()` before serializing with `toBytes()`.
///
/// Alternatively use one of the built-in presets:
/// - [`WasmDisplayProfile.srgb`]
/// - [`WasmDisplayProfile.displayP3`]
/// - [`WasmDisplayProfile.adobeRgb`]
#[wasm_bindgen(js_name = "DisplayProfile")]
pub struct WasmDisplayProfile {
    // Option lets us temporarily take ownership to drive the consuming builder.
    inner: Option<DisplayProfile>,
}

impl WasmDisplayProfile {
    /// Apply a consuming transform to the inner `DisplayProfile`.
    fn mutate<F: FnOnce(DisplayProfile) -> DisplayProfile>(&mut self, f: F) {
        if let Some(p) = self.inner.take() {
            self.inner = Some(f(p));
        }
    }
}

#[wasm_bindgen]
impl WasmDisplayProfile {
    /// Create a new, empty display profile.
    ///
    /// You must set all required tags before the profile is usable by a CMS:
    /// at minimum `profileDescriptionTag`, `copyrightTag`, `mediaWhitePointTag`,
    /// the three matrix-column tags, and the three TRC tags (for matrix-based
    /// display profiles).
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmDisplayProfile {
        WasmDisplayProfile {
            inner: Some(DisplayProfile::new()),
        }
    }

    // -- Presets -------------------------------------------------------------

    /// Returns a ready-to-use sRGB display profile.
    pub fn srgb(intent: WasmRenderingIntent) -> WasmDisplayProfile {
        WasmDisplayProfile {
            inner: Some(DisplayProfile::cmx_srgb(intent.into())),
        }
    }

    /// Returns a ready-to-use Display P3 display profile.
    #[wasm_bindgen(js_name = displayP3)]
    pub fn display_p3(intent: WasmRenderingIntent) -> WasmDisplayProfile {
        WasmDisplayProfile {
            inner: Some(DisplayProfile::cmx_display_p3(intent.into())),
        }
    }

    /// Returns a ready-to-use Adobe RGB display profile.
    #[wasm_bindgen(js_name = adobeRgb)]
    pub fn adobe_rgb(intent: WasmRenderingIntent) -> WasmDisplayProfile {
        WasmDisplayProfile {
            inner: Some(DisplayProfile::cmx_adobe_rgb(intent.into())),
        }
    }

    // -- Header fields -------------------------------------------------------

    /// Set the rendering intent in the profile header.
    #[wasm_bindgen(js_name = setRenderingIntent)]
    pub fn set_rendering_intent(&mut self, intent: WasmRenderingIntent) {
        self.mutate(|p| p.with_rendering_intent(intent.into()));
    }

    // -- Descriptive tags ----------------------------------------------------

    /// Set the profile description (ASCII).
    ///
    /// Writes a legacy `desc` (v2 `TextDescriptionData`) tag, which is the
    /// most broadly compatible format.  For v4 multi-language descriptions use
    /// [`setProfileDescriptionMluc`].
    #[wasm_bindgen(js_name = setProfileDescription)]
    pub fn set_profile_description(&mut self, description: &str) {
        self.mutate(|p| {
            p.with_tag(ProfileDescriptionTag)
                .as_text_description(|t| t.set_ascii(description))
        });
    }

    /// Set the profile description using the v4 `mluc` (MultiLocalizedUnicode) tag.
    ///
    /// `language` and `country` must each be a 2-character ISO code,
    /// e.g. `"en"` / `"US"`.
    #[wasm_bindgen(js_name = setProfileDescriptionMluc)]
    pub fn set_profile_description_mluc(&mut self, language: &str, country: &str, text: &str) {
        self.mutate(|p| {
            p.with_tag(ProfileDescriptionTag)
                .as_multi_localized_unicode(|m| {
                    m.insert(language, Some(country), text);
                })
        });
    }

    /// Set the copyright string (plain ASCII `text` tag).
    #[wasm_bindgen(js_name = setCopyright)]
    pub fn set_copyright(&mut self, text: &str) {
        self.mutate(|p| {
            p.with_tag(CopyrightTag)
                .as_text(|t| t.set_text(text))
        });
    }

    // -- Colorimetric tags ---------------------------------------------------

    /// Set the media white point (XYZ, PCS-adapted to D50).
    #[wasm_bindgen(js_name = setWhitePoint)]
    pub fn set_white_point(&mut self, x: f64, y: f64, z: f64) {
        self.mutate(|p| {
            p.with_tag(MediaWhitePointTag)
                .as_xyz_array(|xyz| xyz.set([x, y, z]))
        });
    }

    /// Set the red matrix column (XYZ).
    #[wasm_bindgen(js_name = setRedMatrixColumn)]
    pub fn set_red_matrix_column(&mut self, x: f64, y: f64, z: f64) {
        self.mutate(|p| {
            p.with_tag(RedMatrixColumnTag)
                .as_xyz_array(|xyz| xyz.set([x, y, z]))
        });
    }

    /// Set the green matrix column (XYZ).
    #[wasm_bindgen(js_name = setGreenMatrixColumn)]
    pub fn set_green_matrix_column(&mut self, x: f64, y: f64, z: f64) {
        self.mutate(|p| {
            p.with_tag(GreenMatrixColumnTag)
                .as_xyz_array(|xyz| xyz.set([x, y, z]))
        });
    }

    /// Set the blue matrix column (XYZ).
    #[wasm_bindgen(js_name = setBlueMatrixColumn)]
    pub fn set_blue_matrix_column(&mut self, x: f64, y: f64, z: f64) {
        self.mutate(|p| {
            p.with_tag(BlueMatrixColumnTag)
                .as_xyz_array(|xyz| xyz.set([x, y, z]))
        });
    }

    /// Set the chromatic adaptation matrix (Bradford, 9 values in row-major order).
    ///
    /// `matrix` must be a `Float64Array` of exactly 9 values.
    /// Throws if the slice length is not 9.
    #[wasm_bindgen(js_name = setChromaticAdaptation)]
    pub fn set_chromatic_adaptation(&mut self, matrix: &[f64]) -> Result<(), JsError> {
        let arr: [f64; 9] = matrix
            .try_into()
            .map_err(|_| JsError::new("setChromaticAdaptation requires exactly 9 values"))?;
        self.mutate(|p| {
            p.with_tag(ChromaticAdaptationTag)
                .as_sf15_fixed_16_array(|a| a.set(arr))
        });
        Ok(())
    }

    // -- TRC tags (simple gamma) ---------------------------------------------

    /// Set the red TRC as a simple power-law gamma curve.
    #[wasm_bindgen(js_name = setRedTrcGamma)]
    pub fn set_red_trc_gamma(&mut self, gamma: f64) {
        self.mutate(|p| {
            p.with_tag(RedTRCTag)
                .as_curve(|c| c.set_gamma(gamma))
        });
    }

    /// Set the green TRC as a simple power-law gamma curve.
    #[wasm_bindgen(js_name = setGreenTrcGamma)]
    pub fn set_green_trc_gamma(&mut self, gamma: f64) {
        self.mutate(|p| {
            p.with_tag(GreenTRCTag)
                .as_curve(|c| c.set_gamma(gamma))
        });
    }

    /// Set the blue TRC as a simple power-law gamma curve.
    #[wasm_bindgen(js_name = setBlueTrcGamma)]
    pub fn set_blue_trc_gamma(&mut self, gamma: f64) {
        self.mutate(|p| {
            p.with_tag(BlueTRCTag)
                .as_curve(|c| c.set_gamma(gamma))
        });
    }

    // -- TRC tags (parametric curve) -----------------------------------------

    /// Set the red TRC as a parametric curve (`para` tag).
    ///
    /// `params` is a `Float64Array` of 1, 3, 4, 5, or 7 values corresponding
    /// to ICC parametric curve types 0–4.  Throws if the count is invalid.
    #[wasm_bindgen(js_name = setRedTrcParametric)]
    pub fn set_red_trc_parametric(&mut self, params: &[f64]) -> Result<(), JsError> {
        let params = params.to_vec();
        self.mutate(|p| {
            p.with_tag(RedTRCTag).as_parametric_curve(|c| {
                c.set_parameters_slice(&params)
                    .expect("invalid parametric curve parameter count");
            })
        });
        Ok(())
    }

    /// Set the green TRC as a parametric curve (`para` tag).
    ///
    /// See [`setRedTrcParametric`] for parameter details.
    #[wasm_bindgen(js_name = setGreenTrcParametric)]
    pub fn set_green_trc_parametric(&mut self, params: &[f64]) -> Result<(), JsError> {
        let params = params.to_vec();
        self.mutate(|p| {
            p.with_tag(GreenTRCTag).as_parametric_curve(|c| {
                c.set_parameters_slice(&params)
                    .expect("invalid parametric curve parameter count");
            })
        });
        Ok(())
    }

    /// Set the blue TRC as a parametric curve (`para` tag).
    ///
    /// See [`setRedTrcParametric`] for parameter details.
    #[wasm_bindgen(js_name = setBlueTrcParametric)]
    pub fn set_blue_trc_parametric(&mut self, params: &[f64]) -> Result<(), JsError> {
        let params = params.to_vec();
        self.mutate(|p| {
            p.with_tag(BlueTRCTag).as_parametric_curve(|c| {
                c.set_parameters_slice(&params)
                    .expect("invalid parametric curve parameter count");
            })
        });
        Ok(())
    }

    // -- Finalization & serialization ----------------------------------------

    /// Compute and embed the MD5 profile ID checksum (fields 84–99 of the
    /// header).  Call this once, after setting all tags, before `toBytes()`.
    pub fn finalize(&mut self) {
        self.mutate(|p| p.with_profile_id());
    }

    /// Serialize the profile to a `Uint8Array`.
    ///
    /// Consumes the object — the `WasmDisplayProfile` instance is no longer
    /// usable after this call.  Throws if serialization fails.
    #[wasm_bindgen(js_name = toBytes)]
    pub fn to_bytes(mut self) -> Result<Vec<u8>, JsError> {
        let profile = self
            .inner
            .take()
            .ok_or_else(|| JsError::new("profile already consumed"))?;
        profile
            .to_bytes()
            .map_err(|e| JsError::new(&e.to_string()))
    }
}
