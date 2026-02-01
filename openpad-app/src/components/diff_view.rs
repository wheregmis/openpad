use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use openpad_widgets::openpad::*;
    use openpad_widgets::theme::*;

    pub DiffView = {{DiffView}} {
        width: Fill, height: Fit
        flow: Down
        visible: false

        summary_header = <RoundedView> {
            width: Fill, height: Fit
            padding: { left: 12, right: 12, top: 8, bottom: 8 }
            cursor: Hand
            show_bg: true
            draw_bg: {
                color: (THEME_COLOR_DIFF_HEADER_BG)
                border_radius: 8.0
            }

            summary_row = <View> {
                width: Fill, height: Fit
                flow: Right
                spacing: 8
                align: { y: 0.5 }

                summary_files_label = <Label> {
                    width: Fit, height: Fit
                    text: ""
                    draw_text: {
                        color: (THEME_COLOR_TEXT_DIM)
                        text_style: <THEME_FONT_BOLD> { font_size: 11 }
                    }
                }

                summary_add_label = <Label> {
                    width: Fit, height: Fit
                    text: ""
                    draw_text: {
                        color: (THEME_COLOR_DIFF_ADD_TEXT)
                        text_style: <THEME_FONT_BOLD> { font_size: 11 }
                    }
                }

                summary_del_label = <Label> {
                    width: Fit, height: Fit
                    text: ""
                    draw_text: {
                        color: (THEME_COLOR_DIFF_DEL_TEXT)
                        text_style: <THEME_FONT_BOLD> { font_size: 11 }
                    }
                }
            }
        }

        diff_content = <RoundedView> {
            width: Fill, height: Fit
            visible: false
            padding: { left: 12, right: 12, top: 8, bottom: 8 }
            margin: { top: 2 }
            show_bg: true
            draw_bg: {
                color: (THEME_COLOR_BG_DARKER)
                border_radius: 0.0
            }

            <ScrollYView> {
                width: Fill, height: Fit

                diff_text = <Label> {
                    width: Fill, height: Fit
                    text: ""
                    draw_text: {
                        color: (THEME_COLOR_DIFF_CONTEXT_TEXT)
                        text_style: <THEME_FONT_CODE> { font_size: 10 }
                        wrap: Word
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct DiffView {
    #[deref]
    view: View,

    #[rust]
    expanded: bool,
    #[rust]
    diff_text_content: String,
    #[rust]
    summary_text: String,
}

impl Widget for DiffView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::MouseDown(mouse) = event {
            let header = self.view.view(&[id!(summary_header)]);
            let header_rect = header.area().rect(cx);
            if header_rect.contains(mouse.abs) {
                self.expanded = !self.expanded;
                self.view
                    .view(&[id!(diff_content)])
                    .set_visible(cx, self.expanded);
                self.redraw(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl DiffView {
    pub fn set_diffs(&mut self, cx: &mut Cx, diffs: &[openpad_protocol::FileDiff]) {
        if diffs.is_empty() {
            self.clear_diffs(cx);
            return;
        }

        let total_additions: i64 = diffs.iter().map(|d| d.additions).sum();
        let total_deletions: i64 = diffs.iter().map(|d| d.deletions).sum();
        let file_count = diffs.len();

        let summary = format!(
            "{} file{} changed  +{}  -{}",
            file_count,
            if file_count == 1 { "" } else { "s" },
            total_additions,
            total_deletions,
        );

        self.summary_text = summary.clone();
        let files_label = format!(
            "{} file{} changed",
            file_count,
            if file_count == 1 { "" } else { "s" }
        );
        self.view
            .label(&[id!(summary_files_label)])
            .set_text(cx, &files_label);
        self.view
            .label(&[id!(summary_add_label)])
            .set_text(cx, &format!("+{}", total_additions));
        self.view
            .label(&[id!(summary_del_label)])
            .set_text(cx, &format!("-{}", total_deletions));

        let mut full_diff = String::new();
        for diff in diffs {
            let header = format!(
                "── {} (+{} -{}) ──\n",
                diff.file, diff.additions, diff.deletions
            );
            full_diff.push_str(&header);

            let unified = compute_unified_diff(&diff.before, &diff.after, 3);
            full_diff.push_str(&unified);
            full_diff.push('\n');
        }

        self.diff_text_content = full_diff.clone();
        self.view.label(&[id!(diff_text)]).set_text(cx, &full_diff);

        self.expanded = false;
        self.view.view(&[id!(diff_content)]).set_visible(cx, false);
        self.view.set_visible(cx, true);
        self.redraw(cx);
    }

    pub fn set_expanded(&mut self, cx: &mut Cx, expanded: bool) {
        self.expanded = expanded;
        self.view.view(&[id!(diff_content)]).set_visible(cx, expanded);
        if expanded {
            self.view.set_visible(cx, true);
        }
        self.redraw(cx);
    }

    pub fn clear_diffs(&mut self, cx: &mut Cx) {
        self.expanded = false;
        self.diff_text_content.clear();
        self.summary_text.clear();
        self.view.label(&[id!(summary_files_label)]).set_text(cx, "");
        self.view.label(&[id!(summary_add_label)]).set_text(cx, "");
        self.view.label(&[id!(summary_del_label)]).set_text(cx, "");
        self.view.set_visible(cx, false);
        self.redraw(cx);
    }
}

pub trait DiffViewApi {
    fn set_diffs(&self, cx: &mut Cx, diffs: &[openpad_protocol::FileDiff]);
    fn clear_diffs(&self, cx: &mut Cx);
    fn set_expanded(&self, cx: &mut Cx, expanded: bool);
}

impl DiffViewApi for DiffViewRef {
    fn set_diffs(&self, cx: &mut Cx, diffs: &[openpad_protocol::FileDiff]) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_diffs(cx, diffs);
        }
    }

    fn clear_diffs(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.clear_diffs(cx);
        }
    }

    fn set_expanded(&self, cx: &mut Cx, expanded: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_expanded(cx, expanded);
        }
    }
}

/// Compute a unified diff between two strings with the given number of context lines.
/// Uses an iterative LCS approach safe for large files.
fn compute_unified_diff(before: &str, after: &str, context: usize) -> String {
    let old_lines: Vec<&str> = before.lines().collect();
    let new_lines: Vec<&str> = after.lines().collect();

    let diff_ops = compute_diff_ops(&old_lines, &new_lines);

    if diff_ops.is_empty() {
        return String::from(" (no changes)\n");
    }

    // Build output with context
    let mut output = String::new();
    let mut i = 0;
    let total = diff_ops.len();

    while i < total {
        // Find the start of a change hunk
        if matches!(diff_ops[i], DiffOp::Equal(_)) {
            i += 1;
            continue;
        }

        // Determine context start
        let context_start = if i >= context { i - context } else { 0 };

        // Find end of this hunk (including trailing context)
        let mut hunk_end = i;
        while hunk_end < total {
            if matches!(diff_ops[hunk_end], DiffOp::Equal(_)) {
                // Count consecutive equals
                let eq_start = hunk_end;
                while hunk_end < total && matches!(diff_ops[hunk_end], DiffOp::Equal(_)) {
                    hunk_end += 1;
                }
                let eq_count = hunk_end - eq_start;
                // If gap between changes is larger than 2*context, break
                if hunk_end < total && eq_count > context * 2 {
                    hunk_end = eq_start + context;
                    break;
                }
                if hunk_end >= total {
                    hunk_end = std::cmp::min(eq_start + context, total);
                    break;
                }
            } else {
                hunk_end += 1;
            }
        }

        // Print separator if not at the start
        if context_start > 0 {
            output.push_str("...\n");
        }

        for j in context_start..hunk_end {
            match &diff_ops[j] {
                DiffOp::Equal(line) => {
                    output.push(' ');
                    output.push_str(line);
                    output.push('\n');
                }
                DiffOp::Delete(line) => {
                    output.push('-');
                    output.push_str(line);
                    output.push('\n');
                }
                DiffOp::Insert(line) => {
                    output.push('+');
                    output.push_str(line);
                    output.push('\n');
                }
            }
        }

        i = hunk_end;
    }

    output
}

#[derive(Debug)]
enum DiffOp<'a> {
    Equal(&'a str),
    Delete(&'a str),
    Insert(&'a str),
}

/// Compute diff operations using iterative LCS (Myers-like approach via DP table).
/// For very large files, we fall back to a simpler line-by-line comparison.
fn compute_diff_ops<'a>(old: &[&'a str], new: &[&'a str]) -> Vec<DiffOp<'a>> {
    let old_len = old.len();
    let new_len = new.len();

    // For very large files, use a simpler approach to avoid memory issues
    if old_len * new_len > 4_000_000 {
        return simple_diff(old, new);
    }

    // Standard LCS DP table
    let mut dp = vec![vec![0u32; new_len + 1]; old_len + 1];
    for i in 1..=old_len {
        for j in 1..=new_len {
            if old[i - 1] == new[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }

    // Backtrack iteratively to build diff ops
    let mut ops = Vec::new();
    let mut i = old_len;
    let mut j = new_len;

    while i > 0 || j > 0 {
        if i > 0 && j > 0 && old[i - 1] == new[j - 1] {
            ops.push(DiffOp::Equal(old[i - 1]));
            i -= 1;
            j -= 1;
        } else if j > 0 && (i == 0 || dp[i][j - 1] >= dp[i - 1][j]) {
            ops.push(DiffOp::Insert(new[j - 1]));
            j -= 1;
        } else {
            ops.push(DiffOp::Delete(old[i - 1]));
            i -= 1;
        }
    }

    ops.reverse();
    ops
}

/// Simple diff for very large files: show all old lines as deleted, all new lines as inserted,
/// with common prefix/suffix preserved.
fn simple_diff<'a>(old: &[&'a str], new: &[&'a str]) -> Vec<DiffOp<'a>> {
    let mut ops = Vec::new();

    // Find common prefix
    let prefix_len = old
        .iter()
        .zip(new.iter())
        .take_while(|(a, b)| a == b)
        .count();

    // Find common suffix (not overlapping with prefix)
    let old_remaining = &old[prefix_len..];
    let new_remaining = &new[prefix_len..];
    let suffix_len = old_remaining
        .iter()
        .rev()
        .zip(new_remaining.iter().rev())
        .take_while(|(a, b)| a == b)
        .count();

    let old_mid = &old[prefix_len..old.len() - suffix_len];
    let new_mid = &new[prefix_len..new.len() - suffix_len];

    for line in &old[..prefix_len] {
        ops.push(DiffOp::Equal(line));
    }
    for line in old_mid {
        ops.push(DiffOp::Delete(line));
    }
    for line in new_mid {
        ops.push(DiffOp::Insert(line));
    }
    for line in &old[old.len() - suffix_len..] {
        ops.push(DiffOp::Equal(line));
    }

    ops
}
