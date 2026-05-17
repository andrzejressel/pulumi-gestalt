#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BudgetActionDefinitionIamActionDefinition {
    /// A list of groups to be attached. There must be at least one group.
    #[builder(into)]
    #[serde(rename = "groups")]
    pub r#groups: Option<Vec<String>>,
    /// The Amazon Resource Name (ARN) of the policy to be attached.
    #[builder(into)]
    #[serde(rename = "policyArn")]
    pub r#policy_arn: String,
    /// A list of roles to be attached. There must be at least one role.
    #[builder(into)]
    #[serde(rename = "roles")]
    pub r#roles: Option<Vec<String>>,
    /// A list of users to be attached. There must be at least one user.
    #[builder(into)]
    #[serde(rename = "users")]
    pub r#users: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BudgetActionDefinitionIamActionDefinition {
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
                "groups".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#groups,
                )
                .await,
            );
            map.insert(
                "policy_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#policy_arn,
                )
                .await,
            );
            map.insert(
                "roles".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#roles,
                )
                .await,
            );
            map.insert(
                "users".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#users,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BudgetActionDefinitionIamActionDefinition {
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
                    r#groups: {
                        let field_value = match fields_map.get("groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#policy_arn: {
                        let field_value = match fields_map.get("policy_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'policy_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#roles: {
                        let field_value = match fields_map.get("roles") {
                            Some(value) => value,
                            None => bail!("Missing field 'roles' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#users: {
                        let field_value = match fields_map.get("users") {
                            Some(value) => value,
                            None => bail!("Missing field 'users' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
