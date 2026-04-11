//! Verus proof orchestration and tracking.
//!
//! This module provides functionality to run Verus verification proofs individually,
//! track their results in CSV format, and generate summary statistics.

use anyhow::{Context, Result};
use chrono::Utc;
use csv::{Reader, Writer};
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Instant;

/// A Verus proof module identifier.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Default, Serialize, Deserialize,
)]
pub struct VerusProof {
    /// Module name (e.g., "bools", "integers")
    module: String,
    /// Proof identifier (e.g., "verify_bool_true")
    name: String,
}

impl VerusProof {
    /// Create a new proof identifier.
    pub fn new(module: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            module: module.into(),
            name: name.into(),
        }
    }

    /// All available Verus proofs from the elicitation_verus crate.
    pub fn all() -> Vec<Self> {
        vec![
            // external_types (25 proofs)
            Self::new("external_types", "verify_datetime_after_construction"),
            Self::new("external_types", "verify_datetime_before_construction"),
            Self::new("external_types", "verify_datetime_construction"),
            Self::new("external_types", "verify_duration_construction"),
            Self::new("external_types", "verify_duration_positive_construction"),
            Self::new("external_types", "verify_ip_private_construction"),
            Self::new("external_types", "verify_ip_public_construction"),
            Self::new("external_types", "verify_ipaddr_construction"),
            Self::new("external_types", "verify_ipv4_construction"),
            Self::new("external_types", "verify_ipv6_construction"),
            Self::new("external_types", "verify_json_array_construction"),
            Self::new("external_types", "verify_json_non_null_construction"),
            Self::new("external_types", "verify_json_object_construction"),
            Self::new("external_types", "verify_json_value_construction"),
            Self::new("external_types", "verify_path_absolute_construction"),
            Self::new("external_types", "verify_path_relative_construction"),
            Self::new("external_types", "verify_pathbuf_construction"),
            Self::new(
                "external_types",
                "verify_regex_case_insensitive_construction",
            ),
            Self::new("external_types", "verify_regex_construction"),
            Self::new("external_types", "verify_url_construction"),
            Self::new("external_types", "verify_url_http_construction"),
            Self::new("external_types", "verify_url_https_construction"),
            Self::new("external_types", "verify_uuid_construction"),
            Self::new("external_types", "verify_uuid_non_nil_construction"),
            Self::new("external_types", "verify_uuid_v4_construction"),
            // primitives (13 proofs)
            Self::new("primitives", "verify_bool_construction"),
            Self::new("primitives", "verify_char_construction"),
            Self::new("primitives", "verify_f32_construction"),
            Self::new("primitives", "verify_f64_construction"),
            Self::new("primitives", "verify_i16_construction"),
            Self::new("primitives", "verify_i32_construction"),
            Self::new("primitives", "verify_i64_construction"),
            Self::new("primitives", "verify_i8_construction"),
            Self::new("primitives", "verify_u16_construction"),
            Self::new("primitives", "verify_u32_construction"),
            Self::new("primitives", "verify_u64_construction"),
            Self::new("primitives", "verify_u8_construction"),
            Self::new("primitives", "verify_unit_construction"),
            // stdlib_collections (11 proofs)
            Self::new("stdlib_collections", "verify_option_is_none_true"),
            Self::new("stdlib_collections", "verify_option_is_some_true"),
            Self::new("stdlib_collections", "verify_option_none"),
            Self::new("stdlib_collections", "verify_option_some"),
            Self::new("stdlib_collections", "verify_result_err"),
            Self::new("stdlib_collections", "verify_result_is_err_true"),
            Self::new("stdlib_collections", "verify_result_is_ok_true"),
            Self::new("stdlib_collections", "verify_result_ok"),
            Self::new("stdlib_collections", "verify_tuple2_construction"),
            Self::new("stdlib_collections", "verify_tuple3_construction"),
            Self::new("stdlib_collections", "verify_tuple4_construction"),
            // clap_types (26 proofs)
            Self::new("clap_types", "verify_color_choice_known_label_accepted"),
            Self::new("clap_types", "verify_color_choice_unknown_rejected"),
            Self::new("clap_types", "verify_color_choice_roundtrip_complete"),
            Self::new("clap_types", "verify_color_choice_label_count_matches"),
            Self::new("clap_types", "verify_arg_action_known_label_accepted"),
            Self::new("clap_types", "verify_arg_action_unknown_rejected"),
            Self::new("clap_types", "verify_arg_action_roundtrip_complete"),
            Self::new("clap_types", "verify_arg_action_label_count_matches"),
            Self::new("clap_types", "verify_value_source_known_label_accepted"),
            Self::new("clap_types", "verify_value_source_unknown_rejected"),
            Self::new("clap_types", "verify_value_source_roundtrip_complete"),
            Self::new("clap_types", "verify_value_source_label_count_matches"),
            Self::new("clap_types", "verify_error_kind_known_label_accepted"),
            Self::new("clap_types", "verify_error_kind_unknown_rejected"),
            Self::new("clap_types", "verify_error_kind_roundtrip_complete"),
            Self::new("clap_types", "verify_error_kind_label_count_matches"),
            Self::new("clap_types", "verify_value_hint_known_label_accepted"),
            Self::new("clap_types", "verify_value_hint_unknown_rejected"),
            Self::new("clap_types", "verify_value_hint_roundtrip_complete"),
            Self::new("clap_types", "verify_value_hint_label_count_matches"),
            Self::new("clap_types", "verify_clap_arg_trusted"),
            Self::new("clap_types", "verify_clap_arg_group_trusted"),
            Self::new("clap_types", "verify_clap_command_trusted"),
            Self::new("clap_types", "verify_clap_id_trusted"),
            Self::new("clap_types", "verify_clap_possible_value_trusted"),
            Self::new("clap_types", "verify_clap_value_range_trusted"),
            // sqlx_types (32 proofs)
            Self::new("sqlx_types", "verify_error_kind_known_label_accepted"),
            Self::new("sqlx_types", "verify_error_kind_unknown_rejected"),
            Self::new("sqlx_types", "verify_error_kind_roundtrip_complete"),
            Self::new("sqlx_types", "verify_error_kind_label_count_matches"),
            Self::new(
                "sqlx_types",
                "verify_any_type_info_kind_known_label_accepted",
            ),
            Self::new("sqlx_types", "verify_any_type_info_kind_unknown_rejected"),
            Self::new("sqlx_types", "verify_any_type_info_kind_roundtrip_complete"),
            Self::new(
                "sqlx_types",
                "verify_any_type_info_kind_label_count_matches",
            ),
            Self::new("sqlx_types", "verify_sql_type_kind_known_label_accepted"),
            Self::new("sqlx_types", "verify_sql_type_kind_unknown_rejected"),
            Self::new("sqlx_types", "verify_sql_type_kind_roundtrip_complete"),
            Self::new("sqlx_types", "verify_sql_type_kind_label_count_matches"),
            Self::new("sqlx_types", "verify_column_value_known_label_accepted"),
            Self::new("sqlx_types", "verify_column_value_unknown_rejected"),
            Self::new("sqlx_types", "verify_column_value_roundtrip_complete"),
            Self::new("sqlx_types", "verify_column_value_label_count_matches"),
            Self::new("sqlx_types", "verify_sql_type_kind_from_conversion_total"),
            Self::new("sqlx_types", "verify_sql_type_kind_roundtrip_faithful"),
            Self::new("sqlx_types", "verify_null_variant_identity_preserved"),
            Self::new("sqlx_types", "verify_column_value_is_null_iff_null_variant"),
            Self::new("sqlx_types", "verify_non_null_column_value_is_not_null"),
            Self::new("sqlx_types", "verify_row_data_column_count_preserved"),
            Self::new("sqlx_types", "verify_row_data_get_some_iff_name_present"),
            Self::new("sqlx_types", "verify_row_data_is_empty_iff_zero_columns"),
            Self::new("sqlx_types", "verify_pool_connect_succeeds_iff_url_valid"),
            Self::new("sqlx_types", "verify_execute_rows_affected_non_negative"),
            Self::new("sqlx_types", "verify_fetch_optional_none_iff_no_rows"),
            Self::new("sqlx_types", "verify_fetch_optional_some_iff_row_exists"),
            Self::new("sqlx_types", "verify_fetch_all_length_equals_row_count"),
            Self::new("sqlx_types", "verify_any_row_len_equals_column_count"),
            Self::new("sqlx_types", "verify_to_row_data_preserves_column_count"),
            Self::new("sqlx_types", "verify_column_names_match_columns"),
            Self::new("sqlx_types", "verify_driver_kind_known_label_accepted"),
            Self::new("sqlx_types", "verify_driver_kind_unknown_rejected"),
            Self::new("sqlx_types", "verify_driver_kind_roundtrip_complete"),
            Self::new("sqlx_types", "verify_driver_kind_label_count_matches"),
            Self::new("sqlx_types", "verify_to_sqlx_args_null_is_single_element"),
            Self::new("sqlx_types", "verify_to_sqlx_args_bool_is_single_element"),
            Self::new(
                "sqlx_types",
                "verify_to_sqlx_args_object_length_matches_fields",
            ),
            Self::new("sqlx_types", "verify_established_is_zero_sized"),
            Self::new("sqlx_types", "verify_and_combinator_is_zero_sized"),
            Self::new("sqlx_types", "verify_both_result_is_zero_sized"),
            // sqlx macro fragment Props (SqlxFragPlugin)
            Self::new("sqlx_types", "verify_query_fragment_emitted"),
            Self::new("sqlx_types", "verify_query_as_fragment_emitted"),
            Self::new("sqlx_types", "verify_query_scalar_fragment_emitted"),
            Self::new("sqlx_types", "verify_migrate_fragment_emitted"),
            Self::new("sqlx_types", "verify_fragment_props_zero_sized"),
            // tokio_types: async operation contracts + Prop zero-cost proofs
            Self::new("tokio_types", "verify_sleep_completed"),
            Self::new("tokio_types", "verify_timeout_resolved"),
            Self::new("tokio_types", "verify_permit_acquired"),
            Self::new("tokio_types", "verify_notification_received"),
            Self::new("tokio_types", "verify_barrier_reached"),
            Self::new("tokio_types", "verify_listener_bound"),
            Self::new("tokio_types", "verify_connection_accepted"),
            Self::new("tokio_types", "verify_stream_connected"),
            Self::new("tokio_types", "verify_data_received"),
            Self::new("tokio_types", "verify_file_read"),
            Self::new("tokio_types", "verify_file_written"),
            Self::new("tokio_types", "verify_dir_created"),
            Self::new("tokio_types", "verify_process_spawned"),
            Self::new("tokio_types", "verify_process_exited"),
            Self::new("tokio_types", "verify_stdin_written"),
            Self::new("tokio_types", "verify_message_sent"),
            Self::new("tokio_types", "verify_message_received"),
            Self::new("tokio_types", "verify_channel_closed"),
            Self::new("tokio_types", "verify_ctrl_c_received"),
            Self::new("tokio_types", "verify_signal_handler_registered"),
            Self::new("tokio_types", "verify_signal_received"),
            Self::new("tokio_types", "verify_duplex_created"),
            Self::new("tokio_types", "verify_bytes_copied"),
            Self::new("tokio_types", "verify_task_yielded"),
            Self::new("tokio_types", "verify_task_spawned"),
            Self::new("tokio_types", "verify_task_joined"),
            Self::new("tokio_types", "verify_task_aborted"),
            Self::new("tokio_types", "verify_runtime_flavored"),
            Self::new("tokio_types", "verify_unix_listener_bound"),
            Self::new("tokio_types", "verify_unix_connection_accepted"),
            Self::new("tokio_types", "verify_unix_stream_connected"),
            Self::new("tokio_types", "verify_unix_data_received"),
            Self::new("tokio_types", "verify_established_is_zero_sized"),
            Self::new("tokio_types", "verify_and_combinator_is_zero_sized"),
            Self::new("tokio_types", "verify_both_result_is_zero_sized"),
            Self::new("tokio_types", "verify_three_way_composition_is_zero_sized"),
            // ui_types: typestate UI verification proofs
            Self::new("ui_types", "verify_meets_min_target_size"),
            Self::new("ui_types", "verify_size_boundary"),
            Self::new("ui_types", "verify_size_both_dimensions"),
            Self::new("ui_types", "verify_no_overflow"),
            Self::new("ui_types", "verify_overflow_detected"),
            Self::new("ui_types", "verify_exact_fit"),
            Self::new("ui_types", "verify_label_non_empty"),
            Self::new("ui_types", "verify_propositions_zero_cost"),
            Self::new("ui_types", "verify_element_id_roundtrip"),
            Self::new("ui_types", "verify_empty_report"),
            Self::new("ui_types", "verify_level_subset"),
            // ui_types: renderer invariant proofs
            Self::new("ui_types", "verify_render_stats_sum"),
            Self::new("ui_types", "verify_progress_clamp"),
            Self::new("ui_types", "verify_heading_size_positive"),
            Self::new("ui_types", "verify_bounds_abs_non_negative"),
            Self::new("ui_types", "verify_render_stats_default"),
            Self::new("ui_types", "verify_stats_accounting"),
            // ui_types: LayoutBuilder invariant proofs
            Self::new("ui_types", "verify_builder_root_is_zero"),
            Self::new("ui_types", "verify_builder_empty_valid"),
            Self::new("ui_types", "verify_builder_node_count"),
            Self::new("ui_types", "verify_builder_container_count"),
            Self::new("ui_types", "verify_builder_stack_depth"),
            Self::new("ui_types", "verify_builder_auto_close"),
            Self::new("ui_types", "verify_builder_reset"),
            Self::new("ui_types", "verify_builder_default_eq_new"),
            Self::new("ui_types", "verify_builder_all_containers"),
            Self::new("ui_types", "verify_builder_id_uniqueness"),
            Self::new("ui_types", "verify_builder_composite_form"),
            // ratatui_types: select enum proofs
            Self::new(
                "ratatui_types",
                "verify_ratatui_alignment_known_label_accepted",
            ),
            Self::new("ratatui_types", "verify_ratatui_alignment_unknown_rejected"),
            Self::new(
                "ratatui_types",
                "verify_ratatui_alignment_roundtrip_complete",
            ),
            Self::new(
                "ratatui_types",
                "verify_ratatui_alignment_label_count_matches",
            ),
            Self::new(
                "ratatui_types",
                "verify_ratatui_direction_known_label_accepted",
            ),
            Self::new("ratatui_types", "verify_ratatui_direction_unknown_rejected"),
            Self::new(
                "ratatui_types",
                "verify_ratatui_direction_roundtrip_complete",
            ),
            Self::new(
                "ratatui_types",
                "verify_ratatui_direction_label_count_matches",
            ),
            Self::new(
                "ratatui_types",
                "verify_ratatui_border_type_known_label_accepted",
            ),
            Self::new(
                "ratatui_types",
                "verify_ratatui_border_type_unknown_rejected",
            ),
            Self::new(
                "ratatui_types",
                "verify_ratatui_border_type_roundtrip_complete",
            ),
            Self::new(
                "ratatui_types",
                "verify_ratatui_border_type_label_count_matches",
            ),
            Self::new("ratatui_types", "verify_ratatui_color_known_label_accepted"),
            Self::new("ratatui_types", "verify_ratatui_color_unknown_rejected"),
            Self::new("ratatui_types", "verify_ratatui_color_roundtrip_complete"),
            Self::new("ratatui_types", "verify_ratatui_color_label_count_matches"),
            Self::new(
                "ratatui_types",
                "verify_ratatui_borders_known_label_accepted",
            ),
            Self::new("ratatui_types", "verify_ratatui_borders_unknown_rejected"),
            Self::new("ratatui_types", "verify_ratatui_borders_roundtrip_complete"),
            Self::new(
                "ratatui_types",
                "verify_ratatui_borders_label_count_matches",
            ),
            Self::new(
                "ratatui_types",
                "verify_ratatui_scrollbar_orientation_known_label_accepted",
            ),
            Self::new(
                "ratatui_types",
                "verify_ratatui_scrollbar_orientation_unknown_rejected",
            ),
            Self::new(
                "ratatui_types",
                "verify_ratatui_scrollbar_orientation_roundtrip_complete",
            ),
            Self::new(
                "ratatui_types",
                "verify_ratatui_scrollbar_orientation_label_count_matches",
            ),
            // ratatui_types: composite struct proofs (shadow types)
            Self::new("ratatui_types", "verify_ratatui_padding_roundtrip"),
            Self::new("ratatui_types", "verify_ratatui_padding_concrete"),
            Self::new("ratatui_types", "verify_ratatui_margin_roundtrip"),
            Self::new("ratatui_types", "verify_ratatui_margin_concrete"),
            Self::new("ratatui_types", "verify_ratatui_style_modifiers"),
            Self::new("ratatui_types", "verify_ratatui_style_colors"),
            Self::new("ratatui_types", "verify_borders_select_roundtrip"),
            // geo_types: shadow struct proofs
            Self::new("geo_types", "verify_geo_coord_roundtrip"),
            Self::new("geo_types", "verify_geo_coord_concrete"),
            Self::new("geo_types", "verify_geo_rect_roundtrip"),
            Self::new("geo_types", "verify_geo_rect_concrete"),
            Self::new("geo_types", "verify_geo_rect_well_formed"),
            Self::new("geo_types", "verify_geo_line_roundtrip"),
            Self::new("geo_types", "verify_geo_line_concrete"),
            Self::new("geo_types", "verify_geo_line_degenerate"),
            Self::new("geo_types", "verify_geo_point_roundtrip"),
            Self::new("geo_types", "verify_geo_point_concrete"),
            Self::new("geo_types", "verify_geo_triangle_roundtrip"),
            Self::new("geo_types", "verify_geo_triangle_concrete"),
            Self::new("geo_types", "verify_geo_geometry_point_variant"),
            Self::new("geo_types", "verify_geo_geometry_rect_variant"),
            // wkt_types: shadow struct proofs
            Self::new("wkt_types", "verify_wkt_coord_roundtrip"),
            Self::new("wkt_types", "verify_wkt_coord_concrete"),
            Self::new("wkt_types", "verify_wkt_point_empty"),
            Self::new("wkt_types", "verify_wkt_geom_point_variant"),
            Self::new("wkt_types", "verify_wkt_string_trusted"),
            // wkb_types: shadow struct proofs
            Self::new("wkb_types", "verify_wkb_endianness_roundtrip"),
            Self::new("wkb_types", "verify_wkb_dimension_roundtrip"),
            Self::new("wkb_types", "verify_wkb_geometry_type_roundtrip"),
            Self::new("wkb_types", "verify_wkb_write_options_roundtrip"),
            Self::new("wkb_types", "verify_wkb_bytes_known_point"),
            // georaster_types: shadow struct proofs
            Self::new(
                "georaster_types",
                "verify_georaster_coordinate_new_semantics",
            ),
            Self::new(
                "georaster_types",
                "verify_georaster_planar_configuration_chunky",
            ),
            Self::new("georaster_types", "verify_georaster_color_type_rgb_bits"),
            Self::new(
                "georaster_types",
                "verify_georaster_raster_value_rgb8_variant",
            ),
            Self::new("georaster_types", "verify_georaster_image_info_fields"),
            // geojson_types: shadow struct proofs
            Self::new("geojson_types", "verify_geojson_value_point_type_name"),
            Self::new("geojson_types", "verify_geojson_geometry_new_point"),
            Self::new("geojson_types", "verify_geojson_feature_property_access"),
            Self::new("geojson_types", "verify_geojson_feature_collection_len"),
            Self::new("geojson_types", "verify_geojson_id_string_variant"),
            // palette_types: shadow struct proofs
            Self::new("palette_types", "verify_palette_srgb_roundtrip"),
            Self::new("palette_types", "verify_palette_srgb_concrete"),
            Self::new("palette_types", "verify_palette_srgb_black"),
            Self::new("palette_types", "verify_palette_srgb_white"),
            Self::new("palette_types", "verify_palette_srgb_red"),
            Self::new("palette_types", "verify_palette_srgb_green"),
            Self::new("palette_types", "verify_palette_srgb_blue"),
            Self::new("palette_types", "verify_palette_srgb_independence"),
            // egui_types: select enum proofs
            Self::new("egui_types", "verify_align_known_label_accepted"),
            Self::new("egui_types", "verify_align_unknown_rejected"),
            Self::new("egui_types", "verify_align_roundtrip_complete"),
            Self::new("egui_types", "verify_align_label_count_matches"),
            Self::new("egui_types", "verify_direction_known_label_accepted"),
            Self::new("egui_types", "verify_direction_unknown_rejected"),
            Self::new("egui_types", "verify_direction_roundtrip_complete"),
            Self::new("egui_types", "verify_direction_label_count_matches"),
            Self::new("egui_types", "verify_theme_known_label_accepted"),
            Self::new("egui_types", "verify_theme_unknown_rejected"),
            Self::new("egui_types", "verify_theme_roundtrip_complete"),
            Self::new("egui_types", "verify_theme_label_count_matches"),
            Self::new("egui_types", "verify_theme_preference_known_label_accepted"),
            Self::new("egui_types", "verify_theme_preference_unknown_rejected"),
            Self::new("egui_types", "verify_theme_preference_roundtrip_complete"),
            Self::new("egui_types", "verify_theme_preference_label_count_matches"),
            Self::new("egui_types", "verify_font_family_known_label_accepted"),
            Self::new("egui_types", "verify_font_family_unknown_rejected"),
            Self::new("egui_types", "verify_font_family_roundtrip_complete"),
            Self::new("egui_types", "verify_font_family_label_count_matches"),
            Self::new("egui_types", "verify_text_wrap_mode_known_label_accepted"),
            Self::new("egui_types", "verify_text_wrap_mode_unknown_rejected"),
            Self::new("egui_types", "verify_text_wrap_mode_roundtrip_complete"),
            Self::new("egui_types", "verify_text_wrap_mode_label_count_matches"),
            Self::new("egui_types", "verify_texture_filter_known_label_accepted"),
            Self::new("egui_types", "verify_texture_filter_unknown_rejected"),
            Self::new("egui_types", "verify_texture_filter_roundtrip_complete"),
            Self::new("egui_types", "verify_texture_filter_label_count_matches"),
            Self::new(
                "egui_types",
                "verify_texture_wrap_mode_known_label_accepted",
            ),
            Self::new("egui_types", "verify_texture_wrap_mode_unknown_rejected"),
            Self::new("egui_types", "verify_texture_wrap_mode_roundtrip_complete"),
            Self::new("egui_types", "verify_texture_wrap_mode_label_count_matches"),
            Self::new("egui_types", "verify_touch_phase_known_label_accepted"),
            Self::new("egui_types", "verify_touch_phase_unknown_rejected"),
            Self::new("egui_types", "verify_touch_phase_roundtrip_complete"),
            Self::new("egui_types", "verify_touch_phase_label_count_matches"),
            Self::new("egui_types", "verify_pointer_button_known_label_accepted"),
            Self::new("egui_types", "verify_pointer_button_unknown_rejected"),
            Self::new("egui_types", "verify_pointer_button_roundtrip_complete"),
            Self::new("egui_types", "verify_pointer_button_label_count_matches"),
            Self::new("egui_types", "verify_order_known_label_accepted"),
            Self::new("egui_types", "verify_order_unknown_rejected"),
            Self::new("egui_types", "verify_order_roundtrip_complete"),
            Self::new("egui_types", "verify_order_label_count_matches"),
            Self::new("egui_types", "verify_text_style_known_label_accepted"),
            Self::new("egui_types", "verify_text_style_unknown_rejected"),
            Self::new("egui_types", "verify_text_style_roundtrip_complete"),
            Self::new("egui_types", "verify_text_style_label_count_matches"),
            Self::new("egui_types", "verify_ui_kind_known_label_accepted"),
            Self::new("egui_types", "verify_ui_kind_unknown_rejected"),
            Self::new("egui_types", "verify_ui_kind_roundtrip_complete"),
            Self::new("egui_types", "verify_ui_kind_label_count_matches"),
            Self::new("egui_types", "verify_widget_type_known_label_accepted"),
            Self::new("egui_types", "verify_widget_type_unknown_rejected"),
            Self::new("egui_types", "verify_widget_type_roundtrip_complete"),
            Self::new("egui_types", "verify_widget_type_label_count_matches"),
            Self::new("egui_types", "verify_cursor_icon_known_label_accepted"),
            Self::new("egui_types", "verify_cursor_icon_unknown_rejected"),
            Self::new("egui_types", "verify_cursor_icon_roundtrip_complete"),
            Self::new("egui_types", "verify_cursor_icon_label_count_matches"),
            Self::new("egui_types", "verify_key_known_label_accepted"),
            Self::new("egui_types", "verify_key_unknown_rejected"),
            Self::new("egui_types", "verify_key_roundtrip_complete"),
            Self::new("egui_types", "verify_key_label_count_matches"),
            // egui_types: composite struct proofs (shadow types)
            Self::new("egui_types", "verify_color32_roundtrip"),
            Self::new("egui_types", "verify_color32_concrete"),
            Self::new("egui_types", "verify_corner_radius_roundtrip"),
            Self::new("egui_types", "verify_corner_radius_concrete"),
            Self::new("egui_types", "verify_margin_roundtrip"),
            Self::new("egui_types", "verify_margin_concrete"),
            // egui_types: float-field composites (boolean stubs)
            Self::new("egui_types", "verify_pos2_roundtrip"),
            Self::new("egui_types", "verify_vec2_roundtrip"),
            Self::new("egui_types", "verify_rect_roundtrip"),
            Self::new("egui_types", "verify_stroke_roundtrip"),
            Self::new("egui_types", "verify_shadow_roundtrip"),
            Self::new("egui_types", "verify_font_id_roundtrip"),
            // ui_types: CssLength shadow proofs
            Self::new("ui_types", "verify_css_px_not_zoom_invariant"),
            Self::new("ui_types", "verify_css_em_zoom_invariant"),
            Self::new("ui_types", "verify_css_rem_zoom_invariant"),
            Self::new("ui_types", "verify_css_vw_zoom_invariant"),
            Self::new("ui_types", "verify_css_vh_zoom_invariant"),
            Self::new("ui_types", "verify_css_percent_zoom_invariant"),
            // ui_types: BoundingBox / LayoutMode shadow proofs
            Self::new("ui_types", "verify_layout_mode_default_is_block"),
            Self::new("ui_types", "verify_bbox_touch_target_44x44"),
            Self::new("ui_types", "verify_bbox_touch_target_43x43"),
            Self::new("ui_types", "verify_bbox_touch_target_large"),
            Self::new("ui_types", "verify_bbox_within_viewport"),
            Self::new("ui_types", "verify_bbox_exceeds_viewport"),
            // ui_types: WCAG contrast threshold proofs
            Self::new("ui_types", "verify_contrast_aa_normal_threshold"),
            Self::new("ui_types", "verify_contrast_aa_large_threshold"),
            Self::new("ui_types", "verify_contrast_aaa_normal_threshold"),
            Self::new("ui_types", "verify_contrast_aaa_large_threshold"),
            Self::new("ui_types", "verify_aaa_stricter_than_aa_normal"),
            Self::new("ui_types", "verify_aaa_stricter_than_aa_large"),
            // ui_types: ConstraintProfile shadow proofs
            Self::new("ui_types", "verify_profile_a_count"),
            Self::new("ui_types", "verify_profile_aa_count"),
            Self::new("ui_types", "verify_profile_aaa_count"),
            Self::new("ui_types", "verify_profile_monotonicity"),
        ]
    }
}

impl std::fmt::Display for VerusProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}::{}", self.module, self.name)
    }
}

/// Result of running a single Verus proof.
#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
pub struct VerusProofResult {
    /// Module name
    #[serde(rename = "Module")]
    module: String,
    /// Proof name
    #[serde(rename = "Proof")]
    proof: String,
    /// Verification status
    #[serde(rename = "Status")]
    status: VerificationStatus,
    /// Time taken in seconds
    #[serde(rename = "Time_Seconds")]
    time_seconds: u64,
    /// Timestamp of verification
    #[serde(rename = "Timestamp")]
    timestamp: String,
    /// Error message (if failed)
    #[serde(rename = "Error_Message")]
    error_message: String,
}

/// Verification status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum VerificationStatus {
    /// Proof verified successfully
    Success,
    /// Proof failed verification
    Failed,
    /// Verification timed out
    Timeout,
    /// Error running verifier
    Error,
}

impl VerusProofResult {
    /// Create a new proof result.
    pub fn new(
        module: impl Into<String>,
        proof: impl Into<String>,
        status: VerificationStatus,
        time_seconds: u64,
        error_message: impl Into<String>,
    ) -> Self {
        Self {
            module: module.into(),
            proof: proof.into(),
            status,
            time_seconds,
            timestamp: Utc::now().to_rfc3339(),
            error_message: error_message.into(),
        }
    }

    /// Whether the proof succeeded.
    pub fn is_success(&self) -> bool {
        self.status == VerificationStatus::Success
    }
}

/// Run a single Verus proof by running entire elicitation_verus crate and extracting results.
pub fn run_verus_proof(
    proof: &VerusProof,
    verus_path: &Path,
    timeout_secs: Option<u64>,
) -> Result<VerusProofResult> {
    let start = Instant::now();

    // Run Verus on the entire elicitation_verus crate with JSON output
    let crate_path = Path::new("crates/elicitation_verus/src/lib.rs");

    let mut cmd = Command::new(verus_path);
    cmd.arg("--crate-type=lib")
        .arg("--output-json")
        .arg(crate_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    if let Some(timeout) = timeout_secs {
        cmd.env("VERUS_TIMEOUT", timeout.to_string());
    }

    let output = cmd
        .output()
        .with_context(|| format!("Failed to execute Verus on {}", proof))?;

    let elapsed = start.elapsed().as_secs();

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Parse JSON output to extract result for this specific proof
    let (status, error_message) = if output.status.success() {
        match parse_verus_json_for_proof(&stdout, proof) {
            Ok(true) => (VerificationStatus::Success, String::new()),
            Ok(false) => (
                VerificationStatus::Failed,
                "Proof verification failed".to_string(),
            ),
            Err(e) => (
                VerificationStatus::Failed,
                format!("JSON parsing error: {}\nstderr: {}", e, stderr),
            ),
        }
    } else if stderr.contains("timeout") || stdout.contains("timeout") {
        (
            VerificationStatus::Timeout,
            "Verification timed out".to_string(),
        )
    } else {
        (
            VerificationStatus::Failed,
            format!("Verus execution failed\nstderr: {}", stderr),
        )
    };

    Ok(VerusProofResult::new(
        proof.module(),
        proof.name(),
        status,
        elapsed,
        error_message,
    ))
}

/// Parse Verus JSON output to check if a specific proof passed.
fn parse_verus_json_for_proof(output: &str, proof: &VerusProof) -> Result<bool> {
    // Find the JSON object in the output (starts after text output)
    let lines: Vec<&str> = output.lines().collect();
    let mut json_lines = Vec::new();
    let mut in_json = false;
    let mut brace_count = 0;

    for line in lines {
        if !in_json && line.trim().starts_with('{') {
            in_json = true;
        }

        if in_json {
            json_lines.push(line);
            brace_count += line.matches('{').count() as i32;
            brace_count -= line.matches('}').count() as i32;

            if brace_count == 0 {
                break;
            }
        }
    }

    if json_lines.is_empty() {
        anyhow::bail!("No JSON found in Verus output");
    }

    let json_str = json_lines.join("\n");
    let data: Value =
        serde_json::from_str(&json_str).context("Failed to parse Verus JSON output")?;

    let func_details = data
        .get("func-details")
        .and_then(|v| v.as_object())
        .context("Missing func-details in JSON")?;

    // Build the expected function name: lib::module::function_name
    let expected_name = format!("lib::{}::{}", proof.module(), proof.name());

    // Find the proof in func-details
    if let Some(details) = func_details.get(&expected_name) {
        let failed_notes = details
            .get("failed_proof_notes")
            .and_then(|v| v.as_array())
            .context("Missing failed_proof_notes")?;

        // If failed_proof_notes is empty, the proof passed
        Ok(failed_notes.is_empty())
    } else {
        anyhow::bail!("Proof {} not found in Verus output", expected_name);
    }
}

/// Run all Verus proofs and track results.
pub fn run_all_proofs(
    verus_path: &Path,
    output_csv: &Path,
    timeout_secs: Option<u64>,
    resume: bool,
) -> Result<VerusSummary> {
    println!("🔬 Running Verus verification proofs...");
    println!("   Verus: {}", verus_path.display());
    println!("   Output: {}", output_csv.display());
    if let Some(t) = timeout_secs {
        println!("   Timeout: {}s per proof", t);
    }
    println!();

    // Load existing results if resuming
    let mut completed_proofs = std::collections::HashSet::new();
    if resume && output_csv.exists() {
        println!("📂 Loading existing results...");
        let mut reader = Reader::from_path(output_csv)
            .with_context(|| format!("Failed to read CSV: {}", output_csv.display()))?;
        for result in reader.deserialize::<VerusProofResult>().flatten() {
            if result.is_success() {
                completed_proofs.insert(format!("{}::{}", result.module(), result.proof()));
            }
        }
        println!("   Found {} completed proofs", completed_proofs.len());
        println!();
    }

    // Create CSV writer
    let mut writer = Writer::from_path(output_csv)
        .with_context(|| format!("Failed to create CSV: {}", output_csv.display()))?;

    let proofs = VerusProof::all();
    let total = proofs.len();
    let mut passed = 0;
    let mut failed = 0;
    let mut errors = 0;
    let mut skipped = 0;

    for (i, proof) in proofs.iter().enumerate() {
        let proof_id = format!("{}::{}", proof.module(), proof.name());

        if completed_proofs.contains(&proof_id) {
            println!(
                "[{:3}/{:3}] ⏭️  Skipping {} (already passed)",
                i + 1,
                total,
                proof_id
            );
            skipped += 1;
            continue;
        }

        print!("[{:3}/{:3}] 🔬 Verifying {}... ", i + 1, total, proof_id);
        std::io::stdout().flush().ok();

        match run_verus_proof(proof, verus_path, timeout_secs) {
            Ok(result) => {
                match result.status() {
                    VerificationStatus::Success => {
                        println!("✅ PASS ({}s)", result.time_seconds());
                        passed += 1;
                    }
                    VerificationStatus::Failed => {
                        println!("❌ FAIL ({}s)", result.time_seconds());
                        failed += 1;
                    }
                    VerificationStatus::Timeout => {
                        println!("⏱️  TIMEOUT ({}s)", result.time_seconds());
                        errors += 1;
                    }
                    VerificationStatus::Error => {
                        println!("🔥 ERROR ({}s)", result.time_seconds());
                        errors += 1;
                    }
                }

                writer
                    .serialize(&result)
                    .with_context(|| format!("Failed to write result for {}", proof))?;
                writer.flush()?;
            }
            Err(e) => {
                println!("🔥 ERROR: {}", e);
                errors += 1;
            }
        }
    }

    println!();
    println!("📊 Summary:");
    println!("   Total:   {}", total);
    println!("   Passed:  {} ✅", passed);
    println!("   Failed:  {} ❌", failed);
    println!("   Errors:  {} 🔥", errors);
    println!("   Skipped: {} ⏭️", skipped);
    println!();
    println!("Results saved to: {}", output_csv.display());

    Ok(VerusSummary::new(total, passed, failed, errors, skipped))
}

/// Summary statistics for Verus verification.
#[derive(Debug, Clone, Getters)]
pub struct VerusSummary {
    /// Total number of proofs
    total: usize,
    /// Number of passed proofs
    passed: usize,
    /// Number of failed proofs
    failed: usize,
    /// Number of errors
    errors: usize,
    /// Number of skipped proofs
    skipped: usize,
}

impl VerusSummary {
    /// Create a new summary.
    pub fn new(total: usize, passed: usize, failed: usize, errors: usize, skipped: usize) -> Self {
        Self {
            total,
            passed,
            failed,
            errors,
            skipped,
        }
    }

    /// Calculate success rate percentage.
    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.passed as f64 / self.total as f64) * 100.0
        }
    }
}

/// Load and summarize results from CSV.
pub fn summarize_results(csv_path: &Path) -> Result<VerusSummary> {
    let mut reader = Reader::from_path(csv_path)
        .with_context(|| format!("Failed to read CSV: {}", csv_path.display()))?;

    let mut passed = 0;
    let mut failed = 0;
    let mut errors = 0;

    for result in reader.deserialize::<VerusProofResult>() {
        let result = result?;
        match result.status() {
            VerificationStatus::Success => passed += 1,
            VerificationStatus::Failed => failed += 1,
            VerificationStatus::Timeout | VerificationStatus::Error => errors += 1,
        }
    }

    let total = passed + failed + errors;

    Ok(VerusSummary::new(total, passed, failed, errors, 0))
}

/// List all failed proofs from CSV.
pub fn list_failed_proofs(csv_path: &Path) -> Result<Vec<VerusProofResult>> {
    let mut reader = Reader::from_path(csv_path)
        .with_context(|| format!("Failed to read CSV: {}", csv_path.display()))?;

    let mut failed = Vec::new();

    for result in reader.deserialize::<VerusProofResult>() {
        let result = result?;
        if !result.is_success() {
            failed.push(result);
        }
    }

    Ok(failed)
}
