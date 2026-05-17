#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDatabasesDatabase {
    /// The charset value. See MySQL's
    /// [Supported Character Sets and Collations](https://dev.mysql.com/doc/refman/5.7/en/charset-charsets.html)
    /// and Postgres' [Character Set Support](https://www.postgresql.org/docs/9.6/static/multibyte.html)
    /// for more details and supported values. Postgres databases only support
    /// a value of 'UTF8' at creation time.
    #[builder(into)]
    #[serde(rename = "charset")]
    pub r#charset: String,
    /// The collation value. See MySQL's
    /// [Supported Character Sets and Collations](https://dev.mysql.com/doc/refman/5.7/en/charset-charsets.html)
    /// and Postgres' [Collation Support](https://www.postgresql.org/docs/9.6/static/collation.html)
    /// for more details and supported values. Postgres databases only support
    /// a value of 'en_US.UTF8' at creation time.
    #[builder(into)]
    #[serde(rename = "collation")]
    pub r#collation: String,
    /// The deletion policy for the database. Setting ABANDON allows the resource
    /// to be abandoned rather than deleted. This is useful for Postgres, where databases cannot be
    /// deleted from the API if there are users other than cloudsqlsuperuser with access. Possible
    /// values are: "ABANDON", "DELETE". Defaults to "DELETE".
    #[builder(into)]
    #[serde(rename = "deletionPolicy")]
    pub r#deletion_policy: String,
    /// The name of the Cloud SQL database instance in which the database belongs.
    #[builder(into)]
    #[serde(rename = "instance")]
    pub r#instance: String,
    /// The name of the database in the Cloud SQL instance.
    /// This does not include the project ID or instance name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The ID of the project in which the instance belongs.
    /// 
    /// > **Note** This datasource performs client-side sorting to provide consistent ordering of the databases.
    #[builder(into)]
    #[serde(rename = "project")]
    pub r#project: String,
    #[builder(into)]
    #[serde(rename = "selfLink")]
    pub r#self_link: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDatabasesDatabase {
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
                    "charset",
                    &self.r#charset,
                ),
                to_pulumi_object_field(
                    "collation",
                    &self.r#collation,
                ),
                to_pulumi_object_field(
                    "deletion_policy",
                    &self.r#deletion_policy,
                ),
                to_pulumi_object_field(
                    "instance",
                    &self.r#instance,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "project",
                    &self.r#project,
                ),
                to_pulumi_object_field(
                    "self_link",
                    &self.r#self_link,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDatabasesDatabase {
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
                    r#charset: {
                        let field_value = match fields_map.get("charset") {
                            Some(value) => value,
                            None => bail!("Missing field 'charset' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#collation: {
                        let field_value = match fields_map.get("collation") {
                            Some(value) => value,
                            None => bail!("Missing field 'collation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#deletion_policy: {
                        let field_value = match fields_map.get("deletion_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'deletion_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance: {
                        let field_value = match fields_map.get("instance") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#project: {
                        let field_value = match fields_map.get("project") {
                            Some(value) => value,
                            None => bail!("Missing field 'project' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#self_link: {
                        let field_value = match fields_map.get("self_link") {
                            Some(value) => value,
                            None => bail!("Missing field 'self_link' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
