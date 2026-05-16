#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetResourcesResourceTagMappingList {
    /// List of objects with information that shows whether a resource is compliant with the effective tag policy, including details on any noncompliant tag keys.
    #[builder(into)]
    #[serde(rename = "complianceDetails")]
    pub r#compliance_details: Vec<super::super::types::resourcegroupstaggingapi::GetResourcesResourceTagMappingListComplianceDetail>,
    /// ARN of the resource.
    #[builder(into)]
    #[serde(rename = "resourceArn")]
    pub r#resource_arn: String,
    /// Map of tags assigned to the resource.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: std::collections::HashMap<String, String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetResourcesResourceTagMappingList {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("compliance_details".to_string(), self.r#compliance_details.to_pulumi_value().await);
            map.insert("resource_arn".to_string(), self.r#resource_arn.to_pulumi_value().await);
            map.insert("tags".to_string(), self.r#tags.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetResourcesResourceTagMappingList {
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
                    r#compliance_details: {
                        let field_value = match fields_map.get("compliance_details") {
                            Some(value) => value,
                            None => bail!("Missing field 'compliance_details' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::resourcegroupstaggingapi::GetResourcesResourceTagMappingListComplianceDetail> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#resource_arn: {
                        let field_value = match fields_map.get("resource_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#tags: {
                        let field_value = match fields_map.get("tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <std::collections::HashMap<String, String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
