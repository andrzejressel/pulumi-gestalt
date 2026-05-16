#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataSourceHttpConfigAuthorizationConfig {
    /// Authorization type that the HTTP endpoint requires. Default values is `AWS_IAM`.
    #[builder(into)]
    #[serde(rename = "authorizationType")]
    pub r#authorization_type: Option<String>,
    /// Identity and Access Management (IAM) settings. See `aws_iam_config` Block for details.
    #[builder(into)]
    #[serde(rename = "awsIamConfig")]
    pub r#aws_iam_config: Option<Box<super::super::types::appsync::DataSourceHttpConfigAuthorizationConfigAwsIamConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataSourceHttpConfigAuthorizationConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("authorization_type".to_string(), self.r#authorization_type.to_pulumi_value().await);
            map.insert("aws_iam_config".to_string(), self.r#aws_iam_config.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataSourceHttpConfigAuthorizationConfig {
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
                    r#authorization_type: {
                        let field_value = match fields_map.get("authorization_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'authorization_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#aws_iam_config: {
                        let field_value = match fields_map.get("aws_iam_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'aws_iam_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appsync::DataSourceHttpConfigAuthorizationConfigAwsIamConfig>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
