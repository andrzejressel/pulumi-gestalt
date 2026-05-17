#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UrlMapDefaultRouteActionWeightedBackendService {
    /// The full or partial URL to the default BackendService resource. Before forwarding the
    /// request to backendService, the loadbalancer applies any relevant headerActions
    /// specified as part of this backendServiceWeight.
    #[builder(into)]
    #[serde(rename = "backendService")]
    pub r#backend_service: Option<String>,
    /// Specifies changes to request and response headers that need to take effect for
    /// the selected backendService.
    /// headerAction specified here take effect before headerAction in the enclosing
    /// HttpRouteRule, PathMatcher and UrlMap.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "headerAction")]
    pub r#header_action: Option<Box<super::super::types::compute::UrlMapDefaultRouteActionWeightedBackendServiceHeaderAction>>,
    /// Specifies the fraction of traffic sent to backendService, computed as
    /// weight / (sum of all weightedBackendService weights in routeAction) .
    /// The selection of a backend service is determined only for new traffic. Once a user's request
    /// has been directed to a backendService, subsequent requests will be sent to the same backendService
    /// as determined by the BackendService's session affinity policy.
    /// The value must be between 0 and 1000
    #[builder(into)]
    #[serde(rename = "weight")]
    pub r#weight: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for UrlMapDefaultRouteActionWeightedBackendService {
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
                    "backend_service",
                    &self.r#backend_service,
                ),
                to_pulumi_object_field(
                    "header_action",
                    &self.r#header_action,
                ),
                to_pulumi_object_field(
                    "weight",
                    &self.r#weight,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for UrlMapDefaultRouteActionWeightedBackendService {
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
                    r#backend_service: {
                        let field_value = match fields_map.get("backend_service") {
                            Some(value) => value,
                            None => bail!("Missing field 'backend_service' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#header_action: {
                        let field_value = match fields_map.get("header_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'header_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#weight: {
                        let field_value = match fields_map.get("weight") {
                            Some(value) => value,
                            None => bail!("Missing field 'weight' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
