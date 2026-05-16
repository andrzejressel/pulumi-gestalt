#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPolicyDocumentStatement {
    /// List of actions that this statement either allows or denies. For example, `["ec2:RunInstances", "s3:*"]`.
    #[builder(into)]
    #[serde(rename = "actions")]
    pub r#actions: Option<Vec<String>>,
    /// Configuration block for a condition. Detailed below.
    #[builder(into)]
    #[serde(rename = "conditions")]
    pub r#conditions: Option<Vec<super::super::types::iam::GetPolicyDocumentStatementCondition>>,
    /// Whether this statement allows or denies the given actions. Valid values are `Allow` and `Deny`. Defaults to `Allow`.
    #[builder(into)]
    #[serde(rename = "effect")]
    pub r#effect: Option<String>,
    /// List of actions that this statement does *not* apply to. Use to apply a policy statement to all actions *except* those listed.
    #[builder(into)]
    #[serde(rename = "notActions")]
    pub r#not_actions: Option<Vec<String>>,
    /// Like `principals` except these are principals that the statement does *not* apply to.
    #[builder(into)]
    #[serde(rename = "notPrincipals")]
    pub r#not_principals: Option<Vec<super::super::types::iam::GetPolicyDocumentStatementNotPrincipal>>,
    /// List of resource ARNs that this statement does *not* apply to. Use to apply a policy statement to all resources *except* those listed. Conflicts with `resources`.
    #[builder(into)]
    #[serde(rename = "notResources")]
    pub r#not_resources: Option<Vec<String>>,
    /// Configuration block for principals. Detailed below.
    #[builder(into)]
    #[serde(rename = "principals")]
    pub r#principals: Option<Vec<super::super::types::iam::GetPolicyDocumentStatementPrincipal>>,
    /// List of resource ARNs that this statement applies to. This is required by AWS if used for an IAM policy. Conflicts with `not_resources`.
    #[builder(into)]
    #[serde(rename = "resources")]
    pub r#resources: Option<Vec<String>>,
    /// Sid (statement ID) is an identifier for a policy statement.
    #[builder(into)]
    #[serde(rename = "sid")]
    pub r#sid: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetPolicyDocumentStatement {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("actions".to_string(), self.r#actions.to_pulumi_value().await);
            map.insert("conditions".to_string(), self.r#conditions.to_pulumi_value().await);
            map.insert("effect".to_string(), self.r#effect.to_pulumi_value().await);
            map.insert("not_actions".to_string(), self.r#not_actions.to_pulumi_value().await);
            map.insert("not_principals".to_string(), self.r#not_principals.to_pulumi_value().await);
            map.insert("not_resources".to_string(), self.r#not_resources.to_pulumi_value().await);
            map.insert("principals".to_string(), self.r#principals.to_pulumi_value().await);
            map.insert("resources".to_string(), self.r#resources.to_pulumi_value().await);
            map.insert("sid".to_string(), self.r#sid.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetPolicyDocumentStatement {
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
                    r#actions: {
                        let field_value = match fields_map.get("actions") {
                            Some(value) => value,
                            None => bail!("Missing field 'actions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#conditions: {
                        let field_value = match fields_map.get("conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::iam::GetPolicyDocumentStatementCondition>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#effect: {
                        let field_value = match fields_map.get("effect") {
                            Some(value) => value,
                            None => bail!("Missing field 'effect' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#not_actions: {
                        let field_value = match fields_map.get("not_actions") {
                            Some(value) => value,
                            None => bail!("Missing field 'not_actions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#not_principals: {
                        let field_value = match fields_map.get("not_principals") {
                            Some(value) => value,
                            None => bail!("Missing field 'not_principals' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::iam::GetPolicyDocumentStatementNotPrincipal>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#not_resources: {
                        let field_value = match fields_map.get("not_resources") {
                            Some(value) => value,
                            None => bail!("Missing field 'not_resources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#principals: {
                        let field_value = match fields_map.get("principals") {
                            Some(value) => value,
                            None => bail!("Missing field 'principals' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::iam::GetPolicyDocumentStatementPrincipal>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#resources: {
                        let field_value = match fields_map.get("resources") {
                            Some(value) => value,
                            None => bail!("Missing field 'resources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#sid: {
                        let field_value = match fields_map.get("sid") {
                            Some(value) => value,
                            None => bail!("Missing field 'sid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
