<!-- cargo-rdme start -->

WebAssembly bindings for the [cmx](https://crates.io/crates/cmx) ICC color
profile library, published to npm as
[`cmx-icc`](https://www.npmjs.com/package/cmx-icc).

Parse, inspect, and build ICC color profiles entirely in the browser or
Node.js — no native dependencies required.

## Installation

```sh
npm install cmx-icc
```

## Quick start

```js
import init, { WasmProfile, WasmDisplayProfile, WasmRenderingIntent } from 'cmx-icc';

await init(); // load the .wasm binary once

// ── Parse an existing profile ─────────────────────────────────────────────
// iccBytes is a Uint8Array, e.g. from fetch() or FileReader
const profile = WasmProfile.fromBytes(iccBytes);
const intent  = profile.renderingIntent(); // WasmRenderingIntent enum value
const bytes   = profile.toBytes();         // Uint8Array — byte-identical round-trip

// ── Use a built-in preset ─────────────────────────────────────────────────
const srgb  = WasmDisplayProfile.srgb(WasmRenderingIntent.RelativeColorimetric);
const p3    = WasmDisplayProfile.displayP3(WasmRenderingIntent.RelativeColorimetric);
const adobe = WasmDisplayProfile.adobeRgb(WasmRenderingIntent.RelativeColorimetric);

// ── Build a custom matrix display profile ────────────────────────────────
const custom = new WasmDisplayProfile();
custom.setRenderingIntent(WasmRenderingIntent.RelativeColorimetric);
custom.setProfileDescription("My Custom Profile");
custom.setCopyright("CC0 1.0");
custom.setWhitePoint(0.950455, 1.0, 1.08905);          // D50 XYZ
custom.setRedMatrixColumn(0.436066, 0.222488, 0.013916);
custom.setGreenMatrixColumn(0.385147, 0.716873, 0.097076);
custom.setBlueMatrixColumn(0.143066, 0.060608, 0.714096);
custom.setRedTrcGamma(2.2);
custom.setGreenTrcGamma(2.2);
custom.setBlueTrcGamma(2.2);
custom.finalize();                                      // embeds MD5 profile ID
const customBytes = custom.toBytes();                   // Uint8Array
```

## TypeScript

Full `.d.ts` declarations are included in the package.  Every class and
method is documented inline, so your IDE will show JSDoc on hover.

## Bundler support

The package is built with `--target bundler` (Webpack, Vite, Rollup).
The `.wasm` binary is a separate file that your bundler will handle via the
standard WebAssembly asset pipeline.

## API summary

| Class | Purpose |
|---|---|
| `WasmProfile` | Parse an existing ICC profile from a `Uint8Array` |
| `WasmDisplayProfile` | Build a display-class ICC profile from scratch |
| `WasmRenderingIntent` | Enum: `Perceptual`, `RelativeColorimetric`, `Saturation`, `AbsoluteColorimetric` |

### `WasmProfile`

| Method | Description |
|---|---|
| `WasmProfile.fromBytes(data)` | Parse a `Uint8Array`; throws on invalid data |
| `profile.toBytes()` | Serialize back to `Uint8Array` (byte-identical round-trip) |
| `profile.renderingIntent()` | Read the rendering intent from the header |

### `WasmDisplayProfile`

| Method | Description |
|---|---|
| `new WasmDisplayProfile()` | Empty profile — set all tags manually |
| `WasmDisplayProfile.srgb(intent)` | sRGB preset |
| `WasmDisplayProfile.displayP3(intent)` | Display P3 preset |
| `WasmDisplayProfile.adobeRgb(intent)` | Adobe RGB preset |
| `setRenderingIntent(intent)` | Header rendering intent |
| `setProfileDescription(text)` | ASCII description tag |
| `setProfileDescriptionMluc(lang, country, text)` | v4 multi-language description |
| `setCopyright(text)` | Copyright tag |
| `setWhitePoint(x, y, z)` | Media white point (XYZ) |
| `setRedMatrixColumn(x, y, z)` | Red primary (XYZ) |
| `setGreenMatrixColumn(x, y, z)` | Green primary (XYZ) |
| `setBlueMatrixColumn(x, y, z)` | Blue primary (XYZ) |
| `setChromaticAdaptation(Float64Array[9])` | Bradford matrix, row-major |
| `setRedTrcGamma(gamma)` | Red TRC — simple power curve |
| `setGreenTrcGamma(gamma)` | Green TRC — simple power curve |
| `setBlueTrcGamma(gamma)` | Blue TRC — simple power curve |
| `setRedTrcParametric(Float64Array)` | Red TRC — ICC parametric curve (1/3/4/5/7 params) |
| `setGreenTrcParametric(Float64Array)` | Green TRC — ICC parametric curve |
| `setBlueTrcParametric(Float64Array)` | Blue TRC — ICC parametric curve |
| `finalize()` | Compute and embed the MD5 profile ID checksum |
| `toBytes()` | Serialize to `Uint8Array`; consumes the object |

## Related

- [`cmx`](https://crates.io/crates/cmx) — the underlying Rust crate
- [ICC specification](https://www.color.org/specification/ICC.1-2022-05.pdf)

<!-- cargo-rdme end -->
