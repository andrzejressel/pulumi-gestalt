#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateTemplateIdentityConstraints {
    /// Required. If this is true, the SubjectAltNames extension may be copied from a certificate request into the signed certificate. Otherwise, the requested SubjectAltNames will be discarded.
    #[builder(into)]
    #[serde(rename = "allowSubjectAltNamesPassthrough")]
    pub r#allow_subject_alt_names_passthrough: bool,
    /// Required. If this is true, the Subject field may be copied from a certificate request into the signed certificate. Otherwise, the requested Subject will be discarded.
    #[builder(into)]
    #[serde(rename = "allowSubjectPassthrough")]
    pub r#allow_subject_passthrough: bool,
    /// Optional. A CEL expression that may be used to validate the resolved X.509 Subject and/or Subject Alternative Name before a certificate is signed. To see the full allowed syntax and some examples, see https://cloud.google.com/certificate-authority-service/docs/using-cel
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "celExpression")]
    pub r#cel_expression: Option<Box<super::super::types::certificateauthority::CertificateTemplateIdentityConstraintsCelExpression>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CertificateTemplateIdentityConstraints {
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
                "allow_subject_alt_names_passthrough".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allow_subject_alt_names_passthrough,
                )
                .await,
            );
            map.insert(
                "allow_subject_passthrough".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allow_subject_passthrough,
                )
                .await,
            );
            map.insert(
                "cel_expression".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cel_expression,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CertificateTemplateIdentityConstraints {
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
                    r#allow_subject_alt_names_passthrough: {
                        let field_value = match fields_map.get("allow_subject_alt_names_passthrough") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_subject_alt_names_passthrough' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allow_subject_passthrough: {
                        let field_value = match fields_map.get("allow_subject_passthrough") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_subject_passthrough' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cel_expression: {
                        let field_value = match fields_map.get("cel_expression") {
                            Some(value) => value,
                            None => bail!("Missing field 'cel_expression' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
