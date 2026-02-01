# Openpad Theme Guide

## Overview

Openpad uses a centralized color theme system defined in `openpad-app/src/theme.rs`. All UI colors are defined as semantic constants that can be referenced throughout the application.

## Using Theme Colors

### In Component Files

To use theme colors in your component:

1. Import the theme module in your `live_design!` block:
```rust
live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::theme::*;  // Add this line
    
    // Your component definition...
}
```

2. Reference theme colors using parentheses:
```rust
draw_bg: {
    color: (THEME_COLOR_BG_APP)
}

draw_text: {
    color: (THEME_COLOR_TEXT_PRIMARY)
}
```

## Available Theme Colors

### Background Colors

- `THEME_COLOR_BG_APP` - Main application background (#1e1e1e)
- `THEME_COLOR_BG_DARKER` - Darker background variant (#1a1a1a)
- `THEME_COLOR_BG_USER_BUBBLE` - User message bubbles (#2d2d30)
- `THEME_COLOR_BG_ASSISTANT_BUBBLE` - Assistant message bubbles (#252526)
- `THEME_COLOR_BG_DIALOG` - Dialog backgrounds (#1f2329)
- `THEME_COLOR_BG_INPUT` - Input field backgrounds (#15181d)
- `THEME_COLOR_BG_BUTTON` - Button backgrounds (#2b2f35)
- `THEME_COLOR_BG_BUTTON_HOVER` - Button hover state (#353a40)

### Border Colors

- `THEME_COLOR_BORDER_LIGHT` - Light borders (#444)
- `THEME_COLOR_BORDER_MEDIUM` - Medium borders (#333)
- `THEME_COLOR_BORDER_DIALOG` - Dialog borders (#2b3138)

### Text Colors

**Primary Text:**
- `THEME_COLOR_TEXT_PRIMARY` - Primary text (#e6e9ee)
- `THEME_COLOR_TEXT_BRIGHT` - Bright white text (#ffffff)
- `THEME_COLOR_TEXT_NORMAL` - Normal text (#ccc)
- `THEME_COLOR_TEXT_LIGHT` - Light text (#ddd)
- `THEME_COLOR_TEXT_DIM` - Dimmed text (#aab3bd)

**Muted Text:**
- `THEME_COLOR_TEXT_MUTED` - Muted text (#888)
- `THEME_COLOR_TEXT_MUTED_LIGHT` - Lightly muted (#666)
- `THEME_COLOR_TEXT_MUTED_LIGHTER` - Even lighter muted (#aaa)
- `THEME_COLOR_TEXT_MUTED_DARK` - Dark muted (#555)
- `THEME_COLOR_TEXT_MUTED_DARKER` - Darker muted (#444)

**Special Text:**
- `THEME_COLOR_TEXT_CODE` - Code text (#9cdcfe)
- `THEME_COLOR_TEXT_BOLD` - Bold text emphasis (#eee)

### Interactive States

- `THEME_COLOR_TRANSPARENT` - Fully transparent (#0000)
- `THEME_COLOR_HOVER_LIGHT` - Light hover state (#2d2d2d)
- `THEME_COLOR_HOVER_MEDIUM` - Medium hover state (#333)

### Accent Colors

**Blue (Primary Actions):**
- `THEME_COLOR_ACCENT_BLUE` - Primary blue (#3b82f6)
- `THEME_COLOR_ACCENT_BLUE_HOVER` - Blue hover (#1d4fed)
- `THEME_COLOR_ACCENT_BLUE_DARK` - Dark blue (#1d4ed8)

**Other Accents:**
- `THEME_COLOR_ACCENT_PURPLE` - Purple accent (#8b5cf6)
- `THEME_COLOR_ACCENT_AMBER` - Warning/working state (#f59e0b)
- `THEME_COLOR_ACCENT_RED` - Danger/delete actions (#ef4444)

### Status Colors

- `THEME_COLOR_STATUS_DOT` - Status indicator dot (#6b7b8c)

### Additional Shades

- `THEME_COLOR_SHADE_1` through `THEME_COLOR_SHADE_11` - Various UI element shades

## Examples

### Creating a Button

```rust
my_button = <Button> {
    width: 80, height: 32
    text: "Click Me"
    draw_bg: {
        color: (THEME_COLOR_ACCENT_BLUE)
        color_hover: (THEME_COLOR_ACCENT_BLUE_HOVER)
        border_radius: 6.0
    }
    draw_text: { 
        color: (THEME_COLOR_TEXT_BRIGHT), 
        text_style: <THEME_FONT_REGULAR> { font_size: 11 } 
    }
}
```

### Creating a Dialog

```rust
my_dialog = <RoundedView> {
    width: 400, height: Fit
    padding: 16
    draw_bg: {
        color: (THEME_COLOR_BG_DIALOG)
        border_color: (THEME_COLOR_BORDER_DIALOG)
        border_radius: 12.0
        border_size: 1.0
    }
    
    title = <Label> {
        text: "Dialog Title"
        draw_text: {
            color: (THEME_COLOR_TEXT_PRIMARY)
            text_style: <THEME_FONT_BOLD> { font_size: 14 }
        }
    }
}
```

### Creating Interactive Elements

```rust
hover_button = <Button> {
    draw_bg: {
        color: (THEME_COLOR_TRANSPARENT)
        color_hover: (THEME_COLOR_HOVER_MEDIUM)
    }
    draw_text: {
        color: (THEME_COLOR_TEXT_MUTED_LIGHT)
        color_hover: (THEME_COLOR_TEXT_MUTED_LIGHTER)
    }
}
```

## Modifying the Theme

To change colors globally:

1. Edit `openpad-app/src/theme.rs`
2. Update the hex values for the desired theme constants
3. Rebuild the application - all components will use the new colors

## Best Practices

1. **Use semantic names**: Choose theme colors based on their purpose, not their appearance
   - ✅ Good: `color: (THEME_COLOR_ACCENT_BLUE)` for a primary action button
   - ❌ Bad: Hardcoding `color: #3b82f6`

2. **Consistent hover states**: Use theme hover colors for interactive elements
   - Buttons: `THEME_COLOR_HOVER_MEDIUM` or `THEME_COLOR_HOVER_LIGHT`
   - Accent buttons: Use the corresponding `*_HOVER` variant

3. **Text hierarchy**: Use text color variants to establish visual hierarchy
   - Primary content: `THEME_COLOR_TEXT_PRIMARY` or `THEME_COLOR_TEXT_NORMAL`
   - Secondary content: `THEME_COLOR_TEXT_MUTED` variants
   - Code/monospace: `THEME_COLOR_TEXT_CODE`

4. **Action colors**: Use consistent accent colors for different action types
   - Primary actions: Blue accents
   - Warnings/working: Amber accent
   - Destructive actions: Red accent
   - Secondary actions: Purple accent

## Future Enhancements

The theme system is designed to support future features like:
- Dark/light mode switching
- Custom user themes
- High contrast modes
- Color accessibility options

To add these features, you would:
1. Create variant theme files (e.g., `theme_light.rs`, `theme_dark.rs`)
2. Add a theme switching mechanism
3. All components will automatically adapt to the new theme
