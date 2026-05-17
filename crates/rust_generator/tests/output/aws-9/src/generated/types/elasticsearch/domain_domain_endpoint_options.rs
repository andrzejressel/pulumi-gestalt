#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainDomainEndpointOptions {
    /// Fully qualified domain for your custom endpoint.
    #[builder(into)]
    #[serde(rename = "customEndpoint")]
    pub r#custom_endpoint: Option<String>,
    /// ACM certificate ARN for your custom endpoint.
    #[builder(into)]
    #[serde(rename = "customEndpointCertificateArn")]
    pub r#custom_endpoint_certificate_arn: Option<String>,
    /// Whether to enable custom endpoint for the Elasticsearch domain.
    #[builder(into)]
    #[serde(rename = "customEndpointEnabled")]
    pub r#custom_endpoint_enabled: Option<bool>,
    /// Whether or not to require HTTPS. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "enforceHttps")]
    pub r#enforce_https: Option<bool>,
    /// Name of the TLS security policy that needs to be applied to the HTTPS endpoint. Valid values:  `Policy-Min-TLS-1-0-2019-07`, `Policy-Min-TLS-1-2-2019-07`, and `Policy-Min-TLS-1-2-PFS-2023-10`. Pulumi will only perform drift detection if a configuration value is provided.
    #[builder(into)]
    #[serde(rename = "tlsSecurityPolicy")]
    pub r#tls_security_policy: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DomainDomainEndpointOptions {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "custom_endpoint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_endpoint,
                )
                .await,
            );
            map.insert(
                "custom_endpoint_certificate_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_endpoint_certificate_arn,
                )
                .await,
            );
            map.insert(
                "custom_endpoint_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_endpoint_enabled,
                )
                .await,
            );
            map.insert(
                "enforce_https".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enforce_https,
                )
                .await,
            );
            map.insert(
                "tls_security_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tls_security_policy,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DomainDomainEndpointOptions {
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
                    r#custom_endpoint: {
                        let field_value = match fields_map.get("custom_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_endpoint_certificate_arn: {
                        let field_value = match fields_map.get("custom_endpoint_certificate_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_endpoint_certificate_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_endpoint_enabled: {
                        let field_value = match fields_map.get("custom_endpoint_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_endpoint_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enforce_https: {
                        let field_value = match fields_map.get("enforce_https") {
                            Some(value) => value,
                            None => bail!("Missing field 'enforce_https' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tls_security_policy: {
                        let field_value = match fields_map.get("tls_security_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'tls_security_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
