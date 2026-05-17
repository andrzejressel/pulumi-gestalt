#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetApplicationGatewaySslProfileSslPolicy {
    /// A List of accepted cipher suites.
    #[builder(into)]
    #[serde(rename = "cipherSuites")]
    pub r#cipher_suites: Vec<String>,
    /// A list of SSL Protocols which are disabled on this Application Gateway.
    #[builder(into)]
    #[serde(rename = "disabledProtocols")]
    pub r#disabled_protocols: Vec<String>,
    /// The minimum TLS version.
    #[builder(into)]
    #[serde(rename = "minProtocolVersion")]
    pub r#min_protocol_version: String,
    /// The Name of the Policy.
    #[builder(into)]
    #[serde(rename = "policyName")]
    pub r#policy_name: String,
    /// The Type of the Policy.
    #[builder(into)]
    #[serde(rename = "policyType")]
    pub r#policy_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetApplicationGatewaySslProfileSslPolicy {
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
                "cipher_suites".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cipher_suites,
                )
                .await,
            );
            map.insert(
                "disabled_protocols".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disabled_protocols,
                )
                .await,
            );
            map.insert(
                "min_protocol_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min_protocol_version,
                )
                .await,
            );
            map.insert(
                "policy_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#policy_name,
                )
                .await,
            );
            map.insert(
                "policy_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#policy_type,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetApplicationGatewaySslProfileSslPolicy {
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
                    r#cipher_suites: {
                        let field_value = match fields_map.get("cipher_suites") {
                            Some(value) => value,
                            None => bail!("Missing field 'cipher_suites' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disabled_protocols: {
                        let field_value = match fields_map.get("disabled_protocols") {
                            Some(value) => value,
                            None => bail!("Missing field 'disabled_protocols' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_protocol_version: {
                        let field_value = match fields_map.get("min_protocol_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_protocol_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#policy_name: {
                        let field_value = match fields_map.get("policy_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'policy_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#policy_type: {
                        let field_value = match fields_map.get("policy_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'policy_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
