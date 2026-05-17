#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TaskNotebookInfrastructureSpec {
    /// Compute resources needed for a Task when using Dataproc Serverless.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "batch")]
    pub r#batch: Option<Box<super::super::types::dataplex::TaskNotebookInfrastructureSpecBatch>>,
    /// Container Image Runtime Configuration.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "containerImage")]
    pub r#container_image: Option<Box<super::super::types::dataplex::TaskNotebookInfrastructureSpecContainerImage>>,
    /// Vpc network.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "vpcNetwork")]
    pub r#vpc_network: Option<Box<super::super::types::dataplex::TaskNotebookInfrastructureSpecVpcNetwork>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TaskNotebookInfrastructureSpec {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "batch",
                    &self.r#batch,
                ),
                to_pulumi_object_field(
                    "container_image",
                    &self.r#container_image,
                ),
                to_pulumi_object_field(
                    "vpc_network",
                    &self.r#vpc_network,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TaskNotebookInfrastructureSpec {
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
                    r#batch: {
                        let field_value = match fields_map.get("batch") {
                            Some(value) => value,
                            None => bail!("Missing field 'batch' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container_image: {
                        let field_value = match fields_map.get("container_image") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_image' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpc_network: {
                        let field_value = match fields_map.get("vpc_network") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpc_network' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
