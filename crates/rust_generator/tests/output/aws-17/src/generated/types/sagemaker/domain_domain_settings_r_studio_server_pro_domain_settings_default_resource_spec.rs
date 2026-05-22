#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainDomainSettingsRStudioServerProDomainSettingsDefaultResourceSpec {
    /// The instance type that the image version runs on.. For valid values see [SageMaker Instance Types](https://docs.aws.amazon.com/sagemaker/latest/dg/notebooks-available-instance-types.html).
    #[builder(into)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: Option<String>,
    /// The Amazon Resource Name (ARN) of the Lifecycle Configuration attached to the Resource.
    #[builder(into)]
    #[serde(rename = "lifecycleConfigArn")]
    pub r#lifecycle_config_arn: Option<String>,
    /// The ARN of the SageMaker image that the image version belongs to.
    #[builder(into)]
    #[serde(rename = "sagemakerImageArn")]
    pub r#sagemaker_image_arn: Option<String>,
    /// The SageMaker Image Version Alias.
    #[builder(into)]
    #[serde(rename = "sagemakerImageVersionAlias")]
    pub r#sagemaker_image_version_alias: Option<String>,
    /// The ARN of the image version created on the instance.
    #[builder(into)]
    #[serde(rename = "sagemakerImageVersionArn")]
    pub r#sagemaker_image_version_arn: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DomainDomainSettingsRStudioServerProDomainSettingsDefaultResourceSpec {
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
                    "instanceType",
                    &self.r#instance_type,
                ),
                to_pulumi_object_field(
                    "lifecycleConfigArn",
                    &self.r#lifecycle_config_arn,
                ),
                to_pulumi_object_field(
                    "sagemakerImageArn",
                    &self.r#sagemaker_image_arn,
                ),
                to_pulumi_object_field(
                    "sagemakerImageVersionAlias",
                    &self.r#sagemaker_image_version_alias,
                ),
                to_pulumi_object_field(
                    "sagemakerImageVersionArn",
                    &self.r#sagemaker_image_version_arn,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DomainDomainSettingsRStudioServerProDomainSettingsDefaultResourceSpec {
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
                    r#instance_type: {
                        let field_value = match fields_map.get("instanceType") {
                            Some(value) => value,
                            None => bail!("Missing field 'instanceType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lifecycle_config_arn: {
                        let field_value = match fields_map.get("lifecycleConfigArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'lifecycleConfigArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sagemaker_image_arn: {
                        let field_value = match fields_map.get("sagemakerImageArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'sagemakerImageArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sagemaker_image_version_alias: {
                        let field_value = match fields_map.get("sagemakerImageVersionAlias") {
                            Some(value) => value,
                            None => bail!("Missing field 'sagemakerImageVersionAlias' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sagemaker_image_version_arn: {
                        let field_value = match fields_map.get("sagemakerImageVersionArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'sagemakerImageVersionArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
