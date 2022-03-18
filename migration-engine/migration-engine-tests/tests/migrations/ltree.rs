use migration_engine_tests::test_api::*;
use sql_schema_describer::ColumnTypeFamily;

#[test_connector(capabilities(Ltree), tags(Postgres13, Postgres14))]
fn ltree_fields_can_be_created(api: TestApi) {
    let dm = r#"
        model Test {
            id String @id @default(cuid())
            path String @db.ltree
        }
    "#;

    api.schema_push_w_datasource(dm).send().assert_green();

    api.assert_schema().assert_table("Test", |table| {
        table.assert_column("path", |column| {
            column.assert_is_required().assert_type_family(ColumnTypeFamily::String)
        })
    });

    api.schema_push_w_datasource(dm).send().assert_green().assert_no_steps();
}

#[test_connector(capabilities(Ltree), tags(Postgres13, Postgres14))]
fn ltree_fields_convert_to_string(api: TestApi) {
    let dm = r#"
        model Test {
            id String @id @default(cuid())
            path String @db.ltree
        }
    "#;

    api.schema_push_w_datasource(dm).send().assert_green();

    let dm2 = r#"
        model Test {
            id String @id @default(cuid())
            path String
        }
    "#;

    api.schema_push_w_datasource(dm2).send().assert_green();

    api.assert_schema().assert_table("Test", |table| {
        table.assert_column("path", |column| {
            column.assert_is_required().assert_type_family(ColumnTypeFamily::String)
        })
    });
}
