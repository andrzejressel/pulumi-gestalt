#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateConfigX509ConfigCaOptions {
    /// When true, the "CA" in Basic Constraints extension will be set to true.
    #[builder(into)]
    #[serde(rename = "isCa")]
    pub r#is_ca: Option<bool>,
    /// Refers to the "path length constraint" in Basic Constraints extension. For a CA certificate, this value describes the depth of
    /// subordinate CA certificates that are allowed. If this value is less than 0, the request will fail.
    #[builder(into)]
    #[serde(rename = "maxIssuerPathLength")]
    pub r#max_issuer_path_length: Option<i32>,
    /// When true, the "CA" in Basic Constraints extension will be set to false.
    /// If both `is_ca` and `non_ca` are unset, the extension will be omitted from the CA certificate.
    #[builder(into)]
    #[serde(rename = "nonCa")]
    pub r#non_ca: Option<bool>,
    /// When true, the "path length constraint" in Basic Constraints extension will be set to 0.
    /// if both `max_issuer_path_length` and `zero_max_issuer_path_length` are unset,
    /// the max path length will be omitted from the CA certificate.
    #[builder(into)]
    #[serde(rename = "zeroMaxIssuerPathLength")]
    pub r#zero_max_issuer_path_length: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CertificateConfigX509ConfigCaOptions {
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
                "is_ca".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#is_ca,
                )
                .await,
            );
            map.insert(
                "max_issuer_path_length".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_issuer_path_length,
                )
                .await,
            );
            map.insert(
                "non_ca".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#non_ca,
                )
                .await,
            );
            map.insert(
                "zero_max_issuer_path_length".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#zero_max_issuer_path_length,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CertificateConfigX509ConfigCaOptions {
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
                    r#is_ca: {
                        let field_value = match fields_map.get("is_ca") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_ca' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_issuer_path_length: {
                        let field_value = match fields_map.get("max_issuer_path_length") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_issuer_path_length' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#non_ca: {
                        let field_value = match fields_map.get("non_ca") {
                            Some(value) => value,
                            None => bail!("Missing field 'non_ca' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#zero_max_issuer_path_length: {
                        let field_value = match fields_map.get("zero_max_issuer_path_length") {
                            Some(value) => value,
                            None => bail!("Missing field 'zero_max_issuer_path_length' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
