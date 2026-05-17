#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProjectServiceCatalogProvisioningDetails {
    /// The path identifier of the product. This value is optional if the product has a default path, and required if the product has more than one path.
    #[builder(into)]
    #[serde(rename = "pathId")]
    pub r#path_id: Option<String>,
    /// The ID of the product to provision.
    #[builder(into)]
    #[serde(rename = "productId")]
    pub r#product_id: String,
    /// The ID of the provisioning artifact.
    #[builder(into)]
    #[serde(rename = "provisioningArtifactId")]
    pub r#provisioning_artifact_id: Option<String>,
    /// A list of key value pairs that you specify when you provision a product. See Provisioning Parameter below.
    #[builder(into)]
    #[serde(rename = "provisioningParameters")]
    pub r#provisioning_parameters: Option<Vec<super::super::types::sagemaker::ProjectServiceCatalogProvisioningDetailsProvisioningParameter>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ProjectServiceCatalogProvisioningDetails {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "path_id",
                    &self.r#path_id,
                ),
                to_pulumi_object_field(
                    "product_id",
                    &self.r#product_id,
                ),
                to_pulumi_object_field(
                    "provisioning_artifact_id",
                    &self.r#provisioning_artifact_id,
                ),
                to_pulumi_object_field(
                    "provisioning_parameters",
                    &self.r#provisioning_parameters,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ProjectServiceCatalogProvisioningDetails {
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
                    r#path_id: {
                        let field_value = match fields_map.get("path_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'path_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#product_id: {
                        let field_value = match fields_map.get("product_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'product_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#provisioning_artifact_id: {
                        let field_value = match fields_map.get("provisioning_artifact_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'provisioning_artifact_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#provisioning_parameters: {
                        let field_value = match fields_map.get("provisioning_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'provisioning_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
