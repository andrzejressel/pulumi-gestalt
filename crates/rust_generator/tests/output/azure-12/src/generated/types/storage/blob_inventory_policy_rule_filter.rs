#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BlobInventoryPolicyRuleFilter {
    /// A set of blob types. Possible values are `blockBlob`, `appendBlob`, and `pageBlob`. The storage account with `is_hns_enabled` is `true` doesn't support `pageBlob`.
    /// 
    /// > **NOTE:** The `rules.*.schema_fields` for this rule has to include `BlobType` so that you can specify the `blob_types`.
    #[builder(into)]
    #[serde(rename = "blobTypes")]
    pub r#blob_types: Vec<String>,
    /// A set of strings for blob prefixes to be excluded. Maximum of 10 blob prefixes.
    #[builder(into)]
    #[serde(rename = "excludePrefixes")]
    pub r#exclude_prefixes: Option<Vec<String>>,
    /// Includes blob versions in blob inventory or not? Defaults to `false`.
    /// 
    /// > **NOTE:** The `rules.*.schema_fields` for this rule has to include `IsCurrentVersion` and `VersionId` so that you can specify the `include_blob_versions`.
    #[builder(into)]
    #[serde(rename = "includeBlobVersions")]
    pub r#include_blob_versions: Option<bool>,
    /// Includes deleted blobs in blob inventory or not? Defaults to `false`.
    /// 
    /// > **NOTE:** If `rules.*.scope` is `Container`, the `rules.*.schema_fields` for this rule must include `Deleted`, `Version`, `DeletedTime`, and `RemainingRetentionDays` so that you can specify the `include_deleted`. If `rules.*.scope` is `Blob`, the `rules.*.schema_fields` must include `Deleted` and `RemainingRetentionDays` so that you can specify the `include_deleted`. If `rules.*.scope` is `Blob` and the storage account specified by `storage_account_id` has hierarchical namespaces enabled (`is_hns_enabled` is `true` on the storage account), the `rules.*.schema_fields` for this rule must include `Deleted`, `Version`, `DeletedTime`, and `RemainingRetentionDays` so that you can specify the `include_deleted`.
    #[builder(into)]
    #[serde(rename = "includeDeleted")]
    pub r#include_deleted: Option<bool>,
    /// Includes blob snapshots in blob inventory or not? Defaults to `false`.
    /// 
    /// > **NOTE:** The `rules.*.schema_fields` for this rule has to include `Snapshot` so that you can specify the `include_snapshots`.
    #[builder(into)]
    #[serde(rename = "includeSnapshots")]
    pub r#include_snapshots: Option<bool>,
    /// A set of strings for blob prefixes to be matched. Maximum of 10 blob prefixes.
    #[builder(into)]
    #[serde(rename = "prefixMatches")]
    pub r#prefix_matches: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BlobInventoryPolicyRuleFilter {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "blob_types".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#blob_types,
                )
                .await,
            );
            map.insert(
                "exclude_prefixes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#exclude_prefixes,
                )
                .await,
            );
            map.insert(
                "include_blob_versions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#include_blob_versions,
                )
                .await,
            );
            map.insert(
                "include_deleted".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#include_deleted,
                )
                .await,
            );
            map.insert(
                "include_snapshots".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#include_snapshots,
                )
                .await,
            );
            map.insert(
                "prefix_matches".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#prefix_matches,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BlobInventoryPolicyRuleFilter {
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
                    r#blob_types: {
                        let field_value = match fields_map.get("blob_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'blob_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exclude_prefixes: {
                        let field_value = match fields_map.get("exclude_prefixes") {
                            Some(value) => value,
                            None => bail!("Missing field 'exclude_prefixes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_blob_versions: {
                        let field_value = match fields_map.get("include_blob_versions") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_blob_versions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_deleted: {
                        let field_value = match fields_map.get("include_deleted") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_deleted' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_snapshots: {
                        let field_value = match fields_map.get("include_snapshots") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_snapshots' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#prefix_matches: {
                        let field_value = match fields_map.get("prefix_matches") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefix_matches' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
