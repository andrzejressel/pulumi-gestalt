#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetIndexUserTokenConfiguration {
    /// A block that specifies the information about the JSON token type configuration.
    #[builder(into)]
    #[serde(rename = "jsonTokenTypeConfigurations")]
    pub r#json_token_type_configurations: Vec<super::super::types::kendra::GetIndexUserTokenConfigurationJsonTokenTypeConfiguration>,
    /// A block that specifies the information about the JWT token type configuration.
    #[builder(into)]
    #[serde(rename = "jwtTokenTypeConfigurations")]
    pub r#jwt_token_type_configurations: Vec<super::super::types::kendra::GetIndexUserTokenConfigurationJwtTokenTypeConfiguration>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetIndexUserTokenConfiguration {
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
                "json_token_type_configurations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#json_token_type_configurations,
                )
                .await,
            );
            map.insert(
                "jwt_token_type_configurations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#jwt_token_type_configurations,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetIndexUserTokenConfiguration {
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
                    r#json_token_type_configurations: {
                        let field_value = match fields_map.get("json_token_type_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'json_token_type_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#jwt_token_type_configurations: {
                        let field_value = match fields_map.get("jwt_token_type_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'jwt_token_type_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
