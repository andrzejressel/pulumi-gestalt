#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationInfoTypeTransformationsTransformationPrimitiveTransformation {
    /// Generalization function that buckets values based on ranges. The ranges and replacement values are dynamically provided by the user for custom behavior, such as 1-30 > LOW 31-65 > MEDIUM 66-100 > HIGH
    /// This can be used on data of type: number, long, string, timestamp.
    /// If the provided value type differs from the type of data being transformed, we will first attempt converting the type of the data to be transformed to match the type of the bound before comparing.
    /// See https://cloud.google.com/dlp/docs/concepts-bucketing to learn more.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "bucketingConfig")]
    pub r#bucketing_config: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationInfoTypeTransformationsTransformationPrimitiveTransformationBucketingConfig>>,
    /// Partially mask a string by replacing a given number of characters with a fixed character. Masking can start from the beginning or end of the string. This can be used on data of any type (numbers, longs, and so on) and when de-identifying structured data we'll attempt to preserve the original data's type. (This allows you to take a long like 123 and modify it to a string like **3).
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "characterMaskConfig")]
    pub r#character_mask_config: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationInfoTypeTransformationsTransformationPrimitiveTransformationCharacterMaskConfig>>,
    /// Pseudonymization method that generates deterministic encryption for the given input. Outputs a base64 encoded representation of the encrypted output. Uses AES-SIV based on the RFC [https://tools.ietf.org/html/rfc5297](https://tools.ietf.org/html/rfc5297).
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "cryptoDeterministicConfig")]
    pub r#crypto_deterministic_config: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationInfoTypeTransformationsTransformationPrimitiveTransformationCryptoDeterministicConfig>>,
    /// Pseudonymization method that generates surrogates via cryptographic hashing. Uses SHA-256. The key size must be either 32 or 64 bytes.
    /// Outputs a base64 encoded representation of the hashed output (for example, L7k0BHmF1ha5U3NfGykjro4xWi1MPVQPjhMAZbSV9mM=).
    /// Currently, only string and integer values can be hashed.
    /// See https://cloud.google.com/dlp/docs/pseudonymization to learn more.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "cryptoHashConfig")]
    pub r#crypto_hash_config: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationInfoTypeTransformationsTransformationPrimitiveTransformationCryptoHashConfig>>,
    /// Replaces an identifier with a surrogate using Format Preserving Encryption (FPE) with the FFX mode of operation; however when used in the `content.reidentify` API method, it serves the opposite function by reversing the surrogate back into the original identifier. The identifier must be encoded as ASCII. For a given crypto key and context, the same identifier will be replaced with the same surrogate. Identifiers must be at least two characters long. In the case that the identifier is the empty string, it will be skipped. See [https://cloud.google.com/dlp/docs/pseudonymization](https://cloud.google.com/dlp/docs/pseudonymization) to learn more.
    /// Note: We recommend using CryptoDeterministicConfig for all use cases which do not require preserving the input alphabet space and size, plus warrant referential integrity.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "cryptoReplaceFfxFpeConfig")]
    pub r#crypto_replace_ffx_fpe_config: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationInfoTypeTransformationsTransformationPrimitiveTransformationCryptoReplaceFfxFpeConfig>>,
    /// Shifts dates by random number of days, with option to be consistent for the same context. See https://cloud.google.com/dlp/docs/concepts-date-shifting to learn more.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "dateShiftConfig")]
    pub r#date_shift_config: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationInfoTypeTransformationsTransformationPrimitiveTransformationDateShiftConfig>>,
    /// Buckets values based on fixed size ranges. The Bucketing transformation can provide all of this functionality, but requires more configuration. This message is provided as a convenience to the user for simple bucketing strategies.
    /// The transformed value will be a hyphenated string of {lower_bound}-{upper_bound}. For example, if lower_bound = 10 and upper_bound = 20, all values that are within this bucket will be replaced with "10-20".
    /// This can be used on data of type: double, long.
    /// If the bound Value type differs from the type of data being transformed, we will first attempt converting the type of the data to be transformed to match the type of the bound before comparing.
    /// See https://cloud.google.com/dlp/docs/concepts-bucketing to learn more.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "fixedSizeBucketingConfig")]
    pub r#fixed_size_bucketing_config: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationInfoTypeTransformationsTransformationPrimitiveTransformationFixedSizeBucketingConfig>>,
    /// Redact a given value. For example, if used with an InfoTypeTransformation transforming PHONE_NUMBER, and input 'My phone number is 206-555-0123', the output would be 'My phone number is '.
    #[builder(into)]
    #[serde(rename = "redactConfig")]
    pub r#redact_config: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationInfoTypeTransformationsTransformationPrimitiveTransformationRedactConfig>>,
    /// Replace each input value with a given value.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "replaceConfig")]
    pub r#replace_config: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationInfoTypeTransformationsTransformationPrimitiveTransformationReplaceConfig>>,
    /// Replace with a value randomly drawn (with replacement) from a dictionary.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "replaceDictionaryConfig")]
    pub r#replace_dictionary_config: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationInfoTypeTransformationsTransformationPrimitiveTransformationReplaceDictionaryConfig>>,
    /// Replace each matching finding with the name of the info type.
    #[builder(into)]
    #[serde(rename = "replaceWithInfoTypeConfig")]
    pub r#replace_with_info_type_config: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationInfoTypeTransformationsTransformationPrimitiveTransformationReplaceWithInfoTypeConfig>>,
    /// For use with Date, Timestamp, and TimeOfDay, extract or preserve a portion of the value.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "timePartConfig")]
    pub r#time_part_config: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationInfoTypeTransformationsTransformationPrimitiveTransformationTimePartConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationInfoTypeTransformationsTransformationPrimitiveTransformation {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "bucketing_config",
                    &self.r#bucketing_config,
                ),
                to_pulumi_object_field(
                    "character_mask_config",
                    &self.r#character_mask_config,
                ),
                to_pulumi_object_field(
                    "crypto_deterministic_config",
                    &self.r#crypto_deterministic_config,
                ),
                to_pulumi_object_field(
                    "crypto_hash_config",
                    &self.r#crypto_hash_config,
                ),
                to_pulumi_object_field(
                    "crypto_replace_ffx_fpe_config",
                    &self.r#crypto_replace_ffx_fpe_config,
                ),
                to_pulumi_object_field(
                    "date_shift_config",
                    &self.r#date_shift_config,
                ),
                to_pulumi_object_field(
                    "fixed_size_bucketing_config",
                    &self.r#fixed_size_bucketing_config,
                ),
                to_pulumi_object_field(
                    "redact_config",
                    &self.r#redact_config,
                ),
                to_pulumi_object_field(
                    "replace_config",
                    &self.r#replace_config,
                ),
                to_pulumi_object_field(
                    "replace_dictionary_config",
                    &self.r#replace_dictionary_config,
                ),
                to_pulumi_object_field(
                    "replace_with_info_type_config",
                    &self.r#replace_with_info_type_config,
                ),
                to_pulumi_object_field(
                    "time_part_config",
                    &self.r#time_part_config,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationInfoTypeTransformationsTransformationPrimitiveTransformation {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#bucketing_config: {
                        let field_value = match fields_map.get("bucketing_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucketing_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#character_mask_config: {
                        let field_value = match fields_map.get("character_mask_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'character_mask_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#crypto_deterministic_config: {
                        let field_value = match fields_map.get("crypto_deterministic_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'crypto_deterministic_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#crypto_hash_config: {
                        let field_value = match fields_map.get("crypto_hash_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'crypto_hash_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#crypto_replace_ffx_fpe_config: {
                        let field_value = match fields_map.get("crypto_replace_ffx_fpe_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'crypto_replace_ffx_fpe_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#date_shift_config: {
                        let field_value = match fields_map.get("date_shift_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'date_shift_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fixed_size_bucketing_config: {
                        let field_value = match fields_map.get("fixed_size_bucketing_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'fixed_size_bucketing_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redact_config: {
                        let field_value = match fields_map.get("redact_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'redact_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#replace_config: {
                        let field_value = match fields_map.get("replace_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'replace_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#replace_dictionary_config: {
                        let field_value = match fields_map.get("replace_dictionary_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'replace_dictionary_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#replace_with_info_type_config: {
                        let field_value = match fields_map.get("replace_with_info_type_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'replace_with_info_type_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time_part_config: {
                        let field_value = match fields_map.get("time_part_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'time_part_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
