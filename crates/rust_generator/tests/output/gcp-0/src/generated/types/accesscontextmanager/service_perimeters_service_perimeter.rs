#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServicePerimetersServicePerimeter {
    /// (Output)
    /// Time the AccessPolicy was created in UTC.
    #[builder(into)]
    #[serde(rename = "createTime")]
    pub r#create_time: Option<String>,
    /// Description of the ServicePerimeter and its use. Does not affect
    /// behavior.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Resource name for the ServicePerimeter. The short_name component must
    /// begin with a letter and only include alphanumeric and '_'.
    /// Format: accessPolicies/{policy_id}/servicePerimeters/{short_name}
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Specifies the type of the Perimeter. There are two types: regular and
    /// bridge. Regular Service Perimeter contains resources, access levels,
    /// and restricted services. Every resource can be in at most
    /// ONE regular Service Perimeter.
    /// In addition to being in a regular service perimeter, a resource can also
    /// be in zero or more perimeter bridges. A perimeter bridge only contains
    /// resources. Cross project operations are permitted if all effected
    /// resources share some perimeter (whether bridge or regular). Perimeter
    /// Bridge does not contain access levels or services: those are governed
    /// entirely by the regular perimeter that resource is in.
    /// Perimeter Bridges are typically useful when building more complex
    /// topologies with many independent perimeters that need to share some data
    /// with a common perimeter, but should not be able to share data among
    /// themselves.
    /// Default value is `PERIMETER_TYPE_REGULAR`.
    /// Possible values are: `PERIMETER_TYPE_REGULAR`, `PERIMETER_TYPE_BRIDGE`.
    #[builder(into)]
    #[serde(rename = "perimeterType")]
    pub r#perimeter_type: Option<String>,
    /// Proposed (or dry run) ServicePerimeter configuration.
    /// This configuration allows to specify and test ServicePerimeter configuration
    /// without enforcing actual access restrictions. Only allowed to be set when
    /// the `useExplicitDryRunSpec` flag is set.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "spec")]
    pub r#spec: Option<Box<super::super::types::accesscontextmanager::ServicePerimetersServicePerimeterSpec>>,
    /// ServicePerimeter configuration. Specifies sets of resources,
    /// restricted services and access levels that determine
    /// perimeter content and boundaries.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<Box<super::super::types::accesscontextmanager::ServicePerimetersServicePerimeterStatus>>,
    /// Human readable title. Must be unique within the Policy.
    #[builder(into)]
    #[serde(rename = "title")]
    pub r#title: String,
    /// (Output)
    /// Time the AccessPolicy was updated in UTC.
    #[builder(into)]
    #[serde(rename = "updateTime")]
    pub r#update_time: Option<String>,
    /// Use explicit dry run spec flag. Ordinarily, a dry-run spec implicitly exists
    /// for all Service Perimeters, and that spec is identical to the status for those
    /// Service Perimeters. When this flag is set, it inhibits the generation of the
    /// implicit spec, thereby allowing the user to explicitly provide a
    /// configuration ("spec") to use in a dry-run version of the Service Perimeter.
    /// This allows the user to test changes to the enforced config ("status") without
    /// actually enforcing them. This testing is done through analyzing the differences
    /// between currently enforced and suggested restrictions. useExplicitDryRunSpec must
    /// bet set to True if any of the fields in the spec are set to non-default values.
    #[builder(into)]
    #[serde(rename = "useExplicitDryRunSpec")]
    pub r#use_explicit_dry_run_spec: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServicePerimetersServicePerimeter {
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
                    "create_time",
                    &self.r#create_time,
                ),
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "perimeter_type",
                    &self.r#perimeter_type,
                ),
                to_pulumi_object_field(
                    "spec",
                    &self.r#spec,
                ),
                to_pulumi_object_field(
                    "status",
                    &self.r#status,
                ),
                to_pulumi_object_field(
                    "title",
                    &self.r#title,
                ),
                to_pulumi_object_field(
                    "update_time",
                    &self.r#update_time,
                ),
                to_pulumi_object_field(
                    "use_explicit_dry_run_spec",
                    &self.r#use_explicit_dry_run_spec,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServicePerimetersServicePerimeter {
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
                    r#create_time: {
                        let field_value = match fields_map.get("create_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'create_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#perimeter_type: {
                        let field_value = match fields_map.get("perimeter_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'perimeter_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spec: {
                        let field_value = match fields_map.get("spec") {
                            Some(value) => value,
                            None => bail!("Missing field 'spec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#status: {
                        let field_value = match fields_map.get("status") {
                            Some(value) => value,
                            None => bail!("Missing field 'status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#title: {
                        let field_value = match fields_map.get("title") {
                            Some(value) => value,
                            None => bail!("Missing field 'title' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#update_time: {
                        let field_value = match fields_map.get("update_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'update_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_explicit_dry_run_spec: {
                        let field_value = match fields_map.get("use_explicit_dry_run_spec") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_explicit_dry_run_spec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
