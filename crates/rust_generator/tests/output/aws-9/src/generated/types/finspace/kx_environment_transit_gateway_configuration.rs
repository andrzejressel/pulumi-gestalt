#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KxEnvironmentTransitGatewayConfiguration {
    /// Rules that define how you manage outbound traffic from kdb network to your internal network. Defined below.
    #[builder(into)]
    #[serde(rename = "attachmentNetworkAclConfigurations")]
    pub r#attachment_network_acl_configurations: Option<Vec<super::super::types::finspace::KxEnvironmentTransitGatewayConfigurationAttachmentNetworkAclConfiguration>>,
    /// Routing CIDR on behalf of KX environment. It could be any “/26 range in the 100.64.0.0 CIDR space. After providing, it will be added to the customer’s transit gateway routing table so that the traffics could be routed to KX network.
    #[builder(into)]
    #[serde(rename = "routableCidrSpace")]
    pub r#routable_cidr_space: String,
    /// Identifier of the transit gateway created by the customer to connect outbound traffics from KX network to your internal network.
    #[builder(into)]
    #[serde(rename = "transitGatewayId")]
    pub r#transit_gateway_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KxEnvironmentTransitGatewayConfiguration {
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
                    "attachmentNetworkAclConfigurations",
                    &self.r#attachment_network_acl_configurations,
                ),
                to_pulumi_object_field(
                    "routableCidrSpace",
                    &self.r#routable_cidr_space,
                ),
                to_pulumi_object_field(
                    "transitGatewayId",
                    &self.r#transit_gateway_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KxEnvironmentTransitGatewayConfiguration {
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
                    r#attachment_network_acl_configurations: {
                        let field_value = match fields_map.get("attachmentNetworkAclConfigurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'attachmentNetworkAclConfigurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#routable_cidr_space: {
                        let field_value = match fields_map.get("routableCidrSpace") {
                            Some(value) => value,
                            None => bail!("Missing field 'routableCidrSpace' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transit_gateway_id: {
                        let field_value = match fields_map.get("transitGatewayId") {
                            Some(value) => value,
                            None => bail!("Missing field 'transitGatewayId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
