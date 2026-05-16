#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct QueueOutboundCallerConfig {
    /// Specifies the caller ID name.
    #[builder(into)]
    #[serde(rename = "outboundCallerIdName")]
    pub r#outbound_caller_id_name: Option<String>,
    /// Specifies the caller ID number.
    #[builder(into)]
    #[serde(rename = "outboundCallerIdNumberId")]
    pub r#outbound_caller_id_number_id: Option<String>,
    /// Specifies outbound whisper flow to be used during an outbound call.
    #[builder(into)]
    #[serde(rename = "outboundFlowId")]
    pub r#outbound_flow_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for QueueOutboundCallerConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("outbound_caller_id_name".to_string(), self.r#outbound_caller_id_name.to_pulumi_value().await);
            map.insert("outbound_caller_id_number_id".to_string(), self.r#outbound_caller_id_number_id.to_pulumi_value().await);
            map.insert("outbound_flow_id".to_string(), self.r#outbound_flow_id.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for QueueOutboundCallerConfig {
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
                    r#outbound_caller_id_name: {
                        let field_value = match fields_map.get("outbound_caller_id_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'outbound_caller_id_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#outbound_caller_id_number_id: {
                        let field_value = match fields_map.get("outbound_caller_id_number_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'outbound_caller_id_number_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#outbound_flow_id: {
                        let field_value = match fields_map.get("outbound_flow_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'outbound_flow_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
