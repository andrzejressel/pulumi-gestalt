#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AuthzPolicyHttpRuleFromSource {
    /// A list of identities derived from the client's certificate. This field will not match on a request unless mutual TLS is enabled for the Forwarding rule or Gateway. Each identity is a string whose value is matched against the URI SAN, or DNS SAN or the subject field in the client's certificate. The match can be exact, prefix, suffix or a substring match. One of exact, prefix, suffix or contains must be specified.
    /// Limited to 5 principals.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "principals")]
    pub r#principals: Option<Vec<super::super::types::networksecurity::AuthzPolicyHttpRuleFromSourcePrincipal>>,
    /// A list of resources to match against the resource of the source VM of a request.
    /// Limited to 5 resources.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "resources")]
    pub r#resources: Option<Vec<super::super::types::networksecurity::AuthzPolicyHttpRuleFromSourceResource>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AuthzPolicyHttpRuleFromSource {
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
                    "principals",
                    &self.r#principals,
                ),
                to_pulumi_object_field(
                    "resources",
                    &self.r#resources,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AuthzPolicyHttpRuleFromSource {
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
                    r#principals: {
                        let field_value = match fields_map.get("principals") {
                            Some(value) => value,
                            None => bail!("Missing field 'principals' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resources: {
                        let field_value = match fields_map.get("resources") {
                            Some(value) => value,
                            None => bail!("Missing field 'resources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
