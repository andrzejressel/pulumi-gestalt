#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BucketWebsiteConfigurationV2RoutingRule {
    /// Configuration block for describing a condition that must be met for the specified redirect to apply. See below.
    #[builder(into)]
    #[serde(rename = "condition")]
    pub r#condition: Option<Box<super::super::types::s3::BucketWebsiteConfigurationV2RoutingRuleCondition>>,
    /// Configuration block for redirect information. See below.
    #[builder(into)]
    #[serde(rename = "redirect")]
    pub r#redirect: Box<super::super::types::s3::BucketWebsiteConfigurationV2RoutingRuleRedirect>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BucketWebsiteConfigurationV2RoutingRule {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("condition".to_string(), self.r#condition.to_pulumi_value().await);
            map.insert("redirect".to_string(), self.r#redirect.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BucketWebsiteConfigurationV2RoutingRule {
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
                    r#condition: {
                        let field_value = match fields_map.get("condition") {
                            Some(value) => value,
                            None => bail!("Missing field 'condition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::s3::BucketWebsiteConfigurationV2RoutingRuleCondition>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#redirect: {
                        let field_value = match fields_map.get("redirect") {
                            Some(value) => value,
                            None => bail!("Missing field 'redirect' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::s3::BucketWebsiteConfigurationV2RoutingRuleRedirect> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
