#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TlsInspectionConfigurationCertificateAuthority {
    /// ARN of the certificate.
    #[builder(into)]
    #[serde(rename = "certificateArn")]
    pub r#certificate_arn: String,
    /// Serial number of the certificate.
    #[builder(into)]
    #[serde(rename = "certificateSerial")]
    pub r#certificate_serial: String,
    /// Status of the certificate.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: String,
    /// Details about the certificate status, including information about certificate errors.
    #[builder(into)]
    #[serde(rename = "statusMessage")]
    pub r#status_message: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TlsInspectionConfigurationCertificateAuthority {
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
                    "certificate_arn",
                    &self.r#certificate_arn,
                ),
                to_pulumi_object_field(
                    "certificate_serial",
                    &self.r#certificate_serial,
                ),
                to_pulumi_object_field(
                    "status",
                    &self.r#status,
                ),
                to_pulumi_object_field(
                    "status_message",
                    &self.r#status_message,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TlsInspectionConfigurationCertificateAuthority {
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
                    r#certificate_arn: {
                        let field_value = match fields_map.get("certificate_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificate_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#certificate_serial: {
                        let field_value = match fields_map.get("certificate_serial") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificate_serial' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#status: {
                        let field_value = match fields_map.get("status") {
                            Some(value) => value,
                            None => bail!("Missing field 'status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#status_message: {
                        let field_value = match fields_map.get("status_message") {
                            Some(value) => value,
                            None => bail!("Missing field 'status_message' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
