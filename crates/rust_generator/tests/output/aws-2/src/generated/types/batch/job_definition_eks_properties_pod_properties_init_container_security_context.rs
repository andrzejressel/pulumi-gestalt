#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobDefinitionEksPropertiesPodPropertiesInitContainerSecurityContext {
    #[builder(into)]
    #[serde(rename = "privileged")]
    pub r#privileged: Option<bool>,
    #[builder(into)]
    #[serde(rename = "readOnlyRootFileSystem")]
    pub r#read_only_root_file_system: Option<bool>,
    #[builder(into)]
    #[serde(rename = "runAsGroup")]
    pub r#run_as_group: Option<i32>,
    #[builder(into)]
    #[serde(rename = "runAsNonRoot")]
    pub r#run_as_non_root: Option<bool>,
    #[builder(into)]
    #[serde(rename = "runAsUser")]
    pub r#run_as_user: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobDefinitionEksPropertiesPodPropertiesInitContainerSecurityContext {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "privileged".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#privileged,
                )
                .await,
            );
            map.insert(
                "read_only_root_file_system".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#read_only_root_file_system,
                )
                .await,
            );
            map.insert(
                "run_as_group".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#run_as_group,
                )
                .await,
            );
            map.insert(
                "run_as_non_root".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#run_as_non_root,
                )
                .await,
            );
            map.insert(
                "run_as_user".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#run_as_user,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for JobDefinitionEksPropertiesPodPropertiesInitContainerSecurityContext {
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
                    r#privileged: {
                        let field_value = match fields_map.get("privileged") {
                            Some(value) => value,
                            None => bail!("Missing field 'privileged' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#read_only_root_file_system: {
                        let field_value = match fields_map.get("read_only_root_file_system") {
                            Some(value) => value,
                            None => bail!("Missing field 'read_only_root_file_system' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#run_as_group: {
                        let field_value = match fields_map.get("run_as_group") {
                            Some(value) => value,
                            None => bail!("Missing field 'run_as_group' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#run_as_non_root: {
                        let field_value = match fields_map.get("run_as_non_root") {
                            Some(value) => value,
                            None => bail!("Missing field 'run_as_non_root' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#run_as_user: {
                        let field_value = match fields_map.get("run_as_user") {
                            Some(value) => value,
                            None => bail!("Missing field 'run_as_user' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
