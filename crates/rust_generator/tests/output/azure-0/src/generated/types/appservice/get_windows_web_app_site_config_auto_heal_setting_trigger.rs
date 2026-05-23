#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetWindowsWebAppSiteConfigAutoHealSettingTrigger {
    /// The amount of Private Memory used.
    #[builder(into)]
    pub r#private_memory_kb: i32,
    /// A `requests` block as defined above.
    #[builder(into)]
    pub r#requests: Vec<super::super::types::appservice::GetWindowsWebAppSiteConfigAutoHealSettingTriggerRequest>,
    /// (Optional) One or more `slow_request_with_path` blocks as defined above.
    #[builder(into)]
    pub r#slow_request_with_paths: Vec<super::super::types::appservice::GetWindowsWebAppSiteConfigAutoHealSettingTriggerSlowRequestWithPath>,
    /// A `slow_request` block as defined above.
    #[builder(into)]
    pub r#slow_requests: Vec<super::super::types::appservice::GetWindowsWebAppSiteConfigAutoHealSettingTriggerSlowRequest>,
    /// A `status_code` block as defined above.
    #[builder(into)]
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
                    "privateMemoryKb",
                    &self.r#private_memory_kb,
                ),
                to_pulumi_object_field(
                    "requests",
                    &self.r#requests,
                ),
                to_pulumi_object_field(
                    "slowRequestWithPaths",
                    &self.r#slow_request_with_paths,
                ),
                to_pulumi_object_field(
                    "slowRequests",
                    &self.r#slow_requests,
                ),
                to_pulumi_object_field(
                    "statusCodes",
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
                        let field_value = match fields_map.get("privateMemoryKb") {
                            Some(value) => value,
                            None => bail!("Missing field 'privateMemoryKb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("slowRequestWithPaths") {
                            Some(value) => value,
                            None => bail!("Missing field 'slowRequestWithPaths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#slow_requests: {
                        let field_value = match fields_map.get("slowRequests") {
                            Some(value) => value,
                            None => bail!("Missing field 'slowRequests' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#status_codes: {
                        let field_value = match fields_map.get("statusCodes") {
                            Some(value) => value,
                            None => bail!("Missing field 'statusCodes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
