//! Creusot verification proofs for elicitation contract types.
//!
//! This crate contains pure Rust proofs that can be verified by Creusot.
//! It imports contract types from the main elicitation crate but avoids
//! async code that Creusot cannot handle.
//!
//! All proof functions are public and serve as documentation of verification coverage.

#![forbid(unsafe_code)]

// Creusot attributes (glob needed for prelude macros: requires, ensures, trusted, etc.)
pub use creusot_std::prelude::*;

// Trusted logic functions for elicitation types (logic-callable in contracts)
mod logic_fns;
#[cfg(creusot)]
pub use logic_fns::{
    char_is_alphabetic, char_is_alphanumeric, char_is_numeric, duration_is_positive, i8pos_get,
    i8pos_inner, ipv4_is_broadcast, ipv4_is_loopback, ipv4_is_multicast, ipv4_is_unspecified,
    ipv4_octets, ipv4addr_first_octet, ipv4addr_second_octet, ipv6_is_loopback,
    ipv6_is_unspecified, ipv6_octets, ipv6addr_is_loopback, ipv6addr_is_private, mac_is_broadcast,
    mac_is_local, mac_is_multicast, mac_is_null, mac_is_unicast, mac_is_universal, mac_octets,
    path_is_empty, path_len, utf8_is_empty, utf8_len, v4_port, v6_port,
};

// Extern spec axioms for elicitation constructors (enables non-trusted proofs)
#[cfg(creusot)]
mod extern_specs;

// Module declarations
mod bools;
mod chars;
mod collections;
mod durations;
mod floats;
mod integers;
mod networks;
mod paths;
mod strings;
mod tuples;

// Trenchcoat verification types (internal wrappers)
mod ipaddr_bytes;
mod macaddr;
mod mechanisms;
mod socketaddr;
mod utf8;

#[cfg(unix)]
mod pathbytes;

// Feature-gated module declarations
#[cfg(feature = "uuid")]
mod uuids;

#[cfg(feature = "uuid")]
mod uuid_bytes;

#[cfg(feature = "serde_json")]
mod values;

#[cfg(feature = "serde_json")]
mod serde_boundary;

#[cfg(feature = "url")]
mod urls;

#[cfg(feature = "url")]
mod urlbytes;

#[cfg(feature = "regex")]
mod regexes;

#[cfg(feature = "regex")]
mod regexbytes;

#[cfg(feature = "chrono")]
mod datetimes_chrono;

#[cfg(feature = "time")]
mod datetimes_time;

#[cfg(feature = "jiff")]
mod datetimes_jiff;

#[cfg(feature = "reqwest")]
mod http;

#[cfg(feature = "clap-types")]
mod clap_types;

#[cfg(feature = "sqlx-types")]
mod sqlx_types;

#[cfg(feature = "tokio-types")]
mod tokio_types;

#[cfg(feature = "egui-types")]
mod egui_types;

#[cfg(feature = "ratatui-types")]
mod ratatui_types;

#[cfg(feature = "geo-types")]
mod geo_types;

#[cfg(feature = "georaster-types")]
mod georaster_types;

#[cfg(feature = "geojson-types")]
mod geojson_types;

#[cfg(feature = "rstar-types")]
mod rstar_types;

#[cfg(feature = "proj-types")]
mod proj_types;

#[cfg(feature = "wkt-types")]
mod wkt_types;

#[cfg(feature = "wkb-types")]
mod wkb_types;

#[cfg(feature = "winit-types")]
mod winit_types;

#[cfg(feature = "wgpu-types")]
mod wgpu_types;

#[cfg(feature = "palette")]
mod palette_types;

#[cfg(feature = "ui-types")]
mod ui_types;

// Re-export all proof functions with explicit names (no globs)

pub use bools::{
    verify_bool_false_invalid, verify_bool_false_valid, verify_bool_true_invalid,
    verify_bool_true_valid,
};

pub use collections::{
    verify_arc_non_null_valid, verify_arc_satisfies_valid, verify_array_all_satisfy_valid,
    verify_box_non_null_valid, verify_box_satisfies_valid, verify_btreemap_non_empty_invalid,
    verify_btreemap_non_empty_valid, verify_btreeset_non_empty_invalid,
    verify_btreeset_non_empty_valid, verify_hashmap_non_empty_invalid,
    verify_hashmap_non_empty_valid, verify_hashset_non_empty_invalid,
    verify_hashset_non_empty_valid, verify_linkedlist_non_empty_invalid,
    verify_linkedlist_non_empty_valid, verify_option_some_invalid, verify_option_some_valid,
    verify_rc_non_null_valid, verify_rc_satisfies_valid, verify_result_ok_invalid,
    verify_result_ok_valid, verify_vec_all_satisfy_valid, verify_vec_non_empty_invalid,
    verify_vec_non_empty_valid, verify_vecdeque_non_empty_invalid, verify_vecdeque_non_empty_valid,
};

pub use floats::{
    verify_f32_finite_invalid, verify_f32_finite_valid, verify_f32_non_negative_invalid,
    verify_f32_non_negative_valid, verify_f32_positive_invalid, verify_f32_positive_valid,
    verify_f64_finite_invalid, verify_f64_finite_valid, verify_f64_non_negative_invalid,
    verify_f64_non_negative_valid, verify_f64_positive_invalid, verify_f64_positive_valid,
};

pub use integers::{
    verify_i8_non_negative_invalid, verify_i8_non_negative_valid, verify_i8_non_zero_invalid,
    verify_i8_non_zero_valid, verify_i8_positive_accessor, verify_i8_positive_invalid,
    verify_i8_positive_valid, verify_i8_range_invalid, verify_i8_range_valid,
    verify_i16_non_negative_valid, verify_i16_non_zero_valid, verify_i16_positive_valid,
    verify_i16_range_valid, verify_i32_non_negative_valid, verify_i32_non_zero_valid,
    verify_i32_positive_valid, verify_i32_range_valid, verify_i64_non_negative_valid,
    verify_i64_non_zero_valid, verify_i64_positive_valid, verify_i64_range_valid,
    verify_i128_non_negative_valid, verify_i128_non_zero_valid, verify_i128_positive_valid,
    verify_isize_non_negative_valid, verify_isize_non_zero_valid, verify_isize_positive_valid,
    verify_isize_range_valid, verify_u8_non_zero_invalid, verify_u8_non_zero_valid,
    verify_u8_positive_invalid, verify_u8_positive_valid, verify_u8_range_valid,
    verify_u16_non_zero_valid, verify_u16_positive_valid, verify_u16_range_valid,
    verify_u32_non_zero_valid, verify_u32_positive_valid, verify_u32_range_valid,
    verify_u64_non_zero_valid, verify_u64_positive_valid, verify_u64_range_valid,
    verify_u128_non_zero_valid, verify_u128_positive_valid, verify_usize_non_zero_valid,
    verify_usize_positive_valid, verify_usize_range_valid,
};

pub use networks::{
    verify_ip_private_invalid, verify_ip_private_valid, verify_ip_public_invalid,
    verify_ip_public_valid, verify_ipv4_invalid, verify_ipv4_loopback_invalid,
    verify_ipv4_loopback_valid, verify_ipv4_valid, verify_ipv6_invalid,
    verify_ipv6_loopback_invalid, verify_ipv6_loopback_valid, verify_ipv6_valid,
};

pub use paths::{
    verify_pathbuf_exists_invalid, verify_pathbuf_exists_valid, verify_pathbuf_isdir_invalid,
    verify_pathbuf_isdir_valid, verify_pathbuf_isfile_invalid, verify_pathbuf_isfile_valid,
    verify_pathbuf_readable_invalid, verify_pathbuf_readable_valid,
};

pub use strings::{
    verify_string_non_empty_bounded_valid, verify_string_non_empty_invalid,
    verify_string_non_empty_too_long, verify_string_non_empty_valid,
};

pub use tuples::{
    verify_tuple2_accessors, verify_tuple2_valid, verify_tuple3_into_inner, verify_tuple3_valid,
    verify_tuple4_into_inner, verify_tuple4_valid,
};

#[cfg(creusot)]
pub use chars::{
    verify_char_alphabetic_invalid, verify_char_alphabetic_valid, verify_char_alphanumeric_invalid,
    verify_char_alphanumeric_valid, verify_char_numeric_invalid, verify_char_numeric_valid,
};

#[cfg(creusot)]
pub use durations::{verify_duration_positive_invalid, verify_duration_positive_valid};

#[cfg(creusot)]
pub use ipaddr_bytes::{
    verify_ipv4_172_15_boundary, verify_ipv4_172_32_boundary, verify_ipv4_broadcast,
    verify_ipv4_construction, verify_ipv4_localhost, verify_ipv4_multicast,
    verify_ipv4_octets_accessor, verify_ipv4_private_10_network, verify_ipv4_private_10_valid,
    verify_ipv4_private_172_16_valid, verify_ipv4_private_172_31_valid,
    verify_ipv4_private_172_network, verify_ipv4_private_192_168_valid,
    verify_ipv4_private_192_network, verify_ipv4_private_get, verify_ipv4_public,
    verify_ipv4_public_cloudflare, verify_ipv4_public_get, verify_ipv4_public_google_dns,
    verify_ipv4_unspecified, verify_ipv6_construction, verify_ipv6_fb00_boundary,
    verify_ipv6_fe00_boundary, verify_ipv6_localhost, verify_ipv6_multicast,
    verify_ipv6_private_fc00, verify_ipv6_private_fc00_valid, verify_ipv6_private_fd00,
    verify_ipv6_private_fd00_valid, verify_ipv6_private_get, verify_ipv6_public,
    verify_ipv6_public_cloudflare, verify_ipv6_public_get, verify_ipv6_public_google_dns,
    verify_ipv6_segments_accessor, verify_ipv6_unspecified, verify_is_ipv4_private_10,
    verify_is_ipv4_private_172, verify_is_ipv4_private_192, verify_is_ipv4_private_public,
    verify_is_ipv6_private_fc00, verify_is_ipv6_private_fd00, verify_is_ipv6_private_public,
};

#[cfg(creusot)]
pub use macaddr::{
    verify_is_multicast_01, verify_is_multicast_03, verify_is_multicast_broadcast,
    verify_is_unicast_00, verify_is_unicast_02, verify_is_universal_00, verify_is_universal_01,
    verify_mac_00_unicast_universal, verify_mac_01_multicast_universal,
    verify_mac_02_unicast_local, verify_mac_03_multicast_local, verify_mac_all_ones,
    verify_mac_all_zeros, verify_mac_alternating, verify_mac_broadcast, verify_mac_cisco_oui,
    verify_mac_construction, verify_mac_even_first_octet, verify_mac_intel_oui,
    verify_mac_multicast_local, verify_mac_multicast_universal, verify_mac_null,
    verify_mac_octets_accessor, verify_mac_odd_first_octet, verify_mac_sequential,
    verify_mac_unicast_local, verify_mac_unicast_universal,
};

#[cfg(creusot)]
pub use mechanisms::{
    verify_mechanism_trenchcoat_preservation, verify_mechanism_type_composition,
    verify_trenchcoat_identity_preservation,
};

#[cfg(creusot)]
pub use socketaddr::{
    verify_port_0_not_nonzero, verify_port_0_privileged, verify_port_0_well_known,
    verify_port_1_nonzero, verify_port_80_nonzero, verify_port_80_well_known,
    verify_port_443_well_known, verify_port_1023_privileged, verify_port_1023_well_known,
    verify_port_1024_not_privileged, verify_port_1024_registered, verify_port_3000_registered,
    verify_port_5432_registered, verify_port_49151_registered, verify_port_49152_dynamic,
    verify_port_65535_dynamic, verify_socket_v4_construction, verify_socket_v4_dev_server,
    verify_socket_v4_into_parts, verify_socket_v4_ip_accessor, verify_socket_v4_localhost_http,
    verify_socket_v4_localhost_https, verify_socket_v4_max_max, verify_socket_v4_port_accessor,
    verify_socket_v4_ssh, verify_socket_v4_zero_zero, verify_socket_v6_construction,
    verify_socket_v6_into_parts, verify_socket_v6_ip_accessor, verify_socket_v6_localhost_http,
    verify_socket_v6_localhost_https, verify_socket_v6_max_max, verify_socket_v6_port_accessor,
    verify_socket_v6_zero_zero,
};

#[cfg(creusot)]
pub use utf8::{
    verify_ascii_valid, verify_empty_utf8, verify_length_overflow, verify_max_length_boundary,
    verify_utf8_len_accessor, verify_utf8_length_check, verify_utf8_length_valid,
};

#[cfg(all(unix, creusot))]
pub use pathbytes::{
    verify_absolute_as_str, verify_absolute_get_accessor, verify_absolute_length_check,
    verify_absolute_root, verify_absolute_with_leading_slash, verify_non_empty_as_str,
    verify_non_empty_get_accessor, verify_non_empty_length_check, verify_non_empty_multi_char,
    verify_non_empty_rejects_empty, verify_non_empty_single_char, verify_path_as_str_valid,
    verify_path_ascii, verify_path_current_dir, verify_path_empty, verify_path_empty_predicate,
    verify_path_large_buffer, verify_path_len_accessor, verify_path_length_check,
    verify_path_length_valid, verify_path_medium_buffer, verify_path_non_empty_predicate,
    verify_path_parent_dir, verify_path_root, verify_path_single_byte, verify_path_small_buffer,
    verify_relative_as_str, verify_relative_current_dir, verify_relative_filename,
    verify_relative_get_accessor, verify_relative_length_check, verify_relative_no_leading_slash,
    verify_relative_parent_dir,
};

#[cfg(feature = "uuid")]
pub use uuids::{
    verify_uuid_non_nil_invalid, verify_uuid_non_nil_valid, verify_uuid_v4_invalid,
    verify_uuid_v4_valid,
};

#[cfg(all(feature = "uuid", creusot))]
pub use uuid_bytes::{
    verify_has_valid_variant_80, verify_has_valid_variant_bf, verify_has_version_4,
    verify_has_version_7, verify_is_valid_v4_accepts, verify_is_valid_v7_accepts,
    verify_uuid_bytes_accessor, verify_uuid_has_version, verify_uuid_maximal_v4,
    verify_uuid_maximal_v7, verify_uuid_minimal_v4, verify_uuid_minimal_v7,
    verify_uuid_v4_accepts_valid, verify_uuid_v4_bytes, verify_uuid_v4_example, verify_uuid_v4_get,
    verify_uuid_v4_randomish, verify_uuid_v4_version, verify_uuid_v7_accepts_valid,
    verify_uuid_v7_bytes, verify_uuid_v7_example, verify_uuid_v7_get, verify_uuid_v7_timestamp,
    verify_uuid_v7_version, verify_uuid_v7_with_timestamp, verify_uuid_valid_variant,
    verify_uuid_version_1, verify_uuid_version_2, verify_uuid_version_3, verify_uuid_version_5,
    verify_uuid_version_extraction, verify_variant_10xx_lower, verify_variant_10xx_upper,
};

#[cfg(feature = "serde_json")]
pub use values::{
    verify_value_array_invalid, verify_value_array_valid, verify_value_non_null_invalid,
    verify_value_non_null_valid, verify_value_object_invalid, verify_value_object_valid,
};

#[cfg(feature = "serde_json")]
pub use serde_boundary::{
    verify_f64_finite_serde_valid, verify_f64_non_negative_serde_invalid,
    verify_f64_non_negative_serde_valid, verify_f64_positive_serde_invalid,
    verify_f64_positive_serde_valid, verify_i8_non_negative_serde_invalid,
    verify_i8_non_negative_serde_valid, verify_i8_non_zero_serde_invalid,
    verify_i8_non_zero_serde_valid, verify_i8_positive_round_trip,
    verify_i8_positive_serde_invalid, verify_i8_positive_serde_valid,
    verify_i16_non_negative_serde_invalid, verify_i16_non_negative_serde_valid,
    verify_i16_non_zero_serde_invalid, verify_i16_non_zero_serde_valid,
    verify_i16_positive_serde_invalid, verify_i16_positive_serde_valid,
    verify_string_non_empty_serde_empty, verify_string_non_empty_serde_valid,
    verify_u8_non_zero_serde_invalid, verify_u8_non_zero_serde_valid,
    verify_u8_positive_serde_invalid, verify_u8_positive_serde_valid,
    verify_u16_non_zero_serde_invalid, verify_u16_non_zero_serde_valid,
    verify_u16_positive_serde_invalid, verify_u16_positive_serde_valid,
};

// serde_boundary url functions require both serde_json AND url features.
#[cfg(all(feature = "serde_json", feature = "url"))]
pub use serde_boundary::{
    verify_url_http_serde_invalid, verify_url_http_serde_valid, verify_url_https_serde_invalid,
    verify_url_https_serde_valid, verify_url_valid_serde_invalid, verify_url_valid_serde_valid,
    verify_url_with_host_serde_valid,
};

#[cfg(feature = "url")]
pub use urls::{
    verify_url_can_be_base_invalid, verify_url_can_be_base_valid, verify_url_http_invalid,
    verify_url_http_valid, verify_url_https_invalid, verify_url_https_valid,
    verify_url_valid_invalid, verify_url_valid_valid, verify_url_with_host_invalid,
    verify_url_with_host_valid,
};

#[cfg(all(feature = "url", creusot))]
pub use urlbytes::{
    verify_authority_as_str, verify_authority_empty, verify_authority_ip, verify_authority_ip_port,
    verify_authority_length_check, verify_authority_length_valid, verify_authority_localhost,
    verify_authority_simple, verify_authority_with_port, verify_file_url, verify_ftp_url,
    verify_http_url, verify_https_url, verify_scheme_as_str, verify_scheme_empty,
    verify_scheme_file, verify_scheme_ftp, verify_scheme_http, verify_scheme_https,
    verify_scheme_is_http, verify_scheme_length_check, verify_scheme_length_valid,
    verify_scheme_with_dash, verify_scheme_with_dot, verify_scheme_with_plus,
    verify_url_absolute_get, verify_url_absolute_http, verify_url_absolute_length_check,
    verify_url_as_str, verify_url_http_get, verify_url_http_http, verify_url_http_https,
    verify_url_http_length_check, verify_url_large_buffer, verify_url_length_check,
    verify_url_length_valid, verify_url_medium_buffer, verify_url_small_buffer,
    verify_url_with_authority_get, verify_url_with_authority_http,
    verify_url_with_authority_length_check, verify_url_with_authority_port,
    verify_url_with_fragment, verify_url_with_path, verify_url_with_port, verify_url_with_query,
};

#[cfg(feature = "regex")]
pub use regexes::{
    verify_regex_case_insensitive_invalid, verify_regex_case_insensitive_valid,
    verify_regex_multiline_invalid, verify_regex_multiline_valid,
    verify_regex_set_non_empty_invalid, verify_regex_set_non_empty_valid,
    verify_regex_set_valid_invalid, verify_regex_set_valid_valid, verify_regex_valid_invalid,
    verify_regex_valid_valid,
};

#[cfg(all(feature = "regex", creusot))]
pub use regexbytes::{
    verify_balanced_as_str, verify_balanced_braces, verify_balanced_brackets,
    verify_balanced_empty, verify_balanced_length_check, verify_balanced_length_valid,
    verify_balanced_nested, verify_balanced_simple, verify_charclass_as_str,
    verify_charclass_escape, verify_charclass_length_check, verify_charclass_length_valid,
    verify_charclass_negated, verify_charclass_range, verify_charclass_set, verify_escape_digit,
    verify_escape_dot, verify_escape_newline, verify_escape_tab, verify_escape_word,
    verify_escapes_as_str, verify_escapes_length_check, verify_escapes_length_valid,
    verify_quantifier_exact, verify_quantifier_plus, verify_quantifier_question,
    verify_quantifier_range, verify_quantifier_star, verify_quantifiers_as_str,
    verify_quantifiers_length_check, verify_quantifiers_length_valid, verify_regex_alternation,
    verify_regex_as_str, verify_regex_charclass, verify_regex_complex, verify_regex_empty,
    verify_regex_escapes, verify_regex_groups, verify_regex_large_buffer,
    verify_regex_length_check, verify_regex_length_valid, verify_regex_literal,
    verify_regex_medium_buffer, verify_regex_quantifiers, verify_regex_small_buffer,
};

#[cfg(feature = "chrono")]
pub use datetimes_chrono::{
    verify_datetime_utc_after_invalid, verify_datetime_utc_after_valid,
    verify_datetime_utc_before_invalid, verify_datetime_utc_before_valid,
    verify_naive_datetime_after_invalid, verify_naive_datetime_after_valid,
};

#[cfg(feature = "time")]
pub use datetimes_time::{
    verify_offset_datetime_after_invalid, verify_offset_datetime_after_valid,
    verify_offset_datetime_before_invalid, verify_offset_datetime_before_valid,
};

#[cfg(feature = "jiff")]
pub use datetimes_jiff::{
    verify_timestamp_after_invalid, verify_timestamp_after_valid, verify_timestamp_before_invalid,
    verify_timestamp_before_valid,
};

#[cfg(feature = "reqwest")]
pub use http::{
    verify_status_code_invalid_99, verify_status_code_invalid_1000, verify_status_code_valid_200,
    verify_status_code_valid_404,
};

// clap_types re-exports verify_error_kind_* which also exist in sqlx_types.
// Only re-export from clap_types to avoid ambiguity; sqlx versions accessible
// via sqlx_types::verify_error_kind_*.
#[cfg(feature = "clap-types")]
pub use clap_types::{
    verify_arg_action_all_labels_roundtrip, verify_arg_action_known_label_accepted,
    verify_arg_action_label_count, verify_arg_action_unknown_rejected,
    verify_clap_arg_group_trusted, verify_clap_arg_trusted, verify_clap_command_trusted,
    verify_clap_id_trusted, verify_clap_possible_value_trusted, verify_clap_value_range_trusted,
    verify_color_choice_all_labels_roundtrip, verify_color_choice_known_label_accepted,
    verify_color_choice_label_count, verify_color_choice_unknown_rejected,
    verify_error_kind_all_labels_roundtrip, verify_error_kind_known_label_accepted,
    verify_error_kind_label_count, verify_error_kind_unknown_rejected,
    verify_value_hint_all_labels_roundtrip, verify_value_hint_known_label_accepted,
    verify_value_hint_label_count, verify_value_hint_unknown_rejected,
    verify_value_source_all_labels_roundtrip, verify_value_source_known_label_accepted,
    verify_value_source_label_count, verify_value_source_unknown_rejected,
};

// sqlx_types: verify_error_kind_* renamed to verify_sqlx_error_kind_* to avoid
// name collision with clap_types.
#[cfg(feature = "sqlx-types")]
pub use sqlx_types::{
    verify_and_combinator_is_zero_sized, verify_any_type_info_kind_all_labels_roundtrip,
    verify_any_type_info_kind_known_label_accepted, verify_any_type_info_kind_label_count,
    verify_any_type_info_kind_unknown_rejected, verify_driver_kind_known_label_accepted,
    verify_driver_kind_label_count, verify_driver_kind_unknown_rejected,
    verify_established_is_zero_sized, verify_fragment_props_zero_sized,
    verify_migrate_fragment_emitted_contract, verify_query_as_fragment_emitted_contract,
    verify_query_fragment_emitted_contract, verify_query_scalar_fragment_emitted_contract,
    verify_sql_type_kind_all_labels_roundtrip, verify_sql_type_kind_from_any_type_info_kind_total,
    verify_sql_type_kind_label_count, verify_sql_type_kind_unknown_rejected,
    verify_sqlx_error_kind_all_labels_roundtrip, verify_sqlx_error_kind_known_label_accepted,
    verify_sqlx_error_kind_label_count, verify_sqlx_error_kind_unknown_rejected,
};

// sqlx_types functions that also require serde_json feature.
#[cfg(all(feature = "sqlx-types", feature = "serde_json"))]
pub use sqlx_types::{
    verify_to_sqlx_args_bool_is_single_element, verify_to_sqlx_args_null_is_single_element,
};

// tokio_types: verify_established/and renamed to verify_tokio_* to avoid
// collision with sqlx_types.
#[cfg(feature = "tokio-types")]
pub use tokio_types::{
    verify_barrier_reached_contract, verify_bytes_copied_contract, verify_channel_closed_contract,
    verify_connection_accepted_contract, verify_ctrl_c_received_contract,
    verify_data_received_contract, verify_dir_created_contract, verify_duplex_created_contract,
    verify_file_read_contract, verify_file_written_contract, verify_listener_bound_contract,
    verify_message_received_contract, verify_message_sent_contract,
    verify_notification_received_contract, verify_permit_acquired_contract,
    verify_process_exited_contract, verify_process_spawned_contract,
    verify_runtime_flavored_contract, verify_signal_handler_registered_contract,
    verify_signal_received_contract, verify_sleep_completed_contract,
    verify_stdin_written_contract, verify_stream_connected_contract, verify_task_aborted_contract,
    verify_task_joined_contract, verify_task_spawned_contract, verify_task_yielded_contract,
    verify_timeout_resolved_contract, verify_tokio_and_combinator_is_zero_sized,
    verify_tokio_established_is_zero_sized, verify_unix_connection_accepted_contract,
    verify_unix_data_received_contract, verify_unix_listener_bound_contract,
    verify_unix_stream_connected_contract,
};

#[cfg(feature = "egui-types")]
pub use egui_types::{
    verify_align_all_labels_roundtrip, verify_align_known_label_accepted, verify_align_label_count,
    verify_align_unknown_rejected, verify_color32_concrete, verify_color32_roundtrip,
    verify_corner_radius_concrete, verify_corner_radius_roundtrip,
    verify_cursor_icon_all_labels_roundtrip, verify_cursor_icon_known_label_accepted,
    verify_cursor_icon_label_count, verify_cursor_icon_unknown_rejected,
    verify_direction_all_labels_roundtrip, verify_direction_known_label_accepted,
    verify_direction_label_count, verify_direction_unknown_rejected,
    verify_font_family_all_labels_roundtrip, verify_font_family_known_label_accepted,
    verify_font_family_label_count, verify_font_family_unknown_rejected,
    verify_font_id_monospace_roundtrip, verify_font_id_proportional_roundtrip,
    verify_key_all_labels_roundtrip, verify_key_known_label_accepted, verify_key_label_count,
    verify_key_unknown_rejected, verify_margin_concrete, verify_margin_roundtrip,
    verify_order_all_labels_roundtrip, verify_order_known_label_accepted, verify_order_label_count,
    verify_order_unknown_rejected, verify_pointer_button_all_labels_roundtrip,
    verify_pointer_button_known_label_accepted, verify_pointer_button_label_count,
    verify_pointer_button_unknown_rejected, verify_pos2_from_roundtrip, verify_rect_from_roundtrip,
    verify_shadow_from_roundtrip, verify_stroke_from_roundtrip,
    verify_text_style_all_labels_roundtrip, verify_text_style_known_label_accepted,
    verify_text_style_label_count, verify_text_style_unknown_rejected,
    verify_text_wrap_mode_all_labels_roundtrip, verify_text_wrap_mode_known_label_accepted,
    verify_text_wrap_mode_label_count, verify_text_wrap_mode_unknown_rejected,
    verify_texture_filter_all_labels_roundtrip, verify_texture_filter_known_label_accepted,
    verify_texture_filter_label_count, verify_texture_filter_unknown_rejected,
    verify_texture_wrap_mode_all_labels_roundtrip, verify_texture_wrap_mode_known_label_accepted,
    verify_texture_wrap_mode_label_count, verify_texture_wrap_mode_unknown_rejected,
    verify_theme_all_labels_roundtrip, verify_theme_known_label_accepted, verify_theme_label_count,
    verify_theme_preference_all_labels_roundtrip, verify_theme_preference_known_label_accepted,
    verify_theme_preference_label_count, verify_theme_preference_unknown_rejected,
    verify_theme_unknown_rejected, verify_touch_phase_all_labels_roundtrip,
    verify_touch_phase_known_label_accepted, verify_touch_phase_label_count,
    verify_touch_phase_unknown_rejected, verify_ui_kind_all_labels_roundtrip,
    verify_ui_kind_known_label_accepted, verify_ui_kind_label_count,
    verify_ui_kind_unknown_rejected, verify_vec2_from_roundtrip,
    verify_widget_type_all_labels_roundtrip, verify_widget_type_known_label_accepted,
    verify_widget_type_label_count, verify_widget_type_unknown_rejected,
};

#[cfg(feature = "ratatui-types")]
pub use ratatui_types::{
    verify_borders_select_into_inner, verify_ratatui_alignment_all_labels_roundtrip,
    verify_ratatui_alignment_known_label_accepted, verify_ratatui_alignment_label_count,
    verify_ratatui_alignment_unknown_rejected, verify_ratatui_border_type_all_labels_roundtrip,
    verify_ratatui_border_type_known_label_accepted, verify_ratatui_border_type_label_count,
    verify_ratatui_border_type_unknown_rejected, verify_ratatui_borders_all_labels_roundtrip,
    verify_ratatui_borders_known_label_accepted, verify_ratatui_borders_label_count,
    verify_ratatui_borders_unknown_rejected, verify_ratatui_color_all_labels_roundtrip,
    verify_ratatui_color_known_label_accepted, verify_ratatui_color_label_count,
    verify_ratatui_color_unknown_rejected, verify_ratatui_direction_all_labels_roundtrip,
    verify_ratatui_direction_known_label_accepted, verify_ratatui_direction_label_count,
    verify_ratatui_direction_unknown_rejected, verify_ratatui_margin_concrete,
    verify_ratatui_margin_roundtrip, verify_ratatui_padding_concrete,
    verify_ratatui_padding_roundtrip, verify_ratatui_scrollbar_orientation_all_labels_roundtrip,
    verify_ratatui_scrollbar_orientation_known_label_accepted,
    verify_ratatui_scrollbar_orientation_label_count,
    verify_ratatui_scrollbar_orientation_unknown_rejected, verify_ratatui_style_all_modifiers,
    verify_ratatui_style_empty_roundtrip, verify_ratatui_style_fg_bg_presence,
};

#[cfg(feature = "geo-types")]
pub use geo_types::{
    verify_geo_coord_concrete, verify_geo_coord_roundtrip, verify_geo_geometry_point_variant,
    verify_geo_geometry_rect_variant, verify_geo_line_degenerate, verify_geo_line_roundtrip,
    verify_geo_line_string_concrete, verify_geo_point_concrete, verify_geo_point_roundtrip,
    verify_geo_rect_roundtrip, verify_geo_rect_well_formed, verify_geo_triangle_concrete,
    verify_geo_triangle_roundtrip,
};

#[cfg(feature = "geojson-types")]
pub use geojson_types::{
    verify_geojson_feature_collection_len, verify_geojson_feature_property_access,
    verify_geojson_geometry_new_point, verify_geojson_id_string_variant,
    verify_geojson_value_point_type_name,
};

#[cfg(feature = "rstar-types")]
pub use rstar_types::{
    verify_rstar_aabb_roundtrip, verify_rstar_line_envelope_bounds, verify_rstar_line_roundtrip,
    verify_rstar_rectangle_envelope_bounds, verify_rstar_rectangle_roundtrip,
};

#[cfg(feature = "proj-types")]
pub use proj_types::{
    verify_proj_area_antimeridian, verify_proj_area_new_fields, verify_proj_area_roundtrip,
};

#[cfg(feature = "georaster-types")]
pub use georaster_types::{
    verify_georaster_color_type_rgb_bits, verify_georaster_coordinate_new_semantics,
    verify_georaster_image_info_fields, verify_georaster_planar_configuration_chunky,
    verify_georaster_raster_value_rgb8_variant,
};

#[cfg(feature = "wkt-types")]
pub use wkt_types::{
    verify_wkt_coord_concrete, verify_wkt_coord_roundtrip, verify_wkt_geom_point_variant,
    verify_wkt_point_empty, verify_wkt_string_trusted,
};

#[cfg(feature = "wkb-types")]
pub use wkb_types::{
    verify_wkb_bytes_known_point_metadata, verify_wkb_dimension_roundtrip,
    verify_wkb_endianness_roundtrip, verify_wkb_geometry_type_roundtrip,
    verify_wkb_write_options_roundtrip,
};

#[cfg(feature = "palette")]
pub use palette_types::{
    verify_palette_srgb_concrete, verify_palette_srgb_extremes, verify_palette_srgb_primaries,
    verify_palette_srgb_roundtrip,
};

#[cfg(feature = "ui-types")]
pub use ui_types::{
    // BoundingBox spatial proofs
    verify_bbox_bottom_concrete,
    verify_bbox_exceeds_viewport,
    verify_bbox_right_concrete,
    verify_bbox_touch_target_failed,
    verify_bbox_touch_target_met,
    verify_bbox_within_viewport,
    verify_bounds_reversed_non_negative,
    verify_bounds_width_non_negative,
    verify_builder_all_container_types,
    verify_builder_auto_close,
    verify_builder_container_with_child,
    verify_builder_default_eq_new,
    verify_builder_empty_is_valid,
    verify_builder_login_form,
    verify_builder_nested_containers,
    verify_builder_reset_after_build,
    verify_builder_root_is_zero,
    verify_builder_single_widget,
    verify_builder_slider,
    // Contrast and constraint proofs
    verify_contrast_black_white_max,
    verify_contrast_identical_is_one,
    verify_contrast_min_bound,
    verify_contrast_symmetric,
    // CssLength resolution proofs
    verify_css_em_resolution,
    verify_css_percent_resolution,
    verify_css_px_resolves_directly,
    verify_css_rem_resolution,
    verify_css_vh_resolution,
    verify_css_vw_resolution,
    verify_css_zoom_invariant_classification,
    verify_element_id_from_roundtrip,
    verify_element_id_roundtrip,
    verify_empty_report_no_errors,
    verify_error_kind_display_non_empty,
    verify_heading_default_size,
    verify_heading_level_1,
    verify_label_accepts_non_empty,
    verify_label_rejects_empty,
    verify_overflow_exact_fit,
    verify_overflow_exceeds_width,
    verify_overflow_origin_fits,
    // ConstraintProfile and typestate proofs
    verify_profile_a_count,
    verify_profile_aa_count,
    verify_profile_aaa_count,
    verify_profile_monotonicity,
    verify_progress_fraction_clamped,
    verify_progress_overflow_clamps,
    verify_propositions_zero_sized,
    verify_render_stats_clone,
    verify_render_stats_default_zeros,
    verify_render_stats_eq,
    verify_size_both_dimensions_required,
    verify_size_fails_below_boundary,
    verify_size_meets_at_boundary,
    verify_srgb_from_u8_bounds,
    verify_typestate_zero_sized,
    verify_viewport_construction,
    verify_wcag_level_display,
};

#[cfg(feature = "winit-types")]
pub use winit_types::{
    verify_winit_logical_position_fields, verify_winit_logical_size_fields,
    verify_winit_physical_size_fields, verify_winit_physical_size_zero,
};

#[cfg(feature = "wgpu-types")]
pub use wgpu_types::{
    verify_wgpu_color_fields, verify_wgpu_extent3d_fields, verify_wgpu_extent3d_zero,
    verify_wgpu_origin3d_fields,
};
