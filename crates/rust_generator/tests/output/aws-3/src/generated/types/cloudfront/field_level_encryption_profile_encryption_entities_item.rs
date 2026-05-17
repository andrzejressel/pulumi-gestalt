#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FieldLevelEncryptionProfileEncryptionEntitiesItem {
    /// Object that contains an attribute `items` that contains the list of field patterns in a field-level encryption content type profile specify the fields that you want to be encrypted.
    #[builder(into)]
    #[serde(rename = "fieldPatterns")]
    pub r#field_patterns: Box<super::super::types::cloudfront::FieldLevelEncryptionProfileEncryptionEntitiesItemFieldPatterns>,
    /// The provider associated with the public key being used for encryption.
    #[builder(into)]
    #[serde(rename = "providerId")]
    pub r#provider_id: String,
    /// The public key associated with a set of field-level encryption patterns, to be used when encrypting the fields that match the patterns.
    #[builder(into)]
    #[serde(rename = "publicKeyId")]
    pub r#public_key_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FieldLevelEncryptionProfileEncryptionEntitiesItem {
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
                "field_patterns".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#field_patterns,
                )
                .await,
            );
            map.insert(
                "provider_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#provider_id,
                )
                .await,
            );
            map.insert(
                "public_key_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#public_key_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FieldLevelEncryptionProfileEncryptionEntitiesItem {
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
                    r#field_patterns: {
                        let field_value = match fields_map.get("field_patterns") {
                            Some(value) => value,
                            None => bail!("Missing field 'field_patterns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#provider_id: {
                        let field_value = match fields_map.get("provider_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'provider_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_key_id: {
                        let field_value = match fields_map.get("public_key_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_key_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
