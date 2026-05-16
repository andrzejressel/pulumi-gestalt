#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LocationFsxOntapFileSystemProtocolSmb {
    /// Fully qualified domain name of the Microsoft Active Directory (AD) that your storage virtual machine belongs to.
    #[builder(into)]
    #[serde(rename = "domain")]
    pub r#domain: Option<String>,
    /// Mount options that are available for DataSync to access an SMB location. See SMB Mount Options below.
    #[builder(into)]
    #[serde(rename = "mountOptions")]
    pub r#mount_options: Box<super::super::types::datasync::LocationFsxOntapFileSystemProtocolSmbMountOptions>,
    /// Password of a user who has permission to access your SVM.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: String,
    /// Username that can mount the location and access the files, folders, and metadata that you need in the SVM.
    #[builder(into)]
    #[serde(rename = "user")]
    pub r#user: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LocationFsxOntapFileSystemProtocolSmb {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("domain".to_string(), self.r#domain.to_pulumi_value().await);
            map.insert("mount_options".to_string(), self.r#mount_options.to_pulumi_value().await);
            map.insert("password".to_string(), self.r#password.to_pulumi_value().await);
            map.insert("user".to_string(), self.r#user.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LocationFsxOntapFileSystemProtocolSmb {
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
                    r#domain: {
                        let field_value = match fields_map.get("domain") {
                            Some(value) => value,
                            None => bail!("Missing field 'domain' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#mount_options: {
                        let field_value = match fields_map.get("mount_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'mount_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::datasync::LocationFsxOntapFileSystemProtocolSmbMountOptions> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#password: {
                        let field_value = match fields_map.get("password") {
                            Some(value) => value,
                            None => bail!("Missing field 'password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#user: {
                        let field_value = match fields_map.get("user") {
                            Some(value) => value,
                            None => bail!("Missing field 'user' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
