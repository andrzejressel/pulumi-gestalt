#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetBackendAddressPoolBackendAddressInboundNatRulePortMapping {
    /// The Backend Port of the Load Balancing Inbound NAT Rules associated with this Backend Address Pool Address.
    #[builder(into)]
    #[serde(rename = "backendPort")]
    pub r#backend_port: i32,
    /// The Frontend Port of the Load Balancing Inbound NAT Rules associated with this Backend Address Pool Address.
    #[builder(into)]
    #[serde(rename = "frontendPort")]
    pub r#frontend_port: i32,
    /// The name of the Load Balancing Inbound NAT Rules associated with this Backend Address Pool Address.
    #[builder(into)]
    #[serde(rename = "inboundNatRuleName")]
    pub r#inbound_nat_rule_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetBackendAddressPoolBackendAddressInboundNatRulePortMapping {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "backend_port",
                    &self.r#backend_port,
                ),
                to_pulumi_object_field(
                    "frontend_port",
                    &self.r#frontend_port,
                ),
                to_pulumi_object_field(
                    "inbound_nat_rule_name",
                    &self.r#inbound_nat_rule_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetBackendAddressPoolBackendAddressInboundNatRulePortMapping {
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
                    r#backend_port: {
                        let field_value = match fields_map.get("backend_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'backend_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#frontend_port: {
                        let field_value = match fields_map.get("frontend_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'frontend_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#inbound_nat_rule_name: {
                        let field_value = match fields_map.get("inbound_nat_rule_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'inbound_nat_rule_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
