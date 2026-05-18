#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionAuthConfigSshPublicKey {
    /// Format of SSH Client cert.
    #[builder(into)]
    #[serde(rename = "certType")]
    pub r#cert_type: Option<String>,
    /// SSH Client Cert. It should contain both public and private key.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "sshClientCert")]
    pub r#ssh_client_cert: Option<Box<super::super::types::integrationconnectors::ConnectionAuthConfigSshPublicKeySshClientCert>>,
    /// Password (passphrase) for ssh client certificate if it has one.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "sshClientCertPass")]
    pub r#ssh_client_cert_pass: Option<Box<super::super::types::integrationconnectors::ConnectionAuthConfigSshPublicKeySshClientCertPass>>,
    /// The user account used to authenticate.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectionAuthConfigSshPublicKey {
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
                    "cert_type",
                    &self.r#cert_type,
                ),
                to_pulumi_object_field(
                    "ssh_client_cert",
                    &self.r#ssh_client_cert,
                ),
                to_pulumi_object_field(
                    "ssh_client_cert_pass",
                    &self.r#ssh_client_cert_pass,
                ),
                to_pulumi_object_field(
                    "username",
                    &self.r#username,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectionAuthConfigSshPublicKey {
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
                    r#cert_type: {
                        let field_value = match fields_map.get("cert_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'cert_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssh_client_cert: {
                        let field_value = match fields_map.get("ssh_client_cert") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssh_client_cert' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssh_client_cert_pass: {
                        let field_value = match fields_map.get("ssh_client_cert_pass") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssh_client_cert_pass' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#username: {
                        let field_value = match fields_map.get("username") {
                            Some(value) => value,
                            None => bail!("Missing field 'username' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
