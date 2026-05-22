#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ManagementPolicyRuleActionsVersion {
    /// The age in days after creation to tier blob version to archive storage. Must be between `0` and `99999`. Defaults to `-1`.
    #[builder(into)]
    #[serde(rename = "changeTierToArchiveAfterDaysSinceCreation")]
    pub r#change_tier_to_archive_after_days_since_creation: Option<i32>,
    /// The age in days creation create to tier blob version to cool storage. Must be between `0` and `99999`. Defaults to `-1`.
    #[builder(into)]
    #[serde(rename = "changeTierToCoolAfterDaysSinceCreation")]
    pub r#change_tier_to_cool_after_days_since_creation: Option<i32>,
    /// The age in days after creation to delete the blob version. Must be between `0` and `99999`. Defaults to `-1`.
    #[builder(into)]
    #[serde(rename = "deleteAfterDaysSinceCreation")]
    pub r#delete_after_days_since_creation: Option<i32>,
    /// The age in days after last tier change to the blobs to skip to be archved. Must be between `0` and `99999`. Defaults to `-1`.
    #[builder(into)]
    #[serde(rename = "tierToArchiveAfterDaysSinceLastTierChangeGreaterThan")]
    pub r#tier_to_archive_after_days_since_last_tier_change_greater_than: Option<i32>,
    /// The age in days after creation to cold storage. Supports blob currently at Hot tier. Must be between `0` and `99999`. Defaults to `-1`.
    #[builder(into)]
    #[serde(rename = "tierToColdAfterDaysSinceCreationGreaterThan")]
    pub r#tier_to_cold_after_days_since_creation_greater_than: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ManagementPolicyRuleActionsVersion {
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
                    "changeTierToArchiveAfterDaysSinceCreation",
                    &self.r#change_tier_to_archive_after_days_since_creation,
                ),
                to_pulumi_object_field(
                    "changeTierToCoolAfterDaysSinceCreation",
                    &self.r#change_tier_to_cool_after_days_since_creation,
                ),
                to_pulumi_object_field(
                    "deleteAfterDaysSinceCreation",
                    &self.r#delete_after_days_since_creation,
                ),
                to_pulumi_object_field(
                    "tierToArchiveAfterDaysSinceLastTierChangeGreaterThan",
                    &self.r#tier_to_archive_after_days_since_last_tier_change_greater_than,
                ),
                to_pulumi_object_field(
                    "tierToColdAfterDaysSinceCreationGreaterThan",
                    &self.r#tier_to_cold_after_days_since_creation_greater_than,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ManagementPolicyRuleActionsVersion {
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
                    r#change_tier_to_archive_after_days_since_creation: {
                        let field_value = match fields_map.get("changeTierToArchiveAfterDaysSinceCreation") {
                            Some(value) => value,
                            None => bail!("Missing field 'changeTierToArchiveAfterDaysSinceCreation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#change_tier_to_cool_after_days_since_creation: {
                        let field_value = match fields_map.get("changeTierToCoolAfterDaysSinceCreation") {
                            Some(value) => value,
                            None => bail!("Missing field 'changeTierToCoolAfterDaysSinceCreation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#delete_after_days_since_creation: {
                        let field_value = match fields_map.get("deleteAfterDaysSinceCreation") {
                            Some(value) => value,
                            None => bail!("Missing field 'deleteAfterDaysSinceCreation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tier_to_archive_after_days_since_last_tier_change_greater_than: {
                        let field_value = match fields_map.get("tierToArchiveAfterDaysSinceLastTierChangeGreaterThan") {
                            Some(value) => value,
                            None => bail!("Missing field 'tierToArchiveAfterDaysSinceLastTierChangeGreaterThan' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tier_to_cold_after_days_since_creation_greater_than: {
                        let field_value = match fields_map.get("tierToColdAfterDaysSinceCreationGreaterThan") {
                            Some(value) => value,
                            None => bail!("Missing field 'tierToColdAfterDaysSinceCreationGreaterThan' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
