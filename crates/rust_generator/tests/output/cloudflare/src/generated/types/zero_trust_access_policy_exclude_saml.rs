#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ZeroTrustAccessPolicyExcludeSaml {
    /// The name of the SAML attribute.
    #[builder(into)]
    #[serde(rename = "attributeName")]
    pub r#attribute_name: Option<String>,
    /// The SAML attribute value to look for.
    #[builder(into)]
    #[serde(rename = "attributeValue")]
    pub r#attribute_value: Option<String>,
    /// The ID of your SAML identity provider.
    #[builder(into)]
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ZeroTrustAccessPolicyExcludeSaml {
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
                "attribute_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#attribute_name,
                )
                .await,
            );
            map.insert(
                "attribute_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#attribute_value,
                )
                .await,
            );
            map.insert(
                "identity_provider_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#identity_provider_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ZeroTrustAccessPolicyExcludeSaml {
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
                    r#attribute_name: {
                        let field_value = match fields_map.get("attribute_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'attribute_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#attribute_value: {
                        let field_value = match fields_map.get("attribute_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'attribute_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#identity_provider_id: {
                        let field_value = match fields_map.get("identity_provider_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'identity_provider_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
