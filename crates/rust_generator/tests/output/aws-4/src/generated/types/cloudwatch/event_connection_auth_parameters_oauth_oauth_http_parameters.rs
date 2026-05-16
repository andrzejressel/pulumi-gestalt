#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EventConnectionAuthParametersOauthOauthHttpParameters {
    /// Contains additional body string parameters for the connection. You can include up to 100 additional body string parameters per request. Each additional parameter counts towards the event payload size, which cannot exceed 64 KB. Each parameter can contain the following:
    #[builder(into)]
    #[serde(rename = "bodies")]
    pub r#bodies: Option<Vec<super::super::types::cloudwatch::EventConnectionAuthParametersOauthOauthHttpParametersBody>>,
    /// Contains additional header parameters for the connection. You can include up to 100 additional body string parameters per request. Each additional parameter counts towards the event payload size, which cannot exceed 64 KB. Each parameter can contain the following:
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Option<Vec<super::super::types::cloudwatch::EventConnectionAuthParametersOauthOauthHttpParametersHeader>>,
    /// Contains additional query string parameters for the connection. You can include up to 100 additional body string parameters per request. Each additional parameter counts towards the event payload size, which cannot exceed 64 KB. Each parameter can contain the following:
    #[builder(into)]
    #[serde(rename = "queryStrings")]
    pub r#query_strings: Option<Vec<super::super::types::cloudwatch::EventConnectionAuthParametersOauthOauthHttpParametersQueryString>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EventConnectionAuthParametersOauthOauthHttpParameters {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("bodies".to_string(), self.r#bodies.to_pulumi_value().await);
            map.insert("headers".to_string(), self.r#headers.to_pulumi_value().await);
            map.insert("query_strings".to_string(), self.r#query_strings.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EventConnectionAuthParametersOauthOauthHttpParameters {
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
                    r#bodies: {
                        let field_value = match fields_map.get("bodies") {
                            Some(value) => value,
                            None => bail!("Missing field 'bodies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::cloudwatch::EventConnectionAuthParametersOauthOauthHttpParametersBody>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#headers: {
                        let field_value = match fields_map.get("headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::cloudwatch::EventConnectionAuthParametersOauthOauthHttpParametersHeader>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#query_strings: {
                        let field_value = match fields_map.get("query_strings") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_strings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::cloudwatch::EventConnectionAuthParametersOauthOauthHttpParametersQueryString>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
