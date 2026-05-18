#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegionNetworkEndpointGroupCloudFunction {
    /// A user-defined name of the Cloud Function.
    /// The function name is case-sensitive and must be 1-63 characters long.
    /// Example value: "func1".
    #[builder(into)]
    #[serde(rename = "function")]
    pub r#function: Option<String>,
    /// A template to parse function field from a request URL. URL mask allows
    /// for routing to multiple Cloud Functions without having to create
    /// multiple Network Endpoint Groups and backend services.
    /// For example, request URLs "mydomain.com/function1" and "mydomain.com/function2"
    /// can be backed by the same Serverless NEG with URL mask "/". The URL mask
    /// will parse them to { function = "function1" } and { function = "function2" } respectively.
    #[builder(into)]
    #[serde(rename = "urlMask")]
    pub r#url_mask: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegionNetworkEndpointGroupCloudFunction {
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
                    "function",
                    &self.r#function,
                ),
                to_pulumi_object_field(
                    "url_mask",
                    &self.r#url_mask,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegionNetworkEndpointGroupCloudFunction {
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
                    r#function: {
                        let field_value = match fields_map.get("function") {
                            Some(value) => value,
                            None => bail!("Missing field 'function' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url_mask: {
                        let field_value = match fields_map.get("url_mask") {
                            Some(value) => value,
                            None => bail!("Missing field 'url_mask' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
