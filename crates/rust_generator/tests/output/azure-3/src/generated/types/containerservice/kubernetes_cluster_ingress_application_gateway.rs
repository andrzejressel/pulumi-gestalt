#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubernetesClusterIngressApplicationGateway {
    /// The ID of the Application Gateway associated with the ingress controller deployed to this Kubernetes Cluster.
    #[builder(into)]
    #[serde(rename = "effectiveGatewayId")]
    pub r#effective_gateway_id: Option<String>,
    /// The ID of the Application Gateway to integrate with the ingress controller of this Kubernetes Cluster. See [this](https://docs.microsoft.com/azure/application-gateway/tutorial-ingress-controller-add-on-existing) page for further details.
    #[builder(into)]
    #[serde(rename = "gatewayId")]
    pub r#gateway_id: Option<String>,
    /// The name of the Application Gateway to be used or created in the Nodepool Resource Group, which in turn will be integrated with the ingress controller of this Kubernetes Cluster. See [this](https://docs.microsoft.com/azure/application-gateway/tutorial-ingress-controller-add-on-new) page for further details.
    #[builder(into)]
    #[serde(rename = "gatewayName")]
    pub r#gateway_name: Option<String>,
    /// An `ingress_application_gateway_identity` block is exported. The exported attributes are defined below.
    #[builder(into)]
    #[serde(rename = "ingressApplicationGatewayIdentities")]
    pub r#ingress_application_gateway_identities: Option<Vec<super::super::types::containerservice::KubernetesClusterIngressApplicationGatewayIngressApplicationGatewayIdentity>>,
    /// The subnet CIDR to be used to create an Application Gateway, which in turn will be integrated with the ingress controller of this Kubernetes Cluster. See [this](https://docs.microsoft.com/azure/application-gateway/tutorial-ingress-controller-add-on-new) page for further details.
    #[builder(into)]
    #[serde(rename = "subnetCidr")]
    pub r#subnet_cidr: Option<String>,
    /// The ID of the subnet on which to create an Application Gateway, which in turn will be integrated with the ingress controller of this Kubernetes Cluster. See [this](https://docs.microsoft.com/azure/application-gateway/tutorial-ingress-controller-add-on-new) page for further details.
    /// 
    /// > **Note:** Exactly one of `gateway_id`, `subnet_id` or `subnet_cidr` must be specified.
    /// 
    /// > **Note:** If specifying `ingress_application_gateway` in conjunction with `only_critical_addons_enabled`, the AGIC pod will fail to start. A separate `azure.containerservice.KubernetesClusterNodePool` is required to run the AGIC pod successfully. This is because AGIC is classed as a "non-critical addon".
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KubernetesClusterIngressApplicationGateway {
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
                    "effectiveGatewayId",
                    &self.r#effective_gateway_id,
                ),
                to_pulumi_object_field(
                    "gatewayId",
                    &self.r#gateway_id,
                ),
                to_pulumi_object_field(
                    "gatewayName",
                    &self.r#gateway_name,
                ),
                to_pulumi_object_field(
                    "ingressApplicationGatewayIdentities",
                    &self.r#ingress_application_gateway_identities,
                ),
                to_pulumi_object_field(
                    "subnetCidr",
                    &self.r#subnet_cidr,
                ),
                to_pulumi_object_field(
                    "subnetId",
                    &self.r#subnet_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KubernetesClusterIngressApplicationGateway {
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
                        let field_value = match fields_map.get("effectiveGatewayId") {
                            Some(value) => value,
                            None => bail!("Missing field 'effectiveGatewayId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gateway_id: {
                        let field_value = match fields_map.get("gatewayId") {
                            Some(value) => value,
                            None => bail!("Missing field 'gatewayId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gateway_name: {
                        let field_value = match fields_map.get("gatewayName") {
                            Some(value) => value,
                            None => bail!("Missing field 'gatewayName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ingress_application_gateway_identities: {
                        let field_value = match fields_map.get("ingressApplicationGatewayIdentities") {
                            Some(value) => value,
                            None => bail!("Missing field 'ingressApplicationGatewayIdentities' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnet_cidr: {
                        let field_value = match fields_map.get("subnetCidr") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnetCidr' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnet_id: {
                        let field_value = match fields_map.get("subnetId") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnetId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
