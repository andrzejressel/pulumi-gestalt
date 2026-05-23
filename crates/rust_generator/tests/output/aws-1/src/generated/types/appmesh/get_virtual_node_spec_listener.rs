#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetVirtualNodeSpecListener {
    #[builder(into)]
    pub r#connection_pools: Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerConnectionPool>,
    #[builder(into)]
    pub r#health_checks: Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerHealthCheck>,
    #[builder(into)]
    pub r#outlier_detections: Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerOutlierDetection>,
    #[builder(into)]
    pub r#port_mappings: Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerPortMapping>,
    #[builder(into)]
    pub r#timeouts: Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerTimeout>,
    #[builder(into)]
    pub r#tls: Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerTl>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetVirtualNodeSpecListener {
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
                    "connectionPools",
                    &self.r#connection_pools,
                ),
                to_pulumi_object_field(
                    "healthChecks",
                    &self.r#health_checks,
                ),
                to_pulumi_object_field(
                    "outlierDetections",
                    &self.r#outlier_detections,
                ),
                to_pulumi_object_field(
                    "portMappings",
                    &self.r#port_mappings,
                ),
                to_pulumi_object_field(
                    "timeouts",
                    &self.r#timeouts,
                ),
                to_pulumi_object_field(
                    "tls",
                    &self.r#tls,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetVirtualNodeSpecListener {
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
                    r#connection_pools: {
                        let field_value = match fields_map.get("connectionPools") {
                            Some(value) => value,
                            None => bail!("Missing field 'connectionPools' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#health_checks: {
                        let field_value = match fields_map.get("healthChecks") {
                            Some(value) => value,
                            None => bail!("Missing field 'healthChecks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#outlier_detections: {
                        let field_value = match fields_map.get("outlierDetections") {
                            Some(value) => value,
                            None => bail!("Missing field 'outlierDetections' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#port_mappings: {
                        let field_value = match fields_map.get("portMappings") {
                            Some(value) => value,
                            None => bail!("Missing field 'portMappings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timeouts: {
                        let field_value = match fields_map.get("timeouts") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeouts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tls: {
                        let field_value = match fields_map.get("tls") {
                            Some(value) => value,
                            None => bail!("Missing field 'tls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
