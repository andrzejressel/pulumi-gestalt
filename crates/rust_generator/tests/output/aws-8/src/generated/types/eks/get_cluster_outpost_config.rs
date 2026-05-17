#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterOutpostConfig {
    /// The Amazon EC2 instance type for all Kubernetes control plane instances.
    #[builder(into)]
    #[serde(rename = "controlPlaneInstanceType")]
    pub r#control_plane_instance_type: String,
    /// An object representing the placement configuration for all the control plane instances of your local Amazon EKS cluster on AWS Outpost.
    #[builder(into)]
    #[serde(rename = "controlPlanePlacements")]
    pub r#control_plane_placements: Vec<super::super::types::eks::GetClusterOutpostConfigControlPlanePlacement>,
    /// List of ARNs of the Outposts hosting the EKS cluster. Only a single ARN is supported currently.
    #[builder(into)]
    #[serde(rename = "outpostArns")]
    pub r#outpost_arns: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetClusterOutpostConfig {
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
                    "control_plane_instance_type",
                    &self.r#control_plane_instance_type,
                ),
                to_pulumi_object_field(
                    "control_plane_placements",
                    &self.r#control_plane_placements,
                ),
                to_pulumi_object_field(
                    "outpost_arns",
                    &self.r#outpost_arns,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetClusterOutpostConfig {
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
                    r#control_plane_instance_type: {
                        let field_value = match fields_map.get("control_plane_instance_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'control_plane_instance_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#control_plane_placements: {
                        let field_value = match fields_map.get("control_plane_placements") {
                            Some(value) => value,
                            None => bail!("Missing field 'control_plane_placements' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#outpost_arns: {
                        let field_value = match fields_map.get("outpost_arns") {
                            Some(value) => value,
                            None => bail!("Missing field 'outpost_arns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
