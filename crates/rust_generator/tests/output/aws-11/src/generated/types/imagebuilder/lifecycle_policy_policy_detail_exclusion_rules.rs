#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LifecyclePolicyPolicyDetailExclusionRules {
    /// Lists configuration values that apply to AMIs that Image Builder should exclude from the lifecycle action. Detailed below.
    #[builder(into)]
    #[serde(rename = "amis")]
    pub r#amis: Option<Box<super::super::types::imagebuilder::LifecyclePolicyPolicyDetailExclusionRulesAmis>>,
    /// Contains a list of tags that Image Builder uses to skip lifecycle actions for Image Builder image resources that have them.
    #[builder(into)]
    #[serde(rename = "tagMap")]
    pub r#tag_map: Option<std::collections::HashMap<String, String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LifecyclePolicyPolicyDetailExclusionRules {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("amis".to_string(), self.r#amis.to_pulumi_value().await);
            map.insert("tag_map".to_string(), self.r#tag_map.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LifecyclePolicyPolicyDetailExclusionRules {
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
                    r#amis: {
                        let field_value = match fields_map.get("amis") {
                            Some(value) => value,
                            None => bail!("Missing field 'amis' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::imagebuilder::LifecyclePolicyPolicyDetailExclusionRulesAmis>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#tag_map: {
                        let field_value = match fields_map.get("tag_map") {
                            Some(value) => value,
                            None => bail!("Missing field 'tag_map' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<std::collections::HashMap<String, String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
