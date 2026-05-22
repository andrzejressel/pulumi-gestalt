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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "duration",
                    &self.r#duration,
                ),
                to_pulumi_object_field(
                    "externalId",
                    &self.r#external_id,
                ),
                to_pulumi_object_field(
                    "policy",
                    &self.r#policy,
                ),
                to_pulumi_object_field(
                    "policyArns",
                    &self.r#policy_arns,
                ),
                to_pulumi_object_field(
                    "roleArn",
                    &self.r#role_arn,
                ),
                to_pulumi_object_field(
                    "sessionName",
                    &self.r#session_name,
                ),
                to_pulumi_object_field(
                    "sourceIdentity",
                    &self.r#source_identity,
                ),
                to_pulumi_object_field(
                    "tags",
                    &self.r#tags,
                ),
                to_pulumi_object_field(
                    "transitiveTagKeys",
                    &self.r#transitive_tag_keys,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("externalId") {
                            Some(value) => value,
                            None => bail!("Missing field 'externalId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("policyArns") {
                            Some(value) => value,
                            None => bail!("Missing field 'policyArns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#role_arn: {
                        let field_value = match fields_map.get("roleArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'roleArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#session_name: {
                        let field_value = match fields_map.get("sessionName") {
                            Some(value) => value,
                            None => bail!("Missing field 'sessionName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_identity: {
                        let field_value = match fields_map.get("sourceIdentity") {
                            Some(value) => value,
                            None => bail!("Missing field 'sourceIdentity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("transitiveTagKeys") {
                            Some(value) => value,
                            None => bail!("Missing field 'transitiveTagKeys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
