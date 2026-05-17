#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkteamMemberDefinition {
    /// The Amazon Cognito user group that is part of the work team. See Cognito Member Definition details below.
    #[builder(into)]
    #[serde(rename = "cognitoMemberDefinition")]
    pub r#cognito_member_definition: Option<Box<super::super::types::sagemaker::WorkteamMemberDefinitionCognitoMemberDefinition>>,
    /// A list user groups that exist in your OIDC Identity Provider (IdP). One to ten groups can be used to create a single private work team. See Cognito Member Definition details below.
    #[builder(into)]
    #[serde(rename = "oidcMemberDefinition")]
    pub r#oidc_member_definition: Option<Box<super::super::types::sagemaker::WorkteamMemberDefinitionOidcMemberDefinition>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WorkteamMemberDefinition {
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
                "cognito_member_definition".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cognito_member_definition,
                )
                .await,
            );
            map.insert(
                "oidc_member_definition".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#oidc_member_definition,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WorkteamMemberDefinition {
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
                    r#cognito_member_definition: {
                        let field_value = match fields_map.get("cognito_member_definition") {
                            Some(value) => value,
                            None => bail!("Missing field 'cognito_member_definition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oidc_member_definition: {
                        let field_value = match fields_map.get("oidc_member_definition") {
                            Some(value) => value,
                            None => bail!("Missing field 'oidc_member_definition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
