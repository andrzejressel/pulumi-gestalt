#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAuthorityConfigX509ConfigKeyUsage {
    /// Describes high-level ways in which a key may be used.
    #[builder(into)]
    #[serde(rename = "baseKeyUsages")]
    pub r#base_key_usages: Vec<super::super::types::certificateauthority::GetAuthorityConfigX509ConfigKeyUsageBaseKeyUsage>,
    /// Describes high-level ways in which a key may be used.
    #[builder(into)]
    #[serde(rename = "extendedKeyUsages")]
    pub r#extended_key_usages: Vec<super::super::types::certificateauthority::GetAuthorityConfigX509ConfigKeyUsageExtendedKeyUsage>,
    /// An ObjectId specifies an object identifier (OID). These provide context and describe types in ASN.1 messages.
    #[builder(into)]
    #[serde(rename = "unknownExtendedKeyUsages")]
    pub r#unknown_extended_key_usages: Vec<super::super::types::certificateauthority::GetAuthorityConfigX509ConfigKeyUsageUnknownExtendedKeyUsage>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetAuthorityConfigX509ConfigKeyUsage {
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
                "base_key_usages".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#base_key_usages,
                )
                .await,
            );
            map.insert(
                "extended_key_usages".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#extended_key_usages,
                )
                .await,
            );
            map.insert(
                "unknown_extended_key_usages".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#unknown_extended_key_usages,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetAuthorityConfigX509ConfigKeyUsage {
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
                    r#base_key_usages: {
                        let field_value = match fields_map.get("base_key_usages") {
                            Some(value) => value,
                            None => bail!("Missing field 'base_key_usages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#extended_key_usages: {
                        let field_value = match fields_map.get("extended_key_usages") {
                            Some(value) => value,
                            None => bail!("Missing field 'extended_key_usages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#unknown_extended_key_usages: {
                        let field_value = match fields_map.get("unknown_extended_key_usages") {
                            Some(value) => value,
                            None => bail!("Missing field 'unknown_extended_key_usages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
