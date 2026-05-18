#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SloBasicSli {
    /// Availability based SLI, dervied from count of requests made to this service that return successfully.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "availability")]
    pub r#availability: Option<Box<super::super::types::monitoring::SloBasicSliAvailability>>,
    /// Parameters for a latency threshold SLI.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "latency")]
    pub r#latency: Option<Box<super::super::types::monitoring::SloBasicSliLatency>>,
    /// An optional set of locations to which this SLI is relevant.
    /// Telemetry from other locations will not be used to calculate
    /// performance for this SLI. If omitted, this SLI applies to all
    /// locations in which the Service has activity. For service types
    /// that don't support breaking down by location, setting this
    /// field will result in an error.
    #[builder(into)]
    #[serde(rename = "locations")]
    pub r#locations: Option<Vec<String>>,
    /// An optional set of RPCs to which this SLI is relevant.
    /// Telemetry from other methods will not be used to calculate
    /// performance for this SLI. If omitted, this SLI applies to all
    /// the Service's methods. For service types that don't support
    /// breaking down by method, setting this field will result in an
    /// error.
    #[builder(into)]
    #[serde(rename = "methods")]
    pub r#methods: Option<Vec<String>>,
    /// The set of API versions to which this SLI is relevant.
    /// Telemetry from other API versions will not be used to
    /// calculate performance for this SLI. If omitted,
    /// this SLI applies to all API versions. For service types
    /// that don't support breaking down by version, setting this
    /// field will result in an error.
    #[builder(into)]
    #[serde(rename = "versions")]
    pub r#versions: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SloBasicSli {
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
                    "availability",
                    &self.r#availability,
                ),
                to_pulumi_object_field(
                    "latency",
                    &self.r#latency,
                ),
                to_pulumi_object_field(
                    "locations",
                    &self.r#locations,
                ),
                to_pulumi_object_field(
                    "methods",
                    &self.r#methods,
                ),
                to_pulumi_object_field(
                    "versions",
                    &self.r#versions,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SloBasicSli {
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
                    r#availability: {
                        let field_value = match fields_map.get("availability") {
                            Some(value) => value,
                            None => bail!("Missing field 'availability' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#latency: {
                        let field_value = match fields_map.get("latency") {
                            Some(value) => value,
                            None => bail!("Missing field 'latency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#locations: {
                        let field_value = match fields_map.get("locations") {
                            Some(value) => value,
                            None => bail!("Missing field 'locations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#methods: {
                        let field_value = match fields_map.get("methods") {
                            Some(value) => value,
                            None => bail!("Missing field 'methods' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#versions: {
                        let field_value = match fields_map.get("versions") {
                            Some(value) => value,
                            None => bail!("Missing field 'versions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
