/// The antialiasing strategy that should be used when rasterizing glyphs.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RasterizationOptions {
    /// "Black-and-white" rendering. Each pixel is either entirely on or off.
    Bilevel,
    /// Grayscale antialiasing. Only one channel is used.
    GrayscaleAa,
    /// Subpixel RGB antialiasing, for LCD screens.
    SubpixelAa,
}