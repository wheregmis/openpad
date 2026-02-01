use makepad_widgets::*;

live_design! {
    // ========================================
    // BACKGROUND COLORS
    // ========================================
    
    // Main application backgrounds
    pub THEME_COLOR_BG_APP = #1e1e1e
    pub THEME_COLOR_BG_DARKER = #1a1a1a
    
    // Component backgrounds
    pub THEME_COLOR_BG_USER_BUBBLE = #2d2d30
    pub THEME_COLOR_BG_ASSISTANT_BUBBLE = #252526
    
    // Dialog backgrounds
    pub THEME_COLOR_BG_DIALOG = #1f2329
    pub THEME_COLOR_BG_INPUT = #15181d
    
    // Button backgrounds
    pub THEME_COLOR_BG_BUTTON = #2b2f35
    pub THEME_COLOR_BG_BUTTON_HOVER = #353a40
    
    // ========================================
    // BORDER COLORS
    // ========================================
    
    pub THEME_COLOR_BORDER_LIGHT = #444
    pub THEME_COLOR_BORDER_MEDIUM = #333
    pub THEME_COLOR_BORDER_DIALOG = #2b3138
    
    // ========================================
    // TEXT COLORS
    // ========================================
    
    // Primary text colors
    pub THEME_COLOR_TEXT_PRIMARY = #e6e9ee
    pub THEME_COLOR_TEXT_BRIGHT = #ffffff
    pub THEME_COLOR_TEXT_NORMAL = #ccc
    pub THEME_COLOR_TEXT_LIGHT = #ddd
    pub THEME_COLOR_TEXT_DIM = #aab3bd
    
    // Muted/secondary text colors
    pub THEME_COLOR_TEXT_MUTED = #888
    pub THEME_COLOR_TEXT_MUTED_LIGHT = #666
    pub THEME_COLOR_TEXT_MUTED_LIGHTER = #aaa
    pub THEME_COLOR_TEXT_MUTED_DARK = #555
    pub THEME_COLOR_TEXT_MUTED_DARKER = #444
    
    // Special text colors
    pub THEME_COLOR_TEXT_CODE = #9cdcfe
    pub THEME_COLOR_TEXT_BOLD = #eee
    
    // ========================================
    // INTERACTIVE STATES
    // ========================================
    
    // Transparent
    pub THEME_COLOR_TRANSPARENT = #0000
    
    // Hover states
    pub THEME_COLOR_HOVER_LIGHT = #2d2d2d
    pub THEME_COLOR_HOVER_MEDIUM = #333
    pub THEME_COLOR_HOVER_SUBTLE = #ffffff10
    
    // ========================================
    // ACCENT COLORS
    // ========================================
    
    // Blue accent (primary actions)
    pub THEME_COLOR_ACCENT_BLUE = #3b82f6
    pub THEME_COLOR_ACCENT_BLUE_HOVER = #1d4fed
    pub THEME_COLOR_ACCENT_BLUE_DARK = #1d4ed8
    
    // Purple accent
    pub THEME_COLOR_ACCENT_PURPLE = #8b5cf6
    
    // Orange/Amber accent (working/warning)
    pub THEME_COLOR_ACCENT_AMBER = #f59e0b
    
    // Red accent (danger/delete)
    pub THEME_COLOR_ACCENT_RED = #ef4444

    // Diff colors
    pub THEME_COLOR_DIFF_ADD_BG = #1a2e1a
    pub THEME_COLOR_DIFF_ADD_TEXT = #4dca4d
    pub THEME_COLOR_DIFF_DEL_BG = #2e1a1a
    pub THEME_COLOR_DIFF_DEL_TEXT = #e06060
    pub THEME_COLOR_DIFF_CONTEXT_TEXT = #888888
    pub THEME_COLOR_DIFF_HEADER_BG = #1a1f2e

    // ========================================
    // STATUS COLORS
    // ========================================
    
    pub THEME_COLOR_STATUS_DOT = #6b7b8c
    
    // ========================================
    // ADDITIONAL UI COLORS
    // ========================================
    
    // Additional shades for various UI elements
    pub THEME_COLOR_SHADE_1 = #27303a
    pub THEME_COLOR_SHADE_2 = #2a2a2a
    pub THEME_COLOR_SHADE_3 = #2a2f36
    pub THEME_COLOR_SHADE_4 = #334155
    pub THEME_COLOR_SHADE_5 = #313843
    pub THEME_COLOR_SHADE_6 = #475569
    pub THEME_COLOR_SHADE_7 = #8fa0b3
    pub THEME_COLOR_SHADE_8 = #9ca3af
    pub THEME_COLOR_SHADE_9 = #a3a3a3
    pub THEME_COLOR_SHADE_10 = #cbd3dc
    pub THEME_COLOR_SHADE_11 = #cccccc
}
