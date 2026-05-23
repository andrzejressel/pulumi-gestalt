#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProductResponse {
    /// Availability information of the product system.
    #[builder(into)]
    pub r#availability_information: Box<super::types::AvailabilityInformationResponse>,
    /// List of configurations for the product
    #[builder(into)]
    pub r#configurations: Vec<super::types::ConfigurationResponse>,
    /// Cost information for the product system.
    #[builder(into)]
    pub r#cost_information: Box<super::types::CostInformationResponse>,
    /// Description related to the product system.
    #[builder(into)]
    pub r#description: Box<super::types::DescriptionResponse>,
    /// Display Name for the product system.
    #[builder(into)]
    pub r#display_name: String,
    /// list of filters supported for a product
    #[builder(into)]
    pub r#filterable_properties: Vec<super::types::FilterablePropertyResponse>,
    /// Hierarchy information of a product.
    #[builder(into)]
    pub r#hierarchy_information: Box<super::types::HierarchyInformationResponse>,
    /// Image information for the product system.
    #[builder(into)]
    pub r#image_information: Vec<super::types::ImageInformationResponse>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ProductResponse {
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
                    "availabilityInformation",
                    &self.r#availability_information,
                ),
                to_pulumi_object_field(
                    "configurations",
                    &self.r#configurations,
                ),
                to_pulumi_object_field(
                    "costInformation",
                    &self.r#cost_information,
                ),
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "displayName",
                    &self.r#display_name,
                ),
                to_pulumi_object_field(
                    "filterableProperties",
                    &self.r#filterable_properties,
                ),
                to_pulumi_object_field(
                    "hierarchyInformation",
                    &self.r#hierarchy_information,
                ),
                to_pulumi_object_field(
                    "imageInformation",
                    &self.r#image_information,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ProductResponse {
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
                    r#availability_information: {
                        let field_value = match fields_map.get("availabilityInformation") {
                            Some(value) => value,
                            None => bail!("Missing field 'availabilityInformation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#configurations: {
                        let field_value = match fields_map.get("configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cost_information: {
                        let field_value = match fields_map.get("costInformation") {
                            Some(value) => value,
                            None => bail!("Missing field 'costInformation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#display_name: {
                        let field_value = match fields_map.get("displayName") {
                            Some(value) => value,
                            None => bail!("Missing field 'displayName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filterable_properties: {
                        let field_value = match fields_map.get("filterableProperties") {
                            Some(value) => value,
                            None => bail!("Missing field 'filterableProperties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hierarchy_information: {
                        let field_value = match fields_map.get("hierarchyInformation") {
                            Some(value) => value,
                            None => bail!("Missing field 'hierarchyInformation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image_information: {
                        let field_value = match fields_map.get("imageInformation") {
                            Some(value) => value,
                            None => bail!("Missing field 'imageInformation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
