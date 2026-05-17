#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetQueueOutboundCallerConfig {
    /// Specifies the caller ID name.
    #[builder(into)]
    #[serde(rename = "outboundCallerIdName")]
    pub r#outbound_caller_id_name: String,
    /// Specifies the caller ID number.
    #[builder(into)]
    #[serde(rename = "outboundCallerIdNumberId")]
    pub r#outbound_caller_id_number_id: String,
    /// Outbound whisper flow to be used during an outbound call.
    #[builder(into)]
    #[serde(rename = "outboundFlowId")]
    pub r#outbound_flow_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetQueueOutboundCallerConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "outbound_caller_id_name",
                    &self.r#outbound_caller_id_name,
                ),
                to_pulumi_object_field(
                    "outbound_caller_id_number_id",
                    &self.r#outbound_caller_id_number_id,
                ),
                to_pulumi_object_field(
                    "outbound_flow_id",
                    &self.r#outbound_flow_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetQueueOutboundCallerConfig {
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
                    r#outbound_caller_id_name: {
                        let field_value = match fields_map.get("outbound_caller_id_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'outbound_caller_id_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#outbound_caller_id_number_id: {
                        let field_value = match fields_map.get("outbound_caller_id_number_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'outbound_caller_id_number_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#outbound_flow_id: {
                        let field_value = match fields_map.get("outbound_flow_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'outbound_flow_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
