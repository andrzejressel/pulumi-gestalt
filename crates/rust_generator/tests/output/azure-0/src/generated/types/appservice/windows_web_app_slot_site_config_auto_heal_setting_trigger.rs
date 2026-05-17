#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WindowsWebAppSlotSiteConfigAutoHealSettingTrigger {
    /// The amount of Private Memory to be consumed for this rule to trigger. Possible values are between `102400` and `13631488`.
    #[builder(into)]
    #[serde(rename = "privateMemoryKb")]
    pub r#private_memory_kb: Option<i32>,
    /// A `requests` block as defined above.
    #[builder(into)]
    #[serde(rename = "requests")]
    pub r#requests: Option<Box<super::super::types::appservice::WindowsWebAppSlotSiteConfigAutoHealSettingTriggerRequests>>,
    /// A `slow_request` block as defined above.
    #[builder(into)]
    #[serde(rename = "slowRequest")]
    pub r#slow_request: Option<Box<super::super::types::appservice::WindowsWebAppSlotSiteConfigAutoHealSettingTriggerSlowRequest>>,
    /// One or more `slow_request_with_path` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "slowRequestWithPaths")]
    pub r#slow_request_with_paths: Option<Vec<super::super::types::appservice::WindowsWebAppSlotSiteConfigAutoHealSettingTriggerSlowRequestWithPath>>,
    /// One or more `status_code` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "statusCodes")]
    pub r#status_codes: Option<Vec<super::super::types::appservice::WindowsWebAppSlotSiteConfigAutoHealSettingTriggerStatusCode>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WindowsWebAppSlotSiteConfigAutoHealSettingTrigger {
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
                "private_memory_kb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#private_memory_kb,
                )
                .await,
            );
            map.insert(
                "requests".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#requests,
                )
                .await,
            );
            map.insert(
                "slow_request".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#slow_request,
                )
                .await,
            );
            map.insert(
                "slow_request_with_paths".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#slow_request_with_paths,
                )
                .await,
            );
            map.insert(
                "status_codes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#status_codes,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WindowsWebAppSlotSiteConfigAutoHealSettingTrigger {
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
                    r#private_memory_kb: {
                        let field_value = match fields_map.get("private_memory_kb") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_memory_kb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#requests: {
                        let field_value = match fields_map.get("requests") {
                            Some(value) => value,
                            None => bail!("Missing field 'requests' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#slow_request: {
                        let field_value = match fields_map.get("slow_request") {
                            Some(value) => value,
                            None => bail!("Missing field 'slow_request' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#slow_request_with_paths: {
                        let field_value = match fields_map.get("slow_request_with_paths") {
                            Some(value) => value,
                            None => bail!("Missing field 'slow_request_with_paths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#status_codes: {
                        let field_value = match fields_map.get("status_codes") {
                            Some(value) => value,
                            None => bail!("Missing field 'status_codes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
