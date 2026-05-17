#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GrafanaSmtp {
    /// Whether to enable the smtp setting of the Grafana instance. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Address used when sending emails.
    #[builder(into)]
    #[serde(rename = "fromAddress")]
    pub r#from_address: String,
    /// Name used when sending emails. Defaults to `Azure Managed Grafana Notification`.
    #[builder(into)]
    #[serde(rename = "fromName")]
    pub r#from_name: Option<String>,
    /// SMTP server hostname with port, e.g. test.email.net:587
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: String,
    /// Password of SMTP authentication.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: String,
    /// Whether to use TLS when connecting to SMTP server. Possible values are `OpportunisticStartTLS`, `NoStartTLS`, `MandatoryStartTLS`.
    #[builder(into)]
    #[serde(rename = "startTlsPolicy")]
    pub r#start_tls_policy: String,
    /// User of SMTP authentication.
    #[builder(into)]
    #[serde(rename = "user")]
    pub r#user: String,
    /// Whether verify SSL for SMTP server. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "verificationSkipEnabled")]
    pub r#verification_skip_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GrafanaSmtp {
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
                "enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enabled,
                )
                .await,
            );
            map.insert(
                "from_address".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#from_address,
                )
                .await,
            );
            map.insert(
                "from_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#from_name,
                )
                .await,
            );
            map.insert(
                "host".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#host,
                )
                .await,
            );
            map.insert(
                "password".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#password,
                )
                .await,
            );
            map.insert(
                "start_tls_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#start_tls_policy,
                )
                .await,
            );
            map.insert(
                "user".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#user,
                )
                .await,
            );
            map.insert(
                "verification_skip_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#verification_skip_enabled,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GrafanaSmtp {
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
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#from_address: {
                        let field_value = match fields_map.get("from_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'from_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#from_name: {
                        let field_value = match fields_map.get("from_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'from_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host: {
                        let field_value = match fields_map.get("host") {
                            Some(value) => value,
                            None => bail!("Missing field 'host' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#password: {
                        let field_value = match fields_map.get("password") {
                            Some(value) => value,
                            None => bail!("Missing field 'password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#start_tls_policy: {
                        let field_value = match fields_map.get("start_tls_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'start_tls_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user: {
                        let field_value = match fields_map.get("user") {
                            Some(value) => value,
                            None => bail!("Missing field 'user' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#verification_skip_enabled: {
                        let field_value = match fields_map.get("verification_skip_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'verification_skip_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
