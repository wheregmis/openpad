use crate::diff_view::{DiffViewApi, DiffViewWidgetRefExt};
use crate::message_logic::{DisplayMessage, MessageProcessor};
use crate::permission_card::{PermissionCardApi, PermissionCardWidgetRefExt};
use makepad_widgets::*;


mod api;
mod events;
mod model;
mod render;

pub(crate) use model::TailMode;
script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*
    use mod.theme.*

    mod.widgets.MessageList = #(MessageList::register_widget(vm)) {
        width: Fill, height: Fill
        flow: Overlay

        empty_state := View {
            visible: false
            width: Fill, height: Fill
            align: Align{ x: 0.5, y: 0.5 }
            flow: Down, spacing: 10

            Label {
                text: "Openpad"
                draw_text +: {
                    color: #ffffff
                    text_style: theme.font_bold { font_size: 16 }
                }
            }
            Label {
                text: "How can I help you today?"
                draw_text +: {
                    color: #aab3bd
                    text_style: theme.font_regular { font_size: 11 }
                }
            }
        }

        list := PortalList {
            auto_tail: false
            smooth_tail: false
            drag_scrolling: false
            scroll_bar: ScrollBar {}

            UserMsg := View {
                width: Fill, height: Fit
                flow: Down,
                padding: Inset{ top: 4, bottom: 4, left: 100, right: 24 }
                align: Align{ x: 1.0 }

                UserBubble {
                    width: Fill, height: Fit
                    flow: Down,
                    align: Align{ x: 1.0 }

                    View {
                        width: Fit, height: Fit
                        flow: Right,
                        spacing: 8,
                        margin: Inset{ bottom: 4 }
                        align: Align{ y: 0.5 }

                        timestamp_label := Label {
                            width: Fit, height: Fit
                            draw_text +: {
                                color: #444,
                                text_style: theme.font_regular { font_size: 8 },
                            }
                            text: "..."
                        }

                        Label {
                            width: Fit, height: Fit
                            draw_text +: {
                                color: #555,
                                text_style: theme.font_bold { font_size: 8 },
                            }
                            text: "YOU"
                        }
                    }

                    msg_text := Label {
                        width: Fill, height: Fit
                        draw_text +: {
                            color: #ddd,
                            text_style: theme.font_regular { font_size: 10, line_spacing: 1.4 },
                        }
                    }

                    msg_actions := View {
                        width: Fit, height: Fit
                        flow: Right,
                        spacing: 6,
                        margin: Inset{ top: 6 }

                        copy_button := Button {
                            width: Fit, height: 20
                            text: "Copy"
                            draw_bg +: {
                                color: #0000
                                color_hover: #333
                                border_size: 0.0
                            }
                            draw_text +: {
                                color: #666
                                color_hover: #aaa
                                text_style: theme.font_regular { font_size: 8 }
                            }
                        }
                    }
                }
            }

            PermissionMsg := PermissionCard {}

            ThinkingMsg := View {
                width: Fill, height: Fit
                flow: Down,
                padding: Inset{ top: 8, bottom: 8, left: 24, right: 100 }

                AssistantBubble {
                    width: Fill, height: Fit
                    flow: Down,
                    padding: Inset{ top: 10, bottom: 10, left: 14, right: 14 }
                    draw_bg +: {
                        color: #252526
                        border_color: #333
                    }

                    View {
                        width: Fill, height: Fit
                        flow: Right,
                        spacing: 8,
                        margin: Inset{ bottom: 6 }
                        align: Align{ y: 0.5 }

                        thinking_indicator := View {
                            width: Fit, height: Fit
                            flow: Right,
                            spacing: 4,
                            align: Align{ y: 0.5 }

                            thinking_icon := Label {
                                width: Fit, height: Fit
                                draw_text +: {
                                    color: #666
                                    text_style: theme.font_regular { font_size: 9 }
                                }
                                text: "◐"
                            }

                            thinking_label := Label {
                                width: Fit, height: Fit
                                draw_text +: {
                                    color: #666
                                    text_style: theme.font_bold { font_size: 9 }
                                }
                                text: "Working"
                            }
                        }

                        View { width: Fill }

                        thinking_timer := Label {
                            width: Fit, height: Fit
                            draw_text +: {
                                color: #444
                                text_style: theme.font_regular { font_size: 8 }
                            }
                            text: ""
                        }
                    }

                    thinking_activity := Label {
                        width: Fill, height: Fit
                        draw_text +: {
                            color: #666
                            text_style: theme.font_italic { font_size: 9, line_spacing: 1.3 }}
                        text: ""
                    }

                    thinking_tools := View {
                        visible: false
                        width: Fill, height: Fit
                        flow: Down,
                        spacing: 3,
                        margin: Inset{ top: 8 }

                        tool_row_0 := View {
                            visible: false
                            width: Fill, height: Fit
                            flow: Right, spacing: 6
                            align: Align{ y: 0.5 }

                            tool_icon_0 := Label {
                                width: Fit, height: Fit
                                draw_text +: { color: #b8c2d3, text_style: theme.font_regular { font_size: 9 } }
                                text: ""
                            }
                            tool_name_0 := Label {
                                width: Fit, height: Fit
                                draw_text +: { color: #d2dae8, text_style: theme.font_bold { font_size: 9 } }
                                text: ""
                            }
                            tool_input_0 := Label {
                                width: Fill, height: Fit
                                draw_text +: { color: #b8c2d3, text_style: theme.font_regular { font_size: 9 } }
                                text: ""
                            }
                        }
                        tool_row_1 := View {
                            visible: false
                            width: Fill, height: Fit
                            flow: Right, spacing: 6
                            align: Align{ y: 0.5 }

                            tool_icon_1 := Label {
                                width: Fit, height: Fit
                                draw_text +: { color: #b8c2d3, text_style: theme.font_regular { font_size: 9 } }
                                text: ""
                            }
                            tool_name_1 := Label {
                                width: Fit, height: Fit
                                draw_text +: { color: #d2dae8, text_style: theme.font_bold { font_size: 9 } }
                                text: ""
                            }
                            tool_input_1 := Label {
                                width: Fill, height: Fit
                                draw_text +: { color: #b8c2d3, text_style: theme.font_regular { font_size: 9 } }
                                text: ""
                            }
                        }
                        tool_row_2 := View {
                            visible: false
                            width: Fill, height: Fit
                            flow: Right, spacing: 6
                            align: Align{ y: 0.5 }

                            tool_icon_2 := Label {
                                width: Fit, height: Fit
                                draw_text +: { color: #b8c2d3, text_style: theme.font_regular { font_size: 9 } }
                                text: ""
                            }
                            tool_name_2 := Label {
                                width: Fit, height: Fit
                                draw_text +: { color: #d2dae8, text_style: theme.font_bold { font_size: 9 } }
                                text: ""
                            }
                            tool_input_2 := Label {
                                width: Fill, height: Fit
                                draw_text +: { color: #b8c2d3, text_style: theme.font_regular { font_size: 9 } }
                                text: ""
                            }
                        }
                        tool_row_3 := View {
                            visible: false
                            width: Fill, height: Fit
                            flow: Right, spacing: 6
                            align: Align{ y: 0.5 }

                            tool_icon_3 := Label {
                                width: Fit, height: Fit
                                draw_text +: { color: #b8c2d3, text_style: theme.font_regular { font_size: 9 } }
                                text: ""
                            }
                            tool_name_3 := Label {
                                width: Fit, height: Fit
                                draw_text +: { color: #d2dae8, text_style: theme.font_bold { font_size: 9 } }
                                text: ""
                            }
                            tool_input_3 := Label {
                                width: Fill, height: Fit
                                draw_text +: { color: #b8c2d3, text_style: theme.font_regular { font_size: 9 } }
                                text: ""
                            }
                        }
                        tool_row_4 := View {
                            visible: false
                            width: Fill, height: Fit
                            flow: Right, spacing: 6
                            align: Align{ y: 0.5 }

                            tool_icon_4 := Label {
                                width: Fit, height: Fit
                                draw_text +: { color: #b8c2d3, text_style: theme.font_regular { font_size: 9 } }
                                text: ""
                            }
                            tool_name_4 := Label {
                                width: Fit, height: Fit
                                draw_text +: { color: #d2dae8, text_style: theme.font_bold { font_size: 9 } }
                                text: ""
                            }
                            tool_input_4 := Label {
                                width: Fill, height: Fit
                                draw_text +: { color: #b8c2d3, text_style: theme.font_regular { font_size: 9 } }
                                text: ""
                            }
                        }
                    }
                }
            }

            AssistantMsg := View {
                width: Fill, height: Fit
                flow: Down,
                padding: Inset{ top: 4, bottom: 4, left: 24, right: 100 }

                AssistantBubble {
                    width: Fill, height: Fit
                    flow: Down,

                    View {
                        width: Fill, height: Fit
                        flow: Right,
                        spacing: 8,
                        margin: Inset{ bottom: 4 }
                        align: Align{ y: 0.5 }

                        model_label := Label {
                            width: Fit, height: Fit
                            draw_text +: {
                                color: #555,
                                text_style: theme.font_bold { font_size: 8 },
                            }
                            text: "ASSISTANT"
                        }

                        timestamp_label := Label {
                            width: Fit, height: Fit
                            draw_text +: {
                                color: #444,
                                text_style: theme.font_regular { font_size: 8 },
                            }
                            text: "..."
                        }

                        View { width: Fill }

                        copy_action_button := Button {
                            width: Fit, height: 20
                            text: "Copy"
                            draw_bg +: {
                                color: #0000
                                color_hover: #333
                                border_size: 0.0
                            }
                            draw_text +: {
                                color: #e6e9ee
                                color_hover: #ffffff
                                text_style: theme.font_regular { font_size: 8 }
                            }
                        }

                        revert_action_button := Button {
                            width: Fit, height: 20
                            text: "Revert"
                            draw_bg +: {
                                color: #0000
                                color_hover: #333
                                border_size: 0.0
                            }
                            draw_text +: {
                                color: #f59e0b
                                color_hover: #ffffff
                                text_style: theme.font_regular { font_size: 8 }
                            }
                        }
                    }

                    steps_summary_row := View {
                        visible: false
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 6
                        margin: Inset{ top: 6, bottom: 4 }
                        align: Align{ y: 0.5 }

                        steps_summary_label := Label {
                            width: Fill, height: Fit
                            draw_text +: {
                                color: #e0e7f2
                                text_style: theme.font_regular { font_size: 9 }
                            }
                            text: ""
                        }

                        steps_button := Button {
                            width: Fit, height: 20
                            draw_bg +: {
                                color: #0000
                                color_hover: #333
                                border_radius: 4.0
                                border_size: 0.0
                            }
                            draw_text +: {
                                color: #e6e9ee
                                color_hover: #ffffff
                                text_style: theme.font_regular { font_size: 8 }
                            }
                            text: "▸ Details"
                        }
                    }

                    steps_expanded := View {
                        visible: false
                        width: Fill, height: Fit
                        flow: Down,
                        margin: Inset{ top: 2, bottom: 4 }

                        steps_scroll := ScrollYView {
                            width: Fill, height: Fit
                            content := View {
                                width: Fill, height: Fit
                                flow: Down
                                spacing: 4
                                step_row_0 := View {
                                    width: Fill, height: Fit
                                    flow: Down, spacing: 2

                                    step_row_0_header_row := View {
                                        width: Fill, height: Fit
                                        flow: Right, spacing: 6
                                        align: Align{ y: 0.0 }

                                        step_row_0_rail := View {
                                            width: 10, height: Fill
                                            flow: Down
                                            align: Align{ x: 0.5 }

                                            step_row_0_dot := RoundedView {
                                                width: 6, height: 6
                                                margin: Inset{ top: 4 }
                                                show_bg: true
                                                draw_bg +: { color: #444, border_radius: 3.0 }
                                            }
                                            step_row_0_line := View {
                                                width: 2, height: Fill
                                                margin: Inset{ top: 4 }
                                                show_bg: true
                                                draw_bg +: { color: #333 }
                                            }
                                        }

                                        step_row_0_header := Button {
                                            width: Fill, height: Fit
                                            padding: Inset{ left: 4, right: 6, top: 2, bottom: 2 }
                                            align: Align{ x: 0.0 }
                                            draw_bg +: { color: #0000, color_hover: #333, border_radius: 4.0, border_size: 0.0 }
                                            draw_text +: { color: #d2dae8, text_style: theme.font_regular { font_size: 9 } }
                                            text: ""
                                        }
                                    }

                                    step_row_0_body := View {
                                        visible: true
                                        width: Fill, height: Fit
                                        flow: Down
                                        padding: Inset{ left: 18, top: 2, bottom: 4 }

                                        step_row_0_content := Label {
                                            width: Fill, height: Fit
                                            draw_text +: { color: #d2dae8, text_style: theme.font_regular { font_size: 9, line_spacing: 1.3 } }
                                            text: ""
                                        }
                                    }
                                }
                                step_row_1 := View {
                                    width: Fill, height: Fit
                                    flow: Down, spacing: 2

                                    step_row_1_header_row := View {
                                        width: Fill, height: Fit
                                        flow: Right, spacing: 6
                                        align: Align{ y: 0.0 }

                                        step_row_1_rail := View {
                                            width: 10, height: Fill
                                            flow: Down
                                            align: Align{ x: 0.5 }

                                            step_row_1_dot := RoundedView {
                                                width: 6, height: 6
                                                margin: Inset{ top: 4 }
                                                show_bg: true
                                                draw_bg +: { color: #444, border_radius: 3.0 }
                                            }
                                            step_row_1_line := View {
                                                width: 2, height: Fill
                                                margin: Inset{ top: 4 }
                                                show_bg: true
                                                draw_bg +: { color: #333 }
                                            }
                                        }

                                        step_row_1_header := Button {
                                            width: Fill, height: Fit
                                            padding: Inset{ left: 4, right: 6, top: 2, bottom: 2 }
                                            align: Align{ x: 0.0 }
                                            draw_bg +: { color: #0000, color_hover: #333, border_radius: 4.0, border_size: 0.0 }
                                            draw_text +: { color: #d2dae8, text_style: theme.font_regular { font_size: 9 } }
                                            text: ""
                                        }
                                    }

                                    step_row_1_body := View {
                                        visible: true
                                        width: Fill, height: Fit
                                        flow: Down
                                        padding: Inset{ left: 18, top: 2, bottom: 4 }

                                        step_row_1_content := Label {
                                            width: Fill, height: Fit
                                            draw_text +: { color: #d2dae8, text_style: theme.font_regular { font_size: 9, line_spacing: 1.3 } }
                                            text: ""
                                        }
                                    }
                                }
                                step_row_2 := View {
                                    width: Fill, height: Fit
                                    flow: Down, spacing: 2

                                    step_row_2_header_row := View {
                                        width: Fill, height: Fit
                                        flow: Right, spacing: 6
                                        align: Align{ y: 0.0 }

                                        step_row_2_rail := View {
                                            width: 10, height: Fill
                                            flow: Down
                                            align: Align{ x: 0.5 }

                                            step_row_2_dot := RoundedView {
                                                width: 6, height: 6
                                                margin: Inset{ top: 4 }
                                                show_bg: true
                                                draw_bg +: { color: #444, border_radius: 3.0 }
                                            }
                                            step_row_2_line := View {
                                                width: 2, height: Fill
                                                margin: Inset{ top: 4 }
                                                show_bg: true
                                                draw_bg +: { color: #333 }
                                            }
                                        }

                                        step_row_2_header := Button {
                                            width: Fill, height: Fit
                                            padding: Inset{ left: 4, right: 6, top: 2, bottom: 2 }
                                            align: Align{ x: 0.0 }
                                            draw_bg +: { color: #0000, color_hover: #333, border_radius: 4.0, border_size: 0.0 }
                                            draw_text +: { color: #d2dae8, text_style: theme.font_regular { font_size: 9 } }
                                            text: ""
                                        }
                                    }

                                    step_row_2_body := View {
                                        visible: true
                                        width: Fill, height: Fit
                                        flow: Down
                                        padding: Inset{ left: 18, top: 2, bottom: 4 }

                                        step_row_2_content := Label {
                                            width: Fill, height: Fit
                                            draw_text +: { color: #d2dae8, text_style: theme.font_regular { font_size: 9, line_spacing: 1.3 } }
                                            text: ""
                                        }
                                    }
                                }
                                step_row_3 := View {
                                    width: Fill, height: Fit
                                    flow: Down, spacing: 2

                                    step_row_3_header_row := View {
                                        width: Fill, height: Fit
                                        flow: Right, spacing: 6
                                        align: Align{ y: 0.0 }

                                        step_row_3_rail := View {
                                            width: 10, height: Fill
                                            flow: Down
                                            align: Align{ x: 0.5 }

                                            step_row_3_dot := RoundedView {
                                                width: 6, height: 6
                                                margin: Inset{ top: 4 }
                                                show_bg: true
                                                draw_bg +: { color: #444, border_radius: 3.0 }
                                            }
                                            step_row_3_line := View {
                                                width: 2, height: Fill
                                                margin: Inset{ top: 4 }
                                                show_bg: true
                                                draw_bg +: { color: #333 }
                                            }
                                        }

                                        step_row_3_header := Button {
                                            width: Fill, height: Fit
                                            padding: Inset{ left: 4, right: 6, top: 2, bottom: 2 }
                                            align: Align{ x: 0.0 }
                                            draw_bg +: { color: #0000, color_hover: #333, border_radius: 4.0, border_size: 0.0 }
                                            draw_text +: { color: #d2dae8, text_style: theme.font_regular { font_size: 9 } }
                                            text: ""
                                        }
                                    }

                                    step_row_3_body := View {
                                        visible: true
                                        width: Fill, height: Fit
                                        flow: Down
                                        padding: Inset{ left: 18, top: 2, bottom: 4 }

                                        step_row_3_content := Label {
                                            width: Fill, height: Fit
                                            draw_text +: { color: #d2dae8, text_style: theme.font_regular { font_size: 9, line_spacing: 1.3 } }
                                            text: ""
                                        }
                                    }
                                }
                                step_row_4 := View {
                                    width: Fill, height: Fit
                                    flow: Down, spacing: 2

                                    step_row_4_header_row := View {
                                        width: Fill, height: Fit
                                        flow: Right, spacing: 6
                                        align: Align{ y: 0.0 }

                                        step_row_4_rail := View {
                                            width: 10, height: Fill
                                            flow: Down
                                            align: Align{ x: 0.5 }

                                            step_row_4_dot := RoundedView {
                                                width: 6, height: 6
                                                margin: Inset{ top: 4 }
                                                show_bg: true
                                                draw_bg +: { color: #444, border_radius: 3.0 }
                                            }
                                            step_row_4_line := View {
                                                width: 2, height: Fill
                                                margin: Inset{ top: 4 }
                                                show_bg: true
                                                draw_bg +: { color: #333 }
                                            }
                                        }

                                        step_row_4_header := Button {
                                            width: Fill, height: Fit
                                            padding: Inset{ left: 4, right: 6, top: 2, bottom: 2 }
                                            align: Align{ x: 0.0 }
                                            draw_bg +: { color: #0000, color_hover: #333, border_radius: 4.0, border_size: 0.0 }
                                            draw_text +: { color: #d2dae8, text_style: theme.font_regular { font_size: 9 } }
                                            text: ""
                                        }
                                    }

                                    step_row_4_body := View {
                                        visible: true
                                        width: Fill, height: Fit
                                        flow: Down
                                        padding: Inset{ left: 18, top: 2, bottom: 4 }

                                        step_row_4_content := Label {
                                            width: Fill, height: Fit
                                            draw_text +: { color: #d2dae8, text_style: theme.font_regular { font_size: 9, line_spacing: 1.3 } }
                                            text: ""
                                        }
                                    }
                                }
                                step_row_5 := View {
                                    width: Fill, height: Fit
                                    flow: Down, spacing: 2

                                    step_row_5_header_row := View {
                                        width: Fill, height: Fit
                                        flow: Right, spacing: 6
                                        align: Align{ y: 0.0 }

                                        step_row_5_rail := View {
                                            width: 10, height: Fill
                                            flow: Down
                                            align: Align{ x: 0.5 }

                                            step_row_5_dot := RoundedView {
                                                width: 6, height: 6
                                                margin: Inset{ top: 4 }
                                                show_bg: true
                                                draw_bg +: { color: #444, border_radius: 3.0 }
                                            }
                                            step_row_5_line := View {
                                                width: 2, height: Fill
                                                margin: Inset{ top: 4 }
                                                show_bg: true
                                                draw_bg +: { color: #333 }
                                            }
                                        }

                                        step_row_5_header := Button {
                                            width: Fill, height: Fit
                                            padding: Inset{ left: 4, right: 6, top: 2, bottom: 2 }
                                            align: Align{ x: 0.0 }
                                            draw_bg +: { color: #0000, color_hover: #333, border_radius: 4.0, border_size: 0.0 }
                                            draw_text +: { color: #d2dae8, text_style: theme.font_regular { font_size: 9 } }
                                            text: ""
                                        }
                                    }

                                    step_row_5_body := View {
                                        visible: true
                                        width: Fill, height: Fit
                                        flow: Down
                                        padding: Inset{ left: 18, top: 2, bottom: 4 }

                                        step_row_5_content := Label {
                                            width: Fill, height: Fit
                                            draw_text +: { color: #d2dae8, text_style: theme.font_regular { font_size: 9, line_spacing: 1.3 } }
                                            text: ""
                                        }
                                    }
                                }
                                step_row_6 := View {
                                    width: Fill, height: Fit
                                    flow: Down, spacing: 2

                                    step_row_6_header_row := View {
                                        width: Fill, height: Fit
                                        flow: Right, spacing: 6
                                        align: Align{ y: 0.0 }

                                        step_row_6_rail := View {
                                            width: 10, height: Fill
                                            flow: Down
                                            align: Align{ x: 0.5 }

                                            step_row_6_dot := RoundedView {
                                                width: 6, height: 6
                                                margin: Inset{ top: 4 }
                                                show_bg: true
                                                draw_bg +: { color: #444, border_radius: 3.0 }
                                            }
                                            step_row_6_line := View {
                                                width: 2, height: Fill
                                                margin: Inset{ top: 4 }
                                                show_bg: true
                                                draw_bg +: { color: #333 }
                                            }
                                        }

                                        step_row_6_header := Button {
                                            width: Fill, height: Fit
                                            padding: Inset{ left: 4, right: 6, top: 2, bottom: 2 }
                                            align: Align{ x: 0.0 }
                                            draw_bg +: { color: #0000, color_hover: #333, border_radius: 4.0, border_size: 0.0 }
                                            draw_text +: { color: #d2dae8, text_style: theme.font_regular { font_size: 9 } }
                                            text: ""
                                        }
                                    }

                                    step_row_6_body := View {
                                        visible: true
                                        width: Fill, height: Fit
                                        flow: Down
                                        padding: Inset{ left: 18, top: 2, bottom: 4 }

                                        step_row_6_content := Label {
                                            width: Fill, height: Fit
                                            draw_text +: { color: #d2dae8, text_style: theme.font_regular { font_size: 9, line_spacing: 1.3 } }
                                            text: ""
                                        }
                                    }
                                }
                                step_row_7 := View {
                                    width: Fill, height: Fit
                                    flow: Down, spacing: 2

                                    step_row_7_header_row := View {
                                        width: Fill, height: Fit
                                        flow: Right, spacing: 6
                                        align: Align{ y: 0.0 }

                                        step_row_7_rail := View {
                                            width: 10, height: Fill
                                            flow: Down
                                            align: Align{ x: 0.5 }

                                            step_row_7_dot := RoundedView {
                                                width: 6, height: 6
                                                margin: Inset{ top: 4 }
                                                show_bg: true
                                                draw_bg +: { color: #444, border_radius: 3.0 }
                                            }
                                            step_row_7_line := View {
                                                width: 2, height: Fill
                                                margin: Inset{ top: 4 }
                                                show_bg: true
                                                draw_bg +: { color: #333 }
                                            }
                                        }

                                        step_row_7_header := Button {
                                            width: Fill, height: Fit
                                            padding: Inset{ left: 4, right: 6, top: 2, bottom: 2 }
                                            align: Align{ x: 0.0 }
                                            draw_bg +: { color: #0000, color_hover: #333, border_radius: 4.0, border_size: 0.0 }
                                            draw_text +: { color: #d2dae8, text_style: theme.font_regular { font_size: 9 } }
                                            text: ""
                                        }
                                    }

                                    step_row_7_body := View {
                                        visible: true
                                        width: Fill, height: Fit
                                        flow: Down
                                        padding: Inset{ left: 18, top: 2, bottom: 4 }

                                        step_row_7_content := Label {
                                            width: Fill, height: Fit
                                            draw_text +: { color: #d2dae8, text_style: theme.font_regular { font_size: 9, line_spacing: 1.3 } }
                                            text: ""
                                        }
                                    }
                                }
                                step_row_8 := View {
                                    width: Fill, height: Fit
                                    flow: Down, spacing: 2

                                    step_row_8_header_row := View {
                                        width: Fill, height: Fit
                                        flow: Right, spacing: 6
                                        align: Align{ y: 0.0 }

                                        step_row_8_rail := View {
                                            width: 10, height: Fill
                                            flow: Down
                                            align: Align{ x: 0.5 }

                                            step_row_8_dot := RoundedView {
                                                width: 6, height: 6
                                                margin: Inset{ top: 4 }
                                                show_bg: true
                                                draw_bg +: { color: #444, border_radius: 3.0 }
                                            }
                                            step_row_8_line := View {
                                                width: 2, height: Fill
                                                margin: Inset{ top: 4 }
                                                show_bg: true
                                                draw_bg +: { color: #333 }
                                            }
                                        }

                                        step_row_8_header := Button {
                                            width: Fill, height: Fit
                                            padding: Inset{ left: 4, right: 6, top: 2, bottom: 2 }
                                            align: Align{ x: 0.0 }
                                            draw_bg +: { color: #0000, color_hover: #333, border_radius: 4.0, border_size: 0.0 }
                                            draw_text +: { color: #d2dae8, text_style: theme.font_regular { font_size: 9 } }
                                            text: ""
                                        }
                                    }

                                    step_row_8_body := View {
                                        visible: true
                                        width: Fill, height: Fit
                                        flow: Down
                                        padding: Inset{ left: 18, top: 2, bottom: 4 }

                                        step_row_8_content := Label {
                                            width: Fill, height: Fit
                                            draw_text +: { color: #d2dae8, text_style: theme.font_regular { font_size: 9, line_spacing: 1.3 } }
                                            text: ""
                                        }
                                    }
                                }
                                step_row_9 := View {
                                    width: Fill, height: Fit
                                    flow: Down, spacing: 2

                                    step_row_9_header_row := View {
                                        width: Fill, height: Fit
                                        flow: Right, spacing: 6
                                        align: Align{ y: 0.0 }

                                        step_row_9_rail := View {
                                            width: 10, height: Fill
                                            flow: Down
                                            align: Align{ x: 0.5 }

                                            step_row_9_dot := RoundedView {
                                                width: 6, height: 6
                                                margin: Inset{ top: 4 }
                                                show_bg: true
                                                draw_bg +: { color: #444, border_radius: 3.0 }
                                            }
                                            step_row_9_line := View {
                                                width: 2, height: Fill
                                                margin: Inset{ top: 4 }
                                                show_bg: true
                                                draw_bg +: { color: #333 }
                                            }
                                        }

                                        step_row_9_header := Button {
                                            width: Fill, height: Fit
                                            padding: Inset{ left: 4, right: 6, top: 2, bottom: 2 }
                                            align: Align{ x: 0.0 }
                                            draw_bg +: { color: #0000, color_hover: #333, border_radius: 4.0, border_size: 0.0 }
                                            draw_text +: { color: #d2dae8, text_style: theme.font_regular { font_size: 9 } }
                                            text: ""
                                        }
                                    }

                                    step_row_9_body := View {
                                        visible: true
                                        width: Fill, height: Fit
                                        flow: Down
                                        padding: Inset{ left: 18, top: 2, bottom: 4 }

                                        step_row_9_content := Label {
                                            width: Fill, height: Fit
                                            draw_text +: { color: #d2dae8, text_style: theme.font_regular { font_size: 9, line_spacing: 1.3 } }
                                            text: ""
                                        }
                                    }
                                }
                            }
                        }
                    }

                    markdown_view := View {
                        visible: false
                        width: Fill, height: Fit
                        msg_text := Markdown {
                            width: Fill, height: Fit
                            font_size: 10
                            font_color: #f2f6ff
                            paragraph_spacing: 8
                            pre_code_spacing: 6
                            use_code_block_widget: true
                        }
                    }

                    label_view := View {
                        visible: false
                        width: Fill, height: Fit
                        msg_label := Label {
                            width: Fill, height: Fit
                            draw_text +: {
                                color: #f2f6ff
                                text_style: theme.font_regular { font_size: 10, line_spacing: 1.4 }
                            }
                        }
                    }

                    error_label := Label {
                        width: Fill, height: Fit
                        text: ""
                        draw_text +: {
                            color: #ef4444
                            text_style: theme.font_regular { font_size: 9, line_spacing: 1.4 }
                        }
                    }

                    diff_view := DiffView {}

                    stats_row := View {
                        width: Fit, height: Fit
                        flow: Right,
                        spacing: 8,
                        margin: Inset{ top: 6 }
                        align: Align{ y: 0.5 }

                        tokens_label := Label {
                            width: Fit, height: Fit
                            draw_text +: {
                                color: #b8c2d3
                                text_style: theme.font_regular { font_size: 8 }
                            }
                            text: ""
                        }

                        cost_label := Label {
                            width: Fit, height: Fit
                            draw_text +: {
                                color: #b8c2d3
                                text_style: theme.font_regular { font_size: 8 }
                            }
                            text: ""
                        }
                    }

                    msg_actions := View {
                        width: Fit, height: Fit
                        flow: Right,
                        spacing: 6,
                        margin: Inset{ top: 8 }

                        copy_button := Button {
                            width: Fit, height: 20
                            text: "Copy"
                            draw_bg +: {
                                color: #0000
                                color_hover: #333
                                border_size: 0.0
                            }
                            draw_text +: {
                                color: #666
                                color_hover: #aaa
                                text_style: theme.font_regular { font_size: 8 }
                            }
                        }

                        revert_button := Button {
                             width: Fit, height: 20
                             text: "Revert"
                             draw_bg +: {
                                 color: #0000
                                 color_hover: #333
                                 border_size: 0.0
                             }
                             draw_text +: {
                                 color: #666
                                 color_hover: #aaa
                                 text_style: theme.font_regular { font_size: 8 }
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

#[derive(Script, ScriptHook, Widget)]
pub struct MessageList {
    #[source]
    source: ScriptObjectRef,

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
    #[rust]
    frame_count: usize,
    #[rust]
    last_timer_secs: u64,
    #[rust]
    cached_timer_text: String,
    #[rust]
    tail_mode: TailMode,
    #[rust]
    streaming_anim_item: Option<usize>,
    #[rust]
    cached_last_assistant_idx: Option<usize>,
    #[rust]
    cached_last_assistant_has_running: bool,
}

impl Widget for MessageList {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.handle_event_impl(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_walk_impl(cx, scope, walk)
    }
}

#[derive(Clone, Debug, Default)]
pub enum MessageListAction {
    #[default]
    None,
    RevertToMessage(String),
}
