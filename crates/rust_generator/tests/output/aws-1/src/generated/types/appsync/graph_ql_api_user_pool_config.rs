#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GraphQlApiUserPoolConfig {
    /// Regular expression for validating the incoming Amazon Cognito User Pool app client ID.
    #[builder(into)]
    #[serde(rename = "appIdClientRegex")]
    pub r#app_id_client_regex: Option<String>,
    /// AWS region in which the user pool was created.
    #[builder(into)]
    #[serde(rename = "awsRegion")]
    pub r#aws_region: Option<String>,
    /// Action that you want your GraphQL API to take when a request that uses Amazon Cognito User Pool authentication doesn't match the Amazon Cognito User Pool configuration. Valid: `ALLOW` and `DENY`
    #[builder(into)]
    #[serde(rename = "defaultAction")]
    pub r#default_action: String,
    /// User pool ID.
    #[builder(into)]
    #[serde(rename = "userPoolId")]
    pub r#user_pool_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GraphQlApiUserPoolConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("app_id_client_regex".to_string(), self.r#app_id_client_regex.to_pulumi_value().await);
            map.insert("aws_region".to_string(), self.r#aws_region.to_pulumi_value().await);
            map.insert("default_action".to_string(), self.r#default_action.to_pulumi_value().await);
            map.insert("user_pool_id".to_string(), self.r#user_pool_id.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GraphQlApiUserPoolConfig {
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
                    r#app_id_client_regex: {
                        let field_value = match fields_map.get("app_id_client_regex") {
                            Some(value) => value,
                            None => bail!("Missing field 'app_id_client_regex' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#aws_region: {
                        let field_value = match fields_map.get("aws_region") {
                            Some(value) => value,
                            None => bail!("Missing field 'aws_region' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#default_action: {
                        let field_value = match fields_map.get("default_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#user_pool_id: {
                        let field_value = match fields_map.get("user_pool_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_pool_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
