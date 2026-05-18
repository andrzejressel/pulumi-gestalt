#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesApplicationServerVirtualMachineDataDisk {
    /// A list of full names of Data Disks per Volume. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "names")]
    pub r#names: Vec<String>,
    /// The name of the Volume. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** Possible value for Application Server and Central Server is `default`.
    /// 
    /// > **Note:** Possible values for Database Server are `hanaData`, `hanaLog`, `hanaShared` and `usrSap`.
    #[builder(into)]
    #[serde(rename = "volumeName")]
    pub r#volume_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesApplicationServerVirtualMachineDataDisk {
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
                    "names",
                    &self.r#names,
                ),
                to_pulumi_object_field(
                    "volume_name",
                    &self.r#volume_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesApplicationServerVirtualMachineDataDisk {
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
                    r#names: {
                        let field_value = match fields_map.get("names") {
                            Some(value) => value,
                            None => bail!("Missing field 'names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#volume_name: {
                        let field_value = match fields_map.get("volume_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'volume_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
