#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetNetworkServicePccRule {
    /// Specifies the name which should be used for this Mobile Network Service.
    #[builder(into)]
    pub r#name: String,
    /// A precedence value that is used to decide between data flow policy rules when identifying the QoS values to use for a particular SIM. A lower value means a higher priority.
    #[builder(into)]
    pub r#precedence: i32,
    /// A `rule_qos_policy` block as defined below. The QoS policy to use for packets matching this rule.
    #[builder(into)]
    pub r#qos_policies: Vec<super::super::types::mobile::GetNetworkServicePccRuleQosPolicy>,
    /// A `service_data_flow_template` block as defined below. The set of service data flow templates to use for this PCC rule.
    #[builder(into)]
    pub r#service_data_flow_templates: Vec<super::super::types::mobile::GetNetworkServicePccRuleServiceDataFlowTemplate>,
    /// Determines whether flows that match this data flow policy rule are permitted.
    #[builder(into)]
    pub r#traffic_control_enabled: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetNetworkServicePccRule {
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
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "precedence",
                    &self.r#precedence,
                ),
                to_pulumi_object_field(
                    "qosPolicies",
                    &self.r#qos_policies,
                ),
                to_pulumi_object_field(
                    "serviceDataFlowTemplates",
                    &self.r#service_data_flow_templates,
                ),
                to_pulumi_object_field(
                    "trafficControlEnabled",
                    &self.r#traffic_control_enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("qosPolicies") {
                            Some(value) => value,
                            None => bail!("Missing field 'qosPolicies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_data_flow_templates: {
                        let field_value = match fields_map.get("serviceDataFlowTemplates") {
                            Some(value) => value,
                            None => bail!("Missing field 'serviceDataFlowTemplates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#traffic_control_enabled: {
                        let field_value = match fields_map.get("trafficControlEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'trafficControlEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
