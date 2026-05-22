#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HadoopClusterRolesEdgeNodeHttpsEndpoint {
    /// A list of access modes for the application.
    #[builder(into)]
    #[serde(rename = "accessModes")]
    pub r#access_modes: Option<Vec<String>>,
    /// The destination port to connect to.
    #[builder(into)]
    #[serde(rename = "destinationPort")]
    pub r#destination_port: Option<i32>,
    /// The value indicates whether the gateway authentication is enabled or not.
    #[builder(into)]
    #[serde(rename = "disableGatewayAuth")]
    pub r#disable_gateway_auth: Option<bool>,
    /// The private ip address of the endpoint.
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Option<String>,
    /// The application's subdomain suffix.
    #[builder(into)]
    #[serde(rename = "subDomainSuffix")]
    pub r#sub_domain_suffix: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for HadoopClusterRolesEdgeNodeHttpsEndpoint {
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
                    "accessModes",
                    &self.r#access_modes,
                ),
                to_pulumi_object_field(
                    "destinationPort",
                    &self.r#destination_port,
                ),
                to_pulumi_object_field(
                    "disableGatewayAuth",
                    &self.r#disable_gateway_auth,
                ),
                to_pulumi_object_field(
                    "privateIpAddress",
                    &self.r#private_ip_address,
                ),
                to_pulumi_object_field(
                    "subDomainSuffix",
                    &self.r#sub_domain_suffix,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for HadoopClusterRolesEdgeNodeHttpsEndpoint {
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
                    r#access_modes: {
                        let field_value = match fields_map.get("accessModes") {
                            Some(value) => value,
                            None => bail!("Missing field 'accessModes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_port: {
                        let field_value = match fields_map.get("destinationPort") {
                            Some(value) => value,
                            None => bail!("Missing field 'destinationPort' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disable_gateway_auth: {
                        let field_value = match fields_map.get("disableGatewayAuth") {
                            Some(value) => value,
                            None => bail!("Missing field 'disableGatewayAuth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_ip_address: {
                        let field_value = match fields_map.get("privateIpAddress") {
                            Some(value) => value,
                            None => bail!("Missing field 'privateIpAddress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sub_domain_suffix: {
                        let field_value = match fields_map.get("subDomainSuffix") {
                            Some(value) => value,
                            None => bail!("Missing field 'subDomainSuffix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
