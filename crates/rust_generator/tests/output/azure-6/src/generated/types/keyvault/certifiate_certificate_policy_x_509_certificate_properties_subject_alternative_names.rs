#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertifiateCertificatePolicyX509CertificatePropertiesSubjectAlternativeNames {
    /// A list of alternative DNS names (FQDNs) identified by the Certificate.
    #[builder(into)]
    #[serde(rename = "dnsNames")]
    pub r#dns_names: Option<Vec<String>>,
    /// A list of email addresses identified by this Certificate.
    #[builder(into)]
    #[serde(rename = "emails")]
    pub r#emails: Option<Vec<String>>,
    /// A list of User Principal Names identified by the Certificate.
    #[builder(into)]
    #[serde(rename = "upns")]
    pub r#upns: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CertifiateCertificatePolicyX509CertificatePropertiesSubjectAlternativeNames {
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
                "dns_names".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dns_names,
                )
                .await,
            );
            map.insert(
                "emails".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#emails,
                )
                .await,
            );
            map.insert(
                "upns".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#upns,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CertifiateCertificatePolicyX509CertificatePropertiesSubjectAlternativeNames {
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
                    r#dns_names: {
                        let field_value = match fields_map.get("dns_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'dns_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#emails: {
                        let field_value = match fields_map.get("emails") {
                            Some(value) => value,
                            None => bail!("Missing field 'emails' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#upns: {
                        let field_value = match fields_map.get("upns") {
                            Some(value) => value,
                            None => bail!("Missing field 'upns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
