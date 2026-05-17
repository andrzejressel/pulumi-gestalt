#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAuthorityConfig {
    /// Specifies some of the values in a certificate that are related to the subject.
    #[builder(into)]
    #[serde(rename = "subjectConfigs")]
    pub r#subject_configs: Vec<super::super::types::certificateauthority::GetAuthorityConfigSubjectConfig>,
    /// When specified this provides a custom SKI to be used in the certificate. This should only be used to maintain a SKI of an existing CA originally created outside CA service, which was not generated using method (1) described in RFC 5280 section 4.2.1.2..
    #[builder(into)]
    #[serde(rename = "subjectKeyIds")]
    pub r#subject_key_ids: Vec<super::super::types::certificateauthority::GetAuthorityConfigSubjectKeyId>,
    /// Describes how some of the technical X.509 fields in a certificate should be populated.
    #[builder(into)]
    #[serde(rename = "x509Configs")]
    pub r#x_509_configs: Vec<super::super::types::certificateauthority::GetAuthorityConfigX509Config>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetAuthorityConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "subject_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subject_configs,
                )
                .await,
            );
            map.insert(
                "subject_key_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subject_key_ids,
                )
                .await,
            );
            map.insert(
                "x_509_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#x_509_configs,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetAuthorityConfig {
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
                    r#subject_configs: {
                        let field_value = match fields_map.get("subject_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'subject_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subject_key_ids: {
                        let field_value = match fields_map.get("subject_key_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'subject_key_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#x_509_configs: {
                        let field_value = match fields_map.get("x_509_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'x_509_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
