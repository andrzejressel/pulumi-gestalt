#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LbRouteExtensionExtensionChain {
    /// A set of extensions to execute for the matching request.
    /// At least one extension is required. Up to 3 extensions can be defined for each extension chain for
    /// LbTrafficExtension resource. LbRouteExtension chains are limited to 1 extension per extension chain.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "extensions")]
    pub r#extensions: Vec<super::super::types::networkservices::LbRouteExtensionExtensionChainExtension>,
    /// Conditions under which this chain is invoked for a request.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "matchCondition")]
    pub r#match_condition: Box<super::super::types::networkservices::LbRouteExtensionExtensionChainMatchCondition>,
    /// The name for this extension chain. The name is logged as part of the HTTP request logs.
    /// The name must conform with RFC-1034, is restricted to lower-cased letters, numbers and hyphens,
    /// and can have a maximum length of 63 characters. Additionally, the first character must be a letter
    /// and the last character must be a letter or a number.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LbRouteExtensionExtensionChain {
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
                    "extensions",
                    &self.r#extensions,
                ),
                to_pulumi_object_field(
                    "match_condition",
                    &self.r#match_condition,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LbRouteExtensionExtensionChain {
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
                    r#extensions: {
                        let field_value = match fields_map.get("extensions") {
                            Some(value) => value,
                            None => bail!("Missing field 'extensions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#match_condition: {
                        let field_value = match fields_map.get("match_condition") {
                            Some(value) => value,
                            None => bail!("Missing field 'match_condition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
