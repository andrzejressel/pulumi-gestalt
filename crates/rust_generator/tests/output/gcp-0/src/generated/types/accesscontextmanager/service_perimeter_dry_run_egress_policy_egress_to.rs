#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServicePerimeterDryRunEgressPolicyEgressTo {
    /// A list of external resources that are allowed to be accessed. A request
    /// matches if it contains an external resource in this list (Example:
    /// s3://bucket/path). Currently '*' is not allowed.
    #[builder(into)]
    #[serde(rename = "externalResources")]
    pub r#external_resources: Option<Vec<String>>,
    /// A list of `ApiOperations` that this egress rule applies to. A request matches
    /// if it contains an operation/service in this list.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "operations")]
    pub r#operations: Option<Vec<super::super::types::accesscontextmanager::ServicePerimeterDryRunEgressPolicyEgressToOperation>>,
    /// A list of resources, currently only projects in the form
    /// `projects/<projectnumber>`, that match this to stanza. A request matches
    /// if it contains a resource in this list. If * is specified for resources,
    /// then this `EgressTo` rule will authorize access to all resources outside
    /// the perimeter.
    #[builder(into)]
    #[serde(rename = "resources")]
    pub r#resources: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServicePerimeterDryRunEgressPolicyEgressTo {
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
                    "external_resources",
                    &self.r#external_resources,
                ),
                to_pulumi_object_field(
                    "operations",
                    &self.r#operations,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServicePerimeterDryRunEgressPolicyEgressTo {
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
                    r#external_resources: {
                        let field_value = match fields_map.get("external_resources") {
                            Some(value) => value,
                            None => bail!("Missing field 'external_resources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#operations: {
                        let field_value = match fields_map.get("operations") {
                            Some(value) => value,
                            None => bail!("Missing field 'operations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
