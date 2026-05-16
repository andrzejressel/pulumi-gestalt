#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TlsInspectionConfigurationCertificate {
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TlsInspectionConfigurationCertificate {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("certificate_arn".to_string(), self.r#certificate_arn.to_pulumi_value().await);
            map.insert("certificate_serial".to_string(), self.r#certificate_serial.to_pulumi_value().await);
            map.insert("status".to_string(), self.r#status.to_pulumi_value().await);
            map.insert("status_message".to_string(), self.r#status_message.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TlsInspectionConfigurationCertificate {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#certificate_arn: {
                        let field_value = match fields_map.get("certificate_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificate_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#certificate_serial: {
                        let field_value = match fields_map.get("certificate_serial") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificate_serial' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#status: {
                        let field_value = match fields_map.get("status") {
                            Some(value) => value,
                            None => bail!("Missing field 'status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#status_message: {
                        let field_value = match fields_map.get("status_message") {
                            Some(value) => value,
                            None => bail!("Missing field 'status_message' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
