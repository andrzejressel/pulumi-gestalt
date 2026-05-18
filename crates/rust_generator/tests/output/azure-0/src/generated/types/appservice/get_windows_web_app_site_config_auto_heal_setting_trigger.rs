#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetWindowsWebAppSiteConfigAutoHealSettingTrigger {
    /// The amount of Private Memory used.
    #[builder(into)]
    #[serde(rename = "privateMemoryKb")]
    pub r#private_memory_kb: i32,
    /// A `requests` block as defined above.
    #[builder(into)]
    #[serde(rename = "requests")]
    pub r#requests: Vec<super::super::types::appservice::GetWindowsWebAppSiteConfigAutoHealSettingTriggerRequest>,
    /// (Optional) One or more `slow_request_with_path` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "slowRequestWithPaths")]
    pub r#slow_request_with_paths: Vec<super::super::types::appservice::GetWindowsWebAppSiteConfigAutoHealSettingTriggerSlowRequestWithPath>,
    /// A `slow_request` block as defined above.
    #[builder(into)]
    #[serde(rename = "slowRequests")]
    pub r#slow_requests: Vec<super::super::types::appservice::GetWindowsWebAppSiteConfigAutoHealSettingTriggerSlowRequest>,
    /// A `status_code` block as defined above.
    #[builder(into)]
    #[serde(rename = "statusCodes")]
    pub r#status_codes: Vec<super::super::types::appservice::GetWindowsWebAppSiteConfigAutoHealSettingTriggerStatusCode>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetWindowsWebAppSiteConfigAutoHealSettingTrigger {
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
                    "private_memory_kb",
                    &self.r#private_memory_kb,
                ),
                to_pulumi_object_field(
                    "requests",
                    &self.r#requests,
                ),
                to_pulumi_object_field(
                    "slow_request_with_paths",
                    &self.r#slow_request_with_paths,
                ),
                to_pulumi_object_field(
                    "slow_requests",
                    &self.r#slow_requests,
                ),
                to_pulumi_object_field(
                    "status_codes",
                    &self.r#status_codes,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetWindowsWebAppSiteConfigAutoHealSettingTrigger {
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
                    r#slow_request_with_paths: {
                        let field_value = match fields_map.get("slow_request_with_paths") {
                            Some(value) => value,
                            None => bail!("Missing field 'slow_request_with_paths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#slow_requests: {
                        let field_value = match fields_map.get("slow_requests") {
                            Some(value) => value,
                            None => bail!("Missing field 'slow_requests' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
