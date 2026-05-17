#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProviderAssumeRole {
    /// The duration, between 15 minutes and 12 hours, of the role session. Valid time units are ns, us (or µs), ms, s, h, or m.
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: Option<String>,
    /// A unique identifier that might be required when you assume a role in another account.
    #[builder(into)]
    #[serde(rename = "externalId")]
    pub r#external_id: Option<String>,
    /// IAM Policy JSON describing further restricting permissions for the IAM Role being assumed.
    #[builder(into)]
    #[serde(rename = "policy")]
    pub r#policy: Option<String>,
    /// Amazon Resource Names (ARNs) of IAM Policies describing further restricting permissions for the IAM Role being assumed.
    #[builder(into)]
    #[serde(rename = "policyArns")]
    pub r#policy_arns: Option<Vec<String>>,
    /// Amazon Resource Name (ARN) of an IAM Role to assume prior to making API calls.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Option<String>,
    /// An identifier for the assumed role session.
    #[builder(into)]
    #[serde(rename = "sessionName")]
    pub r#session_name: Option<String>,
    /// Source identity specified by the principal assuming the role.
    #[builder(into)]
    #[serde(rename = "sourceIdentity")]
    pub r#source_identity: Option<String>,
    /// Assume role session tags.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<std::collections::HashMap<String, String>>,
    /// Assume role session tag keys to pass to any subsequent sessions.
    #[builder(into)]
    #[serde(rename = "transitiveTagKeys")]
    pub r#transitive_tag_keys: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ProviderAssumeRole {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "duration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#duration,
                )
                .await,
            );
            map.insert(
                "external_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#external_id,
                )
                .await,
            );
            map.insert(
                "policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#policy,
                )
                .await,
            );
            map.insert(
                "policy_arns".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#policy_arns,
                )
                .await,
            );
            map.insert(
                "role_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#role_arn,
                )
                .await,
            );
            map.insert(
                "session_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#session_name,
                )
                .await,
            );
            map.insert(
                "source_identity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_identity,
                )
                .await,
            );
            map.insert(
                "tags".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tags,
                )
                .await,
            );
            map.insert(
                "transitive_tag_keys".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#transitive_tag_keys,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ProviderAssumeRole {
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
                    r#duration: {
                        let field_value = match fields_map.get("duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#external_id: {
                        let field_value = match fields_map.get("external_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'external_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#policy: {
                        let field_value = match fields_map.get("policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#policy_arns: {
                        let field_value = match fields_map.get("policy_arns") {
                            Some(value) => value,
                            None => bail!("Missing field 'policy_arns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#role_arn: {
                        let field_value = match fields_map.get("role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#session_name: {
                        let field_value = match fields_map.get("session_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'session_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_identity: {
                        let field_value = match fields_map.get("source_identity") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_identity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tags: {
                        let field_value = match fields_map.get("tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transitive_tag_keys: {
                        let field_value = match fields_map.get("transitive_tag_keys") {
                            Some(value) => value,
                            None => bail!("Missing field 'transitive_tag_keys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
