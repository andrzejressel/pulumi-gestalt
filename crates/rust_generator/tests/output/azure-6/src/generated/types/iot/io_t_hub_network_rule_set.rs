#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IoTHubNetworkRuleSet {
    /// Determines if Network Rule Set is also applied to the BuiltIn EventHub EndPoint of the IotHub. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "applyToBuiltinEventhubEndpoint")]
    pub r#apply_to_builtin_eventhub_endpoint: Option<bool>,
    /// Default Action for Network Rule Set. Possible values are `DefaultActionDeny`, `DefaultActionAllow`. Defaults to `DefaultActionDeny`.
    #[builder(into)]
    #[serde(rename = "defaultAction")]
    pub r#default_action: Option<String>,
    /// One or more `ip_rule` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "ipRules")]
    pub r#ip_rules: Option<Vec<super::super::types::iot::IoTHubNetworkRuleSetIpRule>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for IoTHubNetworkRuleSet {
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
                "apply_to_builtin_eventhub_endpoint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#apply_to_builtin_eventhub_endpoint,
                )
                .await,
            );
            map.insert(
                "default_action".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_action,
                )
                .await,
            );
            map.insert(
                "ip_rules".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_rules,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for IoTHubNetworkRuleSet {
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
                    r#apply_to_builtin_eventhub_endpoint: {
                        let field_value = match fields_map.get("apply_to_builtin_eventhub_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'apply_to_builtin_eventhub_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_action: {
                        let field_value = match fields_map.get("default_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_rules: {
                        let field_value = match fields_map.get("ip_rules") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_rules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
