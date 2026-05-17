#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ManagementPolicyRuleActionsSnapshot {
    /// The age in days after creation to tier blob snapshot to archive storage. Must be between `0` and `99999`. Defaults to `-1`.
    #[builder(into)]
    #[serde(rename = "changeTierToArchiveAfterDaysSinceCreation")]
    pub r#change_tier_to_archive_after_days_since_creation: Option<i32>,
    /// The age in days after creation to tier blob snapshot to cool storage. Must be between `0` and `99999`. Defaults to `-1`.
    #[builder(into)]
    #[serde(rename = "changeTierToCoolAfterDaysSinceCreation")]
    pub r#change_tier_to_cool_after_days_since_creation: Option<i32>,
    /// The age in days after creation to delete the blob snapshot. Must be between `0` and `99999`. Defaults to `-1`.
    #[builder(into)]
    #[serde(rename = "deleteAfterDaysSinceCreationGreaterThan")]
    pub r#delete_after_days_since_creation_greater_than: Option<i32>,
    /// The age in days after last tier change to the blobs to skip to be archved. Must be between `0` and `99999`. Defaults to `-1`.
    #[builder(into)]
    #[serde(rename = "tierToArchiveAfterDaysSinceLastTierChangeGreaterThan")]
    pub r#tier_to_archive_after_days_since_last_tier_change_greater_than: Option<i32>,
    /// The age in days after creation to cold storage. Supports blob currently at Hot tier. Must be between `0` and `99999`. Defaults to `-1`.
    #[builder(into)]
    #[serde(rename = "tierToColdAfterDaysSinceCreationGreaterThan")]
    pub r#tier_to_cold_after_days_since_creation_greater_than: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ManagementPolicyRuleActionsSnapshot {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "change_tier_to_archive_after_days_since_creation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#change_tier_to_archive_after_days_since_creation,
                )
                .await,
            );
            map.insert(
                "change_tier_to_cool_after_days_since_creation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#change_tier_to_cool_after_days_since_creation,
                )
                .await,
            );
            map.insert(
                "delete_after_days_since_creation_greater_than".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#delete_after_days_since_creation_greater_than,
                )
                .await,
            );
            map.insert(
                "tier_to_archive_after_days_since_last_tier_change_greater_than".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tier_to_archive_after_days_since_last_tier_change_greater_than,
                )
                .await,
            );
            map.insert(
                "tier_to_cold_after_days_since_creation_greater_than".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tier_to_cold_after_days_since_creation_greater_than,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ManagementPolicyRuleActionsSnapshot {
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
                        let field_value = match fields_map.get("change_tier_to_archive_after_days_since_creation") {
                            Some(value) => value,
                            None => bail!("Missing field 'change_tier_to_archive_after_days_since_creation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#change_tier_to_cool_after_days_since_creation: {
                        let field_value = match fields_map.get("change_tier_to_cool_after_days_since_creation") {
                            Some(value) => value,
                            None => bail!("Missing field 'change_tier_to_cool_after_days_since_creation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#tier_to_archive_after_days_since_last_tier_change_greater_than: {
                        let field_value = match fields_map.get("tier_to_archive_after_days_since_last_tier_change_greater_than") {
                            Some(value) => value,
                            None => bail!("Missing field 'tier_to_archive_after_days_since_last_tier_change_greater_than' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
