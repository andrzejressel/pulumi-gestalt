#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InferenceClusterSsl {
    /// The certificate for the SSL configuration.Conflicts with `ssl[0].leaf_domain_label`,`ssl[0].overwrite_existing_domain`. Changing this forces a new Machine Learning Inference Cluster to be created. Defaults to `""`.
    #[builder(into)]
    #[serde(rename = "cert")]
    pub r#cert: Option<String>,
    /// The cname of the SSL configuration.Conflicts with `ssl[0].leaf_domain_label`,`ssl[0].overwrite_existing_domain`. Changing this forces a new Machine Learning Inference Cluster to be created. Defaults to `""`.
    #[builder(into)]
    #[serde(rename = "cname")]
    pub r#cname: Option<String>,
    /// The key content for the SSL configuration.Conflicts with `ssl[0].leaf_domain_label`,`ssl[0].overwrite_existing_domain`. Changing this forces a new Machine Learning Inference Cluster to be created. Defaults to `""`.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// The leaf domain label for the SSL configuration. Conflicts with `ssl[0].cert`,`ssl[0].key`,`ssl[0].cname`. Changing this forces a new Machine Learning Inference Cluster to be created. Defaults to `""`.
    #[builder(into)]
    #[serde(rename = "leafDomainLabel")]
    pub r#leaf_domain_label: Option<String>,
    /// Whether or not to overwrite existing leaf domain. Conflicts with `ssl[0].cert`,`ssl[0].key`,`ssl[0].cname` Changing this forces a new Machine Learning Inference Cluster to be created. Defaults to `""`.
    #[builder(into)]
    #[serde(rename = "overwriteExistingDomain")]
    pub r#overwrite_existing_domain: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InferenceClusterSsl {
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
                "cert".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cert,
                )
                .await,
            );
            map.insert(
                "cname".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cname,
                )
                .await,
            );
            map.insert(
                "key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#key,
                )
                .await,
            );
            map.insert(
                "leaf_domain_label".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#leaf_domain_label,
                )
                .await,
            );
            map.insert(
                "overwrite_existing_domain".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#overwrite_existing_domain,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InferenceClusterSsl {
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
                    r#cert: {
                        let field_value = match fields_map.get("cert") {
                            Some(value) => value,
                            None => bail!("Missing field 'cert' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cname: {
                        let field_value = match fields_map.get("cname") {
                            Some(value) => value,
                            None => bail!("Missing field 'cname' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key: {
                        let field_value = match fields_map.get("key") {
                            Some(value) => value,
                            None => bail!("Missing field 'key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#leaf_domain_label: {
                        let field_value = match fields_map.get("leaf_domain_label") {
                            Some(value) => value,
                            None => bail!("Missing field 'leaf_domain_label' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#overwrite_existing_domain: {
                        let field_value = match fields_map.get("overwrite_existing_domain") {
                            Some(value) => value,
                            None => bail!("Missing field 'overwrite_existing_domain' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
