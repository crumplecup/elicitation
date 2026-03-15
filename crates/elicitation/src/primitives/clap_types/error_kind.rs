//! [`clap::error::ErrorKind`] elicitation.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use clap::error::ErrorKind;

impl Prompt for ErrorKind {
    fn prompt() -> Option<&'static str> {
        Some("Choose the clap error kind:")
    }
}

impl Select for ErrorKind {
    fn options() -> Vec<Self> {
        vec![
            ErrorKind::InvalidValue,
            ErrorKind::UnknownArgument,
            ErrorKind::InvalidSubcommand,
            ErrorKind::NoEquals,
            ErrorKind::ValueValidation,
            ErrorKind::TooManyValues,
            ErrorKind::TooFewValues,
            ErrorKind::WrongNumberOfValues,
            ErrorKind::ArgumentConflict,
            ErrorKind::MissingRequiredArgument,
            ErrorKind::MissingSubcommand,
            ErrorKind::InvalidUtf8,
            ErrorKind::DisplayHelp,
            ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand,
            ErrorKind::DisplayVersion,
            ErrorKind::Io,
            ErrorKind::Format,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "InvalidValue".to_string(),
            "UnknownArgument".to_string(),
            "InvalidSubcommand".to_string(),
            "NoEquals".to_string(),
            "ValueValidation".to_string(),
            "TooManyValues".to_string(),
            "TooFewValues".to_string(),
            "WrongNumberOfValues".to_string(),
            "ArgumentConflict".to_string(),
            "MissingRequiredArgument".to_string(),
            "MissingSubcommand".to_string(),
            "InvalidUtf8".to_string(),
            "DisplayHelp".to_string(),
            "DisplayHelpOnMissingArgumentOrSubcommand".to_string(),
            "DisplayVersion".to_string(),
            "Io".to_string(),
            "Format".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "InvalidValue" => Some(ErrorKind::InvalidValue),
            "UnknownArgument" => Some(ErrorKind::UnknownArgument),
            "InvalidSubcommand" => Some(ErrorKind::InvalidSubcommand),
            "NoEquals" => Some(ErrorKind::NoEquals),
            "ValueValidation" => Some(ErrorKind::ValueValidation),
            "TooManyValues" => Some(ErrorKind::TooManyValues),
            "TooFewValues" => Some(ErrorKind::TooFewValues),
            "WrongNumberOfValues" => Some(ErrorKind::WrongNumberOfValues),
            "ArgumentConflict" => Some(ErrorKind::ArgumentConflict),
            "MissingRequiredArgument" => Some(ErrorKind::MissingRequiredArgument),
            "MissingSubcommand" => Some(ErrorKind::MissingSubcommand),
            "InvalidUtf8" => Some(ErrorKind::InvalidUtf8),
            "DisplayHelp" => Some(ErrorKind::DisplayHelp),
            "DisplayHelpOnMissingArgumentOrSubcommand" => {
                Some(ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand)
            }
            "DisplayVersion" => Some(ErrorKind::DisplayVersion),
            "Io" => Some(ErrorKind::Io),
            "Format" => Some(ErrorKind::Format),
            _ => None,
        }
    }
}

crate::default_style!(ErrorKind => ErrorKindStyle);

impl Elicitation for ErrorKind {
    type Style = ErrorKindStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting ErrorKind");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose error kind:"),
            &Self::labels(),
        );
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid ErrorKind: {}",
                label
            )))
        })
    }
}

impl ElicitIntrospect for ErrorKind {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "clap::error::ErrorKind",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|label| VariantMetadata {
                        label,
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}
