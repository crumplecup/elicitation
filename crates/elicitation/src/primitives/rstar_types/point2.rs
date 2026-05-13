//! Shared helpers for 2D `rstar` point values.

use crate::{ElicitCommunicator, ElicitPromptTree, ElicitResult, Elicitation, PromptTree};

pub(super) async fn elicit_point2<C: ElicitCommunicator>(
    communicator: &C,
) -> ElicitResult<[f64; 2]> {
    Ok([
        f64::elicit(communicator).await?,
        f64::elicit(communicator).await?,
    ])
}

pub(super) fn point2_prompt_tree(prompt: &str) -> PromptTree {
    PromptTree::Survey {
        prompt: Some(prompt.to_string()),
        type_name: "[f64; 2]".to_string(),
        fields: vec![
            (
                "x".to_string(),
                Box::new(f64::prompt_tree().with_prompt(Some("X coordinate:".to_string()))),
            ),
            (
                "y".to_string(),
                Box::new(f64::prompt_tree().with_prompt(Some("Y coordinate:".to_string()))),
            ),
        ],
    }
}
