#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDatasetAccess {
    /// Grants all resources of particular types in a particular dataset read access to the current dataset.
    #[builder(into)]
    #[serde(rename = "datasets")]
    pub r#datasets: Vec<super::super::types::bigquery::GetDatasetAccessDataset>,
    /// A domain to grant access to. Any users signed in with the
    /// domain specified will be granted the specified access
    #[builder(into)]
    #[serde(rename = "domain")]
    pub r#domain: String,
    /// An email address of a Google Group to grant access to.
    #[builder(into)]
    #[serde(rename = "groupByEmail")]
    pub r#group_by_email: String,
    /// Some other type of member that appears in the IAM Policy but isn't a user,
    /// group, domain, or special group. For example: 'allUsers'
    #[builder(into)]
    #[serde(rename = "iamMember")]
    pub r#iam_member: String,
    /// Describes the rights granted to the user specified by the other
    /// member of the access object. Basic, predefined, and custom roles
    /// are supported. Predefined roles that have equivalent basic roles
    /// are swapped by the API to their basic counterparts. See
    /// [official docs](https://cloud.google.com/bigquery/docs/access-control).
    #[builder(into)]
    #[serde(rename = "role")]
    pub r#role: String,
    /// A routine from a different dataset to grant access to. Queries
    /// executed against that routine will have read access to tables in
    /// this dataset. The role field is not required when this field is
    /// set. If that routine is updated by any user, access to the routine
    /// needs to be granted again via an update operation.
    #[builder(into)]
    #[serde(rename = "routines")]
    pub r#routines: Vec<super::super::types::bigquery::GetDatasetAccessRoutine>,
    /// A special group to grant access to. Possible values include:
    /// * 'projectOwners': Owners of the enclosing project.
    /// * 'projectReaders': Readers of the enclosing project.
    /// * 'projectWriters': Writers of the enclosing project.
    /// * 'allAuthenticatedUsers': All authenticated BigQuery users.
    #[builder(into)]
    #[serde(rename = "specialGroup")]
    pub r#special_group: String,
    /// An email address of a user to grant access to. For example:
    /// fred@example.com
    #[builder(into)]
    #[serde(rename = "userByEmail")]
    pub r#user_by_email: String,
    /// A view from a different dataset to grant access to. Queries
    /// executed against that view will have read access to tables in
    /// this dataset. The role field is not required when this field is
    /// set. If that view is updated by any user, access to the view
    /// needs to be granted again via an update operation.
    #[builder(into)]
    #[serde(rename = "views")]
    pub r#views: Vec<super::super::types::bigquery::GetDatasetAccessView>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDatasetAccess {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "datasets",
                    &self.r#datasets,
                ),
                to_pulumi_object_field(
                    "domain",
                    &self.r#domain,
                ),
                to_pulumi_object_field(
                    "group_by_email",
                    &self.r#group_by_email,
                ),
                to_pulumi_object_field(
                    "iam_member",
                    &self.r#iam_member,
                ),
                to_pulumi_object_field(
                    "role",
                    &self.r#role,
                ),
                to_pulumi_object_field(
                    "routines",
                    &self.r#routines,
                ),
                to_pulumi_object_field(
                    "special_group",
                    &self.r#special_group,
                ),
                to_pulumi_object_field(
                    "user_by_email",
                    &self.r#user_by_email,
                ),
                to_pulumi_object_field(
                    "views",
                    &self.r#views,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDatasetAccess {
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
                    r#datasets: {
                        let field_value = match fields_map.get("datasets") {
                            Some(value) => value,
                            None => bail!("Missing field 'datasets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#domain: {
                        let field_value = match fields_map.get("domain") {
                            Some(value) => value,
                            None => bail!("Missing field 'domain' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#group_by_email: {
                        let field_value = match fields_map.get("group_by_email") {
                            Some(value) => value,
                            None => bail!("Missing field 'group_by_email' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#iam_member: {
                        let field_value = match fields_map.get("iam_member") {
                            Some(value) => value,
                            None => bail!("Missing field 'iam_member' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#role: {
                        let field_value = match fields_map.get("role") {
                            Some(value) => value,
                            None => bail!("Missing field 'role' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#routines: {
                        let field_value = match fields_map.get("routines") {
                            Some(value) => value,
                            None => bail!("Missing field 'routines' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#special_group: {
                        let field_value = match fields_map.get("special_group") {
                            Some(value) => value,
                            None => bail!("Missing field 'special_group' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_by_email: {
                        let field_value = match fields_map.get("user_by_email") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_by_email' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#views: {
                        let field_value = match fields_map.get("views") {
                            Some(value) => value,
                            None => bail!("Missing field 'views' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
