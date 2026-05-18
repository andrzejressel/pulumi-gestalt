#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubernetesClusterWorkloadAutoscalerProfile {
    /// Specifies whether KEDA Autoscaler can be used for workloads.
    #[builder(into)]
    #[serde(rename = "kedaEnabled")]
    pub r#keda_enabled: Option<bool>,
    /// Specifies whether Vertical Pod Autoscaler should be enabled.
    /// 
    /// > **Note:** This requires that the Preview Feature `Microsoft.ContainerService/AKS-VPAPreview` is enabled and the Resource Provider is re-registered, see the documentation for more information.
    #[builder(into)]
    #[serde(rename = "verticalPodAutoscalerEnabled")]
    pub r#vertical_pod_autoscaler_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KubernetesClusterWorkloadAutoscalerProfile {
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
                    "keda_enabled",
                    &self.r#keda_enabled,
                ),
                to_pulumi_object_field(
                    "vertical_pod_autoscaler_enabled",
                    &self.r#vertical_pod_autoscaler_enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KubernetesClusterWorkloadAutoscalerProfile {
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
                    r#keda_enabled: {
                        let field_value = match fields_map.get("keda_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'keda_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vertical_pod_autoscaler_enabled: {
                        let field_value = match fields_map.get("vertical_pod_autoscaler_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'vertical_pod_autoscaler_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
