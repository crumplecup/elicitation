//! Tests for `elicit_ratatui` serde types, From conversions, and TUI tree composition.

use elicit_ratatui::{
    AlignmentJson, AxisJson, BarChartParams, BarGroupJson, BarJson, BlockJson, BlockParams,
    BorderTypeJson, BordersJson, CellJson, ChartParams, ColorJson, ConstraintJson, DatasetJson,
    DirectionJson, EventJson, GaugeParams, GraphTypeJson, KeyEventJson, LegendPositionJson,
    LineGaugeParams, LineJson, ListParams, ListStateJson, MarginJson, MarkerJson, ModifierJson,
    MouseEventJson, PaddingJson, ParagraphParams, ParagraphText, RowJson, ScrollbarOrientationJson,
    ScrollbarParams, ScrollbarStateJson, SpanJson, SparklineParams, StyleJson, TableParams,
    TableStateJson, TabsParams, TextJson, TuiNode, WidgetJson,
};

// ---------------------------------------------------------------------------
// Colour round-trips
// ---------------------------------------------------------------------------

#[test]
fn test_color_named_round_trip() {
    let json = ColorJson::Red;
    let ratatui_color: ratatui::style::Color = json.clone().into();
    let back: ColorJson = ratatui_color.into();
    assert_eq!(json, back);
}

#[test]
fn test_color_rgb_round_trip() {
    let json = ColorJson::Rgb {
        r: 128,
        g: 64,
        b: 255,
    };
    let ratatui_color: ratatui::style::Color = json.clone().into();
    let back: ColorJson = ratatui_color.into();
    assert_eq!(json, back);
}

#[test]
fn test_color_indexed_round_trip() {
    let json = ColorJson::Indexed { index: 42 };
    let ratatui_color: ratatui::style::Color = json.clone().into();
    let back: ColorJson = ratatui_color.into();
    assert_eq!(json, back);
}

#[test]
fn test_color_reset_round_trip() {
    let json = ColorJson::Reset;
    let ratatui_color: ratatui::style::Color = json.clone().into();
    let back: ColorJson = ratatui_color.into();
    assert_eq!(json, back);
}

#[test]
fn test_color_serde_json() {
    let json = ColorJson::Rgb {
        r: 10,
        g: 20,
        b: 30,
    };
    let s = serde_json::to_string(&json).expect("serialize");
    let back: ColorJson = serde_json::from_str(&s).expect("deserialize");
    assert_eq!(json, back);
}

#[test]
fn test_color_all_named_variants() {
    let variants = [
        ColorJson::Reset,
        ColorJson::Black,
        ColorJson::Red,
        ColorJson::Green,
        ColorJson::Yellow,
        ColorJson::Blue,
        ColorJson::Magenta,
        ColorJson::Cyan,
        ColorJson::White,
        ColorJson::DarkGray,
        ColorJson::LightRed,
        ColorJson::LightGreen,
        ColorJson::LightYellow,
        ColorJson::LightBlue,
        ColorJson::LightMagenta,
        ColorJson::LightCyan,
        ColorJson::Gray,
    ];
    for v in &variants {
        let rt: ratatui::style::Color = v.clone().into();
        let back: ColorJson = rt.into();
        assert_eq!(*v, back);
    }
}

// ---------------------------------------------------------------------------
// Modifier
// ---------------------------------------------------------------------------

#[test]
fn test_modifier_to_ratatui() {
    assert_eq!(
        ModifierJson::Bold.to_modifier(),
        ratatui::style::Modifier::BOLD
    );
    assert_eq!(
        ModifierJson::Italic.to_modifier(),
        ratatui::style::Modifier::ITALIC
    );
    assert_eq!(
        ModifierJson::CrossedOut.to_modifier(),
        ratatui::style::Modifier::CROSSED_OUT
    );
}

#[test]
fn test_modifier_serde() {
    let m = ModifierJson::Underlined;
    let s = serde_json::to_string(&m).expect("serialize");
    let back: ModifierJson = serde_json::from_str(&s).expect("deserialize");
    assert_eq!(m, back);
}

// ---------------------------------------------------------------------------
// Style
// ---------------------------------------------------------------------------

#[test]
fn test_style_default_is_empty() {
    let s = StyleJson::default();
    assert!(s.fg.is_none());
    assert!(s.bg.is_none());
    assert!(s.modifiers.is_empty());
}

#[test]
fn test_style_to_ratatui() {
    let s = StyleJson {
        fg: Some(ColorJson::Red),
        bg: Some(ColorJson::Blue),
        modifiers: vec![ModifierJson::Bold, ModifierJson::Italic],
    };
    let rt: ratatui::style::Style = s.into();
    assert_eq!(rt.fg, Some(ratatui::style::Color::Red));
    assert_eq!(rt.bg, Some(ratatui::style::Color::Blue));
}

#[test]
fn test_style_serde_round_trip() {
    let s = StyleJson {
        fg: Some(ColorJson::Rgb { r: 1, g: 2, b: 3 }),
        bg: None,
        modifiers: vec![ModifierJson::Dim],
    };
    let json = serde_json::to_string(&s).expect("serialize");
    let back: StyleJson = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(s, back);
}

#[test]
fn test_style_empty_modifiers_skipped_in_json() {
    let s = StyleJson::default();
    let json = serde_json::to_string(&s).expect("serialize");
    assert!(!json.contains("modifiers"));
}

// ---------------------------------------------------------------------------
// Borders / BorderType
// ---------------------------------------------------------------------------

#[test]
fn test_borders_from() {
    assert_eq!(
        ratatui::widgets::Borders::from(BordersJson::All),
        ratatui::widgets::Borders::ALL
    );
    assert_eq!(
        ratatui::widgets::Borders::from(BordersJson::None),
        ratatui::widgets::Borders::NONE
    );
    assert_eq!(
        ratatui::widgets::Borders::from(BordersJson::Top),
        ratatui::widgets::Borders::TOP
    );
}

#[test]
fn test_border_type_from() {
    assert_eq!(
        ratatui::widgets::BorderType::from(BorderTypeJson::Rounded),
        ratatui::widgets::BorderType::Rounded
    );
    assert_eq!(
        ratatui::widgets::BorderType::from(BorderTypeJson::Double),
        ratatui::widgets::BorderType::Double
    );
}

// ---------------------------------------------------------------------------
// Padding / Margin
// ---------------------------------------------------------------------------

#[test]
fn test_padding_from() {
    let p = PaddingJson {
        left: 1,
        right: 2,
        top: 3,
        bottom: 4,
    };
    let rt: ratatui::widgets::Padding = p.into();
    assert_eq!(rt, ratatui::widgets::Padding::new(1, 2, 3, 4));
}

#[test]
fn test_margin_serde_defaults() {
    let json = r#"{"horizontal":0,"vertical":0}"#;
    let m: MarginJson = serde_json::from_str(json).expect("deserialize");
    assert_eq!(m.horizontal, 0);
    assert_eq!(m.vertical, 0);
}

// ---------------------------------------------------------------------------
// Direction
// ---------------------------------------------------------------------------

#[test]
fn test_direction_from() {
    assert_eq!(
        ratatui::layout::Direction::from(DirectionJson::Vertical),
        ratatui::layout::Direction::Vertical
    );
    assert_eq!(
        ratatui::layout::Direction::from(DirectionJson::Horizontal),
        ratatui::layout::Direction::Horizontal
    );
}

// ---------------------------------------------------------------------------
// Constraint
// ---------------------------------------------------------------------------

#[test]
fn test_constraint_length_from() {
    let c = ConstraintJson::Length { value: 10 };
    let rt: ratatui::layout::Constraint = c.into();
    assert_eq!(rt, ratatui::layout::Constraint::Length(10));
}

#[test]
fn test_constraint_percentage_from() {
    let c = ConstraintJson::Percentage { value: 50 };
    let rt: ratatui::layout::Constraint = c.into();
    assert_eq!(rt, ratatui::layout::Constraint::Percentage(50));
}

#[test]
fn test_constraint_ratio_from() {
    let c = ConstraintJson::Ratio { num: 1, den: 3 };
    let rt: ratatui::layout::Constraint = c.into();
    assert_eq!(rt, ratatui::layout::Constraint::Ratio(1, 3));
}

#[test]
fn test_constraint_fill_from() {
    let c = ConstraintJson::Fill { value: 1 };
    let rt: ratatui::layout::Constraint = c.into();
    assert_eq!(rt, ratatui::layout::Constraint::Fill(1));
}

#[test]
fn test_constraint_serde_tagged() {
    let c = ConstraintJson::Min { value: 5 };
    let json = serde_json::to_string(&c).expect("serialize");
    assert!(json.contains(r#""type":"Min"#));
    let back: ConstraintJson = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(c, back);
}

// ---------------------------------------------------------------------------
// Alignment
// ---------------------------------------------------------------------------

#[test]
fn test_alignment_round_trip() {
    let json = AlignmentJson::Center;
    let rt: ratatui::layout::Alignment = json.into();
    let back: AlignmentJson = rt.into();
    assert_eq!(back, AlignmentJson::Center);
}

// ---------------------------------------------------------------------------
// Block
// ---------------------------------------------------------------------------

#[test]
fn test_block_default_borders() {
    let json = r#"{"borders":"All"}"#;
    let b: BlockJson = serde_json::from_str(json).expect("deserialize");
    assert_eq!(b.borders, BordersJson::All);
}

#[test]
fn test_block_with_title_and_style() {
    let b = BlockJson {
        borders: BordersJson::All,
        border_type: Some(BorderTypeJson::Rounded),
        title: Some("My Block".to_string()),
        style: Some(StyleJson {
            fg: Some(ColorJson::White),
            bg: None,
            modifiers: vec![],
        }),
        border_style: None,
        padding: Some(PaddingJson {
            left: 1,
            right: 1,
            top: 0,
            bottom: 0,
        }),
    };
    let json = serde_json::to_string(&b).expect("serialize");
    let back: BlockJson = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(b, back);
}

// ---------------------------------------------------------------------------
// Table helpers
// ---------------------------------------------------------------------------

#[test]
fn test_row_cell_serde() {
    let row = RowJson {
        cells: vec![
            CellJson {
                content: "A".to_string(),
                style: None,
            },
            CellJson {
                content: "B".to_string(),
                style: Some(StyleJson {
                    fg: Some(ColorJson::Green),
                    bg: None,
                    modifiers: vec![],
                }),
            },
        ],
        height: Some(2),
        style: None,
    };
    let json = serde_json::to_string(&row).expect("serialize");
    let back: RowJson = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(row, back);
}

// ---------------------------------------------------------------------------
// State types
// ---------------------------------------------------------------------------

#[test]
fn test_list_state_default() {
    let s = ListStateJson::default();
    assert!(s.selected.is_none());
    assert_eq!(s.offset, 0);
}

#[test]
fn test_table_state_serde() {
    let s = TableStateJson {
        selected: Some(3),
        offset: 10,
    };
    let json = serde_json::to_string(&s).expect("serialize");
    let back: TableStateJson = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(s, back);
}

// ---------------------------------------------------------------------------
// Text composition
// ---------------------------------------------------------------------------

#[test]
fn test_span_serde() {
    let span = SpanJson {
        content: "hello".to_string(),
        style: None,
    };
    let json = serde_json::to_string(&span).expect("serialize");
    let back: SpanJson = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(span, back);
}

#[test]
fn test_line_from_spans() {
    let line = LineJson {
        spans: vec![
            SpanJson {
                content: "foo".to_string(),
                style: None,
            },
            SpanJson {
                content: "bar".to_string(),
                style: Some(StyleJson {
                    fg: Some(ColorJson::Red),
                    bg: None,
                    modifiers: vec![],
                }),
            },
        ],
        style: None,
        alignment: Some(AlignmentJson::Center),
    };
    let json = serde_json::to_string(&line).expect("serialize");
    let back: LineJson = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(line, back);
}

#[test]
fn test_text_from_lines() {
    let text = TextJson {
        lines: vec![LineJson {
            spans: vec![SpanJson {
                content: "line 1".to_string(),
                style: None,
            }],
            style: None,
            alignment: None,
        }],
        style: None,
        alignment: Some(AlignmentJson::Right),
    };
    let json = serde_json::to_string(&text).expect("serialize");
    let back: TextJson = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(text, back);
}

// ---------------------------------------------------------------------------
// WidgetJson variants
// ---------------------------------------------------------------------------

#[test]
fn test_widget_block_serde() {
    let w = WidgetJson::Block {
        block: BlockJson {
            borders: BordersJson::All,
            border_type: None,
            title: Some("Title".to_string()),
            style: None,
            border_style: None,
            padding: None,
        },
    };
    let json = serde_json::to_string(&w).expect("serialize");
    assert!(json.contains(r#""type":"Block"#));
    let back: WidgetJson = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(w, back);
}

#[test]
fn test_widget_paragraph_serde() {
    let w = WidgetJson::Paragraph {
        text: "Hello world".into(),
        style: None,
        wrap: true,
        scroll: Some((5, 0)),
        alignment: Some("Center".to_string()),
        block: None,
    };
    let json = serde_json::to_string(&w).expect("serialize");
    let back: WidgetJson = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(w, back);
}

#[test]
fn test_widget_list_serde() {
    let w = WidgetJson::List {
        items: vec!["Item 1".to_string(), "Item 2".to_string()],
        block: None,
        style: None,
        highlight_style: Some(StyleJson {
            fg: Some(ColorJson::Yellow),
            bg: None,
            modifiers: vec![ModifierJson::Bold],
        }),
        highlight_symbol: Some(">> ".to_string()),
        state: Some(ListStateJson {
            selected: Some(0),
            offset: 0,
        }),
    };
    let json = serde_json::to_string(&w).expect("serialize");
    let back: WidgetJson = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(w, back);
}

#[test]
fn test_widget_table_serde() {
    let w = WidgetJson::Table {
        header: Some(RowJson {
            cells: vec![CellJson {
                content: "Name".to_string(),
                style: None,
            }],
            height: None,
            style: None,
        }),
        rows: vec![RowJson {
            cells: vec![CellJson {
                content: "Alice".to_string(),
                style: None,
            }],
            height: None,
            style: None,
        }],
        widths: vec![ConstraintJson::Percentage { value: 100 }],
        column_spacing: None,
        block: None,
        highlight_style: None,
        highlight_symbol: None,
        state: None,
    };
    let json = serde_json::to_string(&w).expect("serialize");
    let back: WidgetJson = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(w, back);
}

#[test]
fn test_widget_gauge_serde() {
    let w = WidgetJson::Gauge {
        ratio: 0.75,
        label: Some("75%".to_string()),
        block: None,
        style: None,
        gauge_style: None,
    };
    let json = serde_json::to_string(&w).expect("serialize");
    let back: WidgetJson = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(w, back);
}

#[test]
fn test_widget_sparkline_serde() {
    let w = WidgetJson::Sparkline {
        data: vec![1, 3, 5, 2, 8],
        block: None,
        style: None,
        max: Some(10),
        direction: Some(DirectionJson::Horizontal),
    };
    let json = serde_json::to_string(&w).expect("serialize");
    let back: WidgetJson = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(w, back);
}

#[test]
fn test_widget_tabs_serde() {
    let w = WidgetJson::Tabs {
        titles: vec!["Tab1".to_string(), "Tab2".to_string()],
        selected: Some(0),
        block: None,
        style: None,
        highlight_style: None,
        divider: Some("|".to_string()),
    };
    let json = serde_json::to_string(&w).expect("serialize");
    let back: WidgetJson = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(w, back);
}

#[test]
fn test_widget_clear_serde() {
    let w = WidgetJson::Clear;
    let json = serde_json::to_string(&w).expect("serialize");
    let back: WidgetJson = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(w, back);
}

#[test]
fn test_widget_bar_chart_serde() {
    let w = WidgetJson::BarChart {
        data: vec![BarGroupJson {
            label: Some("Group".to_string()),
            bars: vec![BarJson {
                label: Some("A".to_string()),
                value: 42,
                style: None,
                value_style: None,
                text_value: None,
            }],
        }],
        block: None,
        max_value: None,
        bar_width: Some(3),
        bar_gap: Some(1),
        group_gap: None,
        bar_style: None,
        value_style: None,
        label_style: None,
        direction: None,
        style: None,
    };
    let json = serde_json::to_string(&w).expect("serialize");
    let back: WidgetJson = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(w, back);
}

#[test]
fn test_widget_chart_serde() {
    let w = WidgetJson::Chart {
        datasets: vec![DatasetJson {
            name: Some("Series 1".to_string()),
            data: vec![(0.0, 1.0), (1.0, 3.0), (2.0, 2.0)],
            style: None,
            marker: Some(MarkerJson::Braille),
            graph_type: Some(GraphTypeJson::Line),
        }],
        block: None,
        x_axis: Some(AxisJson {
            title: Some("X".to_string()),
            bounds: Some((0.0, 10.0)),
            labels: vec!["0".to_string(), "5".to_string(), "10".to_string()],
            style: None,
        }),
        y_axis: None,
        style: None,
        legend_position: Some(LegendPositionJson::TopRight),
    };
    let json = serde_json::to_string(&w).expect("serialize");
    let back: WidgetJson = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(w, back);
}

#[test]
fn test_widget_line_gauge_serde() {
    let w = WidgetJson::LineGauge {
        ratio: 0.5,
        label: None,
        block: None,
        style: None,
        filled_style: Some(StyleJson {
            fg: Some(ColorJson::Green),
            bg: None,
            modifiers: vec![],
        }),
        unfilled_style: None,
    };
    let json = serde_json::to_string(&w).expect("serialize");
    let back: WidgetJson = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(w, back);
}

#[test]
fn test_widget_scrollbar_serde() {
    let w = WidgetJson::Scrollbar {
        orientation: ScrollbarOrientationJson::VerticalRight,
        thumb_symbol: None,
        track_symbol: None,
        begin_symbol: None,
        end_symbol: None,
        style: None,
        thumb_style: None,
        track_style: None,
        state: Some(ScrollbarStateJson {
            content_length: 100,
            position: 25,
            viewport_content_length: None,
        }),
    };
    let json = serde_json::to_string(&w).expect("serialize");
    let back: WidgetJson = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(w, back);
}

// ---------------------------------------------------------------------------
// Event types
// ---------------------------------------------------------------------------

#[test]
fn test_key_event_serde() {
    let e = KeyEventJson {
        code: "Char(q)".to_string(),
        modifiers: vec!["CONTROL".to_string()],
    };
    let json = serde_json::to_string(&e).expect("serialize");
    let back: KeyEventJson = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(e, back);
}

#[test]
fn test_mouse_event_serde() {
    let e = MouseEventJson {
        kind: "Down(Left)".to_string(),
        column: 10,
        row: 20,
        modifiers: vec![],
    };
    let json = serde_json::to_string(&e).expect("serialize");
    let back: MouseEventJson = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(e, back);
}

#[test]
fn test_event_key_variant_serde() {
    let e = EventJson::Key {
        event: KeyEventJson {
            code: "Enter".to_string(),
            modifiers: vec![],
        },
    };
    let json = serde_json::to_string(&e).expect("serialize");
    assert!(json.contains(r#""type":"Key"#));
    let back: EventJson = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(e, back);
}

#[test]
fn test_event_resize_variant_serde() {
    let e = EventJson::Resize {
        width: 80,
        height: 24,
    };
    let json = serde_json::to_string(&e).expect("serialize");
    let back: EventJson = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(e, back);
}

#[test]
fn test_event_focus_variants_serde() {
    for e in [EventJson::FocusGained, EventJson::FocusLost] {
        let json = serde_json::to_string(&e).expect("serialize");
        let back: EventJson = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(e, back);
    }
}

#[test]
fn test_event_paste_serde() {
    let e = EventJson::Paste {
        text: "pasted text".into(),
    };
    let json = serde_json::to_string(&e).expect("serialize");
    let back: EventJson = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(e, back);
}

// ---------------------------------------------------------------------------
// TuiNode tree composition
// ---------------------------------------------------------------------------

#[test]
fn test_tui_node_widget_leaf() {
    let node = TuiNode::Widget {
        widget: Box::new(WidgetJson::Clear),
        proofs: Default::default(),
    };
    let json = serde_json::to_string(&node).expect("serialize");
    assert!(json.contains(r#""node_type":"Widget"#));
    let back: TuiNode = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(node, back);
}

#[test]
fn test_tui_node_layout_with_children() {
    let tree = TuiNode::Layout {
        direction: DirectionJson::Vertical,
        constraints: vec![
            ConstraintJson::Percentage { value: 50 },
            ConstraintJson::Percentage { value: 50 },
        ],
        children: vec![
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Paragraph {
                    text: "Top half".into(),
                    style: None,
                    wrap: false,
                    scroll: None,
                    alignment: None,
                    block: None,
                }),
                proofs: Default::default(),
            },
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Paragraph {
                    text: "Bottom half".into(),
                    style: None,
                    wrap: false,
                    scroll: None,
                    alignment: None,
                    block: None,
                }),
                proofs: Default::default(),
            },
        ],
        margin: None,
    };
    let json = serde_json::to_string(&tree).expect("serialize");
    let back: TuiNode = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(tree, back);
}

#[test]
fn test_tui_node_nested_layouts() {
    let tree = TuiNode::Layout {
        direction: DirectionJson::Vertical,
        constraints: vec![
            ConstraintJson::Length { value: 3 },
            ConstraintJson::Fill { value: 1 },
        ],
        children: vec![
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Block {
                    block: BlockJson {
                        borders: BordersJson::All,
                        border_type: None,
                        title: Some("Header".to_string()),
                        style: None,
                        border_style: None,
                        padding: None,
                    },
                }),
                proofs: Default::default(),
            },
            TuiNode::Layout {
                direction: DirectionJson::Horizontal,
                constraints: vec![
                    ConstraintJson::Percentage { value: 30 },
                    ConstraintJson::Percentage { value: 70 },
                ],
                children: vec![
                    TuiNode::Widget {
                        widget: Box::new(WidgetJson::List {
                            items: vec!["a".to_string(), "b".to_string()],
                            block: None,
                            style: None,
                            highlight_style: None,
                            highlight_symbol: None,
                            state: None,
                        }),
                        proofs: Default::default(),
                    },
                    TuiNode::Widget {
                        widget: Box::new(WidgetJson::Paragraph {
                            text: "Content".into(),
                            style: None,
                            wrap: true,
                            scroll: None,
                            alignment: None,
                            block: None,
                        }),
                        proofs: Default::default(),
                    },
                ],
                margin: None,
            },
        ],
        margin: Some(MarginJson {
            horizontal: 1,
            vertical: 1,
        }),
    };
    let json = serde_json::to_string(&tree).expect("serialize");
    let back: TuiNode = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(tree, back);
}

// ---------------------------------------------------------------------------
// Chart / BarChart helpers
// ---------------------------------------------------------------------------

#[test]
fn test_bar_json_serde() {
    let bar = BarJson {
        label: Some("Sales".to_string()),
        value: 100,
        style: None,
        value_style: None,
        text_value: Some("$100".to_string()),
    };
    let json = serde_json::to_string(&bar).expect("serialize");
    let back: BarJson = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(bar, back);
}

#[test]
fn test_dataset_json_serde() {
    let ds = DatasetJson {
        name: Some("Temperature".to_string()),
        data: vec![(0.0, 20.0), (1.0, 22.5), (2.0, 19.0)],
        style: None,
        marker: Some(MarkerJson::Dot),
        graph_type: Some(GraphTypeJson::Scatter),
    };
    let json = serde_json::to_string(&ds).expect("serialize");
    let back: DatasetJson = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(ds, back);
}

#[test]
fn test_axis_json_serde() {
    let axis = AxisJson {
        title: Some("Time".to_string()),
        bounds: Some((0.0, 100.0)),
        labels: vec!["0".to_string(), "50".to_string(), "100".to_string()],
        style: None,
    };
    let json = serde_json::to_string(&axis).expect("serialize");
    let back: AxisJson = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(axis, back);
}

// ---------------------------------------------------------------------------
// Scrollbar types
// ---------------------------------------------------------------------------

#[test]
fn test_scrollbar_orientation_serde() {
    let orientations = [
        ScrollbarOrientationJson::VerticalRight,
        ScrollbarOrientationJson::VerticalLeft,
        ScrollbarOrientationJson::HorizontalBottom,
        ScrollbarOrientationJson::HorizontalTop,
    ];
    for o in &orientations {
        let json = serde_json::to_string(o).expect("serialize");
        let back: ScrollbarOrientationJson = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(*o, back);
    }
}

#[test]
fn test_scrollbar_state_defaults() {
    let s = ScrollbarStateJson::default();
    assert_eq!(s.content_length, 0);
    assert_eq!(s.position, 0);
    assert!(s.viewport_content_length.is_none());
}

// ---------------------------------------------------------------------------
// Enum variant coverage
// ---------------------------------------------------------------------------

#[test]
fn test_graph_type_variants() {
    for g in [GraphTypeJson::Scatter, GraphTypeJson::Line] {
        let json = serde_json::to_string(&g).expect("serialize");
        let back: GraphTypeJson = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(g, back);
    }
}

#[test]
fn test_marker_variants() {
    for m in [
        MarkerJson::Dot,
        MarkerJson::Braille,
        MarkerJson::Block,
        MarkerJson::Bar,
        MarkerJson::HalfBlock,
    ] {
        let json = serde_json::to_string(&m).expect("serialize");
        let back: MarkerJson = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(m, back);
    }
}

#[test]
fn test_legend_position_variants() {
    for p in [
        LegendPositionJson::TopLeft,
        LegendPositionJson::TopRight,
        LegendPositionJson::BottomLeft,
        LegendPositionJson::BottomRight,
    ] {
        let json = serde_json::to_string(&p).expect("serialize");
        let back: LegendPositionJson = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(p, back);
    }
}

// ---------------------------------------------------------------------------
// JSON deserialization from raw strings
// ---------------------------------------------------------------------------

#[test]
fn test_widget_from_raw_json() {
    let raw = r#"{
        "type": "Gauge",
        "ratio": 0.42,
        "label": "Loading..."
    }"#;
    let w: WidgetJson = serde_json::from_str(raw).expect("deserialize");
    match w {
        WidgetJson::Gauge { ratio, label, .. } => {
            assert!((ratio - 0.42).abs() < f64::EPSILON);
            assert_eq!(label, Some("Loading...".to_string()));
        }
        other => panic!("Expected Gauge, got {other:?}"),
    }
}

#[test]
fn test_tui_node_from_raw_json() {
    let raw = r#"{
        "node_type": "Widget",
        "widget": {
            "type": "Clear"
        }
    }"#;
    let node: TuiNode = serde_json::from_str(raw).expect("deserialize");
    assert_eq!(
        node,
        TuiNode::Widget {
            widget: Box::new(WidgetJson::Clear),
            proofs: Default::default(),
        }
    );
}

#[test]
fn test_constraint_from_raw_json() {
    let raw = r#"{"type":"Ratio","num":2,"den":5}"#;
    let c: ConstraintJson = serde_json::from_str(raw).expect("deserialize");
    assert_eq!(c, ConstraintJson::Ratio { num: 2, den: 5 });
}

#[test]
fn test_color_rgb_from_raw_json() {
    let raw = r#"{"type":"Rgb","r":255,"g":0,"b":128}"#;
    let c: ColorJson = serde_json::from_str(raw).expect("deserialize");
    assert_eq!(
        c,
        ColorJson::Rgb {
            r: 255,
            g: 0,
            b: 128
        }
    );
}

// ---------------------------------------------------------------------------
// Widget param deserialization (simulating MCP input)
// ---------------------------------------------------------------------------

#[test]
fn test_block_params_from_json() {
    let json = r#"{"title": "Test", "borders": "All"}"#;
    let params: BlockParams = serde_json::from_str(json).unwrap();
    assert_eq!(params.title, Some("Test".to_string()));
    assert_eq!(params.borders, Some(BordersJson::All));
}

#[test]
fn test_block_params_minimal_json() {
    let json = r#"{}"#;
    let params: BlockParams = serde_json::from_str(json).unwrap();
    assert_eq!(params.title, None);
    assert_eq!(params.borders, None);
    assert_eq!(params.style, None);
}

#[test]
fn test_paragraph_params_from_json() {
    let json = r#"{"text": "Hello", "wrap": true, "alignment": "Center"}"#;
    let params: ParagraphParams = serde_json::from_str(json).unwrap();
    assert_eq!(params.text.to_plain_string(), "Hello");
    assert!(params.wrap);
    assert_eq!(params.alignment, Some("Center".to_string()));
}

#[test]
fn test_list_params_from_json() {
    let json = r#"{"items": ["a", "b", "c"], "highlight_symbol": ">> "}"#;
    let params: ListParams = serde_json::from_str(json).unwrap();
    assert_eq!(params.items.len(), 3);
    assert_eq!(params.highlight_symbol, Some(">> ".to_string()));
}

#[test]
fn test_table_params_from_json() {
    let json = r#"{
        "rows": [{"cells": [{"content": "a"}, {"content": "b"}]}],
        "widths": [{"type": "Length", "value": 10}, {"type": "Fill", "value": 1}]
    }"#;
    let params: TableParams = serde_json::from_str(json).unwrap();
    assert_eq!(params.rows.len(), 1);
    assert_eq!(params.widths.len(), 2);
    assert_eq!(params.rows[0].cells[0].content, "a");
}

#[test]
fn test_gauge_params_from_json() {
    let json = r#"{"ratio": 0.42, "label": "42%"}"#;
    let params: GaugeParams = serde_json::from_str(json).unwrap();
    assert!((params.ratio - 0.42).abs() < f64::EPSILON);
    assert_eq!(params.label, Some("42%".to_string()));
}

#[test]
fn test_sparkline_params_from_json() {
    let json = r#"{"data": [1, 2, 3, 4, 5], "max": 10}"#;
    let params: SparklineParams = serde_json::from_str(json).unwrap();
    assert_eq!(params.data, vec![1, 2, 3, 4, 5]);
    assert_eq!(params.max, Some(10));
}

#[test]
fn test_tabs_params_from_json() {
    let json = r#"{"titles": ["Home", "Settings"], "selected": 0}"#;
    let params: TabsParams = serde_json::from_str(json).unwrap();
    assert_eq!(params.titles, vec!["Home", "Settings"]);
    assert_eq!(params.selected, Some(0));
}

#[test]
fn test_chart_params_from_json() {
    let json = r#"{
        "datasets": [{
            "name": "data",
            "data": [[0.0, 1.0], [1.0, 2.0]],
            "graph_type": "Line",
            "marker": "Dot"
        }],
        "legend_position": "TopRight"
    }"#;
    let params: ChartParams = serde_json::from_str(json).unwrap();
    assert_eq!(params.datasets.len(), 1);
    assert_eq!(params.datasets[0].name, Some("data".to_string()));
    assert_eq!(params.legend_position, Some(LegendPositionJson::TopRight));
}

#[test]
fn test_bar_chart_params_from_json() {
    let json = r#"{
        "data": [{
            "label": "Group",
            "bars": [{"value": 10, "label": "A"}, {"value": 20}]
        }],
        "bar_width": 3,
        "direction": "Vertical"
    }"#;
    let params: BarChartParams = serde_json::from_str(json).unwrap();
    assert_eq!(params.data.len(), 1);
    assert_eq!(params.data[0].bars.len(), 2);
    assert_eq!(params.bar_width, Some(3));
    assert_eq!(params.direction, Some(DirectionJson::Vertical));
}

#[test]
fn test_line_gauge_params_from_json() {
    let json = r#"{"ratio": 0.8, "label": "80%"}"#;
    let params: LineGaugeParams = serde_json::from_str(json).unwrap();
    assert!((params.ratio - 0.8).abs() < f64::EPSILON);
    assert_eq!(params.label, Some("80%".to_string()));
}

#[test]
fn test_scrollbar_params_from_json() {
    let json = r#"{
        "orientation": "VerticalRight",
        "state": {"content_length": 200, "position": 50}
    }"#;
    let params: ScrollbarParams = serde_json::from_str(json).unwrap();
    assert_eq!(params.orientation, ScrollbarOrientationJson::VerticalRight);
    let state = params.state.unwrap();
    assert_eq!(state.content_length, 200);
    assert_eq!(state.position, 50);
}

#[test]
fn test_table_params_with_header_from_json() {
    let json = r#"{
        "rows": [{"cells": [{"content": "val1"}, {"content": "val2"}]}],
        "widths": [{"type": "Percentage", "value": 50}, {"type": "Percentage", "value": 50}],
        "header": {"cells": [{"content": "Col A"}, {"content": "Col B"}]},
        "column_spacing": 2
    }"#;
    let params: TableParams = serde_json::from_str(json).unwrap();
    assert!(params.header.is_some());
    let header = params.header.unwrap();
    assert_eq!(header.cells[0].content, "Col A");
    assert_eq!(params.column_spacing, Some(2));
}

#[test]
fn test_scrollbar_params_all_symbols() {
    let json = r#"{
        "orientation": "HorizontalBottom",
        "thumb_symbol": "█",
        "track_symbol": "─",
        "begin_symbol": "◄",
        "end_symbol": "►"
    }"#;
    let params: ScrollbarParams = serde_json::from_str(json).unwrap();
    assert_eq!(
        params.orientation,
        ScrollbarOrientationJson::HorizontalBottom
    );
    assert_eq!(params.thumb_symbol, Some("█".to_string()));
    assert_eq!(params.track_symbol, Some("─".to_string()));
    assert_eq!(params.begin_symbol, Some("◄".to_string()));
    assert_eq!(params.end_symbol, Some("►".to_string()));
}

// ---------------------------------------------------------------------------
// ParagraphText — plain and rich text
// ---------------------------------------------------------------------------

#[test]
fn paragraph_text_plain_roundtrip() {
    let w = WidgetJson::Paragraph {
        text: "hello".into(),
        style: None,
        wrap: false,
        scroll: None,
        alignment: None,
        block: None,
    };
    let json = serde_json::to_string(&w).unwrap();
    // Plain strings serialize as JSON strings, not objects.
    assert!(json.contains(r#""text":"hello""#), "got: {json}");
    let back: WidgetJson = serde_json::from_str(&json).unwrap();
    assert_eq!(w, back);
}

#[test]
fn paragraph_text_rich_serde() {
    let rich = TextJson {
        lines: vec![LineJson {
            spans: vec![
                SpanJson {
                    content: "host".into(),
                    style: Some(StyleJson {
                        fg: Some(ColorJson::Cyan),
                        bg: None,
                        modifiers: vec![],
                    }),
                },
                SpanJson {
                    content: ": message".into(),
                    style: None,
                },
            ],
            style: None,
            alignment: None,
        }],
        style: None,
        alignment: None,
    };
    let w = WidgetJson::Paragraph {
        text: ParagraphText::Rich(rich.clone()),
        style: None,
        wrap: true,
        scroll: None,
        alignment: None,
        block: None,
    };
    let json = serde_json::to_string(&w).unwrap();
    let back: WidgetJson = serde_json::from_str(&json).unwrap();
    assert_eq!(w, back);
}

#[test]
fn paragraph_text_to_plain_string() {
    let plain = ParagraphText::Plain("hello".into());
    assert_eq!(plain.to_plain_string(), "hello");

    let rich = ParagraphText::Rich(TextJson {
        lines: vec![
            LineJson {
                spans: vec![
                    SpanJson {
                        content: "foo".into(),
                        style: None,
                    },
                    SpanJson {
                        content: "bar".into(),
                        style: None,
                    },
                ],
                style: None,
                alignment: None,
            },
            LineJson {
                spans: vec![SpanJson {
                    content: "baz".into(),
                    style: None,
                }],
                style: None,
                alignment: None,
            },
        ],
        style: None,
        alignment: None,
    });
    assert_eq!(rich.to_plain_string(), "foobar\nbaz");
}

#[test]
fn paragraph_params_accepts_rich_text_json() {
    // ColorJson is internally tagged: {"type": "Cyan"}, not "Cyan"
    let json = r#"{
        "text": {
            "lines": [{
                "spans": [
                    {"content": "cyan", "style": {"fg": {"type": "Cyan"}}},
                    {"content": " white"}
                ]
            }]
        },
        "wrap": true
    }"#;
    let params: ParagraphParams = serde_json::from_str(json).unwrap();
    assert_eq!(params.text.to_plain_string(), "cyan white");
    assert!(params.wrap);
}

#[test]
fn debug_text_json_deser() {
    // ColorJson is internally tagged: use {"type": "Cyan"} not "Cyan"
    let text_json = r#"{"lines":[{"spans":[{"content":"cyan","style":{"fg":{"type":"Cyan"}}},{"content":" white"}]}]}"#;
    let r: Result<TextJson, _> = serde_json::from_str(text_json);
    assert!(r.is_ok(), "TextJson deser failed: {:?}", r.err());

    let pt: Result<ParagraphText, _> = serde_json::from_str(text_json);
    assert!(pt.is_ok(), "ParagraphText deser failed: {:?}", pt.err());
}
