#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PlanRuleCopyActionLifecycle {
    /// Specifies the number of days after creation that a recovery point is moved to cold storage.
    #[builder(into)]
    #[serde(rename = "coldStorageAfter")]
    pub r#cold_storage_after: Option<i32>,
    /// Specifies the number of days after creation that a recovery point is deleted. Must be 90 days greater than `cold_storage_after`.
    #[builder(into)]
    #[serde(rename = "deleteAfter")]
    pub r#delete_after: Option<i32>,
    /// This setting will instruct your backup plan to transition supported resources to archive (cold) storage tier in accordance with your lifecycle settings.
    #[builder(into)]
    #[serde(rename = "optInToArchiveForSupportedResources")]
    pub r#opt_in_to_archive_for_supported_resources: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PlanRuleCopyActionLifecycle {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("cold_storage_after".to_string(), self.r#cold_storage_after.to_pulumi_value().await);
            map.insert("delete_after".to_string(), self.r#delete_after.to_pulumi_value().await);
            map.insert("opt_in_to_archive_for_supported_resources".to_string(), self.r#opt_in_to_archive_for_supported_resources.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PlanRuleCopyActionLifecycle {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#cold_storage_after: {
                        let field_value = match fields_map.get("cold_storage_after") {
                            Some(value) => value,
                            None => bail!("Missing field 'cold_storage_after' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#delete_after: {
                        let field_value = match fields_map.get("delete_after") {
                            Some(value) => value,
                            None => bail!("Missing field 'delete_after' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#opt_in_to_archive_for_supported_resources: {
                        let field_value = match fields_map.get("opt_in_to_archive_for_supported_resources") {
                            Some(value) => value,
                            None => bail!("Missing field 'opt_in_to_archive_for_supported_resources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
