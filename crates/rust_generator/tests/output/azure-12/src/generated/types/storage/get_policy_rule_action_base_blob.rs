#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPolicyRuleActionBaseBlob {
    /// Whether a blob should automatically be tiered from cool back to hot if it's accessed again after being tiered to cool.
    #[builder(into)]
    #[serde(rename = "autoTierToHotFromCoolEnabled")]
    pub r#auto_tier_to_hot_from_cool_enabled: bool,
    /// The age in days after creation to delete the blob snapshot.
    #[builder(into)]
    #[serde(rename = "deleteAfterDaysSinceCreationGreaterThan")]
    pub r#delete_after_days_since_creation_greater_than: i32,
    /// The age in days after last access time to delete the blob.
    #[builder(into)]
    #[serde(rename = "deleteAfterDaysSinceLastAccessTimeGreaterThan")]
    pub r#delete_after_days_since_last_access_time_greater_than: i32,
    /// The age in days after last modification to delete the blob.
    #[builder(into)]
    #[serde(rename = "deleteAfterDaysSinceModificationGreaterThan")]
    pub r#delete_after_days_since_modification_greater_than: i32,
    /// The age in days after creation to archive storage.
    #[builder(into)]
    #[serde(rename = "tierToArchiveAfterDaysSinceCreationGreaterThan")]
    pub r#tier_to_archive_after_days_since_creation_greater_than: i32,
    /// The age in days after last access time to tier blobs to archive storage.
    #[builder(into)]
    #[serde(rename = "tierToArchiveAfterDaysSinceLastAccessTimeGreaterThan")]
    pub r#tier_to_archive_after_days_since_last_access_time_greater_than: i32,
    /// The age in days after last tier change to the blobs to skip to be archived.
    #[builder(into)]
    #[serde(rename = "tierToArchiveAfterDaysSinceLastTierChangeGreaterThan")]
    pub r#tier_to_archive_after_days_since_last_tier_change_greater_than: i32,
    /// The age in days after last modification to tier blobs to archive storage.
    #[builder(into)]
    #[serde(rename = "tierToArchiveAfterDaysSinceModificationGreaterThan")]
    pub r#tier_to_archive_after_days_since_modification_greater_than: i32,
    /// Optional The age in days after creation to cold storage. Supports blob currently at Hot tier.
    #[builder(into)]
    #[serde(rename = "tierToColdAfterDaysSinceCreationGreaterThan")]
    pub r#tier_to_cold_after_days_since_creation_greater_than: i32,
    /// The age in days after last access time to tier blobs to cold storage. Supports blob currently at Hot tier.
    #[builder(into)]
    #[serde(rename = "tierToColdAfterDaysSinceLastAccessTimeGreaterThan")]
    pub r#tier_to_cold_after_days_since_last_access_time_greater_than: i32,
    /// The age in days after last modification to tier blobs to cold storage. Supports blob currently at Hot tier.
    #[builder(into)]
    #[serde(rename = "tierToColdAfterDaysSinceModificationGreaterThan")]
    pub r#tier_to_cold_after_days_since_modification_greater_than: i32,
    /// Optional The age in days after creation to cool storage. Supports blob currently at Hot tier.
    #[builder(into)]
    #[serde(rename = "tierToCoolAfterDaysSinceCreationGreaterThan")]
    pub r#tier_to_cool_after_days_since_creation_greater_than: i32,
    /// The age in days after last access time to tier blobs to cool storage. Supports blob currently at Hot tier.
    #[builder(into)]
    #[serde(rename = "tierToCoolAfterDaysSinceLastAccessTimeGreaterThan")]
    pub r#tier_to_cool_after_days_since_last_access_time_greater_than: i32,
    /// The age in days after last modification to tier blobs to cool storage. Supports blob currently at Hot tier.
    #[builder(into)]
    #[serde(rename = "tierToCoolAfterDaysSinceModificationGreaterThan")]
    pub r#tier_to_cool_after_days_since_modification_greater_than: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetPolicyRuleActionBaseBlob {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "auto_tier_to_hot_from_cool_enabled",
                    &self.r#auto_tier_to_hot_from_cool_enabled,
                ),
                to_pulumi_object_field(
                    "delete_after_days_since_creation_greater_than",
                    &self.r#delete_after_days_since_creation_greater_than,
                ),
                to_pulumi_object_field(
                    "delete_after_days_since_last_access_time_greater_than",
                    &self.r#delete_after_days_since_last_access_time_greater_than,
                ),
                to_pulumi_object_field(
                    "delete_after_days_since_modification_greater_than",
                    &self.r#delete_after_days_since_modification_greater_than,
                ),
                to_pulumi_object_field(
                    "tier_to_archive_after_days_since_creation_greater_than",
                    &self.r#tier_to_archive_after_days_since_creation_greater_than,
                ),
                to_pulumi_object_field(
                    "tier_to_archive_after_days_since_last_access_time_greater_than",
                    &self.r#tier_to_archive_after_days_since_last_access_time_greater_than,
                ),
                to_pulumi_object_field(
                    "tier_to_archive_after_days_since_last_tier_change_greater_than",
                    &self.r#tier_to_archive_after_days_since_last_tier_change_greater_than,
                ),
                to_pulumi_object_field(
                    "tier_to_archive_after_days_since_modification_greater_than",
                    &self.r#tier_to_archive_after_days_since_modification_greater_than,
                ),
                to_pulumi_object_field(
                    "tier_to_cold_after_days_since_creation_greater_than",
                    &self.r#tier_to_cold_after_days_since_creation_greater_than,
                ),
                to_pulumi_object_field(
                    "tier_to_cold_after_days_since_last_access_time_greater_than",
                    &self.r#tier_to_cold_after_days_since_last_access_time_greater_than,
                ),
                to_pulumi_object_field(
                    "tier_to_cold_after_days_since_modification_greater_than",
                    &self.r#tier_to_cold_after_days_since_modification_greater_than,
                ),
                to_pulumi_object_field(
                    "tier_to_cool_after_days_since_creation_greater_than",
                    &self.r#tier_to_cool_after_days_since_creation_greater_than,
                ),
                to_pulumi_object_field(
                    "tier_to_cool_after_days_since_last_access_time_greater_than",
                    &self.r#tier_to_cool_after_days_since_last_access_time_greater_than,
                ),
                to_pulumi_object_field(
                    "tier_to_cool_after_days_since_modification_greater_than",
                    &self.r#tier_to_cool_after_days_since_modification_greater_than,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetPolicyRuleActionBaseBlob {
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
                    r#auto_tier_to_hot_from_cool_enabled: {
                        let field_value = match fields_map.get("auto_tier_to_hot_from_cool_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_tier_to_hot_from_cool_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#delete_after_days_since_creation_greater_than: {
                        let field_value = match fields_map.get("delete_after_days_since_creation_greater_than") {
                            Some(value) => value,
                            None => bail!("Missing field 'delete_after_days_since_creation_greater_than' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#delete_after_days_since_last_access_time_greater_than: {
                        let field_value = match fields_map.get("delete_after_days_since_last_access_time_greater_than") {
                            Some(value) => value,
                            None => bail!("Missing field 'delete_after_days_since_last_access_time_greater_than' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#delete_after_days_since_modification_greater_than: {
                        let field_value = match fields_map.get("delete_after_days_since_modification_greater_than") {
                            Some(value) => value,
                            None => bail!("Missing field 'delete_after_days_since_modification_greater_than' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tier_to_archive_after_days_since_creation_greater_than: {
                        let field_value = match fields_map.get("tier_to_archive_after_days_since_creation_greater_than") {
                            Some(value) => value,
                            None => bail!("Missing field 'tier_to_archive_after_days_since_creation_greater_than' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tier_to_archive_after_days_since_last_access_time_greater_than: {
                        let field_value = match fields_map.get("tier_to_archive_after_days_since_last_access_time_greater_than") {
                            Some(value) => value,
                            None => bail!("Missing field 'tier_to_archive_after_days_since_last_access_time_greater_than' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tier_to_archive_after_days_since_last_tier_change_greater_than: {
                        let field_value = match fields_map.get("tier_to_archive_after_days_since_last_tier_change_greater_than") {
                            Some(value) => value,
                            None => bail!("Missing field 'tier_to_archive_after_days_since_last_tier_change_greater_than' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tier_to_archive_after_days_since_modification_greater_than: {
                        let field_value = match fields_map.get("tier_to_archive_after_days_since_modification_greater_than") {
                            Some(value) => value,
                            None => bail!("Missing field 'tier_to_archive_after_days_since_modification_greater_than' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tier_to_cold_after_days_since_creation_greater_than: {
                        let field_value = match fields_map.get("tier_to_cold_after_days_since_creation_greater_than") {
                            Some(value) => value,
                            None => bail!("Missing field 'tier_to_cold_after_days_since_creation_greater_than' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tier_to_cold_after_days_since_last_access_time_greater_than: {
                        let field_value = match fields_map.get("tier_to_cold_after_days_since_last_access_time_greater_than") {
                            Some(value) => value,
                            None => bail!("Missing field 'tier_to_cold_after_days_since_last_access_time_greater_than' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tier_to_cold_after_days_since_modification_greater_than: {
                        let field_value = match fields_map.get("tier_to_cold_after_days_since_modification_greater_than") {
                            Some(value) => value,
                            None => bail!("Missing field 'tier_to_cold_after_days_since_modification_greater_than' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tier_to_cool_after_days_since_creation_greater_than: {
                        let field_value = match fields_map.get("tier_to_cool_after_days_since_creation_greater_than") {
                            Some(value) => value,
                            None => bail!("Missing field 'tier_to_cool_after_days_since_creation_greater_than' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tier_to_cool_after_days_since_last_access_time_greater_than: {
                        let field_value = match fields_map.get("tier_to_cool_after_days_since_last_access_time_greater_than") {
                            Some(value) => value,
                            None => bail!("Missing field 'tier_to_cool_after_days_since_last_access_time_greater_than' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tier_to_cool_after_days_since_modification_greater_than: {
                        let field_value = match fields_map.get("tier_to_cool_after_days_since_modification_greater_than") {
                            Some(value) => value,
                            None => bail!("Missing field 'tier_to_cool_after_days_since_modification_greater_than' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
