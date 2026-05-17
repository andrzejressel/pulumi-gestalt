#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationGatewayWafConfiguration {
    /// One or more `disabled_rule_group` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "disabledRuleGroups")]
    pub r#disabled_rule_groups: Option<Vec<super::super::types::network::ApplicationGatewayWafConfigurationDisabledRuleGroup>>,
    /// Is the Web Application Firewall enabled?
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// One or more `exclusion` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "exclusions")]
    pub r#exclusions: Option<Vec<super::super::types::network::ApplicationGatewayWafConfigurationExclusion>>,
    /// The File Upload Limit in MB. Accepted values are in the range `1`MB to `750`MB for the `WAF_v2` SKU, and `1`MB to `500`MB for all other SKUs. Defaults to `100`MB.
    #[builder(into)]
    #[serde(rename = "fileUploadLimitMb")]
    pub r#file_upload_limit_mb: Option<i32>,
    /// The Web Application Firewall Mode. Possible values are `Detection` and `Prevention`.
    #[builder(into)]
    #[serde(rename = "firewallMode")]
    pub r#firewall_mode: String,
    /// The Maximum Request Body Size in KB. Accepted values are in the range `1`KB to `128`KB. Defaults to `128`KB.
    #[builder(into)]
    #[serde(rename = "maxRequestBodySizeKb")]
    pub r#max_request_body_size_kb: Option<i32>,
    /// Is Request Body Inspection enabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "requestBodyCheck")]
    pub r#request_body_check: Option<bool>,
    /// The Type of the Rule Set used for this Web Application Firewall. Possible values are `OWASP`, `Microsoft_BotManagerRuleSet` and `Microsoft_DefaultRuleSet`. Defaults to `OWASP`.
    #[builder(into)]
    #[serde(rename = "ruleSetType")]
    pub r#rule_set_type: Option<String>,
    /// The Version of the Rule Set used for this Web Application Firewall. Possible values are `0.1`, `1.0`, `1.1`, `2.1`, `2.2.9`, `3.0`, `3.1` and `3.2`.
    #[builder(into)]
    #[serde(rename = "ruleSetVersion")]
    pub r#rule_set_version: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ApplicationGatewayWafConfiguration {
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
                "disabled_rule_groups".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disabled_rule_groups,
                )
                .await,
            );
            map.insert(
                "enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enabled,
                )
                .await,
            );
            map.insert(
                "exclusions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#exclusions,
                )
                .await,
            );
            map.insert(
                "file_upload_limit_mb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#file_upload_limit_mb,
                )
                .await,
            );
            map.insert(
                "firewall_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#firewall_mode,
                )
                .await,
            );
            map.insert(
                "max_request_body_size_kb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_request_body_size_kb,
                )
                .await,
            );
            map.insert(
                "request_body_check".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#request_body_check,
                )
                .await,
            );
            map.insert(
                "rule_set_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rule_set_type,
                )
                .await,
            );
            map.insert(
                "rule_set_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rule_set_version,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ApplicationGatewayWafConfiguration {
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
                        let field_value = match fields_map.get("disabled_rule_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'disabled_rule_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("file_upload_limit_mb") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_upload_limit_mb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#firewall_mode: {
                        let field_value = match fields_map.get("firewall_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'firewall_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_request_body_size_kb: {
                        let field_value = match fields_map.get("max_request_body_size_kb") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_request_body_size_kb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_body_check: {
                        let field_value = match fields_map.get("request_body_check") {
                            Some(value) => value,
                            None => bail!("Missing field 'request_body_check' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rule_set_type: {
                        let field_value = match fields_map.get("rule_set_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'rule_set_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rule_set_version: {
                        let field_value = match fields_map.get("rule_set_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'rule_set_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
