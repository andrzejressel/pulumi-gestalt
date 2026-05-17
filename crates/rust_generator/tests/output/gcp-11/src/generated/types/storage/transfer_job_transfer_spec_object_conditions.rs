#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TransferJobTransferSpecObjectConditions {
    /// `exclude_prefixes` must follow the requirements described for `include_prefixes`. See [Requirements](https://cloud.google.com/storage-transfer/docs/reference/rest/v1/TransferSpec#ObjectConditions).
    #[builder(into)]
    #[serde(rename = "excludePrefixes")]
    pub r#exclude_prefixes: Option<Vec<String>>,
    /// If `include_prefixes` is specified, objects that satisfy the object conditions must have names that start with one of the `include_prefixes` and that do not start with any of the `exclude_prefixes`. If `include_prefixes` is not specified, all objects except those that have names starting with one of the `exclude_prefixes` must satisfy the object conditions. See [Requirements](https://cloud.google.com/storage-transfer/docs/reference/rest/v1/TransferSpec#ObjectConditions).
    #[builder(into)]
    #[serde(rename = "includePrefixes")]
    pub r#include_prefixes: Option<Vec<String>>,
    /// If specified, only objects with a "last modification time" before this timestamp and objects that don't have a "last modification time" are transferred. A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    #[builder(into)]
    #[serde(rename = "lastModifiedBefore")]
    pub r#last_modified_before: Option<String>,
    /// If specified, only objects with a "last modification time" on or after this timestamp and objects that don't have a "last modification time" are transferred. A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    #[builder(into)]
    #[serde(rename = "lastModifiedSince")]
    pub r#last_modified_since: Option<String>,
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    #[builder(into)]
    #[serde(rename = "maxTimeElapsedSinceLastModification")]
    pub r#max_time_elapsed_since_last_modification: Option<String>,
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    #[builder(into)]
    #[serde(rename = "minTimeElapsedSinceLastModification")]
    pub r#min_time_elapsed_since_last_modification: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TransferJobTransferSpecObjectConditions {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "exclude_prefixes",
                    &self.r#exclude_prefixes,
                ),
                to_pulumi_object_field(
                    "include_prefixes",
                    &self.r#include_prefixes,
                ),
                to_pulumi_object_field(
                    "last_modified_before",
                    &self.r#last_modified_before,
                ),
                to_pulumi_object_field(
                    "last_modified_since",
                    &self.r#last_modified_since,
                ),
                to_pulumi_object_field(
                    "max_time_elapsed_since_last_modification",
                    &self.r#max_time_elapsed_since_last_modification,
                ),
                to_pulumi_object_field(
                    "min_time_elapsed_since_last_modification",
                    &self.r#min_time_elapsed_since_last_modification,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TransferJobTransferSpecObjectConditions {
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
                    r#exclude_prefixes: {
                        let field_value = match fields_map.get("exclude_prefixes") {
                            Some(value) => value,
                            None => bail!("Missing field 'exclude_prefixes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_prefixes: {
                        let field_value = match fields_map.get("include_prefixes") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_prefixes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_modified_before: {
                        let field_value = match fields_map.get("last_modified_before") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_modified_before' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_modified_since: {
                        let field_value = match fields_map.get("last_modified_since") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_modified_since' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_time_elapsed_since_last_modification: {
                        let field_value = match fields_map.get("max_time_elapsed_since_last_modification") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_time_elapsed_since_last_modification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_time_elapsed_since_last_modification: {
                        let field_value = match fields_map.get("min_time_elapsed_since_last_modification") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_time_elapsed_since_last_modification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
