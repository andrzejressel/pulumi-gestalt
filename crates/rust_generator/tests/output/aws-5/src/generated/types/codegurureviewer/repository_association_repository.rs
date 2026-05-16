#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RepositoryAssociationRepository {
    #[builder(into)]
    #[serde(rename = "bitbucket")]
    pub r#bitbucket: Option<Box<super::super::types::codegurureviewer::RepositoryAssociationRepositoryBitbucket>>,
    #[builder(into)]
    #[serde(rename = "codecommit")]
    pub r#codecommit: Option<Box<super::super::types::codegurureviewer::RepositoryAssociationRepositoryCodecommit>>,
    #[builder(into)]
    #[serde(rename = "githubEnterpriseServer")]
    pub r#github_enterprise_server: Option<Box<super::super::types::codegurureviewer::RepositoryAssociationRepositoryGithubEnterpriseServer>>,
    #[builder(into)]
    #[serde(rename = "s3Bucket")]
    pub r#s_3_bucket: Option<Box<super::super::types::codegurureviewer::RepositoryAssociationRepositoryS3Bucket>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RepositoryAssociationRepository {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("bitbucket".to_string(), self.r#bitbucket.to_pulumi_value().await);
            map.insert("codecommit".to_string(), self.r#codecommit.to_pulumi_value().await);
            map.insert("github_enterprise_server".to_string(), self.r#github_enterprise_server.to_pulumi_value().await);
            map.insert("s_3_bucket".to_string(), self.r#s_3_bucket.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RepositoryAssociationRepository {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#bitbucket: {
                        let field_value = match fields_map.get("bitbucket") {
                            Some(value) => value,
                            None => bail!("Missing field 'bitbucket' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::codegurureviewer::RepositoryAssociationRepositoryBitbucket>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#codecommit: {
                        let field_value = match fields_map.get("codecommit") {
                            Some(value) => value,
                            None => bail!("Missing field 'codecommit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::codegurureviewer::RepositoryAssociationRepositoryCodecommit>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#github_enterprise_server: {
                        let field_value = match fields_map.get("github_enterprise_server") {
                            Some(value) => value,
                            None => bail!("Missing field 'github_enterprise_server' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::codegurureviewer::RepositoryAssociationRepositoryGithubEnterpriseServer>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#s_3_bucket: {
                        let field_value = match fields_map.get("s_3_bucket") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_bucket' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::codegurureviewer::RepositoryAssociationRepositoryS3Bucket>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
