#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateCertificateDescriptionSubjectDescription {
    /// (Output)
    /// The serial number encoded in lowercase hexadecimal.
    #[builder(into)]
    #[serde(rename = "hexSerialNumber")]
    pub r#hex_serial_number: Option<String>,
    /// The desired lifetime of the CA certificate. Used to create the "notBeforeTime" and
    /// "notAfterTime" fields inside an X.509 certificate. A duration in seconds with up to nine
    /// fractional digits, terminated by 's'. Example: "3.5s".
    #[builder(into)]
    #[serde(rename = "lifetime")]
    pub r#lifetime: Option<String>,
    /// (Output)
    /// The time at which the certificate expires.
    #[builder(into)]
    #[serde(rename = "notAfterTime")]
    pub r#not_after_time: Option<String>,
    /// (Output)
    /// The time at which the certificate becomes valid.
    #[builder(into)]
    #[serde(rename = "notBeforeTime")]
    pub r#not_before_time: Option<String>,
    /// (Output)
    /// The subject alternative name fields.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "subjectAltNames")]
    pub r#subject_alt_names: Option<Vec<super::super::types::certificateauthority::CertificateCertificateDescriptionSubjectDescriptionSubjectAltName>>,
    /// (Output)
    /// Contains distinguished name fields such as the location and organization.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "subjects")]
    pub r#subjects: Option<Vec<super::super::types::certificateauthority::CertificateCertificateDescriptionSubjectDescriptionSubject>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CertificateCertificateDescriptionSubjectDescription {
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
                    "hex_serial_number",
                    &self.r#hex_serial_number,
                ),
                to_pulumi_object_field(
                    "lifetime",
                    &self.r#lifetime,
                ),
                to_pulumi_object_field(
                    "not_after_time",
                    &self.r#not_after_time,
                ),
                to_pulumi_object_field(
                    "not_before_time",
                    &self.r#not_before_time,
                ),
                to_pulumi_object_field(
                    "subject_alt_names",
                    &self.r#subject_alt_names,
                ),
                to_pulumi_object_field(
                    "subjects",
                    &self.r#subjects,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CertificateCertificateDescriptionSubjectDescription {
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
                    r#hex_serial_number: {
                        let field_value = match fields_map.get("hex_serial_number") {
                            Some(value) => value,
                            None => bail!("Missing field 'hex_serial_number' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lifetime: {
                        let field_value = match fields_map.get("lifetime") {
                            Some(value) => value,
                            None => bail!("Missing field 'lifetime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#not_after_time: {
                        let field_value = match fields_map.get("not_after_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'not_after_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#not_before_time: {
                        let field_value = match fields_map.get("not_before_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'not_before_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#subjects: {
                        let field_value = match fields_map.get("subjects") {
                            Some(value) => value,
                            None => bail!("Missing field 'subjects' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
