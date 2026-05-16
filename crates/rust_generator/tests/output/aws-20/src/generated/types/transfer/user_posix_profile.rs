#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UserPosixProfile {
    /// The POSIX group ID used for all EFS operations by this user.
    #[builder(into)]
    #[serde(rename = "gid")]
    pub r#gid: i32,
    /// The secondary POSIX group IDs used for all EFS operations by this user.
    #[builder(into)]
    #[serde(rename = "secondaryGids")]
    pub r#secondary_gids: Option<Vec<i32>>,
    /// The POSIX user ID used for all EFS operations by this user.
    #[builder(into)]
    #[serde(rename = "uid")]
    pub r#uid: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for UserPosixProfile {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("gid".to_string(), self.r#gid.to_pulumi_value().await);
            map.insert("secondary_gids".to_string(), self.r#secondary_gids.to_pulumi_value().await);
            map.insert("uid".to_string(), self.r#uid.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for UserPosixProfile {
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
                    r#gid: {
                        let field_value = match fields_map.get("gid") {
                            Some(value) => value,
                            None => bail!("Missing field 'gid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#secondary_gids: {
                        let field_value = match fields_map.get("secondary_gids") {
                            Some(value) => value,
                            None => bail!("Missing field 'secondary_gids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<i32>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#uid: {
                        let field_value = match fields_map.get("uid") {
                            Some(value) => value,
                            None => bail!("Missing field 'uid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
