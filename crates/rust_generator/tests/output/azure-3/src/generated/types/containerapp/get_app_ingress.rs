#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAppIngress {
    /// Should this ingress allow insecure connections?
    #[builder(into)]
    #[serde(rename = "allowInsecureConnections")]
    pub r#allow_insecure_connections: bool,
    /// One or more `custom_domain` block as detailed below.
    #[builder(into)]
    #[serde(rename = "customDomains")]
    pub r#custom_domains: Vec<super::super::types::containerapp::GetAppIngressCustomDomain>,
    /// The exposed port on the container for the Ingress traffic.
    #[builder(into)]
    #[serde(rename = "exposedPort")]
    pub r#exposed_port: i32,
    /// Is this an external Ingress.
    #[builder(into)]
    #[serde(rename = "externalEnabled")]
    pub r#external_enabled: bool,
    /// The FQDN of the ingress.
    #[builder(into)]
    #[serde(rename = "fqdn")]
    pub r#fqdn: String,
    /// One or more `ip_security_restriction` blocks for IP-filtering rules as defined below.
    #[builder(into)]
    #[serde(rename = "ipSecurityRestrictions")]
    pub r#ip_security_restrictions: Vec<super::super::types::containerapp::GetAppIngressIpSecurityRestriction>,
    /// The target port on the container for the Ingress traffic.
    #[builder(into)]
    #[serde(rename = "targetPort")]
    pub r#target_port: i32,
    /// A `traffic_weight` block as detailed below.
    #[builder(into)]
    #[serde(rename = "trafficWeights")]
    pub r#traffic_weights: Vec<super::super::types::containerapp::GetAppIngressTrafficWeight>,
    /// The transport method for the Ingress. Possible values include `auto`, `http`, and `http2`. Defaults to `auto`
    #[builder(into)]
    #[serde(rename = "transport")]
    pub r#transport: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetAppIngress {
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
                    "allow_insecure_connections",
                    &self.r#allow_insecure_connections,
                ),
                to_pulumi_object_field(
                    "custom_domains",
                    &self.r#custom_domains,
                ),
                to_pulumi_object_field(
                    "exposed_port",
                    &self.r#exposed_port,
                ),
                to_pulumi_object_field(
                    "external_enabled",
                    &self.r#external_enabled,
                ),
                to_pulumi_object_field(
                    "fqdn",
                    &self.r#fqdn,
                ),
                to_pulumi_object_field(
                    "ip_security_restrictions",
                    &self.r#ip_security_restrictions,
                ),
                to_pulumi_object_field(
                    "target_port",
                    &self.r#target_port,
                ),
                to_pulumi_object_field(
                    "traffic_weights",
                    &self.r#traffic_weights,
                ),
                to_pulumi_object_field(
                    "transport",
                    &self.r#transport,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetAppIngress {
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
                    r#allow_insecure_connections: {
                        let field_value = match fields_map.get("allow_insecure_connections") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_insecure_connections' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_domains: {
                        let field_value = match fields_map.get("custom_domains") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_domains' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exposed_port: {
                        let field_value = match fields_map.get("exposed_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'exposed_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#external_enabled: {
                        let field_value = match fields_map.get("external_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'external_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fqdn: {
                        let field_value = match fields_map.get("fqdn") {
                            Some(value) => value,
                            None => bail!("Missing field 'fqdn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_security_restrictions: {
                        let field_value = match fields_map.get("ip_security_restrictions") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_security_restrictions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_port: {
                        let field_value = match fields_map.get("target_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#traffic_weights: {
                        let field_value = match fields_map.get("traffic_weights") {
                            Some(value) => value,
                            None => bail!("Missing field 'traffic_weights' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transport: {
                        let field_value = match fields_map.get("transport") {
                            Some(value) => value,
                            None => bail!("Missing field 'transport' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
