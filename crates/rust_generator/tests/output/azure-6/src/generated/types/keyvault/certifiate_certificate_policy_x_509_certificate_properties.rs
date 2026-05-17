#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertifiateCertificatePolicyX509CertificateProperties {
    /// A list of Extended/Enhanced Key Usages.
    #[builder(into)]
    #[serde(rename = "extendedKeyUsages")]
    pub r#extended_key_usages: Option<Vec<String>>,
    /// A list of uses associated with this Key. Possible values include `cRLSign`, `dataEncipherment`, `decipherOnly`, `digitalSignature`, `encipherOnly`, `keyAgreement`, `keyCertSign`, `keyEncipherment` and `nonRepudiation` and are case-sensitive.
    #[builder(into)]
    #[serde(rename = "keyUsages")]
    pub r#key_usages: Vec<String>,
    /// The Certificate's Subject.
    #[builder(into)]
    #[serde(rename = "subject")]
    pub r#subject: String,
    /// A `subject_alternative_names` block as defined below.
    #[builder(into)]
    #[serde(rename = "subjectAlternativeNames")]
    pub r#subject_alternative_names: Option<Box<super::super::types::keyvault::CertifiateCertificatePolicyX509CertificatePropertiesSubjectAlternativeNames>>,
    /// The Certificates Validity Period in Months.
    #[builder(into)]
    #[serde(rename = "validityInMonths")]
    pub r#validity_in_months: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CertifiateCertificatePolicyX509CertificateProperties {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "extended_key_usages".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#extended_key_usages,
                )
                .await,
            );
            map.insert(
                "key_usages".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#key_usages,
                )
                .await,
            );
            map.insert(
                "subject".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subject,
                )
                .await,
            );
            map.insert(
                "subject_alternative_names".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subject_alternative_names,
                )
                .await,
            );
            map.insert(
                "validity_in_months".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#validity_in_months,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CertifiateCertificatePolicyX509CertificateProperties {
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
                    r#extended_key_usages: {
                        let field_value = match fields_map.get("extended_key_usages") {
                            Some(value) => value,
                            None => bail!("Missing field 'extended_key_usages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_usages: {
                        let field_value = match fields_map.get("key_usages") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_usages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subject: {
                        let field_value = match fields_map.get("subject") {
                            Some(value) => value,
                            None => bail!("Missing field 'subject' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subject_alternative_names: {
                        let field_value = match fields_map.get("subject_alternative_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'subject_alternative_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#validity_in_months: {
                        let field_value = match fields_map.get("validity_in_months") {
                            Some(value) => value,
                            None => bail!("Missing field 'validity_in_months' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
