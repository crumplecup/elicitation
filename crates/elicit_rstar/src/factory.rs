//! Manual dynamic factories for `RstarTree<T>`.

use std::{
    any::TypeId,
    collections::HashMap,
    sync::{Arc, LazyLock, RwLock},
};

use elicitation::{
    AnyToolFactory, AnyToolSlot, DynamicToolDescriptor, ElicitComplete, Elicitation,
    ToolFactoryRegistration, dynamic::meta_tool::meta_tool_name,
};
use futures::future::BoxFuture;
use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use rstar::{AABB, PointDistance, RTreeObject};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::RstarTree;

type DescriptorBuilder = Arc<dyn Fn(&str) -> Vec<DynamicToolDescriptor> + Send + Sync>;

struct PrimedFactory {
    build: DescriptorBuilder,
}

static RTREE_OBJECT_PRIMED: LazyLock<RwLock<HashMap<TypeId, PrimedFactory>>> =
    LazyLock::new(Default::default);
static POINT_DISTANCE_PRIMED: LazyLock<RwLock<HashMap<TypeId, PrimedFactory>>> =
    LazyLock::new(Default::default);

const RTREE_OBJECT_TRAIT_NAME: &str = "elicit_rstar::RTreeObjectFactory";
const POINT_DISTANCE_TRAIT_NAME: &str = "elicit_rstar::PointDistanceFactory";
const RTREE_OBJECT_METHODS: &[&str] = &[
    "new",
    "bulk_load",
    "size",
    "items",
    "insert",
    "locate_in_envelope",
    "locate_in_envelope_intersecting",
];
const POINT_DISTANCE_METHODS: &[&str] = &[
    "new",
    "bulk_load",
    "size",
    "items",
    "insert",
    "locate_in_envelope",
    "locate_in_envelope_intersecting",
    "nearest_neighbor",
    "nearest_neighbors",
    "locate_all_at_point",
    "locate_within_distance",
];

/// Factory for tree operations available to any `T: RTreeObject`.
pub struct RTreeObjectFactory;

/// Factory for distance-aware tree operations available to any `T: PointDistance`.
pub struct PointDistanceFactory;

#[derive(Debug, Clone, Deserialize, JsonSchema)]
struct EmptyParams {}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(bound(deserialize = "T: DeserializeOwned"))]
#[schemars(bound = "T: JsonSchema")]
struct BulkLoadParams<T> {
    items: Vec<T>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(bound(deserialize = "Tree: DeserializeOwned"))]
#[schemars(bound = "Tree: JsonSchema")]
struct TreeTargetParams<Tree> {
    target: Tree,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(bound(deserialize = "Tree: DeserializeOwned, T: DeserializeOwned"))]
#[schemars(bound = "Tree: JsonSchema, T: JsonSchema")]
struct InsertParams<Tree, T> {
    target: Tree,
    item: T,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(bound(deserialize = "Tree: DeserializeOwned"))]
#[schemars(bound = "Tree: JsonSchema")]
struct EnvelopeQueryParams<Tree> {
    target: Tree,
    envelope: elicitation::RstarAabb,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(bound(deserialize = "Tree: DeserializeOwned"))]
#[schemars(bound = "Tree: JsonSchema")]
struct PointQueryParams<Tree> {
    target: Tree,
    point: [f64; 2],
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(bound(deserialize = "Tree: DeserializeOwned"))]
#[schemars(bound = "Tree: JsonSchema")]
struct DistanceQueryParams<Tree> {
    target: Tree,
    point: [f64; 2],
    max_distance_2: f64,
}

impl RTreeObjectFactory {
    fn prime<T>()
    where
        T: ElicitComplete + Clone + RTreeObject<Envelope = AABB<[f64; 2]>> + Send + Sync + 'static,
        RstarTree<T>:
            Serialize + DeserializeOwned + JsonSchema + Elicitation + Send + Sync + 'static,
    {
        let mut primed = RTREE_OBJECT_PRIMED
            .write()
            .expect("rstar object factory lock poisoned");
        primed
            .entry(TypeId::of::<RstarTree<T>>())
            .or_insert_with(|| PrimedFactory {
                build: Arc::new(|prefix| build_rtree_object_descriptors::<T>(prefix)),
            });
    }
}

impl PointDistanceFactory {
    fn prime<T>()
    where
        T: ElicitComplete
            + Clone
            + PointDistance
            + RTreeObject<Envelope = AABB<[f64; 2]>>
            + Send
            + Sync
            + 'static,
        RstarTree<T>:
            Serialize + DeserializeOwned + JsonSchema + Elicitation + Send + Sync + 'static,
    {
        let mut primed = POINT_DISTANCE_PRIMED
            .write()
            .expect("rstar point-distance factory lock poisoned");
        primed
            .entry(TypeId::of::<RstarTree<T>>())
            .or_insert_with(|| PrimedFactory {
                build: Arc::new(|prefix| build_point_distance_descriptors::<T>(prefix)),
            });
    }
}

impl AnyToolFactory for RTreeObjectFactory {
    fn trait_name(&self) -> &'static str {
        RTREE_OBJECT_TRAIT_NAME
    }

    fn factory_description(&self) -> &'static str {
        "Instantiate RTree<T> tools for registered tree snapshot types whose items implement rstar::RTreeObject."
    }

    fn method_names(&self) -> &'static [&'static str] {
        RTREE_OBJECT_METHODS
    }

    fn instantiate(&self, slot: &dyn AnyToolSlot) -> Result<Vec<DynamicToolDescriptor>, ErrorData> {
        let primed = RTREE_OBJECT_PRIMED
            .read()
            .expect("rstar object factory lock poisoned");
        let entry = primed.get(&slot.slot_type_id()).ok_or_else(|| {
            ErrorData::invalid_params(
                format!(
                    "`{}` has not been primed for type `{}`. Call `{}` for the item type before instantiating `{}`.",
                    RTREE_OBJECT_TRAIT_NAME,
                    slot.type_name(),
                    "prime_rtree_object_tree::<T>()",
                    meta_tool_name(RTREE_OBJECT_TRAIT_NAME),
                ),
                None,
            )
        })?;
        Ok((entry.build)(slot.prefix()))
    }
}

impl AnyToolFactory for PointDistanceFactory {
    fn trait_name(&self) -> &'static str {
        POINT_DISTANCE_TRAIT_NAME
    }

    fn factory_description(&self) -> &'static str {
        "Instantiate nearest-neighbor RTree<T> tools for registered tree snapshot types whose items implement rstar::PointDistance."
    }

    fn method_names(&self) -> &'static [&'static str] {
        POINT_DISTANCE_METHODS
    }

    fn instantiate(&self, slot: &dyn AnyToolSlot) -> Result<Vec<DynamicToolDescriptor>, ErrorData> {
        let primed = POINT_DISTANCE_PRIMED
            .read()
            .expect("rstar point-distance factory lock poisoned");
        let entry = primed.get(&slot.slot_type_id()).ok_or_else(|| {
            ErrorData::invalid_params(
                format!(
                    "`{}` has not been primed for type `{}`. Call `{}` for the item type before instantiating `{}`.",
                    POINT_DISTANCE_TRAIT_NAME,
                    slot.type_name(),
                    "prime_point_distance_tree::<T>()",
                    meta_tool_name(POINT_DISTANCE_TRAIT_NAME),
                ),
                None,
            )
        })?;
        Ok((entry.build)(slot.prefix()))
    }
}

/// Prime the `RTreeObject` factory for `RstarTree<T>`.
#[tracing::instrument]
pub fn prime_rtree_object_tree<T>()
where
    T: ElicitComplete + Clone + RTreeObject<Envelope = AABB<[f64; 2]>> + Send + Sync + 'static,
    RstarTree<T>: Serialize + DeserializeOwned + JsonSchema + Elicitation + Send + Sync + 'static,
{
    RTreeObjectFactory::prime::<T>();
}

/// Prime the `PointDistance` factory for `RstarTree<T>`.
#[tracing::instrument]
pub fn prime_point_distance_tree<T>()
where
    T: ElicitComplete
        + Clone
        + PointDistance
        + RTreeObject<Envelope = AABB<[f64; 2]>>
        + Send
        + Sync
        + 'static,
    RstarTree<T>: Serialize + DeserializeOwned + JsonSchema + Elicitation + Send + Sync + 'static,
{
    PointDistanceFactory::prime::<T>();
}

fn build_rtree_object_descriptors<T>(prefix: &str) -> Vec<DynamicToolDescriptor>
where
    T: ElicitComplete + Clone + RTreeObject<Envelope = AABB<[f64; 2]>> + Send + Sync + 'static,
    RstarTree<T>: Serialize + DeserializeOwned + JsonSchema + Elicitation + Send + Sync + 'static,
{
    vec![
        new_descriptor(
            format!("{prefix}__new"),
            "Create an empty RTree snapshot.",
            schemars::schema_for!(EmptyParams),
            |params| {
                let _: EmptyParams = parse_params(params)?;
                serialize_success(RstarTree::<T>::empty())
            },
        ),
        new_descriptor(
            format!("{prefix}__bulk_load"),
            "Bulk-load an RTree snapshot from items.",
            schemars::schema_for!(BulkLoadParams<T>),
            |params| {
                let params: BulkLoadParams<T> = parse_params(params)?;
                serialize_success(RstarTree::bulk_load(params.items))
            },
        ),
        new_descriptor(
            format!("{prefix}__size"),
            "Return the number of items in the tree snapshot.",
            schemars::schema_for!(TreeTargetParams<RstarTree<T>>),
            |params| {
                let params: TreeTargetParams<RstarTree<T>> = parse_params(params)?;
                serialize_success(params.target.size())
            },
        ),
        new_descriptor(
            format!("{prefix}__items"),
            "Return the tree snapshot contents as a vector.",
            schemars::schema_for!(TreeTargetParams<RstarTree<T>>),
            |params| {
                let params: TreeTargetParams<RstarTree<T>> = parse_params(params)?;
                serialize_success(params.target.items())
            },
        ),
        new_descriptor(
            format!("{prefix}__insert"),
            "Return a new tree snapshot with one additional item inserted.",
            schemars::schema_for!(InsertParams<RstarTree<T>, T>),
            |params| {
                let params: InsertParams<RstarTree<T>, T> = parse_params(params)?;
                serialize_success(params.target.insert(params.item))
            },
        ),
        new_descriptor(
            format!("{prefix}__locate_in_envelope"),
            "Return items fully contained within the given envelope.",
            schemars::schema_for!(EnvelopeQueryParams<RstarTree<T>>),
            |params| {
                let params: EnvelopeQueryParams<RstarTree<T>> = parse_params(params)?;
                serialize_success(params.target.locate_in_envelope(params.envelope))
            },
        ),
        new_descriptor(
            format!("{prefix}__locate_in_envelope_intersecting"),
            "Return items whose envelopes intersect the given envelope.",
            schemars::schema_for!(EnvelopeQueryParams<RstarTree<T>>),
            |params| {
                let params: EnvelopeQueryParams<RstarTree<T>> = parse_params(params)?;
                serialize_success(
                    params
                        .target
                        .locate_in_envelope_intersecting(params.envelope),
                )
            },
        ),
    ]
}

fn build_point_distance_descriptors<T>(prefix: &str) -> Vec<DynamicToolDescriptor>
where
    T: ElicitComplete
        + Clone
        + PointDistance
        + RTreeObject<Envelope = AABB<[f64; 2]>>
        + Send
        + Sync
        + 'static,
    RstarTree<T>: Serialize + DeserializeOwned + JsonSchema + Elicitation + Send + Sync + 'static,
{
    let mut descriptors = build_rtree_object_descriptors::<T>(prefix);
    descriptors.extend([
        new_descriptor(
            format!("{prefix}__nearest_neighbor"),
            "Return the nearest neighbor to a query point.",
            schemars::schema_for!(PointQueryParams<RstarTree<T>>),
            |params| {
                let params: PointQueryParams<RstarTree<T>> = parse_params(params)?;
                serialize_success(params.target.nearest_neighbor(params.point))
            },
        ),
        new_descriptor(
            format!("{prefix}__nearest_neighbors"),
            "Return all equally-nearest neighbors for a query point.",
            schemars::schema_for!(PointQueryParams<RstarTree<T>>),
            |params| {
                let params: PointQueryParams<RstarTree<T>> = parse_params(params)?;
                serialize_success(params.target.nearest_neighbors(params.point))
            },
        ),
        new_descriptor(
            format!("{prefix}__locate_all_at_point"),
            "Return all items containing the query point.",
            schemars::schema_for!(PointQueryParams<RstarTree<T>>),
            |params| {
                let params: PointQueryParams<RstarTree<T>> = parse_params(params)?;
                serialize_success(params.target.locate_all_at_point(params.point))
            },
        ),
        new_descriptor(
            format!("{prefix}__locate_within_distance"),
            "Return all items within the given squared distance of the query point.",
            schemars::schema_for!(DistanceQueryParams<RstarTree<T>>),
            |params| {
                let params: DistanceQueryParams<RstarTree<T>> = parse_params(params)?;
                serialize_success(
                    params
                        .target
                        .locate_within_distance(params.point, params.max_distance_2),
                )
            },
        ),
    ]);
    descriptors
}

fn new_descriptor<F>(
    name: String,
    description: &str,
    schema: schemars::Schema,
    handler: F,
) -> DynamicToolDescriptor
where
    F: Fn(serde_json::Value) -> Result<CallToolResult, ErrorData> + Send + Sync + 'static,
{
    let schema = serde_json::to_value(schema).unwrap_or_default();
    let description = description.to_string();
    DynamicToolDescriptor {
        name,
        description,
        schema,
        handler: Arc::new(move |params| {
            let result = handler(params);
            Box::pin(async move { result }) as BoxFuture<'static, Result<CallToolResult, ErrorData>>
        }),
    }
}

fn parse_params<T>(params: serde_json::Value) -> Result<T, ErrorData>
where
    T: DeserializeOwned,
{
    serde_json::from_value(params)
        .map_err(|error| ErrorData::invalid_params(error.to_string(), None))
}

fn serialize_success<T>(value: T) -> Result<CallToolResult, ErrorData>
where
    T: Serialize,
{
    let text = serde_json::to_string(&value)
        .map_err(|error| ErrorData::internal_error(error.to_string(), None))?;
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

inventory::submit!(ToolFactoryRegistration {
    trait_name: RTREE_OBJECT_TRAIT_NAME,
    factory: &RTreeObjectFactory,
});

inventory::submit!(ToolFactoryRegistration {
    trait_name: POINT_DISTANCE_TRAIT_NAME,
    factory: &PointDistanceFactory,
});
