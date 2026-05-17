#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetApplicationGatewayRedirectConfiguration {
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// Whether the path is included in the redirected URL.
    #[builder(into)]
    #[serde(rename = "includePath")]
    pub r#include_path: bool,
    /// Whether to include the query string in the redirected URL.
    #[builder(into)]
    #[serde(rename = "includeQueryString")]
    pub r#include_query_string: bool,
    /// The name of this Application Gateway.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The type of redirect.
    #[builder(into)]
    #[serde(rename = "redirectType")]
    pub r#redirect_type: String,
    #[builder(into)]
    #[serde(rename = "targetListenerId")]
    pub r#target_listener_id: String,
    /// The name of the listener to redirect to.
    #[builder(into)]
    #[serde(rename = "targetListenerName")]
    pub r#target_listener_name: String,
    /// The URL to redirect the request to.
    #[builder(into)]
    #[serde(rename = "targetUrl")]
    pub r#target_url: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetApplicationGatewayRedirectConfiguration {
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
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "include_path",
                    &self.r#include_path,
                ),
                to_pulumi_object_field(
                    "include_query_string",
                    &self.r#include_query_string,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "redirect_type",
                    &self.r#redirect_type,
                ),
                to_pulumi_object_field(
                    "target_listener_id",
                    &self.r#target_listener_id,
                ),
                to_pulumi_object_field(
                    "target_listener_name",
                    &self.r#target_listener_name,
                ),
                to_pulumi_object_field(
                    "target_url",
                    &self.r#target_url,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetApplicationGatewayRedirectConfiguration {
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
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_path: {
                        let field_value = match fields_map.get("include_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_query_string: {
                        let field_value = match fields_map.get("include_query_string") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_query_string' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redirect_type: {
                        let field_value = match fields_map.get("redirect_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'redirect_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_listener_id: {
                        let field_value = match fields_map.get("target_listener_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_listener_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_listener_name: {
                        let field_value = match fields_map.get("target_listener_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_listener_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_url: {
                        let field_value = match fields_map.get("target_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
