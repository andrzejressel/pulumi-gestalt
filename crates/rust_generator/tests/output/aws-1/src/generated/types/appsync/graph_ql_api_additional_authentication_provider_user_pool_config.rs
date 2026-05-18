#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GraphQlApiAdditionalAuthenticationProviderUserPoolConfig {
    /// Regular expression for validating the incoming Amazon Cognito User Pool app client ID.
    #[builder(into)]
    #[serde(rename = "appIdClientRegex")]
    pub r#app_id_client_regex: Option<String>,
    /// AWS region in which the user pool was created.
    #[builder(into)]
    #[serde(rename = "awsRegion")]
    pub r#aws_region: Option<String>,
    /// User pool ID.
    #[builder(into)]
    #[serde(rename = "userPoolId")]
    pub r#user_pool_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GraphQlApiAdditionalAuthenticationProviderUserPoolConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "app_id_client_regex",
                    &self.r#app_id_client_regex,
                ),
                to_pulumi_object_field(
                    "aws_region",
                    &self.r#aws_region,
                ),
                to_pulumi_object_field(
                    "user_pool_id",
                    &self.r#user_pool_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GraphQlApiAdditionalAuthenticationProviderUserPoolConfig {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#app_id_client_regex: {
                        let field_value = match fields_map.get("app_id_client_regex") {
                            Some(value) => value,
                            None => bail!("Missing field 'app_id_client_regex' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#aws_region: {
                        let field_value = match fields_map.get("aws_region") {
                            Some(value) => value,
                            None => bail!("Missing field 'aws_region' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_pool_id: {
                        let field_value = match fields_map.get("user_pool_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_pool_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
