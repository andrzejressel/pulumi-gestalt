#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetKeyVaultNetworkAcl {
    #[builder(into)]
    #[serde(rename = "bypass")]
    pub r#bypass: String,
    #[builder(into)]
    #[serde(rename = "defaultAction")]
    pub r#default_action: String,
    #[builder(into)]
    #[serde(rename = "ipRules")]
    pub r#ip_rules: Vec<String>,
    #[builder(into)]
    #[serde(rename = "virtualNetworkSubnetIds")]
    pub r#virtual_network_subnet_ids: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetKeyVaultNetworkAcl {
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
                    "bypass",
                    &self.r#bypass,
                ),
                to_pulumi_object_field(
                    "default_action",
                    &self.r#default_action,
                ),
                to_pulumi_object_field(
                    "ip_rules",
                    &self.r#ip_rules,
                ),
                to_pulumi_object_field(
                    "virtual_network_subnet_ids",
                    &self.r#virtual_network_subnet_ids,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetKeyVaultNetworkAcl {
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
                    r#bypass: {
                        let field_value = match fields_map.get("bypass") {
                            Some(value) => value,
                            None => bail!("Missing field 'bypass' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#virtual_network_subnet_ids: {
                        let field_value = match fields_map.get("virtual_network_subnet_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtual_network_subnet_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
