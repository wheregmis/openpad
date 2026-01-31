# Image and Long Text Paste Feature

## Overview
Openpad now supports pasting both long text and images directly into the chat panel. Images pasted as data URLs are automatically detected, extracted, and sent as file attachments alongside your message.

## Features

### ✅ Image Paste Support
- Automatically detects image data URLs in pasted content
- Supports multiple image formats: PNG, JPEG, GIF, WebP, SVG
- Extracts data URLs from clipboard content
- Shows preview of attached images before sending
- Generates unique filenames for pasted images
- Removes data URLs from message text automatically

### ✅ Long Text Support
- Paste any length of text into the chat input
- No special handling required - works out of the box
- Text is sent as-is to the OpenCode server

### ✅ Visual Feedback
- Attachments preview area appears above input box
- Lists all attached files by filename
- "Clear" button to remove all attachments
- Attachments automatically cleared after sending

## How to Use

### Pasting Images

#### Method 1: Via Screenshot Tools (Limited Support)
Some screenshot tools copy images as HTML with embedded data URLs:
1. Take a screenshot using your OS screenshot tool
2. Paste into the Openpad chat input (Cmd+V or Ctrl+V)
3. If the clipboard contains a data URL, it will be detected automatically

#### Method 2: Manual Data URL Paste
1. Copy an image data URL (e.g., from a browser's "Copy Image")
2. Paste into the chat input
3. The data URL will be extracted and shown in attachments preview
4. Type your message
5. Press Send or Enter

**Example data URL:**
```
data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==
```

### Pasting Long Text
1. Copy any amount of text (code, documents, etc.)
2. Paste into the chat input
3. The text appears in the input field
4. Press Send or Enter

### Mixed Content
You can paste content that contains both text and images:
1. Paste content with embedded data URLs
2. The images are extracted to attachments
3. The remaining text stays in the input
4. Both are sent together when you press Send

## Technical Details

### Data URL Detection
The system uses regex pattern matching to detect image data URLs:
```regex
data:(image/(?:png|jpeg|jpg|gif|webp|svg\+xml));base64,([A-Za-z0-9+/=]+)
```

### Message Structure
When you send a message with attachments, it's sent to the OpenCode server as a `PromptRequest` with multiple parts:
```rust
PromptRequest {
    model: Option<ModelSpec>,
    parts: vec![
        PartInput::Text { text: "your message" },
        PartInput::File { 
            mime: "image/png",
            filename: "pasted_image_123.png",
            url: "data:image/png;base64,..."
        }
    ]
}
```

### Filename Generation
Pasted images get unique filenames based on:
- Current timestamp in milliseconds
- Attachment counter (for images pasted simultaneously)
- File extension based on mime type

**Format:** `pasted_image_{timestamp}_{counter}.{ext}`

**Example:** `pasted_image_1738358400000_0.png`

## Limitations

### Current Limitations
1. **Direct Clipboard Image Support**: Makepad's TextInput doesn't expose native clipboard events, so we can't directly access images copied to the clipboard. Instead, we rely on detecting data URLs in pasted text.

2. **Browser/Tool Dependency**: Whether an image pastes as a data URL depends on the source application:
   - ✅ Works: Some screenshot tools, browser "Copy Image", HTML content
   - ❌ Doesn't work: Direct file system copy, most native image viewers

3. **Large Images**: Data URLs can be very long for large images, which may impact performance when pasting.

### Future Enhancements
- Direct native clipboard image support (requires Makepad enhancement)
- Image preview thumbnails in attachments area
- Support for more file types (PDFs, videos, audio)
- Drag-and-drop file support
- File picker button for selecting files

## Testing

### Manual Testing
See `docs/plans/test_paste_feature.md` for detailed testing instructions.

### Unit Tests
The codebase includes unit tests for data URL detection:
```bash
cargo test --package openpad-app
```

**Note:** Full tests require GUI libraries (X11, etc.) which may not be available in all environments.

### Test Coverage
- ✅ Data URL pattern matching
- ✅ Text extraction around data URLs
- ✅ MIME type detection
- ✅ Multiple data URL handling

## Troubleshooting

### Images Not Being Detected
**Problem:** You paste an image but it doesn't appear in attachments.

**Solutions:**
- Verify the clipboard content includes a data URL (try pasting in a text editor)
- Try copying the image from a different source (browser, etc.)
- Use Method 2 (manual data URL paste) to test

### Data URL Appears in Message Text
**Problem:** The data URL is not being removed from the text.

**Solutions:**
- Check that the data URL format matches: `data:image/{type};base64,{data}`
- Verify the image type is supported (png, jpeg, gif, webp, svg)
- Check browser console for any errors

### Attachments Not Clearing
**Problem:** Attachments persist after sending.

**Solutions:**
- Click the "Clear" button manually
- Check that the message was sent successfully
- Restart the application

## API Reference

### Types

#### `AttachedFile`
```rust
pub struct AttachedFile {
    pub filename: String,
    pub mime_type: String,
    pub data_url: String,
}
```

#### `PartInput`
```rust
pub enum PartInput {
    Text {
        id: Option<String>,
        text: String,
        synthetic: Option<bool>,
        ignored: Option<bool>,
    },
    File {
        id: Option<String>,
        mime: String,
        filename: Option<String>,
        url: String,
        source: Option<FilePartSource>,
    },
}
```

### Helper Methods

#### `PartInput::text(text: impl Into<String>) -> Self`
Creates a text part.

#### `PartInput::file(mime: impl Into<String>, url: impl Into<String>) -> Self`
Creates a file part with mime type and URL.

#### `PartInput::file_with_filename(mime: impl Into<String>, filename: impl Into<String>, url: impl Into<String>) -> Self`
Creates a file part with mime type, filename, and URL.

## Examples

### Example 1: Pasting a Simple Image
```
Input: data:image/png;base64,ABC123==
Result:
  - Attachments: ["pasted_image_1738358400000_0.png"]
  - Message text: ""
```

### Example 2: Pasting Text with Image
```
Input: "Check out this screenshot: data:image/png;base64,ABC123== Pretty cool!"
Result:
  - Attachments: ["pasted_image_1738358400000_0.png"]
  - Message text: "Check out this screenshot:  Pretty cool!"
```

### Example 3: Multiple Images
```
Input: "First: data:image/png;base64,ABC== Second: data:image/jpeg;base64,DEF=="
Result:
  - Attachments: ["pasted_image_1738358400000_0.png", "pasted_image_1738358400000_1.jpg"]
  - Message text: "First:  Second: "
```

## Related Documentation
- [Architecture.md](../Architecture.md) - Overall system architecture
- [openpad-protocol/README.md](../openpad-protocol/README.md) - Protocol API reference
- [OpenAPI Specification](../openapi.json) - Full API schema

## Contributing
To improve paste functionality:
1. Check existing issues for paste-related requests
2. Test your changes with various image sources
3. Update this documentation
4. Add unit tests for new functionality
5. Submit a PR with clear description
