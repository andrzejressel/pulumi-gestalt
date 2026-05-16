#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RuleSource {
    /// Provides the runtime system, policy definition, and whether debug logging is enabled. Required when owner is set to `CUSTOM_POLICY`. See Custom Policy Details Below.
    #[builder(into)]
    #[serde(rename = "customPolicyDetails")]
    pub r#custom_policy_details: Option<Box<super::super::types::cfg::RuleSourceCustomPolicyDetails>>,
    /// Indicates whether AWS or the customer owns and manages the AWS Config rule. Valid values are `AWS`, `CUSTOM_LAMBDA` or `CUSTOM_POLICY`. For more information about managed rules, see the [AWS Config Managed Rules documentation](https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_use-managed-rules.html). For more information about custom rules, see the [AWS Config Custom Rules documentation](https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_develop-rules.html). Custom Lambda Functions require permissions to allow the AWS Config service to invoke them, e.g., via the `aws.lambda.Permission` resource.
    #[builder(into)]
    #[serde(rename = "owner")]
    pub r#owner: String,
    /// Provides the source and type of the event that causes AWS Config to evaluate your AWS resources. Only valid if `owner` is `CUSTOM_LAMBDA` or `CUSTOM_POLICY`. See Source Detail Below.
    #[builder(into)]
    #[serde(rename = "sourceDetails")]
    pub r#source_details: Option<Vec<super::super::types::cfg::RuleSourceSourceDetail>>,
    /// For AWS Config managed rules, a predefined identifier, e.g `IAM_PASSWORD_POLICY`. For custom Lambda rules, the identifier is the ARN of the Lambda Function, such as `arn:aws:lambda:us-east-1:123456789012:function:custom_rule_name` or the `arn` attribute of the `aws.lambda.Function` resource.
    #[builder(into)]
    #[serde(rename = "sourceIdentifier")]
    pub r#source_identifier: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RuleSource {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("custom_policy_details".to_string(), self.r#custom_policy_details.to_pulumi_value().await);
            map.insert("owner".to_string(), self.r#owner.to_pulumi_value().await);
            map.insert("source_details".to_string(), self.r#source_details.to_pulumi_value().await);
            map.insert("source_identifier".to_string(), self.r#source_identifier.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RuleSource {
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
                    r#custom_policy_details: {
                        let field_value = match fields_map.get("custom_policy_details") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_policy_details' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::cfg::RuleSourceCustomPolicyDetails>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#owner: {
                        let field_value = match fields_map.get("owner") {
                            Some(value) => value,
                            None => bail!("Missing field 'owner' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#source_details: {
                        let field_value = match fields_map.get("source_details") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_details' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::cfg::RuleSourceSourceDetail>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#source_identifier: {
                        let field_value = match fields_map.get("source_identifier") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_identifier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
