use crate::components::diff_view::{DiffViewApi, DiffViewWidgetRefExt};
use crate::components::permission_card::PermissionCardWidgetRefExt;
use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_code_editor::code_view::CodeView;
    use openpad_widgets::openpad::*;
    use openpad_widgets::theme::*;
    use crate::components::user_bubble::UserBubble;
    use crate::components::assistant_bubble::AssistantBubble;
    use crate::components::diff_view::DiffView;
    use crate::components::permission_card::PermissionCard;

    pub MessageList = {{MessageList}} {
        width: Fill, height: Fill
        list = <PortalList> {
            scroll_bar: <ScrollBar> {}

            UserMsg = <View> {
                width: Fill, height: Fit
                flow: Down,
                padding: { top: 4, bottom: 4, left: 100, right: 24 }
                align: { x: 1.0 }

                <UserBubble> {
                    width: Fill, height: Fit
                    flow: Down,
                    align: { x: 1.0 }

                    // Metadata row
                    <View> {
                        width: Fit, height: Fit
                        flow: Right,
                        spacing: 8,
                        margin: { bottom: 4 }
                        align: { y: 0.5 }

                        timestamp_label = <Label> {
                            width: Fit, height: Fit
                            draw_text: {
                                color: (THEME_COLOR_TEXT_MUTED_DARKER),
                                text_style: <THEME_FONT_REGULAR> { font_size: 8 },
                            }
                            text: "..."
                        }

                        <Label> {
                            width: Fit, height: Fit
                            draw_text: {
                                color: (THEME_COLOR_TEXT_MUTED_DARK),
                                text_style: <THEME_FONT_BOLD> { font_size: 8 },
                            }
                            text: "YOU"
                        }

                    }

                    msg_text = <Label> {
                        width: Fill, height: Fit
                        draw_text: {
                            color: (THEME_COLOR_TEXT_LIGHT),
                            text_style: <THEME_FONT_REGULAR> { font_size: 10, line_spacing: 1.4 },
                            word: Wrap,
                        }
                    }

                    msg_actions = <View> {
                        width: Fit, height: Fit
                        flow: Right,
                        spacing: 6,
                        margin: { top: 6 }

                        copy_button = <Button> {
                            width: Fit, height: 20
                            text: "Copy"
                            draw_bg: {
                                color: (THEME_COLOR_TRANSPARENT)
                                color_hover: (THEME_COLOR_HOVER_MEDIUM)
                                // radius: 4.0
                                border_size: 0.0
                            }
                            draw_text: {
                                color: (THEME_COLOR_TEXT_MUTED_LIGHT)
                                color_hover: (THEME_COLOR_TEXT_MUTED_LIGHTER)
                                text_style: <THEME_FONT_REGULAR> { font_size: 8 }
                            }
                        }
                    }
                }
            }

            PermissionMsg = <PermissionCard> {}

            // Working indicator shown while agent is processing
            ThinkingMsg = <View> {
                width: Fill, height: Fit
                flow: Down,
                padding: { top: 8, bottom: 8, left: 24, right: 100 }

                <AssistantBubble> {
                    width: Fill, height: Fit
                    flow: Down,
                    padding: { top: 10, bottom: 10, left: 14, right: 14 }
                    draw_bg: {
                        color: (THEME_COLOR_BG_ASSISTANT_BUBBLE)
                        border_color: (THEME_COLOR_BORDER_MEDIUM)
                    }

                    // Header: "Working" label with animated dots and timer
                    <View> {
                        width: Fill, height: Fit
                        flow: Right,
                        spacing: 8,
                        margin: { bottom: 6 }
                        align: { y: 0.5 }

                        thinking_indicator = <View> {
                            width: Fit, height: Fit
                            flow: Right,
                            spacing: 4,
                            align: { y: 0.5 }

                            thinking_icon = <Label> {
                                width: Fit, height: Fit
                                draw_text: {
                                    color: (THEME_COLOR_TEXT_MUTED_LIGHT)
                                    text_style: <THEME_FONT_REGULAR> { font_size: 9 }
                                }
                                text: "◐"
                            }

                            thinking_label = <Label> {
                                width: Fit, height: Fit
                                draw_text: {
                                    color: (THEME_COLOR_TEXT_MUTED_LIGHT)
                                    text_style: <THEME_FONT_BOLD> { font_size: 9 }
                                }
                                text: "Working"
                            }
                        }

                        <View> { width: Fill }

                        thinking_timer = <Label> {
                            width: Fit, height: Fit
                            draw_text: {
                                color: (THEME_COLOR_TEXT_MUTED_DARKER)
                                text_style: <THEME_FONT_REGULAR> { font_size: 8 }
                            }
                            text: ""
                        }
                    }

                    // Current activity description
                    thinking_activity = <Label> {
                        width: Fill, height: Fit
                        draw_text: {
                            color: (THEME_COLOR_TEXT_MUTED_LIGHT)
                            text_style: <THEME_FONT_ITALIC> { font_size: 9, line_spacing: 1.3 }
                            word: Wrap
                        }
                        text: ""
                    }

                    // Active tool calls list
                    thinking_tools = <View> {
                        visible: false
                        width: Fill, height: Fit
                        flow: Down,
                        spacing: 3,
                        margin: { top: 8 }

                        tool_row_0 = <View> { visible: false, width: Fill, height: Fit, flow: Right, spacing: 6, align: { y: 0.5 }, tool_icon_0 = <Label> { width: Fit, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_DARKER), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }, text: "" }, tool_name_0 = <Label> { width: Fit, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_BOLD> { font_size: 9 } }, text: "" }, tool_input_0 = <Label> { width: Fill, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_DARKER), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }, text: "" } }
                        tool_row_1 = <View> { visible: false, width: Fill, height: Fit, flow: Right, spacing: 6, align: { y: 0.5 }, tool_icon_1 = <Label> { width: Fit, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_DARKER), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }, text: "" }, tool_name_1 = <Label> { width: Fit, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_BOLD> { font_size: 9 } }, text: "" }, tool_input_1 = <Label> { width: Fill, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_DARKER), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }, text: "" } }
                        tool_row_2 = <View> { visible: false, width: Fill, height: Fit, flow: Right, spacing: 6, align: { y: 0.5 }, tool_icon_2 = <Label> { width: Fit, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_DARKER), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }, text: "" }, tool_name_2 = <Label> { width: Fit, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_BOLD> { font_size: 9 } }, text: "" }, tool_input_2 = <Label> { width: Fill, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_DARKER), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }, text: "" } }
                        tool_row_3 = <View> { visible: false, width: Fill, height: Fit, flow: Right, spacing: 6, align: { y: 0.5 }, tool_icon_3 = <Label> { width: Fit, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_DARKER), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }, text: "" }, tool_name_3 = <Label> { width: Fit, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_BOLD> { font_size: 9 } }, text: "" }, tool_input_3 = <Label> { width: Fill, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_DARKER), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }, text: "" } }
                        tool_row_4 = <View> { visible: false, width: Fill, height: Fit, flow: Right, spacing: 6, align: { y: 0.5 }, tool_icon_4 = <Label> { width: Fit, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_DARKER), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }, text: "" }, tool_name_4 = <Label> { width: Fit, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_BOLD> { font_size: 9 } }, text: "" }, tool_input_4 = <Label> { width: Fill, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_DARKER), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }, text: "" } }
                    }
                }
            }

            AssistantMsg = <View> {
                width: Fill, height: Fit
                flow: Down,
                padding: { top: 4, bottom: 4, left: 24, right: 100 }

                <AssistantBubble> {
                    width: Fill, height: Fit
                    flow: Down,

                    // Metadata row (model, timestamp | copy icon, revert icon, steps)
                    <View> {
                        width: Fill, height: Fit
                        flow: Right,
                        spacing: 8,
                        margin: { bottom: 4 }
                        align: { y: 0.5 }

                        model_label = <Label> {
                            width: Fit, height: Fit
                            draw_text: {
                                color: (THEME_COLOR_TEXT_MUTED_DARK),
                                text_style: <THEME_FONT_BOLD> { font_size: 8 },
                            }
                            text: "ASSISTANT"
                        }

                        timestamp_label = <Label> {
                            width: Fit, height: Fit
                            draw_text: {
                                color: (THEME_COLOR_TEXT_MUTED_DARKER),
                                text_style: <THEME_FONT_REGULAR> { font_size: 8 },
                            }
                            text: "..."
                        }

                        <View> { width: Fill }

                        copy_action_button = <Button> {
                            width: Fit, height: 20
                            text: "Copy"
                            draw_bg: {
                                color: (THEME_COLOR_TRANSPARENT)
                                color_hover: (THEME_COLOR_HOVER_MEDIUM)
                                border_size: 0.0
                            }
                            draw_text: {
                                color: (THEME_COLOR_TEXT_PRIMARY)
                                color_hover: (THEME_COLOR_TEXT_BRIGHT)
                                text_style: <THEME_FONT_REGULAR> { font_size: 8 }
                            }
                        }

                        revert_action_button = <Button> {
                            width: Fit, height: 20
                            text: "Revert"
                            draw_bg: {
                                color: (THEME_COLOR_TRANSPARENT)
                                color_hover: (THEME_COLOR_HOVER_MEDIUM)
                                border_size: 0.0
                            }
                            draw_text: {
                                color: (THEME_COLOR_ACCENT_AMBER)
                                color_hover: (THEME_COLOR_TEXT_BRIGHT)
                                text_style: <THEME_FONT_REGULAR> { font_size: 8 }
                            }
                        }

                    }

                    steps_summary_row = <View> {
                        visible: false
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 6
                        margin: { top: 6, bottom: 4 }
                        align: { y: 0.5 }

                        steps_summary_label = <Label> {
                            width: Fill, height: Fit
                            draw_text: {
                                color: (THEME_COLOR_TEXT_MUTED_LIGHT)
                                text_style: <THEME_FONT_REGULAR> { font_size: 9 }
                            }
                            text: ""
                        }

                        steps_button = <Button> {
                            width: Fit, height: 20
                            draw_bg: {
                                color: (THEME_COLOR_TRANSPARENT)
                                color_hover: (THEME_COLOR_HOVER_MEDIUM)
                                border_radius: 4.0
                                border_size: 0.0
                            }
                            draw_text: {
                                color: (THEME_COLOR_TEXT_PRIMARY)
                                color_hover: (THEME_COLOR_TEXT_BRIGHT)
                                text_style: <THEME_FONT_REGULAR> { font_size: 8 }
                            }
                            text: "▸ Details"
                        }
                    }

                    steps_expanded = <View> {
                        visible: false
                        width: Fill, height: Fit
                        flow: Down,
                        margin: { top: 2, bottom: 4 }

                        steps_scroll = <ScrollYView> {
                            width: Fill, height: Fit
                            content = <View> {
                                width: Fill, height: Fit
                                flow: Down
                                spacing: 4

                                step_row_0 = <View> { width: Fill, height: Fit, flow: Down, spacing: 2, step_row_0_header_row = <View> { width: Fill, height: Fit, flow: Right, spacing: 6, align: { y: 0.0 }, step_row_0_rail = <View> { width: 10, height: Fill, flow: Down, align: { x: 0.5 }, step_row_0_dot = <View> { width: 6, height: 6, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_TEXT_MUTED_DARKER), border_radius: 3.0 } }, step_row_0_line = <View> { width: 2, height: Fill, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_BORDER_MEDIUM) } } }, step_row_0_header = <Button> { width: Fill, height: Fit, padding: { left: 4, right: 6, top: 2, bottom: 2 }, align: { x: 0.0 }, draw_bg: { color: (THEME_COLOR_TRANSPARENT), color_hover: (THEME_COLOR_HOVER_MEDIUM), border_radius: 4.0, border_size: 0.0 }, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }, text: "" } }, step_row_0_body = <View> { visible: true, width: Fill, height: Fit, flow: Down, padding: { left: 18, top: 2, bottom: 4 }, step_row_0_content = <Label> { width: Fill, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9, line_spacing: 1.3 }, word: Wrap }, text: "" } } }
                                step_row_1 = <View> { width: Fill, height: Fit, flow: Down, spacing: 2, step_row_1_header_row = <View> { width: Fill, height: Fit, flow: Right, spacing: 6, align: { y: 0.0 }, step_row_1_rail = <View> { width: 10, height: Fill, flow: Down, align: { x: 0.5 }, step_row_1_dot = <View> { width: 6, height: 6, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_TEXT_MUTED_DARKER), border_radius: 3.0 } }, step_row_1_line = <View> { width: 2, height: Fill, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_BORDER_MEDIUM) } } }, step_row_1_header = <Button> { width: Fill, height: Fit, padding: { left: 4, right: 6, top: 2, bottom: 2 }, align: { x: 0.0 }, draw_bg: { color: (THEME_COLOR_TRANSPARENT), color_hover: (THEME_COLOR_HOVER_MEDIUM), border_radius: 4.0, border_size: 0.0 }, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }, text: "" } }, step_row_1_body = <View> { visible: true, width: Fill, height: Fit, flow: Down, padding: { left: 18, top: 2, bottom: 4 }, step_row_1_content = <Label> { width: Fill, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9, line_spacing: 1.3 }, word: Wrap }, text: "" } } }
                                step_row_2 = <View> { width: Fill, height: Fit, flow: Down, spacing: 2, step_row_2_header_row = <View> { width: Fill, height: Fit, flow: Right, spacing: 6, align: { y: 0.0 }, step_row_2_rail = <View> { width: 10, height: Fill, flow: Down, align: { x: 0.5 }, step_row_2_dot = <View> { width: 6, height: 6, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_TEXT_MUTED_DARKER), border_radius: 3.0 } }, step_row_2_line = <View> { width: 2, height: Fill, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_BORDER_MEDIUM) } } }, step_row_2_header = <Button> { width: Fill, height: Fit, padding: { left: 4, right: 6, top: 2, bottom: 2 }, align: { x: 0.0 }, draw_bg: { color: (THEME_COLOR_TRANSPARENT), color_hover: (THEME_COLOR_HOVER_MEDIUM), border_radius: 4.0, border_size: 0.0 }, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }, text: "" } }, step_row_2_body = <View> { visible: true, width: Fill, height: Fit, flow: Down, padding: { left: 18, top: 2, bottom: 4 }, step_row_2_content = <Label> { width: Fill, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9, line_spacing: 1.3 }, word: Wrap }, text: "" } } }
                                step_row_3 = <View> { width: Fill, height: Fit, flow: Down, spacing: 2, step_row_3_header_row = <View> { width: Fill, height: Fit, flow: Right, spacing: 6, align: { y: 0.0 }, step_row_3_rail = <View> { width: 10, height: Fill, flow: Down, align: { x: 0.5 }, step_row_3_dot = <View> { width: 6, height: 6, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_TEXT_MUTED_DARKER), border_radius: 3.0 } }, step_row_3_line = <View> { width: 2, height: Fill, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_BORDER_MEDIUM) } } }, step_row_3_header = <Button> { width: Fill, height: Fit, padding: { left: 4, right: 6, top: 2, bottom: 2 }, align: { x: 0.0 }, draw_bg: { color: (THEME_COLOR_TRANSPARENT), color_hover: (THEME_COLOR_HOVER_MEDIUM), border_radius: 4.0, border_size: 0.0 }, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }, text: "" } }, step_row_3_body = <View> { visible: true, width: Fill, height: Fit, flow: Down, padding: { left: 18, top: 2, bottom: 4 }, step_row_3_content = <Label> { width: Fill, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9, line_spacing: 1.3 }, word: Wrap }, text: "" } } }
                                step_row_4 = <View> { width: Fill, height: Fit, flow: Down, spacing: 2, step_row_4_header_row = <View> { width: Fill, height: Fit, flow: Right, spacing: 6, align: { y: 0.0 }, step_row_4_rail = <View> { width: 10, height: Fill, flow: Down, align: { x: 0.5 }, step_row_4_dot = <View> { width: 6, height: 6, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_TEXT_MUTED_DARKER), border_radius: 3.0 } }, step_row_4_line = <View> { width: 2, height: Fill, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_BORDER_MEDIUM) } } }, step_row_4_header = <Button> { width: Fill, height: Fit, padding: { left: 4, right: 6, top: 2, bottom: 2 }, align: { x: 0.0 }, draw_bg: { color: (THEME_COLOR_TRANSPARENT), color_hover: (THEME_COLOR_HOVER_MEDIUM), border_radius: 4.0, border_size: 0.0 }, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }, text: "" } }, step_row_4_body = <View> { visible: true, width: Fill, height: Fit, flow: Down, padding: { left: 18, top: 2, bottom: 4 }, step_row_4_content = <Label> { width: Fill, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9, line_spacing: 1.3 }, word: Wrap }, text: "" } } }
                                step_row_5 = <View> { width: Fill, height: Fit, flow: Down, spacing: 2, step_row_5_header_row = <View> { width: Fill, height: Fit, flow: Right, spacing: 6, align: { y: 0.0 }, step_row_5_rail = <View> { width: 10, height: Fill, flow: Down, align: { x: 0.5 }, step_row_5_dot = <View> { width: 6, height: 6, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_TEXT_MUTED_DARKER), border_radius: 3.0 } }, step_row_5_line = <View> { width: 2, height: Fill, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_BORDER_MEDIUM) } } }, step_row_5_header = <Button> { width: Fill, height: Fit, padding: { left: 4, right: 6, top: 2, bottom: 2 }, align: { x: 0.0 }, draw_bg: { color: (THEME_COLOR_TRANSPARENT), color_hover: (THEME_COLOR_HOVER_MEDIUM), border_radius: 4.0, border_size: 0.0 }, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }, text: "" } }, step_row_5_body = <View> { visible: true, width: Fill, height: Fit, flow: Down, padding: { left: 18, top: 2, bottom: 4 }, step_row_5_content = <Label> { width: Fill, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9, line_spacing: 1.3 }, word: Wrap }, text: "" } } }
                                step_row_6 = <View> { width: Fill, height: Fit, flow: Down, spacing: 2, step_row_6_header_row = <View> { width: Fill, height: Fit, flow: Right, spacing: 6, align: { y: 0.0 }, step_row_6_rail = <View> { width: 10, height: Fill, flow: Down, align: { x: 0.5 }, step_row_6_dot = <View> { width: 6, height: 6, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_TEXT_MUTED_DARKER), border_radius: 3.0 } }, step_row_6_line = <View> { width: 2, height: Fill, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_BORDER_MEDIUM) } } }, step_row_6_header = <Button> { width: Fill, height: Fit, padding: { left: 4, right: 6, top: 2, bottom: 2 }, align: { x: 0.0 }, draw_bg: { color: (THEME_COLOR_TRANSPARENT), color_hover: (THEME_COLOR_HOVER_MEDIUM), border_radius: 4.0, border_size: 0.0 }, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }, text: "" } }, step_row_6_body = <View> { visible: true, width: Fill, height: Fit, flow: Down, padding: { left: 18, top: 2, bottom: 4 }, step_row_6_content = <Label> { width: Fill, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9, line_spacing: 1.3 }, word: Wrap }, text: "" } } }
                                step_row_7 = <View> { width: Fill, height: Fit, flow: Down, spacing: 2, step_row_7_header_row = <View> { width: Fill, height: Fit, flow: Right, spacing: 6, align: { y: 0.0 }, step_row_7_rail = <View> { width: 10, height: Fill, flow: Down, align: { x: 0.5 }, step_row_7_dot = <View> { width: 6, height: 6, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_TEXT_MUTED_DARKER), border_radius: 3.0 } }, step_row_7_line = <View> { width: 2, height: Fill, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_BORDER_MEDIUM) } } }, step_row_7_header = <Button> { width: Fill, height: Fit, padding: { left: 4, right: 6, top: 2, bottom: 2 }, align: { x: 0.0 }, draw_bg: { color: (THEME_COLOR_TRANSPARENT), color_hover: (THEME_COLOR_HOVER_MEDIUM), border_radius: 4.0, border_size: 0.0 }, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }, text: "" } }, step_row_7_body = <View> { visible: true, width: Fill, height: Fit, flow: Down, padding: { left: 18, top: 2, bottom: 4 }, step_row_7_content = <Label> { width: Fill, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9, line_spacing: 1.3 }, word: Wrap }, text: "" } } }
                                step_row_8 = <View> { width: Fill, height: Fit, flow: Down, spacing: 2, step_row_8_header_row = <View> { width: Fill, height: Fit, flow: Right, spacing: 6, align: { y: 0.0 }, step_row_8_rail = <View> { width: 10, height: Fill, flow: Down, align: { x: 0.5 }, step_row_8_dot = <View> { width: 6, height: 6, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_TEXT_MUTED_DARKER), border_radius: 3.0 } }, step_row_8_line = <View> { width: 2, height: Fill, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_BORDER_MEDIUM) } } }, step_row_8_header = <Button> { width: Fill, height: Fit, padding: { left: 4, right: 6, top: 2, bottom: 2 }, align: { x: 0.0 }, draw_bg: { color: (THEME_COLOR_TRANSPARENT), color_hover: (THEME_COLOR_HOVER_MEDIUM), border_radius: 4.0, border_size: 0.0 }, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }, text: "" } }, step_row_8_body = <View> { visible: true, width: Fill, height: Fit, flow: Down, padding: { left: 18, top: 2, bottom: 4 }, step_row_8_content = <Label> { width: Fill, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9, line_spacing: 1.3 }, word: Wrap }, text: "" } } }
                                step_row_9 = <View> { width: Fill, height: Fit, flow: Down, spacing: 2, step_row_9_header_row = <View> { width: Fill, height: Fit, flow: Right, spacing: 6, align: { y: 0.0 }, step_row_9_rail = <View> { width: 10, height: Fill, flow: Down, align: { x: 0.5 }, step_row_9_dot = <View> { width: 6, height: 6, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_TEXT_MUTED_DARKER), border_radius: 3.0 } }, step_row_9_line = <View> { width: 2, height: Fill, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_BORDER_MEDIUM) } } }, step_row_9_header = <Button> { width: Fill, height: Fit, padding: { left: 4, right: 6, top: 2, bottom: 2 }, align: { x: 0.0 }, draw_bg: { color: (THEME_COLOR_TRANSPARENT), color_hover: (THEME_COLOR_HOVER_MEDIUM), border_radius: 4.0, border_size: 0.0 }, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }, text: "" } }, step_row_9_body = <View> { visible: true, width: Fill, height: Fit, flow: Down, padding: { left: 18, top: 2, bottom: 4 }, step_row_9_content = <Label> { width: Fill, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9, line_spacing: 1.3 }, word: Wrap }, text: "" } } }
                            }
                        }
                    }

                    markdown_view = <View> {
                        visible: false
                        width: Fill, height: Fit
                        msg_text = <Markdown> {
                            width: Fill, height: Fit
                            font_size: 10
                            font_color: (THEME_COLOR_TEXT_NORMAL)
                            paragraph_spacing: 8
                            pre_code_spacing: 6
                            use_code_block_widget: true

                            code_block = <RoundedView> {
                                width: Fill, height: Fit
                                flow: Down
                                padding: { left: 8, right: 8, top: 6, bottom: 6 }
                                margin: { top: 4, bottom: 4 }
                                draw_bg: {
                                    color: (THEME_COLOR_BG_INPUT)
                                    border_radius: 6.0
                                }

                                code_view = <CodeView> {
                                    editor: {
                                        width: Fill
                                        height: Fit
                                        draw_bg: { color: (THEME_COLOR_BG_INPUT) }
                                        token_colors: {
                                            unknown: (THEME_COLOR_TEXT_NORMAL)
                                            branch_keyword: (THEME_COLOR_ACCENT_PURPLE)
                                            comment: (THEME_COLOR_TEXT_MUTED_LIGHT)
                                            constant: (THEME_COLOR_ACCENT_AMBER)
                                            delimiter: (THEME_COLOR_TEXT_MUTED_LIGHTER)
                                            delimiter_highlight: (THEME_COLOR_TEXT_BRIGHT)
                                            identifier: (THEME_COLOR_TEXT_NORMAL)
                                            loop_keyword: (THEME_COLOR_ACCENT_PURPLE)
                                            number: (THEME_COLOR_ACCENT_AMBER)
                                            other_keyword: (THEME_COLOR_ACCENT_BLUE)
                                            function: (THEME_COLOR_ACCENT_BLUE)
                                            punctuator: (THEME_COLOR_TEXT_MUTED_LIGHTER)
                                            string: (THEME_COLOR_TEXT_CODE)
                                            typename: (THEME_COLOR_TEXT_BOLD)
                                            whitespace: (THEME_COLOR_TEXT_NORMAL)
                                            error_decoration: (THEME_COLOR_ACCENT_RED)
                                            warning_decoration: (THEME_COLOR_ACCENT_AMBER)
                                        }
                                    }
                                }
                            }

                            draw_normal: {
                                text_style: <THEME_FONT_REGULAR> { font_size: 10, line_spacing: 1.4 }
                                color: (THEME_COLOR_TEXT_NORMAL)
                            }
                            draw_italic: {
                                text_style: <THEME_FONT_ITALIC> { font_size: 10 }
                                color: (THEME_COLOR_TEXT_NORMAL)
                            }
                            draw_bold: {
                                text_style: <THEME_FONT_BOLD> { font_size: 10 }
                                color: (THEME_COLOR_TEXT_BOLD)
                            }
                            draw_fixed: {
                                text_style: <THEME_FONT_CODE> { font_size: 9 }
                                color: (THEME_COLOR_TEXT_CODE)
                            }
                        }
                    }

                    label_view = <View> {
                        visible: false
                        width: Fill, height: Fit
                        msg_label = <Label> {
                            width: Fill, height: Fit
                            draw_text: {
                                color: (THEME_COLOR_TEXT_NORMAL)
                                text_style: <THEME_FONT_REGULAR> { font_size: 10, line_spacing: 1.4 }
                                word: Wrap
                            }
                        }
                    }

                    error_label = <Label> {
                        width: Fill, height: Fit
                        text: ""
                        draw_text: {
                            color: (THEME_COLOR_ACCENT_RED)
                            text_style: <THEME_FONT_REGULAR> { font_size: 9, line_spacing: 1.4 }
                            wrap: Word
                        }
                    }

                    diff_view = <DiffView> {}

                    stats_row = <View> {
                        width: Fit, height: Fit
                        flow: Right,
                        spacing: 8,
                        margin: { top: 6 }
                        align: { y: 0.5 }

                        tokens_label = <Label> {
                            width: Fit, height: Fit
                            draw_text: {
                                color: (THEME_COLOR_TEXT_MUTED_LIGHT)
                                text_style: <THEME_FONT_REGULAR> { font_size: 8 }
                            }
                            text: ""
                        }

                        cost_label = <Label> {
                            width: Fit, height: Fit
                            draw_text: {
                                color: (THEME_COLOR_TEXT_MUTED_LIGHT)
                                text_style: <THEME_FONT_REGULAR> { font_size: 8 }
                            }
                            text: ""
                        }
                    }

                    msg_actions = <View> {
                        width: Fit, height: Fit
                        flow: Right,
                        spacing: 6,
                        margin: { top: 8 }

                        copy_button = <Button> {
                            width: Fit, height: 20
                            text: "Copy"
                            draw_bg: {
                                color: (THEME_COLOR_TRANSPARENT)
                                color_hover: (THEME_COLOR_HOVER_MEDIUM)
                                // radius: 4.0
                                border_size: 0.0
                            }
                            draw_text: {
                                color: (THEME_COLOR_TEXT_MUTED_LIGHT)
                                color_hover: (THEME_COLOR_TEXT_MUTED_LIGHTER)
                                text_style: <THEME_FONT_REGULAR> { font_size: 8 }
                            }
                        }

                        revert_button = <Button> {
                             width: Fit, height: 20
                             text: "Revert"
                             draw_bg: {
                                 color: (THEME_COLOR_TRANSPARENT)
                                 color_hover: (THEME_COLOR_HOVER_MEDIUM)
                                 // radius: 4.0
                                 border_size: 0.0
                             }
                             draw_text: {
                                 color: (THEME_COLOR_TEXT_MUTED_LIGHT)
                                 color_hover: (THEME_COLOR_TEXT_MUTED_LIGHTER)
                                 text_style: <THEME_FONT_REGULAR> { font_size: 8 }
                             }
                         }
                     }
                }
            }
        }
    }
}

/// One tool call within a step (tool name, input summary, output or error).
#[derive(Clone, Debug)]
pub struct StepDetail {
    pub tool: String,
    pub input_summary: String,
    pub result: String,
    /// Whether this tool call is currently running
    pub is_running: bool,
}

/// Per-step info shown under an assistant message (from step-start / step-finish / tool parts).
#[derive(Clone, Debug)]
pub struct DisplayStep {
    pub reason: String,
    pub cost: f64,
    pub tokens: Option<openpad_protocol::TokenUsage>,
    /// Tool calls and their results within this step.
    pub details: Vec<StepDetail>,
    /// Whether this step's details are expanded (collapsible per step).
    pub expanded: bool,
    /// Whether any tool in this step had an error.
    pub has_error: bool,
    /// Whether any tool in this step is currently running.
    pub has_running: bool,
}

#[derive(Clone, Debug)]
pub struct DisplayMessage {
    pub role: String,
    pub text: String,
    pub message_id: Option<String>,
    pub timestamp: Option<i64>,   // Unix timestamp in milliseconds
    pub model_id: Option<String>, // Model ID (for assistant messages)
    pub tokens: Option<openpad_protocol::TokenUsage>,
    pub cost: Option<f64>,
    pub error_text: Option<String>,
    pub is_error: bool,
    pub diffs: Vec<openpad_protocol::FileDiff>,
    pub show_diffs: bool,
    /// Steps (step-start / step-finish) for assistant messages.
    pub steps: Vec<DisplayStep>,
    /// Whether the steps block is expanded (collapsible control).
    pub show_steps: bool,
    /// Duration in ms (completed - created) for "2m, 18s" in steps header.
    pub duration_ms: Option<i64>,
}

#[derive(Clone, Debug)]
pub struct PendingPermissionDisplay {
    pub session_id: String,
    pub request_id: String,
    pub permission: String,
    pub patterns: Vec<String>,
}

#[derive(Live, LiveHook, Widget)]
pub struct MessageList {
    #[deref]
    view: View,
    #[rust]
    messages: Vec<DisplayMessage>,
    #[rust]
    is_working: bool,
    #[rust]
    revert_message_id: Option<String>,
    #[rust]
    pending_permissions: Vec<PendingPermissionDisplay>,
    #[rust]
    working_since: Option<std::time::Instant>,
    #[rust]
    thinking_frame: usize,
}

impl MessageList {
    /// Get the animated thinking icon based on current frame
    fn thinking_icon(&self) -> &'static str {
        // Cycle through unicode spinner characters: ◐◑◒◓◔◕
        match self.thinking_frame {
            0 => "◐",
            1 => "◑",
            2 => "◒",
            3 => "◓",
            4 => "◔",
            _ => "◕",
        }
    }

    fn steps_button_label(msg: &DisplayMessage) -> String {
        if msg.show_steps {
            "▾ Details".to_string()
        } else {
            "▸ Details".to_string()
        }
    }

    fn steps_summary_text(msg: &DisplayMessage) -> String {
        if msg.steps.is_empty() {
            return String::new();
        }
        let has_running = msg.steps.iter().any(|s| s.has_running);
        let mut labels: Vec<String> = Vec::new();
        for step in msg.steps.iter() {
            let desc = Self::get_step_description(step);
            if !desc.is_empty() {
                labels.push(desc);
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
            .map(crate::ui::formatters::format_duration_ms);
        let prefix = if has_running { "Running" } else { "Steps" };
        if let Some(d) = duration {
            format!("{}: {} • {} • {}", prefix, count, summary, d)
        } else {
            format!("{}: {} • {}", prefix, count, summary)
        }
    }

    /// Aggregated step summary for expanded view (OpenCode-style: one line by reason).
    fn format_steps_aggregated(steps: &[DisplayStep]) -> String {
        if steps.is_empty() {
            return String::new();
        }
        use std::collections::HashMap;
        let mut by_reason: HashMap<String, usize> = HashMap::new();
        for s in steps {
            let key = if s.reason.is_empty() {
                "step".to_string()
            } else {
                s.reason.to_string()
            };
            *by_reason.entry(key).or_insert(0) += 1;
        }
        let n = steps.len();
        let mut parts: Vec<String> = by_reason
            .into_iter()
            .map(|(reason, count)| {
                if count == 1 {
                    reason
                } else {
                    format!("{}× {}", count, reason)
                }
            })
            .collect();
        parts.sort();
        let summary = parts.join(", ");
        if n == 1 {
            format!("1 step: {}", summary)
        } else {
            format!("{} steps: {}", n, summary)
        }
    }

    /// Max steps to show in detailed list before truncating.
    const MAX_STEPS_DETAILED: usize = 10;

    /// Max step rows (inline, no PortalList) for collapsible steps inside the bubble.
    const MAX_STEP_ROWS: usize = 10;

    /// Per-step list for expanded view: "Step N: reason" plus tool details (tool, input → result).
    fn format_steps_detailed(steps: &[DisplayStep]) -> String {
        if steps.is_empty() {
            return String::new();
        }
        let rest = steps.len().saturating_sub(Self::MAX_STEPS_DETAILED);
        let show: &[DisplayStep] = if rest > 0 {
            &steps[..steps.len() - rest]
        } else {
            steps
        };
        let mut lines: Vec<String> = Vec::new();
        for (i, s) in show.iter().enumerate() {
            let n = i + 1;
            let header = if s.reason.is_empty() {
                format!("Step {}", n)
            } else {
                format!("Step {}: {}", n, s.reason)
            };
            lines.push(header);
            for d in &s.details {
                let detail_line = if d.input_summary.is_empty() {
                    format!("  • {} → {}", d.tool, d.result)
                } else {
                    format!("  • {} {} → {}", d.tool, d.input_summary, d.result)
                };
                lines.push(detail_line);
            }
        }
        let mut out = lines.join("\n");
        if rest > 0 {
            out.push_str(&format!("\n… {} more steps", rest));
        }
        out
    }

    /// Get a human-readable description of what the step is doing based on tool calls.
    fn get_step_description(step: &DisplayStep) -> String {
        let running_prefix = if step.has_running { "⏳ " } else { "" };

        if step.details.is_empty() {
            return if step.reason.is_empty() {
                format!("{}Working...", running_prefix)
            } else {
                format!("{}{}", running_prefix, step.reason)
            };
        }

        // Analyze the tools in this step to generate a descriptive summary
        let tool_names: Vec<&str> = step.details.iter().map(|d| d.tool.as_str()).collect();

        // Check for specific patterns
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
                // Single read operation - show what file
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
            // Single tool with no special pattern
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

    /// Extract file path from input summary if present.
    fn extract_path(input: &str) -> Option<String> {
        // Look for path= pattern
        if let Some(start) = input.find("path=") {
            let rest = &input[start + 5..];
            // Find the end of the path value
            let end = rest.find(' ').unwrap_or(rest.len());
            let path = &rest[..end];
            if !path.is_empty() {
                return Some(path.to_string());
            }
        }
        None
    }

    /// Format a file path for display - show just the filename unless needed for context.
    fn format_path(path: &str) -> String {
        // If it's a long path, just show the last component
        if path.len() > 40 {
            if let Some(filename) = path.split('/').last().or_else(|| path.split('\\').last()) {
                return format!(".../{}", filename);
            }
        }
        path.to_string()
    }

    /// Format tool name to be more readable.
    fn format_tool_name(tool: &str) -> String {
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

    /// Get icon/indicator for a tool type.
    fn get_tool_icon(tool: &str) -> &'static str {
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

    /// Format tool input to be more concise and readable.
    fn format_tool_input(input: &str) -> String {
        if input.is_empty() {
            return String::new();
        }

        // Parse key=value pairs and format them nicely
        let mut formatted_parts = Vec::new();

        // Extract path
        if let Some(path) = Self::extract_path(input) {
            formatted_parts.push(Self::format_path(&path));
        }

        // Extract other interesting parameters
        for (key, label) in [("offset", "@"), ("limit", "limit"), ("command", "cmd")].iter() {
            if let Some(start) = input.find(&format!("{}=", key)) {
                let rest = &input[start + key.len() + 1..];
                let end = rest.find(' ').unwrap_or(rest.len());
                let value = &rest[..end];
                if !value.is_empty() && value.len() < 50 {
                    if key == &"offset" {
                        formatted_parts.push(format!("@ {}", value));
                    } else if key == &"limit" {
                        // Don't show limit if it's just the default
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
            // Fallback: truncate the raw input
            if input.len() > 50 {
                format!("{}...", &input[..47])
            } else {
                input.to_string()
            }
        } else {
            formatted_parts.join(" ")
        }
    }

    /// Format a single step's body (tool details, optional cost/tokens).
    fn format_step_body(step: &DisplayStep) -> String {
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
                // For running tools, show a progress indicator instead of result
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
                stats.push(crate::ui::formatters::format_cost(step.cost));
            }
            if let Some(ref t) = step.tokens {
                stats.push(crate::ui::formatters::format_token_usage_short(t));
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

    fn rebuild_from_parts(
        messages_with_parts: &[openpad_protocol::MessageWithParts],
    ) -> Vec<DisplayMessage> {
        let mut display = Vec::new();
        let mut pending_diffs: Option<Vec<openpad_protocol::FileDiff>> = None;
        let mut pending_steps_only: Option<DisplayMessage> = None;
        for mwp in messages_with_parts {
            let (role, timestamp, model_id, tokens, cost, error_text, is_error, duration_ms) =
                match &mwp.info {
                    openpad_protocol::Message::User(msg) => (
                        "user",
                        Some(msg.time.created),
                        None,
                        None,
                        None,
                        None,
                        false,
                        None,
                    ),
                    openpad_protocol::Message::Assistant(msg) => {
                        let model = if !msg.model_id.is_empty() {
                            Some(msg.model_id.clone())
                        } else {
                            None
                        };
                        let error_text = msg
                            .error
                            .as_ref()
                            .map(crate::ui::formatters::format_assistant_error);
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
                } else if matches!(p, openpad_protocol::Part::StepStart { .. }) {
                    steps.push(DisplayStep {
                        reason: String::new(),
                        cost: 0.,
                        tokens: None,
                        details: Vec::new(),
                        expanded: false,
                        has_error: false,
                        has_running: false,
                    });
                } else if let Some((tool, input_summary, result)) = p.tool_display() {
                    let has_error = result.starts_with("Error");
                    let is_running = result == "(running)" || result == "(pending)";
                    let detail = StepDetail {
                        tool,
                        input_summary,
                        result: result.clone(),
                        is_running,
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
                        });
                    }
                } else if let Some((reason, cost, tokens)) = p.step_finish_info() {
                    if let Some(last) = steps.last_mut() {
                        last.reason = reason.to_string();
                        last.cost = cost;
                        last.tokens = tokens.cloned();
                        // Step is finished, no longer running
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
                openpad_protocol::Message::User(msg) => {
                    if let Some(summary) = &msg.summary {
                        if !summary.diffs.is_empty() {
                            pending_diffs = Some(summary.diffs.clone());
                        }
                    }
                }
                openpad_protocol::Message::Assistant(_) => {
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
                    // Prefer original (first) duration when merging steps-only messages
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
                        show_steps: false,
                        duration_ms,
                    });
                }
                continue;
            }

            if role == "assistant" && !text.is_empty() {
                if let Some(pending) = pending_steps_only.take() {
                    let mut merged_steps = pending.steps;
                    merged_steps.extend(steps);
                    let merged_duration = duration_ms.or(pending.duration_ms);
                    display.push(DisplayMessage {
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
                    });
                    continue;
                }
            }

            if let Some(prev) = pending_steps_only.take() {
                display.push(prev);
            }

            // Show steps by default if this is an assistant message with steps but no text yet
            // (i.e., still processing/streaming). Once text arrives, hide steps automatically.
            let show_steps = role == "assistant" && text.is_empty() && !steps.is_empty();

            display.push(DisplayMessage {
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
            });
        }
        if let Some(prev) = pending_steps_only.take() {
            display.push(prev);
        }
        display
    }
}

impl Widget for MessageList {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        use crate::state::actions::MessageListAction;

        if self.is_working {
            if let Event::NextFrame(_) = event {
                // Animate thinking indicator (cycle through ◐◑◒◓◔◕)
                self.thinking_frame = (self.thinking_frame + 1) % 6;
                // Only redraw on next frame if we are actually working/streaming
                // We rely on append_text_for_message to trigger redraws when content updates
                // But we still need this for the "working..." timer update
                self.redraw(cx);
            }
            // Throttle timer updates to 100ms instead of every frame to save CPU
            // The timer only updates seconds anyway
            cx.new_next_frame();
        }

        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        let list = self.view.portal_list(&[id!(list)]);
        for (item_id, widget) in list.items_with_actions(&actions) {
            if item_id >= self.messages.len() {
                continue;
            }

            if widget.button(&[id!(copy_action_button)]).clicked(&actions)
                || widget.button(&[id!(copy_button)]).clicked(&actions)
            {
                cx.copy_to_clipboard(&self.messages[item_id].text);
            }

            if widget
                .button(&[id!(revert_action_button)])
                .clicked(&actions)
                || widget.button(&[id!(revert_button)]).clicked(&actions)
            {
                if let Some(message_id) = &self.messages[item_id].message_id {
                    cx.action(MessageListAction::RevertToMessage(message_id.clone()));
                }
            }

            if widget.button(&[id!(steps_button)]).clicked(&actions) {
                if let Some(message) = self.messages.get_mut(item_id) {
                    if !message.steps.is_empty() {
                        message.show_steps = !message.show_steps;
                        self.redraw(cx);
                    }
                }
            }

            // Handle diff view toggle via summary_header click
            if item_id < self.messages.len() {
                let msg = &self.messages[item_id];
                if !msg.diffs.is_empty() {
                    if widget
                        .diff_view(&[id!(diff_view)])
                        .summary_header_clicked(cx)
                    {
                        if let Some(message) = self.messages.get_mut(item_id) {
                            message.show_diffs = !message.show_diffs;
                            self.redraw(cx);
                        }
                    }
                }
            }

            // Per-step collapse/expand (inline step rows)
            if item_id < self.messages.len() {
                let msg = &self.messages[item_id];
                if msg.role == "assistant" && msg.show_steps && !msg.steps.is_empty() {
                    let steps_base =
                        widget.view(&[id!(steps_expanded), id!(steps_scroll), id!(content)]);
                    for step_id in 0..MessageList::MAX_STEP_ROWS.min(msg.steps.len()) {
                        let (row_id, header_id) = match step_id {
                            0 => (live_id!(step_row_0), live_id!(step_row_0_header)),
                            1 => (live_id!(step_row_1), live_id!(step_row_1_header)),
                            2 => (live_id!(step_row_2), live_id!(step_row_2_header)),
                            3 => (live_id!(step_row_3), live_id!(step_row_3_header)),
                            4 => (live_id!(step_row_4), live_id!(step_row_4_header)),
                            5 => (live_id!(step_row_5), live_id!(step_row_5_header)),
                            6 => (live_id!(step_row_6), live_id!(step_row_6_header)),
                            7 => (live_id!(step_row_7), live_id!(step_row_7_header)),
                            8 => (live_id!(step_row_8), live_id!(step_row_8_header)),
                            9 => (live_id!(step_row_9), live_id!(step_row_9_header)),
                            _ => continue,
                        };
                        if steps_base
                            .view(&[row_id])
                            .button(&[header_id])
                            .clicked(&actions)
                        {
                            if let Some(step) = self.messages[item_id].steps.get_mut(step_id) {
                                step.expanded = !step.expanded;
                                self.redraw(cx);
                            }
                            break;
                        }
                    }
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                let total_items = self.messages.len()
                    + self.pending_permissions.len()
                    + if self.is_working { 1 } else { 0 };
                if total_items == 0 {
                    list.set_item_range(cx, 0, 0);
                    continue;
                }

                list.set_item_range(cx, 0, total_items);

                while let Some(item_id) = list.next_visible_item(cx) {
                    // After messages, render pending permissions
                    if item_id >= self.messages.len()
                        && item_id < self.messages.len() + self.pending_permissions.len()
                    {
                        let perm_idx = item_id - self.messages.len();
                        let perm = &self.pending_permissions[perm_idx];
                        let item_widget = list.item(cx, item_id, live_id!(PermissionMsg));
                        use crate::components::permission_card::PermissionCardApi;
                        item_widget.permission_card(&[]).set_permission(
                            cx,
                            perm.session_id.clone(),
                            perm.request_id.clone(),
                            &perm.permission,
                            &perm.patterns,
                        );
                        item_widget.draw_all(cx, scope);
                        continue;
                    }

                    if item_id >= self.messages.len() + self.pending_permissions.len() {
                        if !self.is_working {
                            continue;
                        }
                        // If the most recent assistant message already has running steps,
                        // show progress in that message instead of a separate working card.
                        let last_assistant_has_running = self
                            .messages
                            .iter()
                            .rfind(|m| m.role == "assistant")
                            .map(|m| m.steps.iter().any(|s| s.has_running))
                            .unwrap_or(false);
                        if last_assistant_has_running {
                            continue;
                        }
                        let elapsed = self
                            .working_since
                            .map(|t| t.elapsed().as_secs())
                            .unwrap_or(0);
                        let mins = elapsed / 60;
                        let secs = elapsed % 60;

                        // Get current activity and running tools from the last message
                        let (current_activity, running_tools) =
                            if let Some(msg) = self.messages.last() {
                                if let Some(last_step) = msg.steps.last() {
                                    // Collect running tools from the last step
                                    let tools: Vec<(String, String, String)> = last_step
                                        .details
                                        .iter()
                                        .filter(|d| d.is_running)
                                        .map(|d| {
                                            let icon = Self::get_tool_icon(&d.tool);
                                            let name = Self::format_tool_name(&d.tool);
                                            let input = Self::format_tool_input(&d.input_summary);
                                            (icon.to_string(), name, input)
                                        })
                                        .collect();

                                    let activity = if !tools.is_empty() {
                                        let names: Vec<String> =
                                            tools.iter().map(|t| t.1.clone()).take(3).collect();
                                        if names.is_empty() {
                                            "Working...".to_string()
                                        } else {
                                            format!("Running: {}", names.join(", "))
                                        }
                                    } else {
                                        let desc = Self::get_step_description(last_step);
                                        if desc.is_empty() {
                                            "Working...".to_string()
                                        } else {
                                            format!("Working on: {}", desc)
                                        }
                                    };
                                    (activity, tools)
                                } else if !msg.steps.is_empty() {
                                    ("Working...".to_string(), Vec::new())
                                } else {
                                    ("Working...".to_string(), Vec::new())
                                }
                            } else {
                                ("Working...".to_string(), Vec::new())
                            };

                        // Format timer text
                        let timer_text = if elapsed > 0 {
                            format!("· {}m, {}s", mins, secs)
                        } else {
                            String::new()
                        };

                        // Use the dedicated Thinking template
                        let item_widget = list.item(cx, item_id, live_id!(ThinkingMsg));

                        // Set thinking label with timer and animated icon
                        item_widget
                            .label(&[id!(thinking_label)])
                            .set_text(cx, "Working");
                        item_widget
                            .label(&[id!(thinking_icon)])
                            .set_text(cx, self.thinking_icon());
                        item_widget
                            .label(&[id!(thinking_timer)])
                            .set_text(cx, &timer_text);

                        // Set current activity description
                        item_widget
                            .label(&[id!(thinking_activity)])
                            .set_text(cx, &current_activity);

                        // Show running tools if any
                        let has_tools = !running_tools.is_empty();
                        item_widget
                            .view(&[id!(thinking_tools)])
                            .set_visible(cx, has_tools);
                        if has_tools {
                            // Update each tool row (up to 5)
                            for (idx, (icon, name, input)) in
                                running_tools.iter().take(5).enumerate()
                            {
                                let (row_id, icon_id, name_id, input_id) = match idx {
                                    0 => (
                                        live_id!(tool_row_0),
                                        live_id!(tool_icon_0),
                                        live_id!(tool_name_0),
                                        live_id!(tool_input_0),
                                    ),
                                    1 => (
                                        live_id!(tool_row_1),
                                        live_id!(tool_icon_1),
                                        live_id!(tool_name_1),
                                        live_id!(tool_input_1),
                                    ),
                                    2 => (
                                        live_id!(tool_row_2),
                                        live_id!(tool_icon_2),
                                        live_id!(tool_name_2),
                                        live_id!(tool_input_2),
                                    ),
                                    3 => (
                                        live_id!(tool_row_3),
                                        live_id!(tool_icon_3),
                                        live_id!(tool_name_3),
                                        live_id!(tool_input_3),
                                    ),
                                    4 => (
                                        live_id!(tool_row_4),
                                        live_id!(tool_icon_4),
                                        live_id!(tool_name_4),
                                        live_id!(tool_input_4),
                                    ),
                                    _ => continue,
                                };
                                let tools_view = item_widget.view(&[id!(thinking_tools)]);
                                tools_view.view(&[row_id]).set_visible(cx, true);
                                tools_view.label(&[icon_id]).set_text(cx, icon);
                                tools_view.label(&[name_id]).set_text(cx, name);
                                tools_view.label(&[input_id]).set_text(cx, input);
                            }
                            // Hide unused tool rows
                            for idx in running_tools.len()..5 {
                                let row_id = match idx {
                                    0 => live_id!(tool_row_0),
                                    1 => live_id!(tool_row_1),
                                    2 => live_id!(tool_row_2),
                                    3 => live_id!(tool_row_3),
                                    4 => live_id!(tool_row_4),
                                    _ => continue,
                                };
                                item_widget
                                    .view(&[id!(thinking_tools)])
                                    .view(&[row_id])
                                    .set_visible(cx, false);
                            }
                        }

                        item_widget.draw_all(cx, scope);
                    } else {
                        let msg = &self.messages[item_id];
                        let template = if msg.role == "user" {
                            live_id!(UserMsg)
                        } else {
                            live_id!(AssistantMsg)
                        };

                        let item_widget = list.item(cx, item_id, template);

                        if msg.role == "user" {
                            item_widget.widget(&[id!(msg_text)]).set_text(cx, &msg.text);
                        } else {
                            // HEURISTIC: content triggers markdown if it has backticks, hash headers, or > quotes
                            // This is a simple check to avoid markdown widget cost for plain text
                            let needs_markdown = msg.text.contains("```")
                                || msg.text.contains("`")
                                || msg.text.contains("# ")
                                || msg.text.contains("> ");

                            if needs_markdown {
                                item_widget.view(&[id!(label_view)]).set_visible(cx, false);
                                item_widget
                                    .view(&[id!(markdown_view)])
                                    .set_visible(cx, true);
                                item_widget.widget(&[id!(msg_text)]).set_text(cx, &msg.text);
                            } else {
                                item_widget
                                    .view(&[id!(markdown_view)])
                                    .set_visible(cx, false);
                                item_widget.view(&[id!(label_view)]).set_visible(cx, true);
                                item_widget
                                    .widget(&[id!(msg_label)])
                                    .set_text(cx, &msg.text);
                            }
                        }

                        let is_revert_point = msg
                            .message_id
                            .as_ref()
                            .and_then(|id| self.revert_message_id.as_ref().map(|rev| rev == id))
                            .unwrap_or(false);
                        let last_assistant_idx =
                            self.messages.iter().rposition(|m| m.role == "assistant");
                        let show_revert = is_revert_point
                            || (self.revert_message_id.is_none()
                                && last_assistant_idx == Some(item_id));
                        if msg.role == "assistant" {
                            // Metadata row (first child of AssistantBubble) holds Copy/Revert/Show steps; keep visible
                            item_widget
                                .button(&[id!(copy_action_button)])
                                .set_visible(cx, true);
                            item_widget
                                .button(&[id!(revert_action_button)])
                                .set_visible(cx, show_revert);
                        }

                        // Set timestamp if available
                        if let Some(timestamp) = msg.timestamp {
                            let formatted = crate::ui::formatters::format_timestamp(timestamp);
                            item_widget
                                .label(&[id!(timestamp_label)])
                                .set_text(cx, &formatted);
                        }

                        // Set model ID for assistant messages
                        if msg.role == "assistant" {
                            if let Some(ref model_id) = msg.model_id {
                                item_widget
                                    .label(&[id!(model_label)])
                                    .set_text(cx, model_id);
                            }
                            if let Some(error_text) = &msg.error_text {
                                item_widget
                                    .label(&[id!(error_label)])
                                    .set_text(cx, error_text);
                                item_widget
                                    .widget(&[id!(error_label)])
                                    .set_visible(cx, true);
                            } else {
                                item_widget.label(&[id!(error_label)]).set_text(cx, "");
                                item_widget
                                    .widget(&[id!(error_label)])
                                    .set_visible(cx, false);
                            }

                            let mut show_stats = false;
                            if let Some(tokens) = &msg.tokens {
                                let formatted = crate::ui::formatters::format_token_usage(tokens);
                                item_widget
                                    .label(&[id!(tokens_label)])
                                    .set_text(cx, &formatted);
                                show_stats = true;
                            } else {
                                item_widget.label(&[id!(tokens_label)]).set_text(cx, "");
                            }

                            if let Some(cost) = msg.cost {
                                let formatted = crate::ui::formatters::format_cost(cost);
                                item_widget
                                    .label(&[id!(cost_label)])
                                    .set_text(cx, &formatted);
                                show_stats = true;
                            } else {
                                item_widget.label(&[id!(cost_label)]).set_text(cx, "");
                            }

                            // Hide message-level tokens/cost when steps are expanded to avoid duplicate stats
                            let has_steps_expanded = !msg.steps.is_empty() && msg.show_steps;
                            if has_steps_expanded {
                                show_stats = false;
                            }
                            item_widget
                                .view(&[id!(stats_row)])
                                .set_visible(cx, show_stats);

                            let has_steps = !msg.steps.is_empty();
                            item_widget
                                .view(&[id!(steps_summary_row)])
                                .set_visible(cx, has_steps);
                            if has_steps {
                                let summary = Self::steps_summary_text(msg);
                                item_widget
                                    .label(&[id!(steps_summary_label)])
                                    .set_text(cx, &summary);
                                let header = Self::steps_button_label(msg);
                                item_widget
                                    .button(&[id!(steps_button)])
                                    .set_text(cx, &header);
                            }
                            item_widget
                                .view(&[id!(steps_expanded)])
                                .set_visible(cx, has_steps && msg.show_steps);
                            if has_steps && msg.show_steps {
                                let steps_base = item_widget.view(&[
                                    id!(steps_expanded),
                                    id!(steps_scroll),
                                    id!(content),
                                ]);
                                for step_id in 0..Self::MAX_STEP_ROWS {
                                    let (row_id, header_id, body_id, content_id, dot_id, line_id) =
                                        match step_id {
                                        0 => (
                                            live_id!(step_row_0),
                                            live_id!(step_row_0_header),
                                            live_id!(step_row_0_body),
                                            live_id!(step_row_0_content),
                                            live_id!(step_row_0_dot),
                                            live_id!(step_row_0_line),
                                        ),
                                        1 => (
                                            live_id!(step_row_1),
                                            live_id!(step_row_1_header),
                                            live_id!(step_row_1_body),
                                            live_id!(step_row_1_content),
                                            live_id!(step_row_1_dot),
                                            live_id!(step_row_1_line),
                                        ),
                                        2 => (
                                            live_id!(step_row_2),
                                            live_id!(step_row_2_header),
                                            live_id!(step_row_2_body),
                                            live_id!(step_row_2_content),
                                            live_id!(step_row_2_dot),
                                            live_id!(step_row_2_line),
                                        ),
                                        3 => (
                                            live_id!(step_row_3),
                                            live_id!(step_row_3_header),
                                            live_id!(step_row_3_body),
                                            live_id!(step_row_3_content),
                                            live_id!(step_row_3_dot),
                                            live_id!(step_row_3_line),
                                        ),
                                        4 => (
                                            live_id!(step_row_4),
                                            live_id!(step_row_4_header),
                                            live_id!(step_row_4_body),
                                            live_id!(step_row_4_content),
                                            live_id!(step_row_4_dot),
                                            live_id!(step_row_4_line),
                                        ),
                                        5 => (
                                            live_id!(step_row_5),
                                            live_id!(step_row_5_header),
                                            live_id!(step_row_5_body),
                                            live_id!(step_row_5_content),
                                            live_id!(step_row_5_dot),
                                            live_id!(step_row_5_line),
                                        ),
                                        6 => (
                                            live_id!(step_row_6),
                                            live_id!(step_row_6_header),
                                            live_id!(step_row_6_body),
                                            live_id!(step_row_6_content),
                                            live_id!(step_row_6_dot),
                                            live_id!(step_row_6_line),
                                        ),
                                        7 => (
                                            live_id!(step_row_7),
                                            live_id!(step_row_7_header),
                                            live_id!(step_row_7_body),
                                            live_id!(step_row_7_content),
                                            live_id!(step_row_7_dot),
                                            live_id!(step_row_7_line),
                                        ),
                                        8 => (
                                            live_id!(step_row_8),
                                            live_id!(step_row_8_header),
                                            live_id!(step_row_8_body),
                                            live_id!(step_row_8_content),
                                            live_id!(step_row_8_dot),
                                            live_id!(step_row_8_line),
                                        ),
                                        9 => (
                                            live_id!(step_row_9),
                                            live_id!(step_row_9_header),
                                            live_id!(step_row_9_body),
                                            live_id!(step_row_9_content),
                                            live_id!(step_row_9_dot),
                                            live_id!(step_row_9_line),
                                        ),
                                        _ => continue,
                                    };
                                    if step_id < msg.steps.len() {
                                        let step = &msg.steps[step_id];
                                        let chevron = if step.expanded { "▾" } else { "▸" };
                                        let description = Self::get_step_description(step);
                                        let header = format!("{} {}", chevron, description);
                                        steps_base.view(&[row_id]).set_visible(cx, true);

                                        // Apply error styling if step has errors
                                        let header_button =
                                            steps_base.view(&[row_id]).button(&[header_id]);
                                        header_button.set_text(cx, &header);
                                        // Always set the color to ensure proper state reset
                                        let (text_color, hover_color) = if step.has_error {
                                            // THEME_COLOR_ACCENT_RED = #ef4444, slightly lighter for hover
                                            (
                                                vec4(0.937, 0.267, 0.267, 1.0),
                                                vec4(1.0, 0.4, 0.4, 1.0),
                                            )
                                        } else {
                                            // THEME_COLOR_TEXT_MUTED_LIGHT / THEME_COLOR_TEXT_BRIGHT
                                            (vec4(0.65, 0.65, 0.65, 1.0), vec4(0.9, 0.9, 0.9, 1.0))
                                        };
                                        header_button.apply_over(
                                            cx,
                                            live! {
                                                draw_text: {
                                                    color: (text_color)
                                                    color_hover: (hover_color)
                                                }
                                            },
                                        );
                                        steps_base
                                            .view(&[row_id])
                                            .view(&[body_id])
                                            .set_visible(cx, step.expanded);
                                        steps_base
                                            .view(&[row_id])
                                            .label(&[content_id])
                                            .set_text(cx, &Self::format_step_body(step));

                                        let (dot_color, line_color) = if step.has_error {
                                            (vec4(0.937, 0.267, 0.267, 1.0), vec4(0.4, 0.2, 0.2, 1.0))
                                        } else if step.has_running {
                                            (vec4(0.4, 0.6, 1.0, 1.0), vec4(0.2, 0.3, 0.5, 1.0))
                                        } else {
                                            (vec4(0.5, 0.5, 0.5, 1.0), vec4(0.25, 0.25, 0.25, 1.0))
                                        };
                                        steps_base
                                            .view(&[row_id])
                                            .view(&[dot_id])
                                            .apply_over(cx, live! { draw_bg: { color: (dot_color) } });

                                        let show_line = step_id + 1 < msg.steps.len();
                                        let line_view = steps_base.view(&[row_id]).view(&[line_id]);
                                        line_view.set_visible(cx, show_line);
                                        if show_line {
                                            line_view.apply_over(cx, live! { draw_bg: { color: (line_color) } });
                                        }
                                    } else {
                                        steps_base.view(&[row_id]).set_visible(cx, false);
                                    }
                                }
                            }

                            // Copy and Revert are in the top row as icons; hide them in bottom row
                            item_widget
                                .button(&[id!(copy_button)])
                                .set_visible(cx, false);
                            item_widget
                                .button(&[id!(revert_button)])
                                .set_visible(cx, false);
                            item_widget.view(&[id!(msg_actions)]).set_visible(cx, true);

                            // Set diff view data
                            use crate::components::diff_view::DiffViewApi;
                            let diff_view = item_widget.diff_view(&[id!(diff_view)]);
                            if msg.diffs.is_empty() {
                                diff_view.clear_diffs(cx);
                            } else {
                                diff_view.set_diffs(cx, &msg.diffs);
                            }
                            diff_view.set_expanded(cx, msg.show_diffs);
                        }

                        item_widget.draw_all(cx, scope);
                    }
                }
            }
        }
        DrawStep::done()
    }
}

impl MessageListRef {
    pub fn set_messages(
        &self,
        cx: &mut Cx,
        messages_with_parts: &[openpad_protocol::MessageWithParts],
        revert_message_id: Option<String>,
    ) {
        if let Some(mut inner) = self.borrow_mut() {
            let _had_messages = !inner.messages.is_empty();
            let last_had_running_steps = inner
                .messages
                .last()
                .map(|m| {
                    m.role == "assistant"
                        && m.text.is_empty()
                        && m.steps.iter().any(|s| s.has_running)
                })
                .unwrap_or(false);

            inner.messages = MessageList::rebuild_from_parts(messages_with_parts);
            inner.revert_message_id = revert_message_id;

            // Auto-show steps for the last assistant message if:
            // 1. It has running tools (actively working)
            // 2. Or we were already showing steps with running tools (continue streaming)
            if let Some(last) = inner.messages.last_mut() {
                if last.role == "assistant" && last.text.is_empty() && !last.steps.is_empty() {
                    let has_running = last.steps.iter().any(|s| s.has_running);
                    if has_running || last_had_running_steps {
                        last.show_steps = true;
                    }
                }
            }

            // Scroll to the last message so users see the most recent conversation
            let msg_count = inner.messages.len();
            if msg_count > 0 {
                inner
                    .view
                    .portal_list(&[id!(list)])
                    .set_first_id(msg_count.saturating_sub(1));
            }
            inner.redraw(cx);
        }
    }

    pub fn append_text_for_message(&self, cx: &mut Cx, role: &str, message_id: &str, text: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            // Try to find an existing message to append to (by checking last message)
            // SSE parts arrive in order, so the last message of the matching role is the target
            if let Some(last) = inner.messages.last_mut() {
                if last.role == role {
                    let was_empty = last.text.is_empty();
                    last.text.push_str(text);
                    // Once text starts arriving for assistant messages, hide the steps automatically
                    // (steps were shown during "thinking" phase, now showing the actual response)
                    if role == "assistant" && was_empty && !last.steps.is_empty() {
                        last.show_steps = false;
                    }
                    // Only redraw if content changed
                    inner.redraw(cx);
                    return;
                }
            }
            // New message (no timestamp/model during streaming; will be updated later)
            inner.messages.push(DisplayMessage {
                role: role.to_string(),
                text: text.to_string(),
                message_id: Some(message_id.to_string()),
                timestamp: None,
                model_id: None,
                tokens: None,
                cost: None,
                error_text: None,
                is_error: false,
                diffs: Vec::new(),
                show_diffs: false,
                steps: Vec::new(),
                show_steps: false,
                duration_ms: None,
            });
            inner.redraw(cx);
        }
    }

    pub fn add_user_message(&self, cx: &mut Cx, text: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            // Use current time for user messages
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64;

            inner.messages.push(DisplayMessage {
                role: "user".to_string(),
                text: text.to_string(),
                message_id: None, // User messages don't have IDs yet
                timestamp: Some(now),
                model_id: None,
                tokens: None,
                cost: None,
                error_text: None,
                is_error: false,
                diffs: Vec::new(),
                show_diffs: false,
                steps: Vec::new(),
                show_steps: false,
                duration_ms: None,
            });
            inner.redraw(cx);
        }
    }

    pub fn clear(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.messages.clear();
            inner.redraw(cx);
        }
    }

    pub fn set_working(&self, cx: &mut Cx, working: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.is_working = working;
            if working && inner.working_since.is_none() {
                inner.working_since = Some(std::time::Instant::now());
                inner.thinking_frame = 0; // Reset animation frame
            } else if !working {
                inner.working_since = None;
            }
            inner.redraw(cx);
        }
    }

    pub fn set_pending_permissions(&self, cx: &mut Cx, permissions: &[PendingPermissionDisplay]) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.pending_permissions = permissions.to_vec();
            inner.redraw(cx);
        }
    }

    pub fn remove_permission(&self, cx: &mut Cx, request_id: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner
                .pending_permissions
                .retain(|p| p.request_id != request_id);
            inner.redraw(cx);
        }
    }

    pub fn set_session_diffs(&self, cx: &mut Cx, diffs: &[openpad_protocol::FileDiff]) {
        if let Some(mut inner) = self.borrow_mut() {
            // Apply diffs to the last assistant message
            if let Some(last_assistant) = inner
                .messages
                .iter_mut()
                .rev()
                .find(|m| m.role == "assistant")
            {
                last_assistant.diffs = diffs.to_vec();
                if last_assistant.diffs.is_empty() {
                    last_assistant.show_diffs = false;
                }
            }
            inner.redraw(cx);
        }
    }
}
