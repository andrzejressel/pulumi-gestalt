#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RuleScope {
    /// The IDs of the only AWS resource that you want to trigger an evaluation for the rule. If you specify a resource ID, you must specify one resource type for `compliance_resource_types`.
    #[builder(into)]
    #[serde(rename = "complianceResourceId")]
    pub r#compliance_resource_id: Option<String>,
    /// A list of resource types of only those AWS resources that you want to trigger an evaluation for the ruleE.g., `AWS::EC2::Instance`. You can only specify one type if you also specify a resource ID for `compliance_resource_id`. See [relevant part of AWS Docs](http://docs.aws.amazon.com/config/latest/APIReference/API_ResourceIdentifier.html#config-Type-ResourceIdentifier-resourceType) for available types.
    #[builder(into)]
    #[serde(rename = "complianceResourceTypes")]
    pub r#compliance_resource_types: Option<Vec<String>>,
    /// The tag key that is applied to only those AWS resources that you want you want to trigger an evaluation for the rule.
    #[builder(into)]
    #[serde(rename = "tagKey")]
    pub r#tag_key: Option<String>,
    /// The tag value applied to only those AWS resources that you want to trigger an evaluation for the rule.
    #[builder(into)]
    #[serde(rename = "tagValue")]
    pub r#tag_value: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RuleScope {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("compliance_resource_id".to_string(), self.r#compliance_resource_id.to_pulumi_value().await);
            map.insert("compliance_resource_types".to_string(), self.r#compliance_resource_types.to_pulumi_value().await);
            map.insert("tag_key".to_string(), self.r#tag_key.to_pulumi_value().await);
            map.insert("tag_value".to_string(), self.r#tag_value.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RuleScope {
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
                    r#compliance_resource_id: {
                        let field_value = match fields_map.get("compliance_resource_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'compliance_resource_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#compliance_resource_types: {
                        let field_value = match fields_map.get("compliance_resource_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'compliance_resource_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#tag_key: {
                        let field_value = match fields_map.get("tag_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'tag_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#tag_value: {
                        let field_value = match fields_map.get("tag_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'tag_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
