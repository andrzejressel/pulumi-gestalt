#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceSourceConfigurationImageRepository {
    /// Configuration for running the identified image. See Image Configuration below for more details.
    #[builder(into)]
    #[serde(rename = "imageConfiguration")]
    pub r#image_configuration: Option<Box<super::super::types::apprunner::ServiceSourceConfigurationImageRepositoryImageConfiguration>>,
    /// Identifier of an image. For an image in Amazon Elastic Container Registry (Amazon ECR), this is an image name. For the
    /// image name format, see Pulling an image in the Amazon ECR User Guide.
    #[builder(into)]
    #[serde(rename = "imageIdentifier")]
    pub r#image_identifier: String,
    /// Type of the image repository. This reflects the repository provider and whether the repository is private or public. Valid values: `ECR` , `ECR_PUBLIC`.
    #[builder(into)]
    #[serde(rename = "imageRepositoryType")]
    pub r#image_repository_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceSourceConfigurationImageRepository {
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
                    "image_configuration",
                    &self.r#image_configuration,
                ),
                to_pulumi_object_field(
                    "image_identifier",
                    &self.r#image_identifier,
                ),
                to_pulumi_object_field(
                    "image_repository_type",
                    &self.r#image_repository_type,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceSourceConfigurationImageRepository {
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
                    r#image_configuration: {
                        let field_value = match fields_map.get("image_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image_identifier: {
                        let field_value = match fields_map.get("image_identifier") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_identifier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image_repository_type: {
                        let field_value = match fields_map.get("image_repository_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_repository_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
