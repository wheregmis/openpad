use crate::diff_view::{DiffViewApi, DiffViewWidgetRefExt};
use crate::message_logic::{DisplayMessage, MessageProcessor};
use crate::permission_card::{PermissionCardApi, PermissionCardWidgetRefExt};
use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_code_editor::code_view::CodeView;
    use crate::openpad::*;
    use crate::theme::*;
    use crate::user_bubble::UserBubble;
    use crate::assistant_bubble::AssistantBubble;
    use crate::diff_view::DiffView;
    use crate::permission_card::PermissionCard;

    pub MessageList = {{MessageList}} {
        width: Fill, height: Fill
        flow: Overlay

        empty_state = <View> {
            visible: false
            width: Fill, height: Fill
            align: { x: 0.5, y: 0.5 }
            flow: Down, spacing: 10

            <Label> {
                text: "Openpad"
                draw_text: {
                    color: (THEME_COLOR_TEXT_BRIGHT)
                    text_style: <THEME_FONT_BOLD> { font_size: 16 }
                }
            }
            <Label> {
                text: "How can I help you today?"
                draw_text: {
                    color: (THEME_COLOR_TEXT_DIM)
                    text_style: <THEME_FONT_REGULAR> { font_size: 11 }
                }
            }
        }

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

                    thinking_activity = <Label> {
                        width: Fill, height: Fit
                        draw_text: {
                            color: (THEME_COLOR_TEXT_MUTED_LIGHT)
                            text_style: <THEME_FONT_ITALIC> { font_size: 9, line_spacing: 1.3 }
                            word: Wrap
                        }
                        text: ""
                    }

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
                                step_row_0 = <View> { width: Fill, height: Fit, flow: Down, spacing: 2, step_row_0_header_row = <View> { width: Fill, height: Fit, flow: Right, spacing: 6, align: { y: 0.0 }, step_row_0_rail = <View> { width: 10, height: Fill, flow: Down, align: { x: 0.5 }, step_row_0_dot = <RoundedView> { width: 6, height: 6, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_TEXT_MUTED_DARKER), border_radius: 3.0 } }, step_row_0_line = <View> { width: 2, height: Fill, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_BORDER_MEDIUM) } } }, step_row_0_header = <Button> { width: Fill, height: Fit, padding: { left: 4, right: 6, top: 2, bottom: 2 }, align: { x: 0.0 }, draw_bg: { color: (THEME_COLOR_TRANSPARENT), color_hover: (THEME_COLOR_HOVER_MEDIUM), border_radius: 4.0, border_size: 0.0 }, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }, text: "" } }, step_row_0_body = <View> { visible: true, width: Fill, height: Fit, flow: Down, padding: { left: 18, top: 2, bottom: 4 }, step_row_0_content = <Label> { width: Fill, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9, line_spacing: 1.3 }, word: Wrap }, text: "" } } }
                                step_row_1 = <View> { width: Fill, height: Fit, flow: Down, spacing: 2, step_row_1_header_row = <View> { width: Fill, height: Fit, flow: Right, spacing: 6, align: { y: 0.0 }, step_row_1_rail = <View> { width: 10, height: Fill, flow: Down, align: { x: 0.5 }, step_row_1_dot = <RoundedView> { width: 6, height: 6, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_TEXT_MUTED_DARKER), border_radius: 3.0 } }, step_row_1_line = <View> { width: 2, height: Fill, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_BORDER_MEDIUM) } } }, step_row_1_header = <Button> { width: Fill, height: Fit, padding: { left: 4, right: 6, top: 2, bottom: 2 }, align: { x: 0.0 }, draw_bg: { color: (THEME_COLOR_TRANSPARENT), color_hover: (THEME_COLOR_HOVER_MEDIUM), border_radius: 4.0, border_size: 0.0 }, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }, text: "" } }, step_row_1_body = <View> { visible: true, width: Fill, height: Fit, flow: Down, padding: { left: 18, top: 2, bottom: 4 }, step_row_1_content = <Label> { width: Fill, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9, line_spacing: 1.3 }, word: Wrap }, text: "" } } }
                                step_row_2 = <View> { width: Fill, height: Fit, flow: Down, spacing: 2, step_row_2_header_row = <View> { width: Fill, height: Fit, flow: Right, spacing: 6, align: { y: 0.0 }, step_row_2_rail = <View> { width: 10, height: Fill, flow: Down, align: { x: 0.5 }, step_row_2_dot = <RoundedView> { width: 6, height: 6, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_TEXT_MUTED_DARKER), border_radius: 3.0 } }, step_row_2_line = <View> { width: 2, height: Fill, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_BORDER_MEDIUM) } } }, step_row_2_header = <Button> { width: Fill, height: Fit, padding: { left: 4, right: 6, top: 2, bottom: 2 }, align: { x: 0.0 }, draw_bg: { color: (THEME_COLOR_TRANSPARENT), color_hover: (THEME_COLOR_HOVER_MEDIUM), border_radius: 4.0, border_size: 0.0 }, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }, text: "" } }, step_row_2_body = <View> { visible: true, width: Fill, height: Fit, flow: Down, padding: { left: 18, top: 2, bottom: 4 }, step_row_2_content = <Label> { width: Fill, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9, line_spacing: 1.3 }, word: Wrap }, text: "" } } }
                                step_row_3 = <View> { width: Fill, height: Fit, flow: Down, spacing: 2, step_row_3_header_row = <View> { width: Fill, height: Fit, flow: Right, spacing: 6, align: { y: 0.0 }, step_row_3_rail = <View> { width: 10, height: Fill, flow: Down, align: { x: 0.5 }, step_row_3_dot = <RoundedView> { width: 6, height: 6, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_TEXT_MUTED_DARKER), border_radius: 3.0 } }, step_row_3_line = <View> { width: 2, height: Fill, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_BORDER_MEDIUM) } } }, step_row_3_header = <Button> { width: Fill, height: Fit, padding: { left: 4, right: 6, top: 2, bottom: 2 }, align: { x: 0.0 }, draw_bg: { color: (THEME_COLOR_TRANSPARENT), color_hover: (THEME_COLOR_HOVER_MEDIUM), border_radius: 4.0, border_size: 0.0 }, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }, text: "" } }, step_row_3_body = <View> { visible: true, width: Fill, height: Fit, flow: Down, padding: { left: 18, top: 2, bottom: 4 }, step_row_3_content = <Label> { width: Fill, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9, line_spacing: 1.3 }, word: Wrap }, text: "" } } }
                                step_row_4 = <View> { width: Fill, height: Fit, flow: Down, spacing: 2, step_row_4_header_row = <View> { width: Fill, height: Fit, flow: Right, spacing: 6, align: { y: 0.0 }, step_row_4_rail = <View> { width: 10, height: Fill, flow: Down, align: { x: 0.5 }, step_row_4_dot = <RoundedView> { width: 6, height: 6, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_TEXT_MUTED_DARKER), border_radius: 3.0 } }, step_row_4_line = <View> { width: 2, height: Fill, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_BORDER_MEDIUM) } } }, step_row_4_header = <Button> { width: Fill, height: Fit, padding: { left: 4, right: 6, top: 2, bottom: 2 }, align: { x: 0.0 }, draw_bg: { color: (THEME_COLOR_TRANSPARENT), color_hover: (THEME_COLOR_HOVER_MEDIUM), border_radius: 4.0, border_size: 0.0 }, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }, text: "" } }, step_row_4_body = <View> { visible: true, width: Fill, height: Fit, flow: Down, padding: { left: 18, top: 2, bottom: 4 }, step_row_4_content = <Label> { width: Fill, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9, line_spacing: 1.3 }, word: Wrap }, text: "" } } }
                                step_row_5 = <View> { width: Fill, height: Fit, flow: Down, spacing: 2, step_row_5_header_row = <View> { width: Fill, height: Fit, flow: Right, spacing: 6, align: { y: 0.0 }, step_row_5_rail = <View> { width: 10, height: Fill, flow: Down, align: { x: 0.5 }, step_row_5_dot = <RoundedView> { width: 6, height: 6, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_TEXT_MUTED_DARKER), border_radius: 3.0 } }, step_row_5_line = <View> { width: 2, height: Fill, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_BORDER_MEDIUM) } } }, step_row_5_header = <Button> { width: Fill, height: Fit, padding: { left: 4, right: 6, top: 2, bottom: 2 }, align: { x: 0.0 }, draw_bg: { color: (THEME_COLOR_TRANSPARENT), color_hover: (THEME_COLOR_HOVER_MEDIUM), border_radius: 4.0, border_size: 0.0 }, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }, text: "" } }, step_row_5_body = <View> { visible: true, width: Fill, height: Fit, flow: Down, padding: { left: 18, top: 2, bottom: 4 }, step_row_5_content = <Label> { width: Fill, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9, line_spacing: 1.3 }, word: Wrap }, text: "" } } }
                                step_row_6 = <View> { width: Fill, height: Fit, flow: Down, spacing: 2, step_row_6_header_row = <View> { width: Fill, height: Fit, flow: Right, spacing: 6, align: { y: 0.0 }, step_row_6_rail = <View> { width: 10, height: Fill, flow: Down, align: { x: 0.5 }, step_row_6_dot = <RoundedView> { width: 6, height: 6, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_TEXT_MUTED_DARKER), border_radius: 3.0 } }, step_row_6_line = <View> { width: 2, height: Fill, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_BORDER_MEDIUM) } } }, step_row_6_header = <Button> { width: Fill, height: Fit, padding: { left: 4, right: 6, top: 2, bottom: 2 }, align: { x: 0.0 }, draw_bg: { color: (THEME_COLOR_TRANSPARENT), color_hover: (THEME_COLOR_HOVER_MEDIUM), border_radius: 4.0, border_size: 0.0 }, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }, text: "" } }, step_row_6_body = <View> { visible: true, width: Fill, height: Fit, flow: Down, padding: { left: 18, top: 2, bottom: 4 }, step_row_6_content = <Label> { width: Fill, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9, line_spacing: 1.3 }, word: Wrap }, text: "" } } }
                                step_row_7 = <View> { width: Fill, height: Fit, flow: Down, spacing: 2, step_row_7_header_row = <View> { width: Fill, height: Fit, flow: Right, spacing: 6, align: { y: 0.0 }, step_row_7_rail = <View> { width: 10, height: Fill, flow: Down, align: { x: 0.5 }, step_row_7_dot = <RoundedView> { width: 6, height: 6, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_TEXT_MUTED_DARKER), border_radius: 3.0 } }, step_row_7_line = <View> { width: 2, height: Fill, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_BORDER_MEDIUM) } } }, step_row_7_header = <Button> { width: Fill, height: Fit, padding: { left: 4, right: 6, top: 2, bottom: 2 }, align: { x: 0.0 }, draw_bg: { color: (THEME_COLOR_TRANSPARENT), color_hover: (THEME_COLOR_HOVER_MEDIUM), border_radius: 4.0, border_size: 0.0 }, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }, text: "" } }, step_row_7_body = <View> { visible: true, width: Fill, height: Fit, flow: Down, padding: { left: 18, top: 2, bottom: 4 }, step_row_7_content = <Label> { width: Fill, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9, line_spacing: 1.3 }, word: Wrap }, text: "" } } }
                                step_row_8 = <View> { width: Fill, height: Fit, flow: Down, spacing: 2, step_row_8_header_row = <View> { width: Fill, height: Fit, flow: Right, spacing: 6, align: { y: 0.0 }, step_row_8_rail = <View> { width: 10, height: Fill, flow: Down, align: { x: 0.5 }, step_row_8_dot = <RoundedView> { width: 6, height: 6, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_TEXT_MUTED_DARKER), border_radius: 3.0 } }, step_row_8_line = <View> { width: 2, height: Fill, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_BORDER_MEDIUM) } } }, step_row_8_header = <Button> { width: Fill, height: Fit, padding: { left: 4, right: 6, top: 2, bottom: 2 }, align: { x: 0.0 }, draw_bg: { color: (THEME_COLOR_TRANSPARENT), color_hover: (THEME_COLOR_HOVER_MEDIUM), border_radius: 4.0, border_size: 0.0 }, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }, text: "" } }, step_row_8_body = <View> { visible: true, width: Fill, height: Fit, flow: Down, padding: { left: 18, top: 2, bottom: 4 }, step_row_8_content = <Label> { width: Fill, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9, line_spacing: 1.3 }, word: Wrap }, text: "" } } }
                                step_row_9 = <View> { width: Fill, height: Fit, flow: Down, spacing: 2, step_row_9_header_row = <View> { width: Fill, height: Fit, flow: Right, spacing: 6, align: { y: 0.0 }, step_row_9_rail = <View> { width: 10, height: Fill, flow: Down, align: { x: 0.5 }, step_row_9_dot = <RoundedView> { width: 6, height: 6, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_TEXT_MUTED_DARKER), border_radius: 3.0 } }, step_row_9_line = <View> { width: 2, height: Fill, margin: { top: 4 }, show_bg: true, draw_bg: { color: (THEME_COLOR_BORDER_MEDIUM) } } }, step_row_9_header = <Button> { width: Fill, height: Fit, padding: { left: 4, right: 6, top: 2, bottom: 2 }, align: { x: 0.0 }, draw_bg: { color: (THEME_COLOR_TRANSPARENT), color_hover: (THEME_COLOR_HOVER_MEDIUM), border_radius: 4.0, border_size: 0.0 }, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }, text: "" } }, step_row_9_body = <View> { visible: true, width: Fill, height: Fit, flow: Down, padding: { left: 18, top: 2, bottom: 4 }, step_row_9_content = <Label> { width: Fill, height: Fit, draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9, line_spacing: 1.3 }, word: Wrap }, text: "" } } }
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
    const MAX_STEP_ROWS: usize = 10;

    fn thinking_icon(&self) -> &'static str {
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

    fn steps_summary_text(&self, msg: &DisplayMessage) -> String {
        if msg.steps.is_empty() {
            return String::new();
        }
        let has_running = msg.steps.iter().any(|s| s.has_running);
        let mut labels: Vec<String> = Vec::new();
        for step in msg.steps.iter() {
            let desc = MessageProcessor::get_step_description(step);
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
            .map(crate::utils::formatters::format_duration_ms);
        let prefix = if has_running { "Running" } else { "Steps" };
        if let Some(d) = duration {
            format!("{}: {} • {} • {}", prefix, count, summary, d)
        } else {
            format!("{}: {} • {}", prefix, count, summary)
        }
    }
}

impl Widget for MessageList {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.is_working {
            if let Event::NextFrame(_) = event {
                self.thinking_frame = (self.thinking_frame + 1) % 6;
                self.redraw(cx);
            }
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
        let is_empty =
            self.messages.is_empty() && self.pending_permissions.is_empty() && !self.is_working;
        self.view
            .view(&[id!(empty_state)])
            .set_visible(cx, is_empty);

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
                    if item_id >= self.messages.len()
                        && item_id < self.messages.len() + self.pending_permissions.len()
                    {
                        let perm_idx = item_id - self.messages.len();
                        let perm = &self.pending_permissions[perm_idx];
                        let item_widget = list.item(cx, item_id, live_id!(PermissionMsg));
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

                        let (current_activity, running_tools) = if let Some(msg) =
                            self.messages.last()
                        {
                            if let Some(last_step) = msg.steps.last() {
                                let tools: Vec<(String, String, String)> = last_step
                                    .details
                                    .iter()
                                    .filter(|d| d.is_running)
                                    .map(|d| {
                                        (
                                            MessageProcessor::get_tool_icon(&d.tool).to_string(),
                                            MessageProcessor::format_tool_name(&d.tool),
                                            MessageProcessor::format_tool_input(&d.input_summary),
                                        )
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
                                    let desc = MessageProcessor::get_step_description(last_step);
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
                        } else {
                            ("Working...".to_string(), Vec::new())
                        };

                        let timer_text = if elapsed > 0 {
                            format!("· {}m, {}s", mins, secs)
                        } else {
                            String::new()
                        };
                        let item_widget = list.item(cx, item_id, live_id!(ThinkingMsg));
                        item_widget
                            .label(&[id!(thinking_label)])
                            .set_text(cx, "Working");
                        item_widget
                            .label(&[id!(thinking_icon)])
                            .set_text(cx, self.thinking_icon());
                        item_widget
                            .label(&[id!(thinking_timer)])
                            .set_text(cx, &timer_text);
                        item_widget
                            .label(&[id!(thinking_activity)])
                            .set_text(cx, &current_activity);

                        let has_tools = !running_tools.is_empty();
                        item_widget
                            .view(&[id!(thinking_tools)])
                            .set_visible(cx, has_tools);
                        if has_tools {
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
                            item_widget
                                .button(&[id!(copy_action_button)])
                                .set_visible(cx, true);
                            item_widget
                                .button(&[id!(revert_action_button)])
                                .set_visible(cx, show_revert);
                        }

                        if let Some(timestamp) = msg.timestamp {
                            item_widget.label(&[id!(timestamp_label)]).set_text(
                                cx,
                                &crate::utils::formatters::format_timestamp(timestamp),
                            );
                        }

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
                                item_widget.label(&[id!(tokens_label)]).set_text(
                                    cx,
                                    &crate::utils::formatters::format_token_usage(tokens),
                                );
                                show_stats = true;
                            }
                            if let Some(cost) = msg.cost {
                                item_widget
                                    .label(&[id!(cost_label)])
                                    .set_text(cx, &crate::utils::formatters::format_cost(cost));
                                show_stats = true;
                            }
                            if !msg.steps.is_empty() && msg.show_steps {
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
                                item_widget
                                    .label(&[id!(steps_summary_label)])
                                    .set_text(cx, &self.steps_summary_text(msg));
                                item_widget
                                    .button(&[id!(steps_button)])
                                    .set_text(cx, &Self::steps_button_label(msg));
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
                                        let description =
                                            MessageProcessor::get_step_description(step);
                                        let header = format!(
                                            "{} {}",
                                            if step.expanded { "▾" } else { "▸" },
                                            description
                                        );
                                        steps_base.view(&[row_id]).set_visible(cx, true);
                                        let header_button =
                                            steps_base.view(&[row_id]).button(&[header_id]);
                                        header_button.set_text(cx, &header);
                                        let (text_color, hover_color) = if step.has_error {
                                            (
                                                vec4(0.937, 0.267, 0.267, 1.0),
                                                vec4(1.0, 0.4, 0.4, 1.0),
                                            )
                                        } else {
                                            (vec4(0.65, 0.65, 0.65, 1.0), vec4(0.9, 0.9, 0.9, 1.0))
                                        };
                                        header_button.apply_over(cx, live! { draw_text: { color: (text_color) color_hover: (hover_color) } });
                                        steps_base
                                            .view(&[row_id])
                                            .view(&[body_id])
                                            .set_visible(cx, step.expanded);
                                        steps_base.view(&[row_id]).label(&[content_id]).set_text(
                                            cx,
                                            &MessageProcessor::format_step_body(step),
                                        );
                                        let (dot_color, line_color) = if step.has_error {
                                            (
                                                vec4(0.937, 0.267, 0.267, 1.0),
                                                vec4(0.4, 0.2, 0.2, 1.0),
                                            )
                                        } else if step.has_running {
                                            (vec4(0.4, 0.6, 1.0, 1.0), vec4(0.2, 0.3, 0.5, 1.0))
                                        } else {
                                            (vec4(0.5, 0.5, 0.5, 1.0), vec4(0.25, 0.25, 0.25, 1.0))
                                        };
                                        steps_base.view(&[row_id]).view(&[dot_id]).apply_over(
                                            cx,
                                            live! { draw_bg: { color: (dot_color) } },
                                        );
                                        let show_line = step_id + 1 < msg.steps.len();
                                        let line_view = steps_base.view(&[row_id]).view(&[line_id]);
                                        line_view.set_visible(cx, show_line);
                                        if show_line {
                                            line_view.apply_over(
                                                cx,
                                                live! { draw_bg: { color: (line_color) } },
                                            );
                                        }
                                    } else {
                                        steps_base.view(&[row_id]).set_visible(cx, false);
                                    }
                                }
                            }
                            item_widget
                                .button(&[id!(copy_button)])
                                .set_visible(cx, false);
                            item_widget
                                .button(&[id!(revert_button)])
                                .set_visible(cx, false);
                            item_widget.view(&[id!(msg_actions)]).set_visible(cx, true);
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
            let last_had_running_steps = inner
                .messages
                .last()
                .map(|m| {
                    m.role == "assistant"
                        && m.text.is_empty()
                        && m.steps.iter().any(|s| s.has_running)
                })
                .unwrap_or(false);
            inner.messages = MessageProcessor::rebuild_from_parts(messages_with_parts);
            inner.revert_message_id = revert_message_id;
            if let Some(last) = inner.messages.last_mut() {
                if last.role == "assistant" && last.text.is_empty() && !last.steps.is_empty() {
                    if last.steps.iter().any(|s| s.has_running) || last_had_running_steps {
                        last.show_steps = true;
                    }
                }
            }
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
            if let Some(last) = inner.messages.last_mut() {
                if last.role == role {
                    let was_empty = last.text.is_empty();
                    last.text.push_str(text);
                    if role == "assistant" && was_empty && !last.steps.is_empty() {
                        last.show_steps = false;
                    }
                    inner.redraw(cx);
                    return;
                }
            }
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
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64;
            inner.messages.push(DisplayMessage {
                role: "user".to_string(),
                text: text.to_string(),
                message_id: None,
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
                inner.thinking_frame = 0;
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

#[derive(Clone, Debug, DefaultNone)]
pub enum MessageListAction {
    None,
    RevertToMessage(String),
}
