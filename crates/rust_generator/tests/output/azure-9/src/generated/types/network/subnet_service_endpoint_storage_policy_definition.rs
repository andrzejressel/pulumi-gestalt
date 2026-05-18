#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SubnetServiceEndpointStoragePolicyDefinition {
    /// The description of this Subnet Service Endpoint Storage Policy Definition.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The name which should be used for this Subnet Service Endpoint Storage Policy Definition.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The type of service resources. Valid values are `Microsoft.Storage` or `Global`. When the `service_resources` property contains resource IDs, this property must be `Microsoft.Storage`. When the `service_resources` property contains Aliases, this property must be `Global`. Defaults to `Microsoft.Storage`.
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Option<String>,
    /// Specifies a list of resources or aliases that this Subnet Service Endpoint Storage Policy Definition applies to.
    /// 
    /// > **NOTE** The `service_resources` property must contain either Aliases or Resource IDs, but not both.
    #[builder(into)]
    #[serde(rename = "serviceResources")]
    pub r#service_resources: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SubnetServiceEndpointStoragePolicyDefinition {
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
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "service",
                    &self.r#service,
                ),
                to_pulumi_object_field(
                    "service_resources",
                    &self.r#service_resources,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SubnetServiceEndpointStoragePolicyDefinition {
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
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#service: {
                        let field_value = match fields_map.get("service") {
                            Some(value) => value,
                            None => bail!("Missing field 'service' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_resources: {
                        let field_value = match fields_map.get("service_resources") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_resources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
