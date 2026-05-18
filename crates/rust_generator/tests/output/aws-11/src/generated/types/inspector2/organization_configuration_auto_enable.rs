#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OrganizationConfigurationAutoEnable {
    /// Whether Amazon EC2 scans are automatically enabled for new members of your Amazon Inspector organization.
    #[builder(into)]
    #[serde(rename = "ec2")]
    pub r#ec_2: bool,
    /// Whether Amazon ECR scans are automatically enabled for new members of your Amazon Inspector organization.
    #[builder(into)]
    #[serde(rename = "ecr")]
    pub r#ecr: bool,
    /// Whether Lambda Function scans are automatically enabled for new members of your Amazon Inspector organization.
    #[builder(into)]
    #[serde(rename = "lambda")]
    pub r#lambda: Option<bool>,
    /// Whether AWS Lambda code scans are automatically enabled for new members of your Amazon Inspector organization. **Note:** Lambda code scanning requires Lambda standard scanning to be activated. Consequently, if you are setting this argument to `true`, you must also set the `lambda` argument to `true`. See [Scanning AWS Lambda functions with Amazon Inspector](https://docs.aws.amazon.com/inspector/latest/user/scanning-lambda.html#lambda-code-scans) for more information.
    #[builder(into)]
    #[serde(rename = "lambdaCode")]
    pub r#lambda_code: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for OrganizationConfigurationAutoEnable {
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
                    "ec_2",
                    &self.r#ec_2,
                ),
                to_pulumi_object_field(
                    "ecr",
                    &self.r#ecr,
                ),
                to_pulumi_object_field(
                    "lambda",
                    &self.r#lambda,
                ),
                to_pulumi_object_field(
                    "lambda_code",
                    &self.r#lambda_code,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for OrganizationConfigurationAutoEnable {
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
                    r#ec_2: {
                        let field_value = match fields_map.get("ec_2") {
                            Some(value) => value,
                            None => bail!("Missing field 'ec_2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ecr: {
                        let field_value = match fields_map.get("ecr") {
                            Some(value) => value,
                            None => bail!("Missing field 'ecr' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lambda: {
                        let field_value = match fields_map.get("lambda") {
                            Some(value) => value,
                            None => bail!("Missing field 'lambda' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lambda_code: {
                        let field_value = match fields_map.get("lambda_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'lambda_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
