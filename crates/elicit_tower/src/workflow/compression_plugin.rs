//! `TowerCompressionPlugin` — CompressionLayer and DecompressionLayer MCP tools.

use elicitation::{ElicitPlugin, ToCodeLiteral, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

fn json_err(e: impl std::fmt::Display) -> ErrorData {
    ErrorData::internal_error(e.to_string(), None)
}

// ── Types ─────────────────────────────────────────────────────────────────────

/// Configuration for a CompressionLayer.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct CompressionConfig {
    /// Whether gzip compression is enabled.
    pub gzip: bool,
    /// Whether brotli compression is enabled.
    pub br: bool,
    /// Whether zstd compression is enabled.
    pub zstd: bool,
    /// Whether deflate compression is enabled.
    pub deflate: bool,
    /// Compression quality level ("Default", "Fastest", "Best", or a precise value).
    pub quality: String,
}

/// Configuration for a DecompressionLayer.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct DecompressionConfig {
    /// Whether brotli decompression is accepted.
    pub accept_br: bool,
    /// Whether deflate decompression is accepted.
    pub accept_deflate: bool,
    /// Whether gzip decompression is accepted.
    pub accept_gzip: bool,
    /// Whether zstd decompression is accepted.
    pub accept_zstd: bool,
}

// ── Unique per-tool params ────────────────────────────────────────────────────

/// Parameters for compression_new (no inputs).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CompressionNewParams {}

/// Parameters for compression_quality_default.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CompressionQualityDefaultParams {
    /// The current compression configuration.
    pub config: CompressionConfig,
}

/// Parameters for compression_quality_fastest.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CompressionQualityFastestParams {
    /// The current compression configuration.
    pub config: CompressionConfig,
}

/// Parameters for compression_quality_best.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CompressionQualityBestParams {
    /// The current compression configuration.
    pub config: CompressionConfig,
}

/// Parameters for compression_quality_precise.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CompressionQualityPreciseParams {
    /// The current compression configuration.
    pub config: CompressionConfig,
    /// The precise quality level (0–9 or algorithm-specific).
    pub level: u32,
}

/// Parameters for compression_enable_gzip.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CompressionEnableGzipParams {
    /// The current compression configuration.
    pub config: CompressionConfig,
}

/// Parameters for compression_enable_br.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CompressionEnableBrParams {
    /// The current compression configuration.
    pub config: CompressionConfig,
}

/// Parameters for compression_enable_zstd.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CompressionEnableZstdParams {
    /// The current compression configuration.
    pub config: CompressionConfig,
}

/// Parameters for compression_enable_deflate.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CompressionEnableDeflateParams {
    /// The current compression configuration.
    pub config: CompressionConfig,
}

/// Parameters for compression_disable_gzip.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CompressionDisableGzipParams {
    /// The current compression configuration.
    pub config: CompressionConfig,
}

/// Parameters for compression_disable_br.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CompressionDisableBrParams {
    /// The current compression configuration.
    pub config: CompressionConfig,
}

/// Parameters for compression_disable_zstd.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CompressionDisableZstdParams {
    /// The current compression configuration.
    pub config: CompressionConfig,
}

/// Parameters for compression_disable_deflate.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CompressionDisableDeflateParams {
    /// The current compression configuration.
    pub config: CompressionConfig,
}

/// Parameters for decompression_new (no inputs).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DecompressionNewParams {}

/// Parameters for decompression_accept_br.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DecompressionAcceptBrParams {
    /// The current decompression configuration.
    pub config: DecompressionConfig,
}

/// Parameters for decompression_accept_deflate.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DecompressionAcceptDeflateParams {
    /// The current decompression configuration.
    pub config: DecompressionConfig,
}

/// Parameters for decompression_accept_gzip.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DecompressionAcceptGzipParams {
    /// The current decompression configuration.
    pub config: DecompressionConfig,
}

/// Parameters for decompression_accept_zstd.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DecompressionAcceptZstdParams {
    /// The current decompression configuration.
    pub config: DecompressionConfig,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "tower_compression",
    name = "compression_new",
    description = "Create a default CompressionLayer configuration."
)]
#[instrument]
async fn compression_new(_p: CompressionNewParams) -> Result<CallToolResult, ErrorData> {
    let result = CompressionConfig {
        gzip: true,
        br: true,
        zstd: true,
        deflate: true,
        quality: "Default".to_string(),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_compression",
    name = "compression_quality_default",
    description = "Set compression quality to Default."
)]
#[instrument]
async fn compression_quality_default(
    p: CompressionQualityDefaultParams,
) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.quality = "Default".to_string();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_compression",
    name = "compression_quality_fastest",
    description = "Set compression quality to Fastest."
)]
#[instrument]
async fn compression_quality_fastest(
    p: CompressionQualityFastestParams,
) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.quality = "Fastest".to_string();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_compression",
    name = "compression_quality_best",
    description = "Set compression quality to Best."
)]
#[instrument]
async fn compression_quality_best(
    p: CompressionQualityBestParams,
) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.quality = "Best".to_string();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_compression",
    name = "compression_quality_precise",
    description = "Set compression quality to a precise numeric level."
)]
#[instrument]
async fn compression_quality_precise(
    p: CompressionQualityPreciseParams,
) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.quality = format!("Precise({})", p.level);
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_compression",
    name = "compression_enable_gzip",
    description = "Enable gzip compression in the configuration."
)]
#[instrument]
async fn compression_enable_gzip(
    p: CompressionEnableGzipParams,
) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.gzip = true;
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_compression",
    name = "compression_enable_br",
    description = "Enable brotli compression in the configuration."
)]
#[instrument]
async fn compression_enable_br(p: CompressionEnableBrParams) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.br = true;
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_compression",
    name = "compression_enable_zstd",
    description = "Enable zstd compression in the configuration."
)]
#[instrument]
async fn compression_enable_zstd(
    p: CompressionEnableZstdParams,
) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.zstd = true;
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_compression",
    name = "compression_enable_deflate",
    description = "Enable deflate compression in the configuration."
)]
#[instrument]
async fn compression_enable_deflate(
    p: CompressionEnableDeflateParams,
) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.deflate = true;
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_compression",
    name = "compression_disable_gzip",
    description = "Disable gzip compression in the configuration."
)]
#[instrument]
async fn compression_disable_gzip(
    p: CompressionDisableGzipParams,
) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.gzip = false;
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_compression",
    name = "compression_disable_br",
    description = "Disable brotli compression in the configuration."
)]
#[instrument]
async fn compression_disable_br(
    p: CompressionDisableBrParams,
) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.br = false;
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_compression",
    name = "compression_disable_zstd",
    description = "Disable zstd compression in the configuration."
)]
#[instrument]
async fn compression_disable_zstd(
    p: CompressionDisableZstdParams,
) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.zstd = false;
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_compression",
    name = "compression_disable_deflate",
    description = "Disable deflate compression in the configuration."
)]
#[instrument]
async fn compression_disable_deflate(
    p: CompressionDisableDeflateParams,
) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.deflate = false;
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_compression",
    name = "decompression_new",
    description = "Create a default DecompressionLayer configuration."
)]
#[instrument]
async fn decompression_new(_p: DecompressionNewParams) -> Result<CallToolResult, ErrorData> {
    let result = DecompressionConfig {
        accept_br: true,
        accept_deflate: true,
        accept_gzip: true,
        accept_zstd: true,
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_compression",
    name = "decompression_accept_br",
    description = "Enable brotli decompression in the configuration."
)]
#[instrument]
async fn decompression_accept_br(
    p: DecompressionAcceptBrParams,
) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.accept_br = true;
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_compression",
    name = "decompression_accept_deflate",
    description = "Enable deflate decompression in the configuration."
)]
#[instrument]
async fn decompression_accept_deflate(
    p: DecompressionAcceptDeflateParams,
) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.accept_deflate = true;
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_compression",
    name = "decompression_accept_gzip",
    description = "Enable gzip decompression in the configuration."
)]
#[instrument]
async fn decompression_accept_gzip(
    p: DecompressionAcceptGzipParams,
) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.accept_gzip = true;
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_compression",
    name = "decompression_accept_zstd",
    description = "Enable zstd decompression in the configuration."
)]
#[instrument]
async fn decompression_accept_zstd(
    p: DecompressionAcceptZstdParams,
) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.accept_zstd = true;
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

/// Plugin exposing CompressionLayer and DecompressionLayer configuration tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "tower_compression")]
pub struct TowerCompressionPlugin;
