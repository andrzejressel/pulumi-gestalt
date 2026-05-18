#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionProfileMysqlProfileSslConfig {
    /// PEM-encoded certificate of the CA that signed the source database
    /// server's certificate.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into)]
    #[serde(rename = "caCertificate")]
    pub r#ca_certificate: Option<String>,
    /// (Output)
    /// Indicates whether the clientKey field is set.
    #[builder(into)]
    #[serde(rename = "caCertificateSet")]
    pub r#ca_certificate_set: Option<bool>,
    /// PEM-encoded certificate that will be used by the replica to
    /// authenticate against the source database server. If this field
    /// is used then the 'clientKey' and the 'caCertificate' fields are
    /// mandatory.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into)]
    #[serde(rename = "clientCertificate")]
    pub r#client_certificate: Option<String>,
    /// (Output)
    /// Indicates whether the clientCertificate field is set.
    #[builder(into)]
    #[serde(rename = "clientCertificateSet")]
    pub r#client_certificate_set: Option<bool>,
    /// PEM-encoded private key associated with the Client Certificate.
    /// If this field is used then the 'client_certificate' and the
    /// 'ca_certificate' fields are mandatory.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into)]
    #[serde(rename = "clientKey")]
    pub r#client_key: Option<String>,
    /// (Output)
    /// Indicates whether the clientKey field is set.
    #[builder(into)]
    #[serde(rename = "clientKeySet")]
    pub r#client_key_set: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectionProfileMysqlProfileSslConfig {
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
                    "ca_certificate",
                    &self.r#ca_certificate,
                ),
                to_pulumi_object_field(
                    "ca_certificate_set",
                    &self.r#ca_certificate_set,
                ),
                to_pulumi_object_field(
                    "client_certificate",
                    &self.r#client_certificate,
                ),
                to_pulumi_object_field(
                    "client_certificate_set",
                    &self.r#client_certificate_set,
                ),
                to_pulumi_object_field(
                    "client_key",
                    &self.r#client_key,
                ),
                to_pulumi_object_field(
                    "client_key_set",
                    &self.r#client_key_set,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectionProfileMysqlProfileSslConfig {
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
                    r#ca_certificate: {
                        let field_value = match fields_map.get("ca_certificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'ca_certificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ca_certificate_set: {
                        let field_value = match fields_map.get("ca_certificate_set") {
                            Some(value) => value,
                            None => bail!("Missing field 'ca_certificate_set' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_certificate: {
                        let field_value = match fields_map.get("client_certificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_certificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_certificate_set: {
                        let field_value = match fields_map.get("client_certificate_set") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_certificate_set' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_key: {
                        let field_value = match fields_map.get("client_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_key_set: {
                        let field_value = match fields_map.get("client_key_set") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_key_set' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
