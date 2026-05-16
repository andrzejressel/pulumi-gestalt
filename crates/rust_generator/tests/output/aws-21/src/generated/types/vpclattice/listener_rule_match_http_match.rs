#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ListenerRuleMatchHttpMatch {
    /// The header matches. Matches incoming requests with rule based on request header value before applying rule action.
    #[builder(into)]
    #[serde(rename = "headerMatches")]
    pub r#header_matches: Option<Vec<super::super::types::vpclattice::ListenerRuleMatchHttpMatchHeaderMatch>>,
    /// The HTTP method type.
    #[builder(into)]
    #[serde(rename = "method")]
    pub r#method: Option<String>,
    /// The path match.
    #[builder(into)]
    #[serde(rename = "pathMatch")]
    pub r#path_match: Option<Box<super::super::types::vpclattice::ListenerRuleMatchHttpMatchPathMatch>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ListenerRuleMatchHttpMatch {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("header_matches".to_string(), self.r#header_matches.to_pulumi_value().await);
            map.insert("method".to_string(), self.r#method.to_pulumi_value().await);
            map.insert("path_match".to_string(), self.r#path_match.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ListenerRuleMatchHttpMatch {
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
                    r#header_matches: {
                        let field_value = match fields_map.get("header_matches") {
                            Some(value) => value,
                            None => bail!("Missing field 'header_matches' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::vpclattice::ListenerRuleMatchHttpMatchHeaderMatch>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#method: {
                        let field_value = match fields_map.get("method") {
                            Some(value) => value,
                            None => bail!("Missing field 'method' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#path_match: {
                        let field_value = match fields_map.get("path_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'path_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::vpclattice::ListenerRuleMatchHttpMatchPathMatch>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
