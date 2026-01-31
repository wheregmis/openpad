use makepad_widgets::*;

// Status indicator colors
pub const COLOR_STATUS_CONNECTED: Vec4 = vec4(0.231, 0.824, 0.435, 1.0); // Green
pub const COLOR_STATUS_ERROR: Vec4 = vec4(0.886, 0.333, 0.353, 1.0); // Red
pub const COLOR_STATUS_DISCONNECTED: Vec4 = vec4(0.55, 0.57, 0.60, 1.0); // Gray

// Text colors
pub const COLOR_TEXT_TITLE_ACTIVE: Vec4 = vec4(0.90, 0.91, 0.93, 1.0); // Bright white
pub const COLOR_TEXT_TITLE_INACTIVE: Vec4 = vec4(0.42, 0.48, 0.55, 1.0); // Muted gray

// Project context bar colors
pub const COLOR_PROJECT_BADGE_DEFAULT: Vec4 = vec4(0.12, 0.16, 0.21, 1.0);
pub const COLOR_PROJECT_BADGE_ACTIVE: Vec4 = vec4(0.16, 0.20, 0.27, 1.0);
pub const COLOR_PROJECT_BADGE_TEXT_ACTIVE: Vec4 = vec4(0.90, 0.91, 0.93, 1.0);
pub const COLOR_PROJECT_BADGE_TEXT_INACTIVE: Vec4 = vec4(0.64, 0.70, 0.78, 1.0);
pub const COLOR_PROJECT_PATH_TEXT: Vec4 = vec4(0.63, 0.72, 0.83, 1.0);

// Session selection colors (for projects panel)
pub const COLOR_SESSION_SELECTED: Vec4 = vec4(0.18, 0.22, 0.27, 1.0); // Darker blue-gray
pub const COLOR_SESSION_NORMAL: Vec4 = vec4(0.12, 0.14, 0.17, 1.0); // Dark gray

// Status text messages
pub const STATUS_CONNECTING: &str = "Connecting...";
pub const STATUS_CONNECTED: &str = "Connected";
pub const STATUS_DISCONNECTED: &str = "Disconnected";
pub const STATUS_ERROR_PREFIX: &str = "Error: ";

// Session UI text
pub const SESSION_TITLE_DEFAULT: &str = "Select a session or start a new one";
pub const SESSION_TITLE_NEW: &str = "New session";
pub const PROJECT_CONTEXT_NO_PROJECT: &str = "No active project";
