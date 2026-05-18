#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DistributionConfigurationDistributionContainerDistributionConfiguration {
    /// Set of tags that are attached to the container distribution configuration.
    #[builder(into)]
    #[serde(rename = "containerTags")]
    pub r#container_tags: Option<Vec<String>>,
    /// Description of the container distribution configuration.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Configuration block with the destination repository for the container distribution configuration.
    #[builder(into)]
    #[serde(rename = "targetRepository")]
    pub r#target_repository: Box<super::super::types::imagebuilder::DistributionConfigurationDistributionContainerDistributionConfigurationTargetRepository>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DistributionConfigurationDistributionContainerDistributionConfiguration {
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
                    "container_tags",
                    &self.r#container_tags,
                ),
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "target_repository",
                    &self.r#target_repository,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DistributionConfigurationDistributionContainerDistributionConfiguration {
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
                    r#container_tags: {
                        let field_value = match fields_map.get("container_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_repository: {
                        let field_value = match fields_map.get("target_repository") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_repository' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
