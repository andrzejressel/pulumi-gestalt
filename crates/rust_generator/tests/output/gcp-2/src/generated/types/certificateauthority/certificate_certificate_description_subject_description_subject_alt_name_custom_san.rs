#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateCertificateDescriptionSubjectDescriptionSubjectAltNameCustomSan {
    /// (Output)
    /// Indicates whether or not the name constraints are marked critical.
    #[builder(into)]
    #[serde(rename = "critical")]
    pub r#critical: Option<bool>,
    /// (Output)
    /// Describes how some of the technical fields in a certificate should be populated.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "obectIds")]
    pub r#obect_ids: Option<Vec<super::super::types::certificateauthority::CertificateCertificateDescriptionSubjectDescriptionSubjectAltNameCustomSanObectId>>,
    /// The value of this X.509 extension. A base64-encoded string.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CertificateCertificateDescriptionSubjectDescriptionSubjectAltNameCustomSan {
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
                    "critical",
                    &self.r#critical,
                ),
                to_pulumi_object_field(
                    "obect_ids",
                    &self.r#obect_ids,
                ),
                to_pulumi_object_field(
                    "value",
                    &self.r#value,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CertificateCertificateDescriptionSubjectDescriptionSubjectAltNameCustomSan {
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
                    r#critical: {
                        let field_value = match fields_map.get("critical") {
                            Some(value) => value,
                            None => bail!("Missing field 'critical' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#obect_ids: {
                        let field_value = match fields_map.get("obect_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'obect_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#value: {
                        let field_value = match fields_map.get("value") {
                            Some(value) => value,
                            None => bail!("Missing field 'value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
