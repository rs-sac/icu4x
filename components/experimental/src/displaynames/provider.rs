// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]
#![warn(unused_imports)]
//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use icu_provider::prelude::*;
use potential_utf::PotentialUtf8;
use tinystr::UnvalidatedTinyAsciiStr;
use zerovec::ZeroMap;

// We use raw TinyAsciiStrs for map keys, as we then don't have to
// validate them as subtags on deserialization. Map lookup can be
// done even if they are not valid tags (an invalid key will just
// become inaccessible).
type UnvalidatedRegion = UnvalidatedTinyAsciiStr<3>;
type UnvalidatedLanguage = UnvalidatedTinyAsciiStr<3>;
type UnvalidatedScript = UnvalidatedTinyAsciiStr<4>;
type UnvalidatedLocale = PotentialUtf8;
type UnvalidatedVariant = UnvalidatedTinyAsciiStr<8>;

#[cfg(feature = "compiled_data")]
/// Baked data
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. In particular, the `DataProvider` implementations are only
/// guaranteed to match with this version's `*_unstable` providers. Use with caution.
/// </div>
pub use crate::provider::Baked;

icu_provider::data_marker!(
    /// `LocaleDisplayNamesV1`
    LocaleDisplayNamesV1,
    LocaleDisplayNames<'static>
);
icu_provider::data_marker!(
    /// `VariantDisplayNamesV1`
    VariantDisplayNamesV1,
    VariantDisplayNames<'static>
);
icu_provider::data_marker!(
    /// `ScriptDisplayNamesV1`
    ScriptDisplayNamesV1,
    ScriptDisplayNames<'static>
);
icu_provider::data_marker!(
    /// `LanguageDisplayNamesV1`
    LanguageDisplayNamesV1,
    LanguageDisplayNames<'static>
);
icu_provider::data_marker!(
    /// `RegionDisplayNamesV1`
    RegionDisplayNamesV1,
    RegionDisplayNames<'static>
);

#[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::displaynames::provider))]
#[yoke(prove_covariance_manually)]
/// RegionDisplayNames provides mapping between a region code and locale display name.
pub struct RegionDisplayNames<'data> {
    /// Mapping for region to locale display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub names: ZeroMap<'data, UnvalidatedRegion, str>,
    /// Mapping for region to locale display short name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub short_names: ZeroMap<'data, UnvalidatedRegion, str>,
}

icu_provider::data_struct!(RegionDisplayNames<'_>, #[cfg(feature = "datagen")]);

#[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::displaynames::provider))]
#[yoke(prove_covariance_manually)]
/// LanguageDisplayNames provides mapping between languages and display names.
pub struct LanguageDisplayNames<'data> {
    /// Mapping for language to display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub names: ZeroMap<'data, UnvalidatedLanguage, str>,
    /// Mapping for language to short display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub short_names: ZeroMap<'data, UnvalidatedLanguage, str>,
    /// Mapping for language to long display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub long_names: ZeroMap<'data, UnvalidatedLanguage, str>,
    /// Mapping for language to menu variant display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub menu_names: ZeroMap<'data, UnvalidatedLanguage, str>,
}

icu_provider::data_struct!(LanguageDisplayNames<'_>, #[cfg(feature = "datagen")]);

#[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::displaynames::provider))]
#[yoke(prove_covariance_manually)]
/// ScriptDisplayNames provides mapping between a script code and it's display name.
pub struct ScriptDisplayNames<'data> {
    /// Mapping for script to locale display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub names: ZeroMap<'data, UnvalidatedScript, str>,
    /// Mapping for script to locale display short name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub short_names: ZeroMap<'data, UnvalidatedScript, str>,
}

icu_provider::data_struct!(ScriptDisplayNames<'_>, #[cfg(feature = "datagen")]);

#[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::displaynames::provider))]
#[yoke(prove_covariance_manually)]
/// LocaleDisplayNames provides mapping between locales and display names.
pub struct LocaleDisplayNames<'data> {
    /// Mapping for locale to display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub names: ZeroMap<'data, UnvalidatedLocale, str>,
    /// Mapping for locale to short display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub short_names: ZeroMap<'data, UnvalidatedLocale, str>,
    /// Mapping for locale to long display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub long_names: ZeroMap<'data, UnvalidatedLocale, str>,
    /// Mapping for locale to menu variant display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub menu_names: ZeroMap<'data, UnvalidatedLocale, str>,
}

icu_provider::data_struct!(LocaleDisplayNames<'_>, #[cfg(feature = "datagen")]);

#[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::displaynames::provider))]
#[yoke(prove_covariance_manually)]
/// VariantDisplayNames provides the user-translated names for the variant-code values.
pub struct VariantDisplayNames<'data> {
    /// Mapping for Variant to locale display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub names: ZeroMap<'data, UnvalidatedVariant, str>,
}

icu_provider::data_struct!(VariantDisplayNames<'_>, #[cfg(feature = "datagen")]);
