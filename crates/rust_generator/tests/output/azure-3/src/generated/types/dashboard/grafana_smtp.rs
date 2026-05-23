#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GrafanaSmtp {
    /// Whether to enable the smtp setting of the Grafana instance. Defaults to `false`.
    #[builder(into)]
    pub r#enabled: Option<bool>,
    /// Address used when sending emails.
    #[builder(into)]
    pub r#from_address: String,
    /// Name used when sending emails. Defaults to `Azure Managed Grafana Notification`.
    #[builder(into)]
    pub r#from_name: Option<String>,
    /// SMTP server hostname with port, e.g. test.email.net:587
    #[builder(into)]
    pub r#host: String,
    /// Password of SMTP authentication.
    #[builder(into)]
    pub r#password: String,
    /// Whether to use TLS when connecting to SMTP server. Possible values are `OpportunisticStartTLS`, `NoStartTLS`, `MandatoryStartTLS`.
    #[builder(into)]
    pub r#start_tls_policy: String,
    /// User of SMTP authentication.
    #[builder(into)]
    pub r#user: String,
    /// Whether verify SSL for SMTP server. Defaults to `false`.
    #[builder(into)]
    pub r#verification_skip_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GrafanaSmtp {
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
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "fromAddress",
                    &self.r#from_address,
                ),
                to_pulumi_object_field(
                    "fromName",
                    &self.r#from_name,
                ),
                to_pulumi_object_field(
                    "host",
                    &self.r#host,
                ),
                to_pulumi_object_field(
                    "password",
                    &self.r#password,
                ),
                to_pulumi_object_field(
                    "startTlsPolicy",
                    &self.r#start_tls_policy,
                ),
                to_pulumi_object_field(
                    "user",
                    &self.r#user,
                ),
                to_pulumi_object_field(
                    "verificationSkipEnabled",
                    &self.r#verification_skip_enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("fromAddress") {
                            Some(value) => value,
                            None => bail!("Missing field 'fromAddress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#from_name: {
                        let field_value = match fields_map.get("fromName") {
                            Some(value) => value,
                            None => bail!("Missing field 'fromName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("startTlsPolicy") {
                            Some(value) => value,
                            None => bail!("Missing field 'startTlsPolicy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("verificationSkipEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'verificationSkipEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
