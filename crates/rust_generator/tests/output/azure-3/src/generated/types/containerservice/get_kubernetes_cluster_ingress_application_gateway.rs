#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetKubernetesClusterIngressApplicationGateway {
    /// The ID of the Application Gateway associated with the ingress controller deployed to this Kubernetes Cluster.
    #[builder(into)]
    #[serde(rename = "effectiveGatewayId")]
    pub r#effective_gateway_id: String,
    /// The ID of the Application Gateway integrated with the ingress controller of this Kubernetes Cluster. This attribute is only set when gateway_id is specified when configuring the `ingress_application_gateway` addon.
    #[builder(into)]
    #[serde(rename = "gatewayId")]
    pub r#gateway_id: String,
    #[builder(into)]
    #[serde(rename = "gatewayName")]
    pub r#gateway_name: String,
    /// An `ingress_application_gateway_identity` block as defined below.
    #[builder(into)]
    #[serde(rename = "ingressApplicationGatewayIdentities")]
    pub r#ingress_application_gateway_identities: Vec<super::super::types::containerservice::GetKubernetesClusterIngressApplicationGatewayIngressApplicationGatewayIdentity>,
    /// The subnet CIDR used to create an Application Gateway, which in turn will be integrated with the ingress controller of this Kubernetes Cluster. This attribute is only set when `subnet_cidr` is specified when configuring the `ingress_application_gateway` addon.
    #[builder(into)]
    #[serde(rename = "subnetCidr")]
    pub r#subnet_cidr: String,
    /// The ID of the subnet on which to create an Application Gateway, which in turn will be integrated with the ingress controller of this Kubernetes Cluster. This attribute is only set when `subnet_id` is specified when configuring the `ingress_application_gateway` addon.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetKubernetesClusterIngressApplicationGateway {
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
                    "effective_gateway_id",
                    &self.r#effective_gateway_id,
                ),
                to_pulumi_object_field(
                    "gateway_id",
                    &self.r#gateway_id,
                ),
                to_pulumi_object_field(
                    "gateway_name",
                    &self.r#gateway_name,
                ),
                to_pulumi_object_field(
                    "ingress_application_gateway_identities",
                    &self.r#ingress_application_gateway_identities,
                ),
                to_pulumi_object_field(
                    "subnet_cidr",
                    &self.r#subnet_cidr,
                ),
                to_pulumi_object_field(
                    "subnet_id",
                    &self.r#subnet_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetKubernetesClusterIngressApplicationGateway {
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
                    r#effective_gateway_id: {
                        let field_value = match fields_map.get("effective_gateway_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'effective_gateway_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gateway_id: {
                        let field_value = match fields_map.get("gateway_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'gateway_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gateway_name: {
                        let field_value = match fields_map.get("gateway_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'gateway_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ingress_application_gateway_identities: {
                        let field_value = match fields_map.get("ingress_application_gateway_identities") {
                            Some(value) => value,
                            None => bail!("Missing field 'ingress_application_gateway_identities' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnet_cidr: {
                        let field_value = match fields_map.get("subnet_cidr") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnet_cidr' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnet_id: {
                        let field_value = match fields_map.get("subnet_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnet_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
