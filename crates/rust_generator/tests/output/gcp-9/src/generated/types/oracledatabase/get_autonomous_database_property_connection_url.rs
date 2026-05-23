#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAutonomousDatabasePropertyConnectionUrl {
    /// Oracle Application Express (APEX) URL.
    #[builder(into)]
    pub r#apex_uri: String,
    /// The URL of the Database Transforms for the Autonomous Database.
    #[builder(into)]
    pub r#database_transforms_uri: String,
    /// The URL of the Graph Studio for the Autonomous Database.
    #[builder(into)]
    pub r#graph_studio_uri: String,
    /// The URL of the Oracle Machine Learning (OML) Notebook for the Autonomous
    /// Database.
    #[builder(into)]
    pub r#machine_learning_notebook_uri: String,
    /// The URL of Machine Learning user management the Autonomous Database.
    #[builder(into)]
    pub r#machine_learning_user_management_uri: String,
    /// The URL of the MongoDB API for the Autonomous Database.
    #[builder(into)]
    pub r#mongo_db_uri: String,
    /// The Oracle REST Data Services (ORDS) URL of the Web Access for the
    /// Autonomous Database.
    #[builder(into)]
    pub r#ords_uri: String,
    /// The URL of the Oracle SQL Developer Web for the Autonomous Database.
    #[builder(into)]
    pub r#sql_dev_web_uri: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetAutonomousDatabasePropertyConnectionUrl {
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
                    "apexUri",
                    &self.r#apex_uri,
                ),
                to_pulumi_object_field(
                    "databaseTransformsUri",
                    &self.r#database_transforms_uri,
                ),
                to_pulumi_object_field(
                    "graphStudioUri",
                    &self.r#graph_studio_uri,
                ),
                to_pulumi_object_field(
                    "machineLearningNotebookUri",
                    &self.r#machine_learning_notebook_uri,
                ),
                to_pulumi_object_field(
                    "machineLearningUserManagementUri",
                    &self.r#machine_learning_user_management_uri,
                ),
                to_pulumi_object_field(
                    "mongoDbUri",
                    &self.r#mongo_db_uri,
                ),
                to_pulumi_object_field(
                    "ordsUri",
                    &self.r#ords_uri,
                ),
                to_pulumi_object_field(
                    "sqlDevWebUri",
                    &self.r#sql_dev_web_uri,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetAutonomousDatabasePropertyConnectionUrl {
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
                    r#apex_uri: {
                        let field_value = match fields_map.get("apexUri") {
                            Some(value) => value,
                            None => bail!("Missing field 'apexUri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#database_transforms_uri: {
                        let field_value = match fields_map.get("databaseTransformsUri") {
                            Some(value) => value,
                            None => bail!("Missing field 'databaseTransformsUri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#graph_studio_uri: {
                        let field_value = match fields_map.get("graphStudioUri") {
                            Some(value) => value,
                            None => bail!("Missing field 'graphStudioUri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#machine_learning_notebook_uri: {
                        let field_value = match fields_map.get("machineLearningNotebookUri") {
                            Some(value) => value,
                            None => bail!("Missing field 'machineLearningNotebookUri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#machine_learning_user_management_uri: {
                        let field_value = match fields_map.get("machineLearningUserManagementUri") {
                            Some(value) => value,
                            None => bail!("Missing field 'machineLearningUserManagementUri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mongo_db_uri: {
                        let field_value = match fields_map.get("mongoDbUri") {
                            Some(value) => value,
                            None => bail!("Missing field 'mongoDbUri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ords_uri: {
                        let field_value = match fields_map.get("ordsUri") {
                            Some(value) => value,
                            None => bail!("Missing field 'ordsUri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sql_dev_web_uri: {
                        let field_value = match fields_map.get("sqlDevWebUri") {
                            Some(value) => value,
                            None => bail!("Missing field 'sqlDevWebUri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
