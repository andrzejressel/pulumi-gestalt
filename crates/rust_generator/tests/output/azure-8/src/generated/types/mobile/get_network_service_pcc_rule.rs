#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetNetworkServicePccRule {
    /// Specifies the name which should be used for this Mobile Network Service.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// A precedence value that is used to decide between data flow policy rules when identifying the QoS values to use for a particular SIM. A lower value means a higher priority.
    #[builder(into)]
    #[serde(rename = "precedence")]
    pub r#precedence: i32,
    /// A `rule_qos_policy` block as defined below. The QoS policy to use for packets matching this rule.
    #[builder(into)]
    #[serde(rename = "qosPolicies")]
    pub r#qos_policies: Vec<super::super::types::mobile::GetNetworkServicePccRuleQosPolicy>,
    /// A `service_data_flow_template` block as defined below. The set of service data flow templates to use for this PCC rule.
    #[builder(into)]
    #[serde(rename = "serviceDataFlowTemplates")]
    pub r#service_data_flow_templates: Vec<super::super::types::mobile::GetNetworkServicePccRuleServiceDataFlowTemplate>,
    /// Determines whether flows that match this data flow policy rule are permitted.
    #[builder(into)]
    #[serde(rename = "trafficControlEnabled")]
    pub r#traffic_control_enabled: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetNetworkServicePccRule {
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
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "precedence".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#precedence,
                )
                .await,
            );
            map.insert(
                "qos_policies".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#qos_policies,
                )
                .await,
            );
            map.insert(
                "service_data_flow_templates".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service_data_flow_templates,
                )
                .await,
            );
            map.insert(
                "traffic_control_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#traffic_control_enabled,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetNetworkServicePccRule {
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
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#precedence: {
                        let field_value = match fields_map.get("precedence") {
                            Some(value) => value,
                            None => bail!("Missing field 'precedence' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#qos_policies: {
                        let field_value = match fields_map.get("qos_policies") {
                            Some(value) => value,
                            None => bail!("Missing field 'qos_policies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_data_flow_templates: {
                        let field_value = match fields_map.get("service_data_flow_templates") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_data_flow_templates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#traffic_control_enabled: {
                        let field_value = match fields_map.get("traffic_control_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'traffic_control_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
