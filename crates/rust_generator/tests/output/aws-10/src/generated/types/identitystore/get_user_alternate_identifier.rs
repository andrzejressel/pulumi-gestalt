#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetUserAlternateIdentifier {
    /// Configuration block for filtering by the identifier issued by an external identity provider. Detailed below.
    #[builder(into)]
    #[serde(rename = "externalId")]
    pub r#external_id: Option<Box<super::super::types::identitystore::GetUserAlternateIdentifierExternalId>>,
    /// An entity attribute that's unique to a specific entity. Detailed below.
    /// 
    /// > Exactly one of the above arguments must be provided.
    #[builder(into)]
    #[serde(rename = "uniqueAttribute")]
    pub r#unique_attribute: Option<Box<super::super::types::identitystore::GetUserAlternateIdentifierUniqueAttribute>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetUserAlternateIdentifier {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("external_id".to_string(), self.r#external_id.to_pulumi_value().await);
            map.insert("unique_attribute".to_string(), self.r#unique_attribute.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetUserAlternateIdentifier {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#external_id: {
                        let field_value = match fields_map.get("external_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'external_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::identitystore::GetUserAlternateIdentifierExternalId>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#unique_attribute: {
                        let field_value = match fields_map.get("unique_attribute") {
                            Some(value) => value,
                            None => bail!("Missing field 'unique_attribute' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::identitystore::GetUserAlternateIdentifierUniqueAttribute>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
