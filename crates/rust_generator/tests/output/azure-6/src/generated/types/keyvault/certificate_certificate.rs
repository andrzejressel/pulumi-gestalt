#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateCertificate {
    /// The base64-encoded certificate contents.
    #[builder(into)]
    #[serde(rename = "contents")]
    pub r#contents: String,
    /// The password associated with the certificate.
    /// 
    /// > **NOTE:** A PEM certificate is already base64 encoded. To successfully import, the `contents` property should include a PEM encoded X509 certificate and a private_key in pkcs8 format. There should only be linux style `\n` line endings and the whole block should have the PEM begin/end blocks around the certificate data and the private key data.
    /// 
    /// To convert a private key to pkcs8 format with openssl use:
    /// ```shell
    /// openssl pkcs8 -topk8 -nocrypt -in private_key.pem > private_key_pk8.pem
    /// ```
    /// 
    /// The PEM content should look something like:
    /// ```text
    /// -----BEGIN CERTIFICATE-----
    /// aGVsbG8KaGVsbG8KaGVsbG8KaGVsbG8KaGVsbG8KaGVsbG8KaGVsbG8KaGVsbG8K
    /// :
    /// aGVsbG8KaGVsbG8KaGVsbG8KaGVsbG8KaGVsbG8KaGVsbG8KaGVsbG8KaGVsbG8K
    /// -----END CERTIFICATE-----
    /// -----BEGIN PRIVATE KEY-----
    /// d29ybGQKd29ybGQKd29ybGQKd29ybGQKd29ybGQKd29ybGQKd29ybGQKd29ybGQK
    /// :
    /// d29ybGQKd29ybGQKd29ybGQKd29ybGQKd29ybGQKd29ybGQKd29ybGQKd29ybGQK
    /// -----END PRIVATE KEY-----
    /// ```
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CertificateCertificate {
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
                    "contents",
                    &self.r#contents,
                ),
                to_pulumi_object_field(
                    "password",
                    &self.r#password,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CertificateCertificate {
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
                    r#contents: {
                        let field_value = match fields_map.get("contents") {
                            Some(value) => value,
                            None => bail!("Missing field 'contents' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
