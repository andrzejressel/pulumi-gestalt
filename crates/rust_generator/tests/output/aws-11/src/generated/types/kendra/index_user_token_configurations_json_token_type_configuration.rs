#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IndexUserTokenConfigurationsJsonTokenTypeConfiguration {
    /// The group attribute field. Minimum length of 1. Maximum length of 2048.
    #[builder(into)]
    #[serde(rename = "groupAttributeField")]
    pub r#group_attribute_field: String,
    /// The user name attribute field. Minimum length of 1. Maximum length of 2048.
    #[builder(into)]
    #[serde(rename = "userNameAttributeField")]
    pub r#user_name_attribute_field: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for IndexUserTokenConfigurationsJsonTokenTypeConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("group_attribute_field".to_string(), self.r#group_attribute_field.to_pulumi_value().await);
            map.insert("user_name_attribute_field".to_string(), self.r#user_name_attribute_field.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for IndexUserTokenConfigurationsJsonTokenTypeConfiguration {
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
                    r#group_attribute_field: {
                        let field_value = match fields_map.get("group_attribute_field") {
                            Some(value) => value,
                            None => bail!("Missing field 'group_attribute_field' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#user_name_attribute_field: {
                        let field_value = match fields_map.get("user_name_attribute_field") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_name_attribute_field' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
