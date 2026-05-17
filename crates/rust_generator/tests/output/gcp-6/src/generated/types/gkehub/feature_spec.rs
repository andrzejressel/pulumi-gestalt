#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FeatureSpec {
    /// Clusterupgrade feature spec.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "clusterupgrade")]
    pub r#clusterupgrade: Option<Box<super::super::types::gkehub::FeatureSpecClusterupgrade>>,
    /// Fleet Observability feature spec.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "fleetobservability")]
    pub r#fleetobservability: Option<Box<super::super::types::gkehub::FeatureSpecFleetobservability>>,
    /// Multicluster Ingress-specific spec.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "multiclusteringress")]
    pub r#multiclusteringress: Option<Box<super::super::types::gkehub::FeatureSpecMulticlusteringress>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FeatureSpec {
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
                    "clusterupgrade",
                    &self.r#clusterupgrade,
                ),
                to_pulumi_object_field(
                    "fleetobservability",
                    &self.r#fleetobservability,
                ),
                to_pulumi_object_field(
                    "multiclusteringress",
                    &self.r#multiclusteringress,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FeatureSpec {
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
                    r#clusterupgrade: {
                        let field_value = match fields_map.get("clusterupgrade") {
                            Some(value) => value,
                            None => bail!("Missing field 'clusterupgrade' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fleetobservability: {
                        let field_value = match fields_map.get("fleetobservability") {
                            Some(value) => value,
                            None => bail!("Missing field 'fleetobservability' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#multiclusteringress: {
                        let field_value = match fields_map.get("multiclusteringress") {
                            Some(value) => value,
                            None => bail!("Missing field 'multiclusteringress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
