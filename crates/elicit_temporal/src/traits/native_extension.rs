//! Native descriptor/carrier bridges for higher-order temporal extension families.
//!
//! These seams exist only where a backend has a meaningful upstream carrier
//! for the relevant ISO 8601-2 or CalConnect form. They keep the same
//! proven-carrier grammar as the rest of the native interface: descriptors
//! cross the seam together with named semantic bundles, and runtime values
//! return wrapped in proven carriers rather than loose proof tuples.

use crate::{
    DateTimeFormulaDescriptor, DateTimeFormulaSemanticBundle,
    ExplicitDurationDescriptor, ExplicitDurationSemanticBundle, ExplicitTemporalFormDescriptor,
    ExplicitTemporalFormSemanticBundle, ExplicitTimeIntervalDescriptor,
    ExplicitTimeIntervalSemanticBundle, GroupedTimeScaleUnitDescriptor,
    GroupedTimeScaleUnitSemanticBundle, NativeEvaluatedProvenDateTimeFormulaResult,
    ProvenDateTimeFormulaCarrier, ProvenExplicitDurationCarrier,
    ProvenExplicitTemporalFormCarrier, ProvenExplicitTimeIntervalCarrier,
    ProvenGroupedTimeScaleUnitCarrier, ProvenQualifiedTemporalValueCarrier,
    ProvenTemporalSetCarrier, QualifiedTemporalValueDescriptor,
    QualifiedTemporalValueSemanticBundle, RealizedProvenDateTimeFormulaResult,
    RealizedProvenExplicitDurationResult, RealizedProvenExplicitTemporalFormResult,
    RealizedProvenExplicitTimeIntervalResult, RealizedProvenGroupedTimeScaleUnitResult,
    RealizedProvenQualifiedTemporalValueResult, RealizedProvenTemporalSetResult,
    ReflectedProvenDateTimeFormulaResult, ReflectedProvenExplicitDurationResult,
    ReflectedProvenExplicitTemporalFormResult, ReflectedProvenExplicitTimeIntervalResult,
    ReflectedProvenGroupedTimeScaleUnitResult, ReflectedProvenQualifiedTemporalValueResult,
    ReflectedProvenTemporalSetResult, TemporalDateTimeFormulaProps,
    TemporalExplicitDurationProps, TemporalExplicitTemporalFormProps,
    TemporalExplicitTimeIntervalProps, TemporalGroupedTimeScaleUnitProps,
    TemporalQualifiedTemporalValueProps, TemporalSetDescriptor, TemporalSetProps,
    TemporalSetSemanticBundle,
};

/// Realize and reflect native qualified temporal value carriers.
pub trait TemporalQualifiedTemporalValueNativeBridge:
    TemporalQualifiedTemporalValueProps + Send + Sync
{
    /// Realize a validated qualified temporal value descriptor as a proven backend-native carrier.
    fn realize_qualified_temporal_value(
        &self,
        value: &QualifiedTemporalValueDescriptor,
        semantics: &QualifiedTemporalValueSemanticBundle,
    ) -> RealizedProvenQualifiedTemporalValueResult<Self::QualifiedTemporalValue>;

    /// Reflect a proven backend-native qualified temporal value carrier into the neutral descriptor accord.
    fn reflect_qualified_temporal_value(
        &self,
        value: &ProvenQualifiedTemporalValueCarrier<Self::QualifiedTemporalValue>,
    ) -> ReflectedProvenQualifiedTemporalValueResult;
}

/// Realize and reflect native explicit temporal form carriers.
pub trait TemporalExplicitTemporalFormNativeBridge:
    TemporalExplicitTemporalFormProps + Send + Sync
{
    /// Realize a validated explicit temporal form descriptor as a proven backend-native carrier.
    fn realize_explicit_temporal_form(
        &self,
        form: &ExplicitTemporalFormDescriptor,
        semantics: &ExplicitTemporalFormSemanticBundle,
    ) -> RealizedProvenExplicitTemporalFormResult<Self::ExplicitTemporalForm>;

    /// Reflect a proven backend-native explicit temporal form carrier into the neutral descriptor accord.
    fn reflect_explicit_temporal_form(
        &self,
        form: &ProvenExplicitTemporalFormCarrier<Self::ExplicitTemporalForm>,
    ) -> ReflectedProvenExplicitTemporalFormResult;
}

/// Realize and reflect native explicit duration carriers.
pub trait TemporalExplicitDurationNativeBridge: TemporalExplicitDurationProps + Send + Sync {
    /// Realize a validated explicit duration descriptor as a proven backend-native carrier.
    fn realize_explicit_duration(
        &self,
        duration: &ExplicitDurationDescriptor,
        semantics: &ExplicitDurationSemanticBundle,
    ) -> RealizedProvenExplicitDurationResult<Self::ExplicitDuration>;

    /// Reflect a proven backend-native explicit duration carrier into the neutral descriptor accord.
    fn reflect_explicit_duration(
        &self,
        duration: &ProvenExplicitDurationCarrier<Self::ExplicitDuration>,
    ) -> ReflectedProvenExplicitDurationResult;
}

/// Realize and reflect native explicit time-interval carriers.
pub trait TemporalExplicitTimeIntervalNativeBridge:
    TemporalExplicitTimeIntervalProps + Send + Sync
{
    /// Realize a validated explicit time-interval descriptor as a proven backend-native carrier.
    fn realize_explicit_time_interval(
        &self,
        interval: &ExplicitTimeIntervalDescriptor,
        semantics: &ExplicitTimeIntervalSemanticBundle,
    ) -> RealizedProvenExplicitTimeIntervalResult<Self::ExplicitTimeInterval>;

    /// Reflect a proven backend-native explicit time-interval carrier into the neutral descriptor accord.
    fn reflect_explicit_time_interval(
        &self,
        interval: &ProvenExplicitTimeIntervalCarrier<Self::ExplicitTimeInterval>,
    ) -> ReflectedProvenExplicitTimeIntervalResult;
}

/// Realize and reflect native grouped time-scale-unit carriers.
pub trait TemporalGroupedTimeScaleUnitNativeBridge:
    TemporalGroupedTimeScaleUnitProps + Send + Sync
{
    /// Realize a validated grouped time-scale-unit descriptor as a proven backend-native carrier.
    fn realize_grouped_time_scale_unit(
        &self,
        grouped: &GroupedTimeScaleUnitDescriptor,
        semantics: &GroupedTimeScaleUnitSemanticBundle,
    ) -> RealizedProvenGroupedTimeScaleUnitResult<Self::GroupedTimeScaleUnit>;

    /// Reflect a proven backend-native grouped time-scale-unit carrier into the neutral descriptor accord.
    fn reflect_grouped_time_scale_unit(
        &self,
        grouped: &ProvenGroupedTimeScaleUnitCarrier<Self::GroupedTimeScaleUnit>,
    ) -> ReflectedProvenGroupedTimeScaleUnitResult;
}

/// Realize and reflect native temporal-set carriers.
pub trait TemporalSetNativeBridge: TemporalSetProps + Send + Sync {
    /// Realize a validated temporal-set descriptor as a proven backend-native carrier.
    fn realize_temporal_set(
        &self,
        set: &TemporalSetDescriptor,
        semantics: &TemporalSetSemanticBundle,
    ) -> RealizedProvenTemporalSetResult<Self::TemporalSet>;

    /// Reflect a proven backend-native temporal-set carrier into the neutral descriptor accord.
    fn reflect_temporal_set(
        &self,
        set: &ProvenTemporalSetCarrier<Self::TemporalSet>,
    ) -> ReflectedProvenTemporalSetResult;
}

/// Realize and reflect native date-time formula carriers.
pub trait TemporalDateTimeFormulaNativeBridge: TemporalDateTimeFormulaProps + Send + Sync {
    /// Realize a validated date-time formula descriptor as a proven backend-native carrier.
    fn realize_date_time_formula(
        &self,
        formula: &DateTimeFormulaDescriptor,
        semantics: &DateTimeFormulaSemanticBundle,
    ) -> RealizedProvenDateTimeFormulaResult<Self::DateTimeFormula>;

    /// Reflect a proven backend-native date-time formula carrier into the neutral descriptor accord.
    fn reflect_date_time_formula(
        &self,
        formula: &ProvenDateTimeFormulaCarrier<Self::DateTimeFormula>,
    ) -> ReflectedProvenDateTimeFormulaResult;
}

/// Evaluate native date-time formulas into native explicit temporal forms.
pub trait TemporalNativeDateTimeFormulaFactory:
    TemporalDateTimeFormulaProps + TemporalExplicitTemporalFormProps + Send + Sync
{
    /// Evaluate a proven native date-time formula into a proven native explicit temporal form.
    ///
    /// The returned exchange keeps both proof layers visible: the explicit
    /// temporal form carrier re-issues its own structural semantics, and the
    /// accompanying provenance bundle proves that this form was lawfully
    /// produced by formula evaluation under the declared semantics.
    fn evaluate_date_time_formula_native(
        &self,
        formula: &ProvenDateTimeFormulaCarrier<Self::DateTimeFormula>,
    ) -> NativeEvaluatedProvenDateTimeFormulaResult<Self::ExplicitTemporalForm>;
}

/// Aggregate native bridge for the currently modeled extension-carrier families.
pub trait TemporalNativeExtensionBridge:
    TemporalQualifiedTemporalValueNativeBridge
    + TemporalExplicitTemporalFormNativeBridge
    + TemporalExplicitDurationNativeBridge
    + TemporalExplicitTimeIntervalNativeBridge
    + TemporalGroupedTimeScaleUnitNativeBridge
    + TemporalSetNativeBridge
    + TemporalDateTimeFormulaNativeBridge
{
}

impl<T> TemporalNativeExtensionBridge for T where
    T: TemporalQualifiedTemporalValueNativeBridge
        + TemporalExplicitTemporalFormNativeBridge
        + TemporalExplicitDurationNativeBridge
        + TemporalExplicitTimeIntervalNativeBridge
        + TemporalGroupedTimeScaleUnitNativeBridge
        + TemporalSetNativeBridge
        + TemporalDateTimeFormulaNativeBridge
{
}
