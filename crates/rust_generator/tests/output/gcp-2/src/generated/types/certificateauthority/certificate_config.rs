#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateConfig {
    /// A PublicKey describes a public key.
    /// Structure is documented below.
    /// 
    /// 
    /// <a name="nested_x509_config"></a>The `x509_config` block supports:
    #[builder(into)]
    #[serde(rename = "publicKey")]
    pub r#public_key: Box<super::super::types::certificateauthority::CertificateConfigPublicKey>,
    /// Specifies some of the values in a certificate that are related to the subject.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "subjectConfig")]
    pub r#subject_config: Box<super::super::types::certificateauthority::CertificateConfigSubjectConfig>,
    /// When specified this provides a custom SKI to be used in the certificate. This should only be used to maintain a SKI of an existing CA originally created outside CA service, which was not generated using method (1) described in RFC 5280 section 4.2.1.2..
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "subjectKeyId")]
    pub r#subject_key_id: Option<Box<super::super::types::certificateauthority::CertificateConfigSubjectKeyId>>,
    /// Describes how some of the technical X.509 fields in a certificate should be populated.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "x509Config")]
    pub r#x_509_config: Box<super::super::types::certificateauthority::CertificateConfigX509Config>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CertificateConfig {
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
                    "public_key",
                    &self.r#public_key,
                ),
                to_pulumi_object_field(
                    "subject_config",
                    &self.r#subject_config,
                ),
                to_pulumi_object_field(
                    "subject_key_id",
                    &self.r#subject_key_id,
                ),
                to_pulumi_object_field(
                    "x_509_config",
                    &self.r#x_509_config,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CertificateConfig {
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
                    r#public_key: {
                        let field_value = match fields_map.get("public_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subject_config: {
                        let field_value = match fields_map.get("subject_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'subject_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subject_key_id: {
                        let field_value = match fields_map.get("subject_key_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'subject_key_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#x_509_config: {
                        let field_value = match fields_map.get("x_509_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'x_509_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
