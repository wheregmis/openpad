use openpad_protocol::{FileDiff, Message, MessageWithParts, Part, TokenUsage};
use std::collections::HashMap;

/// Categories for grouping tools by type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ToolCategory {
    Files,    // read, grep, glob, search, cat, find
    Commands, // bash, execute, shell, run
    Edits,    // edit, write, patch, apply_patch
    Tools,    // everything else
}

impl ToolCategory {
    pub fn from_tool_name(tool: &str) -> Self {
        let lower = tool.to_lowercase();
        if lower.contains("read")
            || lower.contains("grep")
            || lower.contains("glob")
            || lower.contains("search")
            || lower.contains("cat")
            || lower.contains("find")
        {
            ToolCategory::Files
        } else if lower.contains("bash")
            || lower.contains("execute")
            || lower.contains("shell")
            || lower.contains("run")
        {
            ToolCategory::Commands
        } else if lower.contains("edit")
            || lower.contains("write")
            || lower.contains("patch")
            || lower.contains("apply")
        {
            ToolCategory::Edits
        } else {
            ToolCategory::Tools
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            ToolCategory::Files => "📄",
            ToolCategory::Commands => "🔧",
            ToolCategory::Edits => "✏️",
            ToolCategory::Tools => "🔨",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            ToolCategory::Files => "files",
            ToolCategory::Commands => "commands",
            ToolCategory::Edits => "edits",
            ToolCategory::Tools => "tools",
        }
    }
}

/// Summary of tools grouped by category
#[derive(Clone, Debug)]
pub struct ToolGroupSummary {
    pub category: ToolCategory,
    pub count: usize,
    pub detail_indices: Vec<(usize, usize)>, // (step_idx, detail_idx) pairs
}

#[derive(Clone, Debug)]
pub struct StepDetail {
    pub tool: String,
    pub input_summary: String,
    pub result: String,
    pub is_running: bool,
    pub category: ToolCategory,
    pub duration_ms: Option<i64>,
    pub title: Option<String>,
}

#[derive(Clone, Debug)]
pub struct DisplayStep {
    pub reason: String,
    pub cost: f64,
    pub tokens: Option<TokenUsage>,
    pub details: Vec<StepDetail>,
    pub expanded: bool,
    pub has_error: bool,
    pub has_running: bool,
    pub cached_description: String,
    pub cached_body: String,
    pub cached_header_expanded: String,
    pub cached_header_collapsed: String,
}

#[derive(Clone, Debug)]
pub struct DisplayMessage {
    pub role: String,
    pub text: String,
    pub message_id: Option<String>,
    pub timestamp: Option<i64>,
    pub model_id: Option<String>,
    pub tokens: Option<TokenUsage>,
    pub cost: Option<f64>,
    pub error_text: Option<String>,
    pub is_error: bool,
    pub diffs: Vec<FileDiff>,
    pub show_diffs: bool,
    pub steps: Vec<DisplayStep>,
    pub show_steps: bool,
    pub duration_ms: Option<i64>,
    pub cached_steps_summary: String,
    pub cached_grouped_summary: String,
    pub cached_tool_groups: Vec<ToolGroupSummary>,
    pub cached_needs_markdown: bool,
    pub cached_thinking_activity: String,
    pub cached_running_tools: Vec<(String, String, String)>,
    pub cached_timestamp: String,
    pub cached_token_usage: String,
    pub cached_cost: String,
}

pub struct MessageProcessor;

impl MessageProcessor {
    pub fn rebuild_from_parts(messages_with_parts: &[MessageWithParts]) -> Vec<DisplayMessage> {
        let mut display = Vec::new();
        let mut pending_diffs: Option<Vec<FileDiff>> = None;
        let mut pending_steps_only: Option<DisplayMessage> = None;

        for mwp in messages_with_parts {
            let (role, timestamp, model_id, tokens, cost, error_text, is_error, duration_ms) =
                match &mwp.info {
                    Message::User(msg) => (
                        "user",
                        Some(msg.time.created),
                        None,
                        None,
                        None,
                        None,
                        false,
                        None,
                    ),
                    Message::Assistant(msg) => {
                        let model = if !msg.model_id.is_empty() {
                            Some(msg.model_id.clone())
                        } else {
                            None
                        };
                        let error_text = msg
                            .error
                            .as_ref()
                            .map(crate::utils::formatters::format_assistant_error);
                        let duration_ms = msg
                            .time
                            .completed
                            .map(|completed| completed - msg.time.created)
                            .filter(|d| *d >= 0);
                        (
                            "assistant",
                            Some(msg.time.created),
                            model,
                            msg.tokens.clone(),
                            Some(msg.cost),
                            error_text,
                            msg.error.is_some(),
                            duration_ms,
                        )
                    }
                };

            let message_id = mwp.info.id().to_string();
            let mut text_parts: Vec<String> = Vec::new();
            let mut steps: Vec<DisplayStep> = Vec::new();

            for p in &mwp.parts {
                if let Some(text) = p.text_content() {
                    text_parts.push(text.to_string());
                } else if let Some((_mime, filename, _url)) = p.file_info() {
                    let name = filename.unwrap_or("attachment");
                    text_parts.push(format!("[Attachment: {}]", name));
                } else if matches!(p, Part::StepStart { .. }) {
                    steps.push(DisplayStep {
                        reason: String::new(),
                        cost: 0.,
                        tokens: None,
                        details: Vec::new(),
                        expanded: false,
                        has_error: false,
                        has_running: false,
                        cached_description: String::new(),
                        cached_body: String::new(),
                        cached_header_expanded: String::new(),
                        cached_header_collapsed: String::new(),
                    });
                } else if let Part::Tool { tool, state, .. } = p {
                    // Use tool_display for basic info
                    let (tool_name, input_summary, result) = p.tool_display().unwrap_or_default();
                    let has_error = result.starts_with("Error");
                    let is_running = result == "(running)" || result == "(pending)";

                    // Extract duration from ToolStateTime
                    let duration_ms = match state {
                        openpad_protocol::ToolState::Completed { time, .. }
                        | openpad_protocol::ToolState::Error { time, .. } => time
                            .start
                            .zip(time.end)
                            .map(|(s, e)| ((e - s) * 1000.0) as i64),
                        _ => None,
                    };

                    // Extract title from ToolState
                    let title = match state {
                        openpad_protocol::ToolState::Running { title, .. }
                        | openpad_protocol::ToolState::Completed { title, .. }
                            if !title.is_empty() =>
                        {
                            Some(title.clone())
                        }
                        _ => None,
                    };

                    let detail = StepDetail {
                        tool: tool_name,
                        input_summary,
                        result: result.clone(),
                        is_running,
                        category: ToolCategory::from_tool_name(tool),
                        duration_ms,
                        title,
                    };
                    if let Some(last) = steps.last_mut() {
                        last.details.push(detail);
                        if has_error {
                            last.has_error = true;
                        }
                        if is_running {
                            last.has_running = true;
                        }
                    } else {
                        steps.push(DisplayStep {
                            reason: String::new(),
                            cost: 0.,
                            tokens: None,
                            details: vec![detail],
                            expanded: false,
                            has_error,
                            has_running: is_running,
                            cached_description: String::new(),
                            cached_body: String::new(),
                            cached_header_expanded: String::new(),
                            cached_header_collapsed: String::new(),
                        });
                    }
                } else if let Some((reason, cost, tokens)) = p.step_finish_info() {
                    if let Some(last) = steps.last_mut() {
                        last.reason = reason.to_string();
                        last.cost = cost;
                        last.tokens = tokens.cloned();
                        last.has_running = false;
                    } else {
                        steps.push(DisplayStep {
                            reason: reason.to_string(),
                            cost,
                            tokens: tokens.cloned(),
                            details: Vec::new(),
                            expanded: false,
                            has_error: false,
                            has_running: false,
                            cached_description: String::new(),
                            cached_body: String::new(),
                            cached_header_expanded: String::new(),
                            cached_header_collapsed: String::new(),
                        });
                    }
                }
            }

            let mut text = text_parts.join("\n");
            if text.is_empty() && error_text.is_some() {
                text = "Assistant error".to_string();
            }
            let has_content = !text.is_empty() || (role == "assistant" && !steps.is_empty());
            if !has_content {
                continue;
            }

            let mut diffs = Vec::new();
            match &mwp.info {
                Message::User(msg) => {
                    if let Some(summary) = &msg.summary {
                        if !summary.diffs.is_empty() {
                            pending_diffs = Some(summary.diffs.clone());
                        }
                    }
                }
                Message::Assistant(_) => {
                    if let Some(pending) = pending_diffs.take() {
                        diffs = pending;
                    }
                }
            }

            let steps_only =
                role == "assistant" && text.is_empty() && !steps.is_empty() && !is_error;

            if steps_only {
                if let Some(ref mut pending) = pending_steps_only {
                    pending.steps.extend(steps);
                    pending.duration_ms = pending.duration_ms.or(duration_ms);
                } else {
                    pending_steps_only = Some(DisplayMessage {
                        role: role.to_string(),
                        text: String::new(),
                        message_id: Some(message_id),
                        timestamp,
                        model_id,
                        tokens,
                        cost,
                        error_text: None,
                        is_error: false,
                        diffs: Vec::new(),
                        show_diffs: false,
                        steps,
                        show_steps: true,
                        duration_ms,
                        cached_steps_summary: String::new(),
                        cached_grouped_summary: String::new(),
                        cached_tool_groups: Vec::new(),
                        cached_needs_markdown: false,
                        cached_thinking_activity: String::new(),
                        cached_running_tools: Vec::new(),
                        cached_timestamp: String::new(),
                        cached_token_usage: String::new(),
                        cached_cost: String::new(),
                    });
                }
                continue;
            }

            if role == "assistant" && !text.is_empty() {
                if let Some(pending) = pending_steps_only.take() {
                    let mut merged_steps = pending.steps;
                    merged_steps.extend(steps);
                    let merged_duration = duration_ms.or(pending.duration_ms);
                    let mut msg = DisplayMessage {
                        role: role.to_string(),
                        text,
                        message_id: Some(message_id),
                        timestamp,
                        model_id,
                        tokens,
                        cost,
                        error_text,
                        is_error,
                        diffs,
                        show_diffs: false,
                        steps: merged_steps,
                        show_steps: false,
                        duration_ms: merged_duration,
                        cached_steps_summary: String::new(),
                        cached_grouped_summary: String::new(),
                        cached_tool_groups: Vec::new(),
                        cached_needs_markdown: false,
                        cached_thinking_activity: String::new(),
                        cached_running_tools: Vec::new(),
                        cached_timestamp: String::new(),
                        cached_token_usage: String::new(),
                        cached_cost: String::new(),
                    };
                    Self::refresh_message_caches(&mut msg);
                    display.push(msg);
                    continue;
                }
            }

            if let Some(prev) = pending_steps_only.take() {
                display.push(prev);
            }

            let show_steps = role == "assistant" && text.is_empty() && !steps.is_empty();

            let mut msg = DisplayMessage {
                role: role.to_string(),
                text,
                message_id: Some(message_id),
                timestamp,
                model_id,
                tokens,
                cost,
                error_text,
                is_error,
                diffs,
                show_diffs: false,
                steps,
                show_steps,
                duration_ms,
                cached_steps_summary: String::new(),
                cached_grouped_summary: String::new(),
                cached_tool_groups: Vec::new(),
                cached_needs_markdown: false,
                cached_thinking_activity: String::new(),
                cached_running_tools: Vec::new(),
                cached_timestamp: String::new(),
                cached_token_usage: String::new(),
                cached_cost: String::new(),
            };
            Self::refresh_message_caches(&mut msg);
            display.push(msg);
        }
        if let Some(mut prev) = pending_steps_only.take() {
            if prev.role == "assistant" && prev.text.is_empty() && !prev.steps.is_empty() {
                prev.show_steps = true;
            }
            Self::refresh_message_caches(&mut prev);
            display.push(prev);
        }
        display
    }

    pub fn refresh_step_caches(step: &mut DisplayStep) {
        step.cached_description = Self::get_step_description(step);
        step.cached_body = Self::format_step_body(step);
        // Optimization: avoid reformatting step headers (including error prefix) in the draw loop
        let prefix = if step.has_error { "! " } else { "" };
        step.cached_header_expanded = format!("{}▾ {}", prefix, step.cached_description);
        step.cached_header_collapsed = format!("{}▸ {}", prefix, step.cached_description);
    }

    pub fn refresh_message_caches(msg: &mut DisplayMessage) {
        for step in &mut msg.steps {
            Self::refresh_step_caches(step);
        }
        msg.cached_steps_summary = Self::compute_steps_summary(msg);

        // Compute grouped summary for cleaner display
        let (grouped_summary, tool_groups) = Self::compute_grouped_summary(msg);
        msg.cached_grouped_summary = grouped_summary;
        msg.cached_tool_groups = tool_groups;

        msg.cached_needs_markdown = Self::compute_needs_markdown(&msg.text);

        let (activity, tools) = Self::compute_thinking_data(msg);
        msg.cached_thinking_activity = activity;
        msg.cached_running_tools = tools;

        // Optimization: cache formatted strings to avoid repeated allocations in draw_walk
        msg.cached_timestamp = msg
            .timestamp
            .map(crate::utils::formatters::format_timestamp)
            .unwrap_or_default();
        msg.cached_token_usage = msg
            .tokens
            .as_ref()
            .map(crate::utils::formatters::format_token_usage)
            .unwrap_or_default();
        msg.cached_cost = msg
            .cost
            .map(crate::utils::formatters::format_cost)
            .unwrap_or_default();
    }

    pub fn compute_thinking_data(msg: &DisplayMessage) -> (String, Vec<(String, String, String)>) {
        if let Some(last_step) = msg.steps.last() {
            let tools: Vec<(String, String, String)> = last_step
                .details
                .iter()
                .filter(|d| d.is_running)
                .map(|d| {
                    (
                        Self::get_tool_icon(&d.tool).to_string(),
                        Self::format_tool_name(&d.tool),
                        Self::format_tool_input(&d.input_summary),
                    )
                })
                .collect();
            let activity = if !tools.is_empty() {
                let names: Vec<String> = tools.iter().map(|t| t.1.clone()).take(3).collect();
                if names.is_empty() {
                    "Working...".to_string()
                } else {
                    format!("Running: {}", names.join(", "))
                }
            } else {
                let desc = &last_step.cached_description;
                if desc.is_empty() {
                    "Working...".to_string()
                } else {
                    format!("Working on: {}", desc)
                }
            };
            (activity, tools)
        } else {
            ("Working...".to_string(), Vec::new())
        }
    }

    pub fn compute_needs_markdown(text: &str) -> bool {
        text.contains("```") || text.contains("`") || text.contains("# ") || text.contains("> ")
    }

    pub fn compute_steps_summary(msg: &DisplayMessage) -> String {
        if msg.steps.is_empty() {
            return String::new();
        }
        let has_running = msg.steps.iter().any(|s| s.has_running);
        let mut labels: Vec<String> = Vec::new();
        for step in msg.steps.iter() {
            let desc = &step.cached_description;
            if !desc.is_empty() {
                labels.push(desc.clone());
            }
            if labels.len() >= 3 {
                break;
            }
        }
        let summary = if labels.is_empty() {
            "Steps".to_string()
        } else {
            labels.join(", ")
        };
        let count = msg.steps.len();
        let duration = msg
            .duration_ms
            .map(crate::utils::formatters::format_duration_ms);
        let prefix = if has_running { "Running" } else { "Steps" };
        if let Some(d) = duration {
            format!("{}: {} • {} • {}", prefix, count, summary, d)
        } else {
            format!("{}: {} • {}", prefix, count, summary)
        }
    }

    /// Compute grouped summary in format: "📄 3 • 🔧 2 • ⏱️ 2s"
    pub fn compute_grouped_summary(msg: &DisplayMessage) -> (String, Vec<ToolGroupSummary>) {
        if msg.steps.is_empty() {
            return (String::new(), Vec::new());
        }

        let mut category_map: HashMap<ToolCategory, Vec<(usize, usize)>> = HashMap::new();
        let mut total_duration_ms: i64 = 0;

        // Collect all tool details by category
        for (step_idx, step) in msg.steps.iter().enumerate() {
            for (detail_idx, detail) in step.details.iter().enumerate() {
                category_map
                    .entry(detail.category)
                    .or_default()
                    .push((step_idx, detail_idx));
                if let Some(d) = detail.duration_ms {
                    total_duration_ms += d;
                }
            }
        }

        // Build groups in preferred order: Files, Commands, Edits, Tools
        let order = [
            ToolCategory::Files,
            ToolCategory::Commands,
            ToolCategory::Edits,
            ToolCategory::Tools,
        ];

        let mut groups = Vec::new();
        let mut summary_parts = Vec::new();

        for cat in order {
            if let Some(indices) = category_map.get(&cat) {
                if !indices.is_empty() {
                    groups.push(ToolGroupSummary {
                        category: cat,
                        count: indices.len(),
                        detail_indices: indices.clone(),
                    });
                    summary_parts.push(format!("{} {}", cat.icon(), indices.len()));
                }
            }
        }

        // Add duration
        let has_running = msg.steps.iter().any(|s| s.has_running);
        let dur_ms = msg.duration_ms.unwrap_or(total_duration_ms);
        if dur_ms > 0 {
            let formatted = crate::utils::formatters::format_duration_ms(dur_ms);
            summary_parts.push(format!("⏱️ {}", formatted));
        }

        let prefix = if has_running { "Running: " } else { "" };
        let summary = format!("{}{}", prefix, summary_parts.join(" • "));

        (summary, groups)
    }
}

impl MessageProcessor {
    pub fn get_step_description(step: &DisplayStep) -> String {
        let running_prefix = if step.has_running { "⏳ " } else { "" };
        if step.details.is_empty() {
            return if step.reason.is_empty() {
                format!("{}Working...", running_prefix)
            } else {
                format!("{}{}", running_prefix, step.reason)
            };
        }

        let tool_names: Vec<&str> = step.details.iter().map(|d| d.tool.as_str()).collect();
        let has_read = tool_names
            .iter()
            .any(|t| t.contains("read") || t.contains("grep") || t.contains("search"));
        let has_write = tool_names
            .iter()
            .any(|t| t.contains("write") || t.contains("patch") || t.contains("apply"));
        let has_execute = tool_names
            .iter()
            .any(|t| t.contains("execute") || t.contains("run") || t.contains("shell"));

        let description = if has_write && has_read {
            "Reading and modifying files".to_string()
        } else if has_write {
            "Modifying files".to_string()
        } else if has_read {
            if tool_names.len() == 1 {
                if let Some(detail) = step.details.first() {
                    if let Some(path) = Self::extract_path(&detail.input_summary) {
                        return format!("{}Reading {}", running_prefix, Self::format_path(&path));
                    }
                }
                "Reading files".to_string()
            } else {
                format!("Reading {} files", step.details.len())
            }
        } else if has_execute {
            "Running commands".to_string()
        } else if tool_names.len() == 1 {
            if let Some(detail) = step.details.first() {
                format!("{}", Self::format_tool_name(&detail.tool))
            } else {
                "Processing".to_string()
            }
        } else {
            format!("{} operations", step.details.len())
        };

        format!("{}{}", running_prefix, description)
    }

    pub fn extract_path(input: &str) -> Option<String> {
        if let Some(start) = input.find("path=") {
            let rest = &input[start + 5..];
            let end = rest.find(' ').unwrap_or(rest.len());
            let path = &rest[..end];
            if !path.is_empty() {
                return Some(path.to_string());
            }
        }
        None
    }

    pub fn format_path(path: &str) -> String {
        if path.len() > 40 {
            if let Some(filename) = path.split('/').last().or_else(|| path.split('\\').last()) {
                return format!(".../{}", filename);
            }
        }
        path.to_string()
    }

    pub fn format_tool_name(tool: &str) -> String {
        match tool {
            "apply_patch" | "patch" => "Applying changes",
            "read" | "read_file" => "Reading file",
            "write" | "write_file" => "Writing file",
            "grep" | "search" => "Searching",
            "execute" | "shell" | "run" => "Running command",
            "list" | "ls" => "Listing directory",
            "cat" => "Viewing file",
            _ => tool,
        }
        .to_string()
    }

    pub fn get_tool_icon(tool: &str) -> &'static str {
        match tool {
            "apply_patch" | "patch" => "📝",
            "read" | "read_file" | "cat" => "📄",
            "write" | "write_file" => "💾",
            "grep" | "search" => "🔍",
            "execute" | "shell" | "run" => "⚡",
            "list" | "ls" => "📁",
            _ => "•",
        }
    }

    pub fn format_tool_input(input: &str) -> String {
        if input.is_empty() {
            return String::new();
        }
        let mut formatted_parts = Vec::new();
        if let Some(path) = Self::extract_path(input) {
            formatted_parts.push(Self::format_path(&path));
        }
        for (key, label) in [("offset", "@"), ("limit", "limit"), ("command", "cmd")].iter() {
            if let Some(start) = input.find(&format!("{}=", key)) {
                let rest = &input[start + key.len() + 1..];
                let end = rest.find(' ').unwrap_or(rest.len());
                let value = &rest[..end];
                if !value.is_empty() && value.len() < 50 {
                    if key == &"offset" {
                        formatted_parts.push(format!("@ {}", value));
                    } else if key == &"limit" {
                        if value != "50" && value != "100" {
                            formatted_parts.push(format!("limit {}", value));
                        }
                    } else {
                        formatted_parts.push(format!("{}: {}", label, value));
                    }
                }
            }
        }
        if formatted_parts.is_empty() {
            if input.len() > 50 {
                format!("{}...", input.chars().take(47).collect::<String>())
            } else {
                input.to_string()
            }
        } else {
            formatted_parts.join(" ")
        }
    }

    pub fn format_step_body(step: &DisplayStep) -> String {
        let mut lines: Vec<String> = Vec::new();
        for d in &step.details {
            let icon = if d.is_running {
                "⏳"
            } else {
                Self::get_tool_icon(&d.tool)
            };
            let tool_name = Self::format_tool_name(&d.tool);
            let input = Self::format_tool_input(&d.input_summary);

            let line = if d.is_running {
                if input.is_empty() {
                    format!("{} {} ...", icon, tool_name)
                } else {
                    format!("{} {} {} ...", icon, tool_name, input)
                }
            } else if input.is_empty() {
                format!("{} {} → {}", icon, tool_name, d.result)
            } else {
                format!("{} {} {} → {}", icon, tool_name, input, d.result)
            };
            lines.push(line);
        }
        if step.cost > 0.0 || step.tokens.is_some() {
            let mut stats = Vec::new();
            if step.cost > 0.0 {
                stats.push(crate::utils::formatters::format_cost(step.cost));
            }
            if let Some(ref t) = step.tokens {
                stats.push(crate::utils::formatters::format_token_usage_short(t));
            }
            if !stats.is_empty() {
                lines.push(stats.join(" · "));
            }
        }
        if lines.is_empty() {
            if step.reason.is_empty() {
                "—".to_string()
            } else {
                step.reason.clone()
            }
        } else {
            lines.join("\n")
        }
    }
}
