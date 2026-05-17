#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BackendServiceSecuritySettings {
    /// The configuration needed to generate a signature for access to private storage buckets that support AWS's Signature Version 4 for authentication.
    /// Allowed only for INTERNET_IP_PORT and INTERNET_FQDN_PORT NEG backends.
    /// Structure is documented below.
    /// 
    /// 
    /// <a name="nested_aws_v4_authentication"></a>The `aws_v4_authentication` block supports:
    #[builder(into)]
    #[serde(rename = "awsV4Authentication")]
    pub r#aws_v_4_authentication: Option<Box<super::super::types::compute::BackendServiceSecuritySettingsAwsV4Authentication>>,
    /// ClientTlsPolicy is a resource that specifies how a client should authenticate
    /// connections to backends of a service. This resource itself does not affect
    /// configuration unless it is attached to a backend service resource.
    #[builder(into)]
    #[serde(rename = "clientTlsPolicy")]
    pub r#client_tls_policy: Option<String>,
    /// A list of alternate names to verify the subject identity in the certificate.
    /// If specified, the client will verify that the server certificate's subject
    /// alt name matches one of the specified values.
    #[builder(into)]
    #[serde(rename = "subjectAltNames")]
    pub r#subject_alt_names: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BackendServiceSecuritySettings {
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
                    "aws_v_4_authentication",
                    &self.r#aws_v_4_authentication,
                ),
                to_pulumi_object_field(
                    "client_tls_policy",
                    &self.r#client_tls_policy,
                ),
                to_pulumi_object_field(
                    "subject_alt_names",
                    &self.r#subject_alt_names,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BackendServiceSecuritySettings {
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
                    r#aws_v_4_authentication: {
                        let field_value = match fields_map.get("aws_v_4_authentication") {
                            Some(value) => value,
                            None => bail!("Missing field 'aws_v_4_authentication' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_tls_policy: {
                        let field_value = match fields_map.get("client_tls_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_tls_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subject_alt_names: {
                        let field_value = match fields_map.get("subject_alt_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'subject_alt_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
