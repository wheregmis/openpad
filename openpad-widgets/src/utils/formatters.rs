use makepad_widgets::*;
use openpad_protocol::{AssistantError, TokenUsage};
use std::time::{SystemTime, UNIX_EPOCH};

pub const SECONDS_PER_MINUTE: i64 = 60;
pub const SECONDS_PER_HOUR: i64 = 3600;
pub const SECONDS_PER_DAY: i64 = 86400;

pub fn live_design(_cx: &mut Cx) {}

pub fn format_timestamp(timestamp_ms: i64) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    let diff_ms = now - timestamp_ms;
    let diff_secs = diff_ms / 1000;
    let diff_mins = diff_secs / SECONDS_PER_MINUTE;
    let diff_hours = diff_secs / SECONDS_PER_HOUR;

    if diff_secs < SECONDS_PER_MINUTE {
        return "just now".to_string();
    }
    if diff_mins < SECONDS_PER_MINUTE {
        return format!("{} min ago", diff_mins);
    }
    if diff_hours < 24 {
        if diff_hours == 1 {
            return "1 hour ago".to_string();
        }
        return format!("{} hours ago", diff_hours);
    }
    let timestamp_secs = timestamp_ms / 1000;
    let time_of_day_secs = timestamp_secs % SECONDS_PER_DAY;
    let hours = (time_of_day_secs / SECONDS_PER_HOUR) as u8;
    let minutes = ((time_of_day_secs % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE) as u8;
    let time_str = format!("{:02}:{:02}", hours, minutes);
    let days_ago = diff_hours / 24;
    if days_ago > 0 {
        format!(
            "{} ({} {} ago)",
            time_str,
            days_ago,
            if days_ago == 1 { "day" } else { "days" }
        )
    } else {
        time_str
    }
}

pub fn format_token_usage(tokens: &TokenUsage) -> String {
    format!(
        "Tokens: in {} / out {} / r {} / cache {}r {}w",
        tokens.input, tokens.output, tokens.reasoning, tokens.cache.read, tokens.cache.write
    )
}

pub fn format_token_usage_short(tokens: &TokenUsage) -> String {
    format!("in {} / out {}", tokens.input, tokens.output)
}

pub fn format_cost(cost: f64) -> String {
    format!("Cost: ${:.4}", cost)
}

pub fn format_duration_ms(ms: i64) -> String {
    let total_secs = (ms / 1000).max(0) as u64;
    let mins = total_secs / 60;
    let secs = total_secs % 60;
    if mins > 0 {
        format!("{}m, {}s", mins, secs)
    } else {
        format!("{}s", secs)
    }
}

pub fn format_assistant_error(error: &AssistantError) -> String {
    match error {
        AssistantError::ProviderAuthError { provider_id, message } => format!("Authentication error for {}: {}\nAction: check your API key or re-authenticate this provider.", provider_id, message),
        AssistantError::APIError { message, status_code, is_retryable, .. } => {
            let mut detail = format!("API error: {}", message);
            if let Some(code) = status_code { detail.push_str(&format!(" (HTTP {})", code)); }
            let retryable = *is_retryable;
            let guidance = match status_code {
                Some(401) | Some(403) => "Action: check your API key and account permissions.",
                Some(429) => "Action: rate limited; wait and retry, or reduce request rate.",
                Some(code) if *code >= 500 => "Action: provider error; try again shortly.",
                _ if retryable => "Action: transient error; retry the request.",
                _ => "Action: review request details and try again.",
            };
            format!("{}\n{}", detail, guidance)
        }
        AssistantError::MessageOutputLengthError => "Message exceeded output length.\nAction: ask for a shorter answer or split the request.".to_string(),
        AssistantError::MessageAbortedError { message } => format!("Message aborted: {}\nAction: resend if this was unintended.", message),
        AssistantError::StructuredOutputError { message, retries } => format!(
            "Structured output error: {} (retries: {})\nAction: simplify the requested schema/format and retry.",
            message, retries
        ),
        AssistantError::ContextOverflowError { message, .. } => format!(
            "Context overflow: {}\nAction: reduce prompt length or start a new thread with less prior context.",
            message
        ),
        AssistantError::UnknownError { message } => format!("Unexpected error: {}\nAction: try again or check provider status.", message),
    }
}
