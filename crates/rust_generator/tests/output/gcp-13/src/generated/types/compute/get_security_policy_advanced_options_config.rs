#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSecurityPolicyAdvancedOptionsConfig {
    /// Custom configuration to apply the JSON parsing. Only applicable when JSON parsing is set to STANDARD.
    #[builder(into)]
    #[serde(rename = "jsonCustomConfigs")]
    pub r#json_custom_configs: Vec<super::super::types::compute::GetSecurityPolicyAdvancedOptionsConfigJsonCustomConfig>,
    /// JSON body parsing. Supported values include: "DISABLED", "STANDARD".
    #[builder(into)]
    #[serde(rename = "jsonParsing")]
    pub r#json_parsing: String,
    /// Logging level. Supported values include: "NORMAL", "VERBOSE".
    #[builder(into)]
    #[serde(rename = "logLevel")]
    pub r#log_level: String,
    /// An optional list of case-insensitive request header names to use for resolving the callers client IP address.
    #[builder(into)]
    #[serde(rename = "userIpRequestHeaders")]
    pub r#user_ip_request_headers: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetSecurityPolicyAdvancedOptionsConfig {
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
                "json_custom_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#json_custom_configs,
                )
                .await,
            );
            map.insert(
                "json_parsing".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#json_parsing,
                )
                .await,
            );
            map.insert(
                "log_level".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#log_level,
                )
                .await,
            );
            map.insert(
                "user_ip_request_headers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#user_ip_request_headers,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetSecurityPolicyAdvancedOptionsConfig {
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
                    r#json_custom_configs: {
                        let field_value = match fields_map.get("json_custom_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'json_custom_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#json_parsing: {
                        let field_value = match fields_map.get("json_parsing") {
                            Some(value) => value,
                            None => bail!("Missing field 'json_parsing' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_level: {
                        let field_value = match fields_map.get("log_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'log_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_ip_request_headers: {
                        let field_value = match fields_map.get("user_ip_request_headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_ip_request_headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
