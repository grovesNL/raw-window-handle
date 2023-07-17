/// Raw display handle for the Web.
///
/// ## Construction
/// ```
/// # use raw_window_handle::WebDisplayHandle;
/// let mut display_handle = WebDisplayHandle::empty();
/// /* set fields */
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WebDisplayHandle;

impl WebDisplayHandle {
    pub fn empty() -> Self {
        Self {}
    }
}

/// Raw window handle for the Web.
///
/// ## Construction
/// ```
/// # use raw_window_handle::WebWindowHandle;
/// let mut window_handle = WebWindowHandle::empty();
/// /* set fields */
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WebWindowHandle {
    /// An ID value inserted into the [data attributes] of the canvas element as '`raw-handle`'.
    ///
    /// When accessing from JS, the attribute will automatically be called `rawHandle`.
    ///
    /// Each canvas created by the windowing system should be assigned their own unique ID.
    /// 0 should be reserved for invalid / null IDs.
    ///
    /// [data attributes]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/data-*
    pub id: u32,
}

impl WebWindowHandle {
    pub fn empty() -> Self {
        Self { id: 0 }
    }
}

/// Raw window handle for a Web canvas registered via [`wasm-bindgen`].
///
/// ## Construction
/// ```
/// # use raw_window_handle::Wbg02CanvasWindowHandle;
/// let mut window_handle = Wbg02CanvasWindowHandle::empty();
/// /* set fields */
/// ```
///
/// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Wbg02CanvasWindowHandle {
    /// The index of the canvas element in the [`wasm-bindgen`] table.
    ///
    /// If this index if non-zero, it is implied that it represents an [`HtmlCanvasElement`]
    /// that is registered in the [`wasm-bindgen`] table. It can be converted to and from the
    /// [`HtmlCanvasElement`] using [`wasm-bindgen`]'s [`FromWasmAbi`] and [`IntoWasmAbi`] traits.
    ///
    /// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
    /// [`HtmlCanvasElement`]: https://docs.rs/web-sys/latest/web_sys/struct.HtmlCanvasElement.html
    /// [`FromWasmAbi`]: https://docs.rs/wasm-bindgen/latest/wasm_bindgen/convert/trait.FromWasmAbi.html
    /// [`IntoWasmAbi`]: https://docs.rs/wasm-bindgen/latest/wasm_bindgen/convert/trait.IntoWasmAbi.html
    pub idx: u32,
}

impl Wbg02CanvasWindowHandle {
    pub fn empty() -> Self {
        Self { idx: 0 }
    }
}

/// Raw window handle for a Web offscreen canvas registered via [`wasm-bindgen`].
///
/// ## Construction
/// ```
/// # use raw_window_handle::Wbg02OffscreenCanvasWindowHandle;
/// let mut window_handle = Wbg02OffscreenCanvasWindowHandle::empty();
/// /* set fields */
/// ```
///
/// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Wbg02OffscreenCanvasWindowHandle {
    /// The index of the canvas element in the [`wasm-bindgen`] table.
    ///
    /// If this index if non-zero, it is implied that it represents an [`OffscreenCanvas`]
    /// that is registered in the [`wasm-bindgen`] table. It can be converted to and from the
    /// [`OffscreenCanvas`] using [`wasm-bindgen`]'s [`FromWasmAbi`] and [`IntoWasmAbi`] traits.
    ///
    /// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
    /// [`OffscreenCanvas`]: https://docs.rs/web-sys/latest/web_sys/struct.OffscreenCanvas.html
    /// [`FromWasmAbi`]: https://docs.rs/wasm-bindgen/latest/wasm_bindgen/convert/trait.FromWasmAbi.html
    /// [`IntoWasmAbi`]: https://docs.rs/wasm-bindgen/latest/wasm_bindgen/convert/trait.IntoWasmAbi.html
    pub idx: u32,
}

impl Wbg02OffscreenCanvasWindowHandle {
    pub fn empty() -> Self {
        Self { idx: 0 }
    }
}
