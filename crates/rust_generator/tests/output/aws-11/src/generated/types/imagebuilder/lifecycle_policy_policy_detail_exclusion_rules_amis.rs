#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LifecyclePolicyPolicyDetailExclusionRulesAmis {
    /// Configures whether public AMIs are excluded from the lifecycle action.
    #[builder(into)]
    #[serde(rename = "isPublic")]
    pub r#is_public: Option<bool>,
    /// Specifies configuration details for Image Builder to exclude the most recent resources from lifecycle actions. Detailed below.
    #[builder(into)]
    #[serde(rename = "lastLaunched")]
    pub r#last_launched: Option<Box<super::super::types::imagebuilder::LifecyclePolicyPolicyDetailExclusionRulesAmisLastLaunched>>,
    /// Configures AWS Regions that are excluded from the lifecycle action.
    #[builder(into)]
    #[serde(rename = "regions")]
    pub r#regions: Option<Vec<String>>,
    /// Specifies AWS accounts whose resources are excluded from the lifecycle action.
    #[builder(into)]
    #[serde(rename = "sharedAccounts")]
    pub r#shared_accounts: Option<Vec<String>>,
    /// Lists tags that should be excluded from lifecycle actions for the AMIs that have them.
    #[builder(into)]
    #[serde(rename = "tagMap")]
    pub r#tag_map: Option<std::collections::HashMap<String, String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LifecyclePolicyPolicyDetailExclusionRulesAmis {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("is_public".to_string(), self.r#is_public.to_pulumi_value().await);
            map.insert("last_launched".to_string(), self.r#last_launched.to_pulumi_value().await);
            map.insert("regions".to_string(), self.r#regions.to_pulumi_value().await);
            map.insert("shared_accounts".to_string(), self.r#shared_accounts.to_pulumi_value().await);
            map.insert("tag_map".to_string(), self.r#tag_map.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LifecyclePolicyPolicyDetailExclusionRulesAmis {
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
                    r#is_public: {
                        let field_value = match fields_map.get("is_public") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_public' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#last_launched: {
                        let field_value = match fields_map.get("last_launched") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_launched' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::imagebuilder::LifecyclePolicyPolicyDetailExclusionRulesAmisLastLaunched>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#regions: {
                        let field_value = match fields_map.get("regions") {
                            Some(value) => value,
                            None => bail!("Missing field 'regions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#shared_accounts: {
                        let field_value = match fields_map.get("shared_accounts") {
                            Some(value) => value,
                            None => bail!("Missing field 'shared_accounts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
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
