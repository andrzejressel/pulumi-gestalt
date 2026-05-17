#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AppIngress {
    /// Should this ingress allow insecure connections?
    #[builder(into)]
    #[serde(rename = "allowInsecureConnections")]
    pub r#allow_insecure_connections: Option<bool>,
    /// One or more `custom_domain` block as detailed below.
    #[builder(into)]
    #[serde(rename = "customDomains")]
    pub r#custom_domains: Option<Vec<super::super::types::containerapp::AppIngressCustomDomain>>,
    /// The exposed port on the container for the Ingress traffic.
    /// 
    /// > **Note:** `exposed_port` can only be specified when `transport` is set to `tcp`.
    #[builder(into)]
    #[serde(rename = "exposedPort")]
    pub r#exposed_port: Option<i32>,
    /// Are connections to this Ingress from outside the Container App Environment enabled? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "externalEnabled")]
    pub r#external_enabled: Option<bool>,
    /// The FQDN of the ingress.
    #[builder(into)]
    #[serde(rename = "fqdn")]
    pub r#fqdn: Option<String>,
    /// One or more `ip_security_restriction` blocks for IP-filtering rules as defined below.
    #[builder(into)]
    #[serde(rename = "ipSecurityRestrictions")]
    pub r#ip_security_restrictions: Option<Vec<super::super::types::containerapp::AppIngressIpSecurityRestriction>>,
    /// The target port on the container for the Ingress traffic.
    #[builder(into)]
    #[serde(rename = "targetPort")]
    pub r#target_port: i32,
    /// One or more `traffic_weight` blocks as detailed below.
    #[builder(into)]
    #[serde(rename = "trafficWeights")]
    pub r#traffic_weights: Vec<super::super::types::containerapp::AppIngressTrafficWeight>,
    /// The transport method for the Ingress. Possible values are `auto`, `http`, `http2` and `tcp`. Defaults to `auto`.
    /// 
    /// > **Note:**  if `transport` is set to `tcp`, `exposed_port` and `target_port` should be set at the same time.
    #[builder(into)]
    #[serde(rename = "transport")]
    pub r#transport: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AppIngress {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "allow_insecure_connections".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allow_insecure_connections,
                )
                .await,
            );
            map.insert(
                "custom_domains".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_domains,
                )
                .await,
            );
            map.insert(
                "exposed_port".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#exposed_port,
                )
                .await,
            );
            map.insert(
                "external_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#external_enabled,
                )
                .await,
            );
            map.insert(
                "fqdn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fqdn,
                )
                .await,
            );
            map.insert(
                "ip_security_restrictions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_security_restrictions,
                )
                .await,
            );
            map.insert(
                "target_port".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_port,
                )
                .await,
            );
            map.insert(
                "traffic_weights".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#traffic_weights,
                )
                .await,
            );
            map.insert(
                "transport".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#transport,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AppIngress {
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
