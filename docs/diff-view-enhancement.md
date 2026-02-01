# Diff View Enhancement

## Overview
This document describes the enhancements made to the diff view to add colored line rendering.

## Implementation

### ColoredDiffText Widget
A new custom Makepad widget (`openpad-app/src/components/colored_diff_text.rs`) was created to render diff text with per-line coloring:

#### Features:
- **Addition lines** (starting with `+`): Bright green (#4dca4d)
- **Deletion lines** (starting with `-`): Soft red (#e06060)  
- **Header lines** (starting with `──` or `...`): Soft blue (#88b0db)
- **Context lines** (all others): Light gray (#bbc1c9)

#### Architecture:
```rust
pub struct ColoredDiffText {
    view: View,
    draw_text: DrawText,
    lines: Vec<DiffLine>,
}
```

The widget:
1. Parses the diff text into individual lines
2. Categorizes each line by its type (addition, deletion, context, header)
3. Renders each line in the `draw_walk` method with the appropriate color
4. Uses Makepad's `DrawText` API for efficient rendering

### Integration with DiffView
The existing `DiffView` component was updated to use `ColoredDiffText` instead of a single monochrome `Label`:

**Before:**
```rust
diff_text = <Label> {
    draw_text: {
        color: (THEME_COLOR_DIFF_CONTEXT_TEXT)
    }
}
```

**After:**
```rust
diff_text = <ColoredDiffText> {
    width: Fill, height: Fit
}
```

## Future Enhancements

### Syntax Highlighting
To add syntax highlighting to the code content within diff lines, the following approach could be used:

1. **Option A: Use makepad-code-editor's tokenizer**
   - Tokenize each line's code content (after the +/- prefix)
   - Render each token with syntax-specific colors
   - Adjust token colors based on whether the line is an addition or deletion

2. **Option B: Integrate tree-sitter**
   - Add tree-sitter as a dependency
   - Parse code content with language-specific grammars
   - Render highlighted tokens while preserving diff colors

3. **Option C: Simple regex-based highlighting**
   - Match common keywords, strings, comments, etc.
   - Apply basic coloring rules
   - Faster but less accurate than full parsing

#### Implementation Challenges:
- Need to detect the programming language (from file extension or content)
- Must overlay syntax colors on top of diff colors (complex blending)
- Performance considerations for large diffs
- Maintaining readability when combining multiple color layers

### Background Colors
Currently only text colors are used. For better visual distinction, background colors could be added:
- Light green background (#1a2e1a) for addition lines
- Light red background (#2e1a1a) for deletion lines

This would match the style of GitHub and other modern diff viewers.

## Testing
Due to the local path dependencies on Makepad, testing requires:
1. A local Makepad installation at the configured path
2. Building with `cargo run --release`
3. Creating a test session with file changes
4. Viewing the diff in the expanded diff view

## Color Reference
All colors are defined inline but reference the theme:
- `THEME_COLOR_DIFF_ADD_TEXT`: #4dca4d
- `THEME_COLOR_DIFF_DEL_TEXT`: #e06060  
- `THEME_COLOR_DIFF_ADD_BG`: #1a2e1a (not currently used)
- `THEME_COLOR_DIFF_DEL_BG`: #2e1a1a (not currently used)
- `THEME_COLOR_DIFF_CONTEXT_TEXT`: #888888 (old value)
- Custom context color: #bbc1c9 (lighter for better readability)
- Custom header color: #88b0db (soft blue)
