#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCertificateCertificatePolicyX509CertificateProperty {
    /// A list of Extended/Enhanced Key Usages.
    #[builder(into)]
    #[serde(rename = "extendedKeyUsages")]
    pub r#extended_key_usages: Vec<String>,
    /// A list of uses associated with this Key.
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
    pub r#subject_alternative_names: Vec<super::super::types::keyvault::GetCertificateCertificatePolicyX509CertificatePropertySubjectAlternativeName>,
    /// The Certificates Validity Period in Months.
    #[builder(into)]
    #[serde(rename = "validityInMonths")]
    pub r#validity_in_months: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetCertificateCertificatePolicyX509CertificateProperty {
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
                    "extended_key_usages",
                    &self.r#extended_key_usages,
                ),
                to_pulumi_object_field(
                    "key_usages",
                    &self.r#key_usages,
                ),
                to_pulumi_object_field(
                    "subject",
                    &self.r#subject,
                ),
                to_pulumi_object_field(
                    "subject_alternative_names",
                    &self.r#subject_alternative_names,
                ),
                to_pulumi_object_field(
                    "validity_in_months",
                    &self.r#validity_in_months,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetCertificateCertificatePolicyX509CertificateProperty {
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
