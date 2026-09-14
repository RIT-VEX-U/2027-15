/**
 * @file v5_display_api_documented.h
 * @brief Reverse-engineered documentation for the V5 Brain display C API.
 *
 * This companion header redeclares every screen/display-related symbol exported
 * by the copy of libv5rt.a bundled with this workspace.  Public declarations
 * retain the types from v5_api.h; declarations marked "archive-only" are not
 * part of the published V5 C SDK header and may change or disappear.
 *
 * The behavioral notes describe VEXOS 1.1.5.0 and its matching user library.
 * They should not be treated as an ABI guarantee for another VEXOS release.
 *
 * Coordinate and color conventions:
 *
 * - The hardware surface is 480 by 272 pixels.
 * - The user library initially adds 32 to pixel-based Y coordinates, reserving
 *   the top status-bar area and presenting the usual 480 by 240 user surface.
 *   Line-number text functions instead calculate their own absolute Y value.
 * - Rectangle and copied-image endpoints are inclusive.
 * - Colors are packed as 0x00RRGGBB.  The high byte is not surface alpha.
 * - Rendering state is global/current-context state.  Serialize concurrent
 *   drawing, text formatting, image decoding, and scratch-memory use.
 */

#ifndef V5_DISPLAY_API_DOCUMENTED_H_
#define V5_DISPLAY_API_DOCUMENTED_H_

#include "v5_api.h"

#ifdef __cplusplus
extern "C" {
#endif

/** @defgroup v5_display_color Display color and clearing
 *  @{
 */

/**
 * @brief Set the foreground drawing color.
 * @param col Color packed as 0x00RRGGBB.
 *
 * Pixels, lines, outlines, fills, and text use this color unless the operation
 * is explicitly a clear operation.
 */
void vexDisplayForegroundColor(uint32_t col);

/**
 * @brief Set the background drawing color.
 * @param col Color packed as 0x00RRGGBB.
 *
 * Pixel, line, rectangle, and circle clear operations use this color.  Text
 * also uses it when opaque rendering is selected.
 */
void vexDisplayBackgroundColor(uint32_t col);

/**
 * @brief Return the current foreground color.
 * @return Color packed as 0x00RRGGBB.
 */
uint32_t vexDisplayForegroundColorGet(void);

/**
 * @brief Return the current background color.
 * @return Color packed as 0x00RRGGBB.
 */
uint32_t vexDisplayBackgroundColorGet(void);

/**
 * @brief Erase the display using the firmware's current erase/theme color.
 *
 * @warning This primitive does not reliably use the value set by
 * vexDisplayBackgroundColor().  To clear with the selected background color,
 * use vexDisplayRectClear() over the desired bounds.
 */
void vexDisplayErase(void);

/** @} */

/** @defgroup v5_display_primitives Drawing primitives
 *  @{
 */

/**
 * @brief Set one pixel to the current foreground color.
 * @param x X coordinate.
 * @param y User Y coordinate; the SDK normally adds the 32-pixel status offset.
 *
 * A pixel outside the current clip region is ignored.
 */
void vexDisplayPixelSet(uint32_t x, uint32_t y);

/**
 * @brief Set one pixel to the current background color.
 * @param x X coordinate.
 * @param y User Y coordinate; the SDK normally adds the 32-pixel status offset.
 *
 * A pixel outside the current clip region is ignored.
 */
void vexDisplayPixelClear(uint32_t x, uint32_t y);

/**
 * @brief Draw a one-pixel line in the current foreground color.
 * @param x1 First endpoint X coordinate.
 * @param y1 First endpoint user Y coordinate.
 * @param x2 Second endpoint X coordinate.
 * @param y2 Second endpoint user Y coordinate.
 */
void vexDisplayLineDraw(int32_t x1, int32_t y1, int32_t x2, int32_t y2);

/**
 * @brief Draw a one-pixel line in the current background color.
 * @param x1 First endpoint X coordinate.
 * @param y1 First endpoint user Y coordinate.
 * @param x2 Second endpoint X coordinate.
 * @param y2 Second endpoint user Y coordinate.
 */
void vexDisplayLineClear(int32_t x1, int32_t y1, int32_t x2, int32_t y2);

/**
 * @brief Draw a one-pixel rectangle outline in the foreground color.
 * @param x1 Inclusive left endpoint.
 * @param y1 Inclusive top user Y endpoint.
 * @param x2 Inclusive right endpoint.
 * @param y2 Inclusive bottom user Y endpoint.
 */
void vexDisplayRectDraw(int32_t x1, int32_t y1, int32_t x2, int32_t y2);

/**
 * @brief Fill a rectangle with the current background color.
 * @param x1 Inclusive left endpoint.
 * @param y1 Inclusive top user Y endpoint.
 * @param x2 Inclusive right endpoint.
 * @param y2 Inclusive bottom user Y endpoint.
 */
void vexDisplayRectClear(int32_t x1, int32_t y1, int32_t x2, int32_t y2);

/**
 * @brief Fill a rectangle with the current foreground color.
 * @param x1 Inclusive left endpoint.
 * @param y1 Inclusive top user Y endpoint.
 * @param x2 Inclusive right endpoint.
 * @param y2 Inclusive bottom user Y endpoint.
 */
void vexDisplayRectFill(int32_t x1, int32_t y1, int32_t x2, int32_t y2);

/**
 * @brief Draw a one-pixel circle outline in the foreground color.
 * @param xc Center X coordinate.
 * @param yc Center user Y coordinate.
 * @param radius Radius in pixels.
 */
void vexDisplayCircleDraw(int32_t xc, int32_t yc, int32_t radius);

/**
 * @brief Fill a circle with the current background color.
 * @param xc Center X coordinate.
 * @param yc Center user Y coordinate.
 * @param radius Radius in pixels.
 */
void vexDisplayCircleClear(int32_t xc, int32_t yc, int32_t radius);

/**
 * @brief Fill a circle with the current foreground color.
 * @param xc Center X coordinate.
 * @param yc Center user Y coordinate.
 * @param radius Radius in pixels.
 */
void vexDisplayCircleFill(int32_t xc, int32_t yc, int32_t radius);

/** @} */

/** @defgroup v5_display_transfer Scrolling and pixel transfer
 *  @{
 */

/**
 * @brief Vertically scroll the display from an absolute hardware Y coordinate.
 * @param nStartLine First hardware scanline to scroll.
 * @param nLines Signed pixel displacement.
 *
 * Positive values move existing pixels upward and clear the vacated bottom
 * band; negative values move them downward and clear the top band.  Unlike
 * most pixel APIs, the SDK stub does not add its configured Y offset here.
 */
void vexDisplayScroll(int32_t nStartLine, int32_t nLines);

/**
 * @brief Vertically scroll a rectangular region.
 * @param x1 Inclusive left endpoint.
 * @param y1 Inclusive top user Y endpoint.
 * @param x2 Inclusive right endpoint.
 * @param y2 Inclusive bottom user Y endpoint.
 * @param nLines Signed pixel displacement; positive moves content upward.
 *
 * Vacated pixels are filled with the current background color.
 */
void vexDisplayScrollRect(int32_t x1, int32_t y1, int32_t x2,
                          int32_t y2, int32_t nLines);

/**
 * @brief Copy packed RGB pixels into a display rectangle.
 * @param x1 Inclusive destination left endpoint.
 * @param y1 Inclusive destination top user Y endpoint.
 * @param x2 Inclusive destination right endpoint.
 * @param y2 Inclusive destination bottom user Y endpoint.
 * @param pSrc First source pixel, packed as 0x00RRGGBB.
 * @param srcStride Source row stride in pixels, not bytes.
 *
 * Destination clipping also advances the source origin so pixel alignment is
 * preserved.  The function has no source-length argument and does not validate
 * that every addressed source pixel exists.
 */
void vexDisplayCopyRect(int32_t x1, int32_t y1, int32_t x2, int32_t y2,
                        uint32_t *pSrc, int32_t srcStride);

/** @} */

/** @defgroup v5_display_text Text rendering
 *  @{
 */

/**
 * @brief Format and draw text at a pixel position.
 * @param xpos Left X coordinate.
 * @param ypos Baseline/origin user Y coordinate.
 * @param bOpaque Nonzero to paint the background behind glyphs; zero for
 * transparent text.
 * @param format printf-compatible format string.
 * @param ... Format arguments.
 *
 * Formatting uses a shared 256-byte firmware buffer, so output is truncated to
 * fit and concurrent/reentrant calls must be serialized.
 */
void vexDisplayPrintf(int32_t xpos, int32_t ypos, uint32_t bOpaque,
                      const char *format, ...);

/**
 * @brief Format and draw normal text on a legacy numbered line.
 * @param nLineNumber Zero-based line number.
 * @param format printf-compatible format string.
 * @param ... Format arguments.
 *
 * The firmware uses absolute Y = line * 20 + 34 and X = 0.  Text is opaque.
 */
void vexDisplayString(const int32_t nLineNumber, const char *format, ...);

/**
 * @brief Format and draw normal opaque text at a pixel position.
 * @param xpos Left X coordinate.
 * @param ypos User Y coordinate.
 * @param format printf-compatible format string.
 * @param ... Format arguments.
 */
void vexDisplayStringAt(int32_t xpos, int32_t ypos, const char *format, ...);

/**
 * @brief Format and draw large text on a legacy numbered line.
 * @param nLineNumber Zero-based line number.
 * @param format printf-compatible format string.
 * @param ... Format arguments.
 *
 * The firmware uses absolute Y = line * 20 + 34 and X = 0.  Text is opaque.
 */
void vexDisplayBigString(const int32_t nLineNumber, const char *format, ...);

/**
 * @brief Format and draw large opaque text at a pixel position.
 * @param xpos Left X coordinate.
 * @param ypos User Y coordinate.
 * @param format printf-compatible format string.
 * @param ... Format arguments.
 */
void vexDisplayBigStringAt(int32_t xpos, int32_t ypos,
                           const char *format, ...);

/**
 * @brief Format and draw small opaque text at a pixel position.
 * @param xpos Left X coordinate.
 * @param ypos User Y coordinate.
 * @param format printf-compatible format string.
 * @param ... Format arguments.
 */
void vexDisplaySmallStringAt(int32_t xpos, int32_t ypos,
                             const char *format, ...);

/**
 * @brief Format, horizontally center, and draw normal opaque text.
 * @param nLineNumber Zero-based legacy line number.
 * @param format printf-compatible format string.
 * @param ... Format arguments.
 *
 * The X coordinate is (480 - measured_width) / 2 and the absolute Y coordinate
 * is line * 20 + 34.
 */
void vexDisplayCenteredString(const int32_t nLineNumber,
                              const char *format, ...);

/**
 * @brief Format, horizontally center, and draw large opaque text.
 * @param nLineNumber Zero-based legacy line number.
 * @param format printf-compatible format string.
 * @param ... Format arguments.
 */
void vexDisplayBigCenteredString(const int32_t nLineNumber,
                                 const char *format, ...);

/**
 * @brief va_list form of vexDisplayPrintf().
 * @param xpos Left X coordinate.
 * @param ypos User Y coordinate.
 * @param bOpaque Nonzero for opaque text, zero for transparent text.
 * @param format printf-compatible format string.
 * @param args Format arguments.
 */
void vexDisplayVPrintf(int32_t xpos, int32_t ypos, uint32_t bOpaque,
                       const char *format, va_list args);

/**
 * @brief va_list form of vexDisplayString().
 * @param nLineNumber Zero-based legacy line number.
 * @param format printf-compatible format string.
 * @param args Format arguments.
 */
void vexDisplayVString(const int32_t nLineNumber, const char *format,
                       va_list args);

/**
 * @brief va_list form of vexDisplayStringAt().
 * @param xpos Left X coordinate.
 * @param ypos User Y coordinate.
 * @param format printf-compatible format string.
 * @param args Format arguments.
 */
void vexDisplayVStringAt(int32_t xpos, int32_t ypos, const char *format,
                         va_list args);

/**
 * @brief va_list form of vexDisplayBigString().
 * @param nLineNumber Zero-based legacy line number.
 * @param format printf-compatible format string.
 * @param args Format arguments.
 */
void vexDisplayVBigString(const int32_t nLineNumber, const char *format,
                          va_list args);

/**
 * @brief va_list form of vexDisplayBigStringAt().
 * @param xpos Left X coordinate.
 * @param ypos User Y coordinate.
 * @param format printf-compatible format string.
 * @param args Format arguments.
 */
void vexDisplayVBigStringAt(int32_t xpos, int32_t ypos,
                            const char *format, va_list args);

/**
 * @brief va_list form of vexDisplaySmallStringAt().
 * @param xpos Left X coordinate.
 * @param ypos User Y coordinate.
 * @param format printf-compatible format string.
 * @param args Format arguments.
 */
void vexDisplayVSmallStringAt(int32_t xpos, int32_t ypos,
                              const char *format, va_list args);

/**
 * @brief va_list form of vexDisplayCenteredString().
 * @param nLineNumber Zero-based legacy line number.
 * @param format printf-compatible format string.
 * @param args Format arguments.
 */
void vexDisplayVCenteredString(const int32_t nLineNumber,
                               const char *format, va_list args);

/**
 * @brief va_list form of vexDisplayBigCenteredString().
 * @param nLineNumber Zero-based legacy line number.
 * @param format printf-compatible format string.
 * @param args Format arguments.
 */
void vexDisplayVBigCenteredString(const int32_t nLineNumber,
                                  const char *format, va_list args);

/**
 * @brief Set the current text scale as a ratio.
 * @param n Scale numerator; only the low 16 bits are used by this firmware.
 * @param d Scale denominator; only the low 16 bits are used.
 *
 * @warning Pass a nonzero denominator.
 */
void vexDisplayTextSize(uint32_t n, uint32_t d);

/**
 * @brief Select an installed font by resource name.
 * @param pFontName NUL-terminated font/resource name.
 *
 * Recognized aliases in this build include `monospace`, `proportional`,
 * `notosanslatin`, `g_sFontMono18`, `g_pui8Notosansmono39pt`, and
 * `g_pui8Notosansmono49pt`.  Other installed resource names may also work.
 */
void vexDisplayFontNamedSet(const char *pFontName);

/**
 * @brief Measure a string using the current font and scale.
 * @param pString NUL-terminated text.
 * @return Rendered width in pixels.
 */
int32_t vexDisplayStringWidthGet(const char *pString);

/**
 * @brief Measure a string using the current font and scale.
 * @param pString NUL-terminated text.
 * @return Rendered height in pixels.
 */
int32_t vexDisplayStringHeightGet(const char *pString);

/** @} */

/** @defgroup v5_display_clip Clipping and coordinate offset
 *  @{
 */

/**
 * @brief Set the inclusive clipping rectangle for the current display context.
 * @param x1 Inclusive left endpoint.
 * @param y1 Inclusive top user Y endpoint.
 * @param x2 Inclusive right endpoint.
 * @param y2 Inclusive bottom user Y endpoint.
 *
 * Firmware clamps the rectangle against the 480 by 272 hardware surface and
 * collapses inverted bounds.  The SDK's configured Y offset is applied.
 */
void vexDisplayClipRegionSet(int32_t x1, int32_t y1,
                             int32_t x2, int32_t y2);

/**
 * @brief Restore the broad default clip region.
 *
 * The library implements this as Set(0, 0, 480, 272).  The SDK Y offset is
 * still added and the firmware then clamps the endpoints.
 */
void vexDisplayClipRegionClear(void);

/** @} */

/** @defgroup v5_display_render Double buffering and vertical sync
 *  @{
 */

/**
 * @brief Present the current drawing surface.
 * @param bVsyncWait If true, wait for a vertical-sync event before committing.
 * @param bRunScheduler If true, yield while waiting when the caller is a
 * registered VEX task; otherwise poll.
 * @return true when the frame was committed; false if no render buffer was
 * available or another render was still pending after about 20 ms.
 *
 * The first call establishes the double-buffered drawing surface.  Later
 * calls with bVsyncWait set clear the VSync latch, poll for at most about
 * 20 ms, and then commit even if VSync timed out.  Normal user rendering
 * presents the 480 by 240 region below the 32-pixel status bar.
 */
bool vexDisplayRender(bool bVsyncWait, bool bRunScheduler);

/**
 * @brief Disable double buffering and restore direct front-buffer drawing.
 *
 * If a separate back buffer is active, the firmware swaps back to the front
 * surface.  Otherwise this call has no visible effect.
 */
void vexDisplayDoubleBufferDisable(void);

/** @} */

/** @defgroup v5_display_image In-memory image decoding
 *  @{
 */

/**
 * @brief Decode a BMP file from memory into caller-provided RGB storage.
 * @param ibuf Pointer to the complete BMP file.
 * @param oBuf Output descriptor.  oBuf->data must already point to storage.
 * @param maxw Maximum accepted width and recommended output row capacity.
 * @param maxh Maximum accepted height.
 * @return 1 on success, 0 on validation or decoding failure.
 *
 * Supported inputs observed in this build are uncompressed 24-, 16-, and
 * 8-bit BMP plus RLE8.  Output is top-down 0x00RRGGBB.  Allocate at least
 * maxw * maxh uint32_t pixels.  The API has no input-length parameter, so an
 * untrusted truncated buffer can cause out-of-bounds reads.
 */
uint32_t vexImageBmpRead(const uint8_t *ibuf, v5_image *oBuf,
                         uint32_t maxw, uint32_t maxh);

/**
 * @brief Decode a PNG file from memory into caller-provided RGB storage.
 * @param ibuf Pointer to the PNG bytes.
 * @param oBuf Output descriptor.  oBuf->data must already point to storage.
 * @param maxw Maximum accepted width and recommended output row capacity.
 * @param maxh Maximum accepted height.
 * @param ibuflen Number of readable bytes at ibuf.
 * @return 1 on success, 0 on validation or decoding failure.
 *
 * Allocate at least maxw * maxh uint32_t pixels.  The decoder uses shared
 * firmware state and should not be called concurrently.
 */
uint32_t vexImagePngRead(const uint8_t *ibuf, v5_image *oBuf,
                         uint32_t maxw, uint32_t maxh, uint32_t ibuflen);

/** @} */

/** @defgroup v5_display_touch Touch input
 *  @{
 */

/**
 * @brief Install or remove the user touch-event callback.
 * @param callback Function called with event, X, and Y, or NULL to disable it.
 *
 * The firmware stores the pointer directly; the callback and any code/data it
 * references must remain valid until replaced.
 */
void vexTouchUserCallbackSet(
    void (*callback)(V5_TouchEvent event, int32_t x, int32_t y));

/**
 * @brief Read the most recent touch event, coordinates, and event counters.
 * @param[out] status Non-NULL destination status structure.
 * @return true only when lastEvent is exactly kTouchEventPress; false for
 * release and kTouchEventPressAuto.
 *
 * For a release event the returned coordinates are the release coordinates;
 * for press and auto-repeat events they are the current press coordinates.
 */
bool vexTouchDataGet(V5_TouchStatus *status);

/** @} */

/** @defgroup v5_display_scratch Shared scratch memory
 *  @{
 */

/**
 * @brief Return the fixed VEXOS user scratch-memory region.
 * @param[out] ptr Receives the base address, or NULL when no region is exposed.
 * @return Region size in bytes; 0 when ptr is NULL or the configured size is
 * not positive.
 *
 * In this build the SDK table exposes 0x03200000 with size 0x00200000
 * (2 MiB).  This function neither allocates nor clears memory and does not
 * acquire vexScratchMemoryLock().
 */
int32_t vexScratchMemoryPtr(void **ptr);

/**
 * @brief Try to acquire the advisory scratch-memory lock.
 * @return true if the byte lock was previously clear and is now set; false if
 * it was already set.
 *
 * This is a nonblocking, non-recursive load/test/store lock with no owner
 * tracking and no scheduler yield.  The implementation is not an atomic
 * hardware synchronization primitive.
 */
bool vexScratchMemoryLock(void);

/**
 * @brief Clear the advisory scratch-memory lock.
 *
 * No ownership or balanced-lock check is performed.
 */
void vexScratchMemoryUnlock(void);

/** @} */

/** @defgroup v5_display_internal Archive-only display exports
 *
 * These functions are exported by this libv5rt.a but omitted from v5_api.h.
 * Their declarations and behavior are reconstructed for VEXOS 1.1.5.0 only.
 * @{ 
 */

/**
 * @brief Set the SDK coordinate offsets used by pixel-positioned wrappers.
 * @param x_offset Stored X offset.
 * @param y_offset Y offset added by most pixel-positioned SDK entry points.
 *
 * @warning Archive-only.  In this library build x_offset is stored but never
 * read; only y_offset affects calls.  The initial values are X=0 and Y=32.
 */
void vexDisplayOffsetSet(int32_t x_offset, int32_t y_offset);

/**
 * @brief Set a private font-rasterizer smoothing parameter.
 * @param smoothing Value whose low 16 bits are forwarded to the renderer.
 * @warning Archive-only; value units are private.
 */
void vexDisplayTextSmoothing(uint32_t smoothing);

/**
 * @brief Set a private text reference/alignment field.
 * @param reference Value masked to 14 bits and stored after multiplication by 4.
 * @warning Archive-only; the public C++ wrapper normally writes 1.
 */
void vexDisplayTextReference(uint32_t reference);

/**
 * @brief Set private glyph-spacing state.
 * @param spacing Value whose low 16 bits are forwarded to the renderer.
 * @warning Archive-only; value units are private.
 */
void vexDisplayTextSpacing(uint32_t spacing);

/**
 * @brief Set the SDK-visible pen-size state.
 * @param pen_size Pen size; firmware stores the low 16 bits.
 *
 * @warning Archive-only.  The raw C line and rectangle primitives remain
 * one-pixel operations; the C++ layer implements thicker strokes itself.
 */
void vexDisplayPenSizeSet(uint32_t pen_size);

/**
 * @brief Return the SDK-visible pen-size state.
 * @return Current 16-bit pen-size value.
 * @warning Archive-only.
 */
uint32_t vexDisplayPenSizeGet(void);

/**
 * @brief Select a custom private font object.
 * @param font_data Pointer to a firmware-format font structure.
 * @warning Archive-only.  The structure layout and lifetime contract are not
 * public; arbitrary data can crash the font renderer.
 */
void vexDisplayFontCustomSet(const void *font_data);

/**
 * @brief Set the private display-surface orientation mode.
 * @param orientation Firmware-private orientation value.
 * @warning Archive-only; accepted enum values were not recovered.
 */
void vexDisplayOrientation(uint32_t orientation);

/**
 * @brief Select an installed localization language by numeric ID.
 * @param language_id Language ID; firmware compares the low 16 bits.
 * @return true if a matching installed language was selected.
 * @warning Archive-only.
 */
bool vexDisplayLanguageSet(uint32_t language_id);

/**
 * @brief Expand a localized string into a caller buffer.
 * @param string_id Firmware string-table index.
 * @param[out] buffer Destination byte buffer.
 * @param buffer_size Writable destination capacity including the terminator.
 * @return Number of output bytes, or 0 for an invalid request.
 * @warning Archive-only.  String-table encoding and ID stability are private.
 */
int32_t vexDisplayStringGet(uint32_t string_id, char *buffer,
                            uint32_t buffer_size);

/**
 * @brief Copy the complete displayed surface into caller storage.
 * @param[out] pixels Storage for 480 * 272 packed 0x00RRGGBB pixels.
 *
 * The copy is normalized for the current surface reversal flags.  The
 * function always writes 130560 uint32_t values (522240 bytes).
 * @warning Archive-only; there is no capacity argument.
 */
void vexDisplayScreenGrab(uint32_t *pixels);

/**
 * @brief Clear the firmware vertical-sync event latch.
 * @warning Archive-only.
 */
void vexDisplayClearVsyncState(void);

/**
 * @brief Read the firmware vertical-sync event latch.
 * @return 1 when latched, otherwise 0.
 * @warning Archive-only.
 */
uint32_t vexDisplayGetVsyncState(void);

/**
 * @brief Return the current private display rotation flag.
 * @return Firmware rotation flag value.
 * @warning Archive-only; interpretation is version-dependent.
 */
uint32_t vexDisplayRotateFlagGet(void);

/**
 * @brief Return the current private theme identifier.
 * @return Firmware theme ID, or 0 on older system versions.
 * @warning Archive-only; interpretation is version-dependent.
 */
uint32_t vexDisplayThemeIdGet(void);

/**
 * @brief Set the clip rectangle in an indexed private graphics context.
 * @param index Context index; this firmware accepts 0 through 129.
 * @param x1 Inclusive left endpoint.
 * @param y1 Inclusive top user Y endpoint.
 * @param x2 Inclusive right endpoint.
 * @param y2 Inclusive bottom user Y endpoint.
 * @warning Archive-only.  Out-of-range indices are ignored.
 */
void vexDisplayClipRegionSetWithIndex(int32_t index, int32_t x1, int32_t y1,
                                      int32_t x2, int32_t y2);

/**
 * @brief Descriptor returned by vexDisplayOffscreenBufferGet().
 * @warning Archive-only.  Rows use a fixed 512-pixel stride regardless of
 * width; callers should not allocate or modify this descriptor themselves.
 */
typedef struct V5_DisplayOffscreenBuffer {
    uint32_t *pixels;          /**< Pixel storage with a 512-pixel row stride. */
    uint32_t width;            /**< Visible width, at most 512. */
    uint32_t height;           /**< Visible height, at most 512. */
    uint32_t background_color; /**< Color used to initialize and scroll-fill. */
} V5_DisplayOffscreenBuffer;

/**
 * @brief Allocate a private offscreen pixel buffer.
 * @param width Visible width in pixels, at most 512.
 * @param height Visible height in pixels, at most 512.
 * @param background_color Initial/fill color packed as 0x00RRGGBB.
 * @return Allocated descriptor, or NULL when dimensions exceed 512.
 *
 * Storage is height * 512 pixels and is initialized to background_color.
 * @warning Archive-only.  This build handles allocation failure incorrectly;
 * callers cannot rely on a safe NULL result when the heap is exhausted.
 */
V5_DisplayOffscreenBuffer *vexDisplayOffscreenBufferGet(
    uint32_t width, uint32_t height, uint32_t background_color);

/**
 * @brief Destroy an offscreen buffer and its pixel storage.
 * @param buffer Buffer returned by vexDisplayOffscreenBufferGet(), or NULL.
 * @warning Archive-only.  The pointer is invalid after this call.
 */
void vexDisplayOffscreenBufferDestroy(V5_DisplayOffscreenBuffer *buffer);

/**
 * @brief Set one offscreen pixel.
 * @param buffer Offscreen buffer.
 * @param x X coordinate.
 * @param y Y coordinate.
 * @param color Color packed as 0x00RRGGBB.
 * @warning Archive-only.  Out-of-bounds coordinates are ignored.
 */
void vexDisplayOffscreenBufferPixelSet(V5_DisplayOffscreenBuffer *buffer,
                                       int32_t x, int32_t y, uint32_t color);

/**
 * @brief Read one offscreen pixel.
 * @param buffer Offscreen buffer.
 * @param x X coordinate.
 * @param y Y coordinate.
 * @return Packed color, or 0 for a NULL buffer or out-of-bounds coordinate.
 * @warning Archive-only.
 */
uint32_t vexDisplayOffscreenBufferPixelGet(
    const V5_DisplayOffscreenBuffer *buffer, int32_t x, int32_t y);

/**
 * @brief Draw a one-pixel rectangle outline into an offscreen buffer.
 * @param buffer Offscreen buffer.
 * @param x1 Inclusive left endpoint.
 * @param y1 Inclusive top endpoint.
 * @param x2 Inclusive right endpoint.
 * @param y2 Inclusive bottom endpoint.
 * @param color Color packed as 0x00RRGGBB.
 * @warning Archive-only.  Individual pixels are clipped to the visible size.
 */
void vexDisplayOffscreenBufferRectDraw(V5_DisplayOffscreenBuffer *buffer,
                                       int32_t x1, int32_t y1,
                                       int32_t x2, int32_t y2,
                                       uint32_t color);

/**
 * @brief Fill a rectangle in an offscreen buffer.
 * @param buffer Offscreen buffer.
 * @param x1 Inclusive left endpoint.
 * @param y1 Inclusive top endpoint.
 * @param x2 Inclusive right endpoint.
 * @param y2 Inclusive bottom endpoint.
 * @param color Color packed as 0x00RRGGBB.
 * @warning Archive-only.  Individual pixels are clipped to the visible size.
 */
void vexDisplayOffscreenBufferRectFill(V5_DisplayOffscreenBuffer *buffer,
                                       int32_t x1, int32_t y1,
                                       int32_t x2, int32_t y2,
                                       uint32_t color);

/**
 * @brief Shift every offscreen row left and fill the vacated right side.
 * @param buffer Offscreen buffer.
 * @param amount Number of pixels to shift left.
 *
 * Vacated pixels receive buffer->background_color.
 * @warning Archive-only.  The implementation does not validate amount; pass a
 * value no greater than the fixed 512-pixel stride.
 */
void vexDisplayOffscreenBufferScrollH(V5_DisplayOffscreenBuffer *buffer,
                                      uint32_t amount);

/**
 * @brief Copy an offscreen buffer region to the display.
 * @param buffer Offscreen source buffer.
 * @param x1 Inclusive destination/source left endpoint.
 * @param y1 Inclusive destination/source top user Y endpoint.
 * @param x2 Inclusive destination/source right endpoint.
 * @param y2 Inclusive destination/source bottom user Y endpoint.
 *
 * The wrapper calls vexDisplayCopyRect() with the fixed 512-pixel source
 * stride.  It does not check buffer for NULL.
 * @warning Archive-only.
 */
void vexDisplayOffscreenBufferBlit(V5_DisplayOffscreenBuffer *buffer,
                                   int32_t x1, int32_t y1,
                                   int32_t x2, int32_t y2);

/** @} */

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* V5_DISPLAY_API_DOCUMENTED_H_ */
