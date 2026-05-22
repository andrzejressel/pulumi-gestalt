#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetApplicationGatewayWafConfiguration {
    /// One or more `disabled_rule_group` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "disabledRuleGroups")]
    pub r#disabled_rule_groups: Vec<super::super::types::network::GetApplicationGatewayWafConfigurationDisabledRuleGroup>,
    /// Is the Web Application Firewall enabled?
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// One or more `exclusion` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "exclusions")]
    pub r#exclusions: Vec<super::super::types::network::GetApplicationGatewayWafConfigurationExclusion>,
    /// The File Upload Limit in MB.
    #[builder(into)]
    #[serde(rename = "fileUploadLimitMb")]
    pub r#file_upload_limit_mb: i32,
    /// The Web Application Firewall Mode.
    #[builder(into)]
    #[serde(rename = "firewallMode")]
    pub r#firewall_mode: String,
    /// The Maximum Request Body Size in KB.
    #[builder(into)]
    #[serde(rename = "maxRequestBodySizeKb")]
    pub r#max_request_body_size_kb: i32,
    /// Is Request Body Inspection enabled?
    #[builder(into)]
    #[serde(rename = "requestBodyCheck")]
    pub r#request_body_check: bool,
    /// The Type of the Rule Set used for this Web Application Firewall.
    #[builder(into)]
    #[serde(rename = "ruleSetType")]
    pub r#rule_set_type: String,
    /// The Version of the Rule Set used for this Web Application Firewall.
    #[builder(into)]
    #[serde(rename = "ruleSetVersion")]
    pub r#rule_set_version: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetApplicationGatewayWafConfiguration {
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
                    "disabledRuleGroups",
                    &self.r#disabled_rule_groups,
                ),
                to_pulumi_object_field(
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "exclusions",
                    &self.r#exclusions,
                ),
                to_pulumi_object_field(
                    "fileUploadLimitMb",
                    &self.r#file_upload_limit_mb,
                ),
                to_pulumi_object_field(
                    "firewallMode",
                    &self.r#firewall_mode,
                ),
                to_pulumi_object_field(
                    "maxRequestBodySizeKb",
                    &self.r#max_request_body_size_kb,
                ),
                to_pulumi_object_field(
                    "requestBodyCheck",
                    &self.r#request_body_check,
                ),
                to_pulumi_object_field(
                    "ruleSetType",
                    &self.r#rule_set_type,
                ),
                to_pulumi_object_field(
                    "ruleSetVersion",
                    &self.r#rule_set_version,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetApplicationGatewayWafConfiguration {
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
                    r#disabled_rule_groups: {
                        let field_value = match fields_map.get("disabledRuleGroups") {
                            Some(value) => value,
                            None => bail!("Missing field 'disabledRuleGroups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exclusions: {
                        let field_value = match fields_map.get("exclusions") {
                            Some(value) => value,
                            None => bail!("Missing field 'exclusions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_upload_limit_mb: {
                        let field_value = match fields_map.get("fileUploadLimitMb") {
                            Some(value) => value,
                            None => bail!("Missing field 'fileUploadLimitMb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#firewall_mode: {
                        let field_value = match fields_map.get("firewallMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'firewallMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_request_body_size_kb: {
                        let field_value = match fields_map.get("maxRequestBodySizeKb") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxRequestBodySizeKb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_body_check: {
                        let field_value = match fields_map.get("requestBodyCheck") {
                            Some(value) => value,
                            None => bail!("Missing field 'requestBodyCheck' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rule_set_type: {
                        let field_value = match fields_map.get("ruleSetType") {
                            Some(value) => value,
                            None => bail!("Missing field 'ruleSetType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rule_set_version: {
                        let field_value = match fields_map.get("ruleSetVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'ruleSetVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
