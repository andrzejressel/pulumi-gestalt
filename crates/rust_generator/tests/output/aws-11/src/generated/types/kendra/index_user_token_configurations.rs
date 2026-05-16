#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IndexUserTokenConfigurations {
    /// A block that specifies the information about the JSON token type configuration. Detailed below.
    #[builder(into)]
    #[serde(rename = "jsonTokenTypeConfiguration")]
    pub r#json_token_type_configuration: Option<Box<super::super::types::kendra::IndexUserTokenConfigurationsJsonTokenTypeConfiguration>>,
    /// A block that specifies the information about the JWT token type configuration. Detailed below.
    #[builder(into)]
    #[serde(rename = "jwtTokenTypeConfiguration")]
    pub r#jwt_token_type_configuration: Option<Box<super::super::types::kendra::IndexUserTokenConfigurationsJwtTokenTypeConfiguration>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for IndexUserTokenConfigurations {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("json_token_type_configuration".to_string(), self.r#json_token_type_configuration.to_pulumi_value().await);
            map.insert("jwt_token_type_configuration".to_string(), self.r#jwt_token_type_configuration.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for IndexUserTokenConfigurations {
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
                    r#json_token_type_configuration: {
                        let field_value = match fields_map.get("json_token_type_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'json_token_type_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::kendra::IndexUserTokenConfigurationsJsonTokenTypeConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#jwt_token_type_configuration: {
                        let field_value = match fields_map.get("jwt_token_type_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'jwt_token_type_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::kendra::IndexUserTokenConfigurationsJwtTokenTypeConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
