use std::time::{SystemTime, UNIX_EPOCH};

/// Formats a Unix timestamp (milliseconds since epoch) into a human-readable time string.
/// Returns a string like "2:30 PM" or "14:30" for today's messages,
/// or "5 min ago" for very recent messages.
pub fn format_timestamp(timestamp_ms: i64) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    let diff_ms = now - timestamp_ms;
    let diff_secs = diff_ms / 1000;
    let diff_mins = diff_secs / 60;
    let diff_hours = diff_mins / 60;

    // Less than 1 minute: "just now"
    if diff_secs < 60 {
        return "just now".to_string();
    }

    // Less than 60 minutes: "X min ago"
    if diff_mins < 60 {
        return format!("{} min ago", diff_mins);
    }

    // Less than 24 hours: "X hours ago"
    if diff_hours < 24 {
        if diff_hours == 1 {
            return "1 hour ago".to_string();
        }
        return format!("{} hours ago", diff_hours);
    }

    // For older messages, show formatted date/time
    // Convert milliseconds to seconds for standard time conversion
    let timestamp_secs = timestamp_ms / 1000;

    // Simple formatting using chrono-like calculations
    // Days since epoch
    let days = timestamp_secs / 86400;
    let time_of_day_secs = timestamp_secs % 86400;
    let hours = (time_of_day_secs / 3600) as u8;
    let minutes = ((time_of_day_secs % 3600) / 60) as u8;

    // Format time (24-hour format for simplicity)
    let time_str = format!("{:02}:{:02}", hours, minutes);

    // If it's more than 7 days old, show date too
    if diff_hours > 24 * 7 {
        // Very simple date representation (just showing days since epoch is not useful)
        // In a real app, you'd use chrono or time crate for proper date formatting
        // For now, just show the time
        format!("{} ({} days ago)", time_str, diff_hours / 24)
    } else {
        format!("{} ({} days ago)", time_str, diff_hours / 24)
    }
}
