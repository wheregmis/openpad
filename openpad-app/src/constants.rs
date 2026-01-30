use makepad_widgets::*;

// Status indicator colors
pub const COLOR_STATUS_CONNECTED: Vec4 = vec4(0.231, 0.824, 0.435, 1.0); // Green
pub const COLOR_STATUS_ERROR: Vec4 = vec4(0.886, 0.333, 0.353, 1.0); // Red
pub const COLOR_STATUS_DISCONNECTED: Vec4 = vec4(0.55, 0.57, 0.60, 1.0); // Gray

// Text colors
pub const COLOR_TEXT_TITLE_ACTIVE: Vec4 = vec4(0.90, 0.91, 0.93, 1.0); // Bright white
pub const COLOR_TEXT_TITLE_INACTIVE: Vec4 = vec4(0.42, 0.48, 0.55, 1.0); // Muted gray

// Status text messages
pub const STATUS_CONNECTING: &str = "Connecting...";
pub const STATUS_CONNECTED: &str = "Connected";
pub const STATUS_DISCONNECTED: &str = "Disconnected";
pub const STATUS_ERROR_PREFIX: &str = "Error: ";

// Session UI text
pub const SESSION_TITLE_DEFAULT: &str = "Select a session or start a new one";
pub const SESSION_TITLE_NEW: &str = "New session";
