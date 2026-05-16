#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DistributionCacheBehaviorSettingsForwardedQueryStrings {
    /// Indicates whether the distribution forwards and caches based on query strings.
    #[builder(into)]
    #[serde(rename = "option")]
    pub r#option: Option<bool>,
    /// The specific query strings that the distribution forwards to the origin.
    #[builder(into)]
    #[serde(rename = "queryStringsAllowedLists")]
    pub r#query_strings_allowed_lists: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DistributionCacheBehaviorSettingsForwardedQueryStrings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("option".to_string(), self.r#option.to_pulumi_value().await);
            map.insert("query_strings_allowed_lists".to_string(), self.r#query_strings_allowed_lists.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DistributionCacheBehaviorSettingsForwardedQueryStrings {
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
                    r#option: {
                        let field_value = match fields_map.get("option") {
                            Some(value) => value,
                            None => bail!("Missing field 'option' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#query_strings_allowed_lists: {
                        let field_value = match fields_map.get("query_strings_allowed_lists") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_strings_allowed_lists' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
