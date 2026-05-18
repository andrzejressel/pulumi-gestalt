#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NextGenerationFirewallVirtualNetworkPanoramaNetworkProfileVnetConfiguration {
    #[builder(into)]
    #[serde(rename = "ipOfTrustForUserDefinedRoutes")]
    pub r#ip_of_trust_for_user_defined_routes: Option<String>,
    /// The ID of the Trust subnet.
    #[builder(into)]
    #[serde(rename = "trustedSubnetId")]
    pub r#trusted_subnet_id: Option<String>,
    /// The ID of the UnTrust subnet.
    #[builder(into)]
    #[serde(rename = "untrustedSubnetId")]
    pub r#untrusted_subnet_id: Option<String>,
    /// The ID of the Virtual Network.
    #[builder(into)]
    #[serde(rename = "virtualNetworkId")]
    pub r#virtual_network_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NextGenerationFirewallVirtualNetworkPanoramaNetworkProfileVnetConfiguration {
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
                    "ip_of_trust_for_user_defined_routes",
                    &self.r#ip_of_trust_for_user_defined_routes,
                ),
                to_pulumi_object_field(
                    "trusted_subnet_id",
                    &self.r#trusted_subnet_id,
                ),
                to_pulumi_object_field(
                    "untrusted_subnet_id",
                    &self.r#untrusted_subnet_id,
                ),
                to_pulumi_object_field(
                    "virtual_network_id",
                    &self.r#virtual_network_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NextGenerationFirewallVirtualNetworkPanoramaNetworkProfileVnetConfiguration {
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
                    r#ip_of_trust_for_user_defined_routes: {
                        let field_value = match fields_map.get("ip_of_trust_for_user_defined_routes") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_of_trust_for_user_defined_routes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#trusted_subnet_id: {
                        let field_value = match fields_map.get("trusted_subnet_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'trusted_subnet_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#untrusted_subnet_id: {
                        let field_value = match fields_map.get("untrusted_subnet_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'untrusted_subnet_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_network_id: {
                        let field_value = match fields_map.get("virtual_network_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtual_network_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
