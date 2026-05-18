#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AuthzPolicyHttpRuleFrom {
    /// Describes the properties of a request's sources. At least one of sources or notSources must be specified. Limited to 5 sources. A match occurs when ANY source (in sources or notSources) matches the request. Within a single source, the match follows AND semantics across fields and OR semantics within a single field, i.e. a match occurs when ANY principal matches AND ANY ipBlocks match.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "notSources")]
    pub r#not_sources: Option<Vec<super::super::types::networksecurity::AuthzPolicyHttpRuleFromNotSource>>,
    /// Describes the properties of a request's sources. At least one of sources or notSources must be specified. Limited to 5 sources. A match occurs when ANY source (in sources or notSources) matches the request. Within a single source, the match follows AND semantics across fields and OR semantics within a single field, i.e. a match occurs when ANY principal matches AND ANY ipBlocks match.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "sources")]
    pub r#sources: Option<Vec<super::super::types::networksecurity::AuthzPolicyHttpRuleFromSource>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AuthzPolicyHttpRuleFrom {
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
                    "not_sources",
                    &self.r#not_sources,
                ),
                to_pulumi_object_field(
                    "sources",
                    &self.r#sources,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AuthzPolicyHttpRuleFrom {
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
                    r#not_sources: {
                        let field_value = match fields_map.get("not_sources") {
                            Some(value) => value,
                            None => bail!("Missing field 'not_sources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sources: {
                        let field_value = match fields_map.get("sources") {
                            Some(value) => value,
                            None => bail!("Missing field 'sources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
