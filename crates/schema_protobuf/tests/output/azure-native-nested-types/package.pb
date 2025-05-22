
azure-nativeAzure Native"0.0.1*ŸF
V

documentdbSqlResourceSqlContainer/azure-native:documentdb:SqlResourceSqlContainerÓDAn Azure Cosmos DB container.
API Version: 2021-03-15.

{{% examples %}}
## Example Usage
{{% example %}}
### CosmosDBSqlContainerCreateUpdate
```csharp
using Pulumi;
using AzureNative = Pulumi.AzureNative;

class MyStack : Stack
{
    public MyStack()
    {
        var sqlResourceSqlContainer = new AzureNative.DocumentDB.SqlResourceSqlContainer("sqlResourceSqlContainer", new AzureNative.DocumentDB.SqlResourceSqlContainerArgs
        {
            AccountName = "ddb1",
            ContainerName = "containerName",
            DatabaseName = "databaseName",
            Location = "West US",
            Options = ,
            Resource = new AzureNative.DocumentDB.Inputs.SqlContainerResourceArgs
            {
                ConflictResolutionPolicy = new AzureNative.DocumentDB.Inputs.ConflictResolutionPolicyArgs
                {
                    ConflictResolutionPath = "/path",
                    Mode = "LastWriterWins",
                },
                DefaultTtl = 100,
                Id = "containerName",
                IndexingPolicy = new AzureNative.DocumentDB.Inputs.IndexingPolicyArgs
                {
                    Automatic = true,
                    ExcludedPaths = {},
                    IncludedPaths = 
                    {
                        new AzureNative.DocumentDB.Inputs.IncludedPathArgs
                        {
                            Indexes = 
                            {
                                new AzureNative.DocumentDB.Inputs.IndexesArgs
                                {
                                    DataType = "String",
                                    Kind = "Range",
                                    Precision = -1,
                                },
                                new AzureNative.DocumentDB.Inputs.IndexesArgs
                                {
                                    DataType = "Number",
                                    Kind = "Range",
                                    Precision = -1,
                                },
                            },
                            Path = "/*",
                        },
                    },
                    IndexingMode = "consistent",
                },
                PartitionKey = new AzureNative.DocumentDB.Inputs.ContainerPartitionKeyArgs
                {
                    Kind = "Hash",
                    Paths = 
                    {
                        "/AccountNumber",
                    },
                },
                UniqueKeyPolicy = new AzureNative.DocumentDB.Inputs.UniqueKeyPolicyArgs
                {
                    UniqueKeys = 
                    {
                        new AzureNative.DocumentDB.Inputs.UniqueKeyArgs
                        {
                            Paths = 
                            {
                                "/testPath",
                            },
                        },
                    },
                },
            },
            ResourceGroupName = "rg1",
            Tags = ,
        });
    }

}

```

```go
package main

import (
	documentdb "github.com/pulumi/pulumi-azure-native/sdk/go/azure/documentdb"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := documentdb.NewSqlResourceSqlContainer(ctx, "sqlResourceSqlContainer", &documentdb.SqlResourceSqlContainerArgs{
			AccountName:   pulumi.String("ddb1"),
			ContainerName: pulumi.String("containerName"),
			DatabaseName:  pulumi.String("databaseName"),
			Location:      pulumi.String("West US"),
			Options:       nil,
			Resource: &documentdb.SqlContainerResourceArgs{
				ConflictResolutionPolicy: &documentdb.ConflictResolutionPolicyArgs{
					ConflictResolutionPath: pulumi.String("/path"),
					Mode:                   pulumi.String("LastWriterWins"),
				},
				DefaultTtl: pulumi.Int(100),
				Id:         pulumi.String("containerName"),
				IndexingPolicy: &documentdb.IndexingPolicyArgs{
					Automatic:     pulumi.Bool(true),
					ExcludedPaths: documentdb.ExcludedPathArray{},
					IncludedPaths: documentdb.IncludedPathArray{
						&documentdb.IncludedPathArgs{
							Indexes: documentdb.IndexesArray{
								&documentdb.IndexesArgs{
									DataType:  pulumi.String("String"),
									Kind:      pulumi.String("Range"),
									Precision: -1,
								},
								&documentdb.IndexesArgs{
									DataType:  pulumi.String("Number"),
									Kind:      pulumi.String("Range"),
									Precision: -1,
								},
							},
							Path: pulumi.String("/*"),
						},
					},
					IndexingMode: pulumi.String("consistent"),
				},
				PartitionKey: &documentdb.ContainerPartitionKeyArgs{
					Kind: pulumi.String("Hash"),
					Paths: pulumi.StringArray{
						pulumi.String("/AccountNumber"),
					},
				},
				UniqueKeyPolicy: &documentdb.UniqueKeyPolicyArgs{
					UniqueKeys: documentdb.UniqueKeyArray{
						&documentdb.UniqueKeyArgs{
							Paths: pulumi.StringArray{
								pulumi.String("/testPath"),
							},
						},
					},
				},
			},
			ResourceGroupName: pulumi.String("rg1"),
			Tags:              nil,
		})
		if err != nil {
			return err
		}
		return nil
	})
}

```

```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure_native from "@pulumi/azure-native";

const sqlResourceSqlContainer = new azure_native.documentdb.SqlResourceSqlContainer("sqlResourceSqlContainer", {
    accountName: "ddb1",
    containerName: "containerName",
    databaseName: "databaseName",
    location: "West US",
    options: {},
    resource: {
        conflictResolutionPolicy: {
            conflictResolutionPath: "/path",
            mode: "LastWriterWins",
        },
        defaultTtl: 100,
        id: "containerName",
        indexingPolicy: {
            automatic: true,
            excludedPaths: [],
            includedPaths: [{
                indexes: [
                    {
                        dataType: "String",
                        kind: "Range",
                        precision: -1,
                    },
                    {
                        dataType: "Number",
                        kind: "Range",
                        precision: -1,
                    },
                ],
                path: "/*",
            }],
            indexingMode: "consistent",
        },
        partitionKey: {
            kind: "Hash",
            paths: ["/AccountNumber"],
        },
        uniqueKeyPolicy: {
            uniqueKeys: [{
                paths: ["/testPath"],
            }],
        },
    },
    resourceGroupName: "rg1",
    tags: {},
});

```

```python
import pulumi
import pulumi_azure_native as azure_native

sql_resource_sql_container = azure_native.documentdb.SqlResourceSqlContainer("sqlResourceSqlContainer",
    account_name="ddb1",
    container_name="containerName",
    database_name="databaseName",
    location="West US",
    options=azure_native.documentdb.CreateUpdateOptionsArgs(),
    resource=azure_native.documentdb.SqlContainerResourceArgs(
        conflict_resolution_policy=azure_native.documentdb.ConflictResolutionPolicyArgs(
            conflict_resolution_path="/path",
            mode="LastWriterWins",
        ),
        default_ttl=100,
        id="containerName",
        indexing_policy=azure_native.documentdb.IndexingPolicyArgs(
            automatic=True,
            excluded_paths=[],
            included_paths=[azure_native.documentdb.IncludedPathArgs(
                indexes=[
                    azure_native.documentdb.IndexesArgs(
                        data_type="String",
                        kind="Range",
                        precision=-1,
                    ),
                    azure_native.documentdb.IndexesArgs(
                        data_type="Number",
                        kind="Range",
                        precision=-1,
                    ),
                ],
                path="/*",
            )],
            indexing_mode="consistent",
        ),
        partition_key=azure_native.documentdb.ContainerPartitionKeyArgs(
            kind="Hash",
            paths=["/AccountNumber"],
        ),
        unique_key_policy=azure_native.documentdb.UniqueKeyPolicyArgs(
            unique_keys=[azure_native.documentdb.UniqueKeyArgs(
                paths=["/testPath"],
            )],
        ),
    ),
    resource_group_name="rg1",
    tags={})

```

{{% /example %}}
{{% /examples %}}

## Import

An existing resource can be imported using its type token, name, and identifier, e.g.

```sh
$ pulumi import azure-native:documentdb:SqlResourceSqlContainer containerName /subscriptions/subid/resourceGroups/rg1/providers/Microsoft.DocumentDB/databaseAccounts/ddb1/sqlDatabases/databaseName/sqlContainers/containerName 
```
"ç
resourceÄB~:|
z

documentdb)SqlContainerGetPropertiesResponseResourceAazure-native:documentdb:SqlContainerGetPropertiesResponseResource:ï
R

documentdbCompositePathResponse-azure-native:documentdb:CompositePathResponseæ
ª.
orderB" Sort order for composite paths.à
pathB" zThe path for which the indexing behavior applies to. Index paths typically start with root and end with wildcard (/path/*):á
T

documentdbIndexingPolicyResponse.azure-native:documentdb:IndexingPolicyResponseÆ
´
Cosmos DB indexing policyç
compositeIndexes\BZ*X*V:T
R

documentdbCompositePathResponse-azure-native:documentdb:CompositePathResponseList of composite path list:Ó
z

documentdb)SqlContainerGetPropertiesResponseResourceAazure-native:documentdb:SqlContainerGetPropertiesResponseResourceÔ
ÏÈ
indexingPolicyZBX:V
T

documentdbIndexingPolicyResponse.azure-native:documentdb:IndexingPolicyResponse{The configuration of the indexing policy. By default, the indexing is automatic for all document paths within the container