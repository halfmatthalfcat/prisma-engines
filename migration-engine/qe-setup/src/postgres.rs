use datamodel::datamodel_connector::capabilities::*;
use migration_core::migration_connector::{ConnectorError, ConnectorResult};
use quaint::{prelude::*, single::Quaint};
use std::collections::HashMap;
use url::Url;

pub(crate) async fn postgres_setup(
    url: String,
    prisma_schema: &str,
    capabilities: &[ConnectorCapability],
) -> ConnectorResult<()> {
    {
        let mut url = Url::parse(&url).map_err(ConnectorError::url_parse_error)?;
        let quaint_url = quaint::connector::PostgresUrl::new(url.clone()).unwrap();
        let (db_name, schema) = (quaint_url.dbname(), quaint_url.schema());

        strip_schema_param_from_url(&mut url);
        let conn = create_postgres_admin_conn(url.clone()).await?;

        let query = format!("DROP DATABASE IF EXISTS \"{}\"", db_name);
        conn.raw_cmd(&query).await.ok();

        let query = format!("CREATE DATABASE \"{}\"", db_name);
        conn.raw_cmd(&query).await.ok();

        // Now create the schema
        url.set_path(&format!("/{}", db_name));

        let conn = Quaint::new(&url.to_string()).await.unwrap();

        let drop_and_recreate_schema = format!(
            "DROP SCHEMA IF EXISTS \"{schema}\" CASCADE;\nCREATE SCHEMA \"{schema}\";",
            schema = schema
        );
        conn.raw_cmd(&drop_and_recreate_schema)
            .await
            .map_err(|e| ConnectorError::from_source(e, ""))?;

        let search_path = format!("SET search_path TO \"{schema}\",public;");
        conn.raw_cmd(&search_path)
            .await
            .map_err(|e| ConnectorError::from_source(e, ""))?;

        if capabilities.contains(&ConnectorCapability::Ltree) {
            let ltree_ext = format!(
                "CREATE EXTENSION IF NOT EXISTS ltree WITH SCHEMA \"{schema}\";",
                schema = schema,
            );
            conn.raw_cmd(&ltree_ext)
                .await
                .map_err(|e| ConnectorError::from_source(e, ""))?;
        }
    }

    crate::diff_and_apply(prisma_schema).await;
    Ok(())
}

pub(crate) async fn postgres_teardown(url: &str) -> ConnectorResult<()> {
    let mut url = Url::parse(url).map_err(ConnectorError::url_parse_error)?;
    strip_schema_param_from_url(&mut url);

    let conn = create_postgres_admin_conn(url.clone()).await?;
    let db_name = url.path().strip_prefix('/').unwrap();

    let query = format!("DROP DATABASE \"{}\" CASCADE", db_name);
    conn.raw_cmd(&query).await.ok();

    Ok(())
}

async fn create_postgres_admin_conn(mut url: Url) -> ConnectorResult<Quaint> {
    url.set_path("/postgres");
    Ok(Quaint::new(&url.to_string()).await.unwrap())
}

fn strip_schema_param_from_url(url: &mut Url) {
    let mut params: HashMap<String, String> = url.query_pairs().into_owned().collect();
    params.remove("schema");
    let params: Vec<String> = params.into_iter().map(|(k, v)| format!("{}={}", k, v)).collect();
    let params: String = params.join("&");
    url.set_query(Some(&params));
}
