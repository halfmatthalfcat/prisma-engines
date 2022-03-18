use query_engine_tests::*;

#[test_suite(schema(pg_ltree), only(Postgres(13), Postgres(14)), capabilities(Ltree))]
mod ltree {
    use crate::queries::filters::ltree::ltree_query;
    use indoc::indoc;

    fn pg_ltree() -> String {
        let schema = indoc! {
            r#"model Path {
                #id(id, Int, @id)
                path String @test.ltree
            }"#
        };

        schema.to_owned()
    }

    #[connector_test]
    async fn test_equal_to(runner: Runner) -> TestResult<()> {
        create_test_data(&runner).await?;

        ltree_query(&runner, "path", "equals", r#""A.B""#, vec![2], false).await?;

        Ok(())
    }

    #[connector_test]
    async fn test_not_equal_to(runner: Runner) -> TestResult<()> {
        create_test_data(&runner).await?;

        ltree_query(
            &runner,
            "path",
            "equals",
            r#""A.B""#,
            vec![1, 3, 4, 5, 6, 7, 8, 9],
            true,
        )
        .await?;

        Ok(())
    }

    #[connector_test]
    async fn test_less_than(runner: Runner) -> TestResult<()> {
        create_test_data(&runner).await?;

        ltree_query(&runner, "path", "lt", r#""B""#, vec![1, 2, 3], false).await?;

        Ok(())
    }

    #[connector_test]
    async fn test_not_less_than(runner: Runner) -> TestResult<()> {
        create_test_data(&runner).await?;

        ltree_query(&runner, "path", "lt", r#""B""#, vec![4, 5, 6, 7, 8, 9], true).await?;

        Ok(())
    }

    #[connector_test]
    async fn test_less_than_or_equal_to(runner: Runner) -> TestResult<()> {
        create_test_data(&runner).await?;

        ltree_query(&runner, "path", "lte", r#""B""#, vec![1, 2, 3, 4], false).await?;

        Ok(())
    }

    #[connector_test]
    async fn test_not_less_than_or_equal_to(runner: Runner) -> TestResult<()> {
        create_test_data(&runner).await?;

        ltree_query(&runner, "path", "lte", r#""B""#, vec![5, 6, 7, 8, 9], true).await?;

        Ok(())
    }

    #[connector_test]
    async fn test_ancestor_of_single(runner: Runner) -> TestResult<()> {
        create_test_data(&runner).await?;

        ltree_query(&runner, "path", "ancestor_of", r#"["A.B"]"#, vec![1, 2], false).await?;

        Ok(())
    }

    #[connector_test]
    async fn test_not_ancestor_of_single(runner: Runner) -> TestResult<()> {
        create_test_data(&runner).await?;

        ltree_query(
            &runner,
            "path",
            "ancestor_of",
            r#"["A.B"]"#,
            vec![3, 4, 5, 6, 7, 8, 9],
            true,
        )
        .await?;

        Ok(())
    }

    #[connector_test]
    async fn test_ancestor_of_multiple(runner: Runner) -> TestResult<()> {
        create_test_data(&runner).await?;

        ltree_query(
            &runner,
            "path",
            "ancestor_of",
            r#"["A.B", "C.D"]"#,
            vec![1, 2, 7, 8],
            false,
        )
        .await?;

        Ok(())
    }

    #[connector_test]
    async fn test_not_ancestor_of_multiple(runner: Runner) -> TestResult<()> {
        create_test_data(&runner).await?;

        ltree_query(
            &runner,
            "path",
            "ancestor_of",
            r#"["A.B", "C.D"]"#,
            vec![3, 4, 5, 6, 9],
            true,
        )
        .await?;

        Ok(())
    }

    #[connector_test]
    async fn test_descendent_of_single(runner: Runner) -> TestResult<()> {
        create_test_data(&runner).await?;

        ltree_query(&runner, "path", "descendent_of", r#"["C"]"#, vec![7, 8, 9], false).await?;

        Ok(())
    }

    #[connector_test]
    async fn test_not_descendent_of_single(runner: Runner) -> TestResult<()> {
        create_test_data(&runner).await?;

        ltree_query(
            &runner,
            "path",
            "descendent_of",
            r#"["C"]"#,
            vec![1, 2, 3, 4, 5, 6],
            true,
        )
        .await?;

        Ok(())
    }

    #[connector_test]
    async fn test_descendent_of_multiple(runner: Runner) -> TestResult<()> {
        create_test_data(&runner).await?;

        ltree_query(
            &runner,
            "path",
            "descendent_of",
            r#"["B", "C"]"#,
            vec![4, 5, 6, 7, 8, 9],
            false,
        )
        .await?;

        Ok(())
    }

    #[connector_test]
    async fn test_not_descendent_of_multiple(runner: Runner) -> TestResult<()> {
        create_test_data(&runner).await?;

        ltree_query(&runner, "path", "descendent_of", r#"["B", "C"]"#, vec![1, 2, 3], true).await?;

        Ok(())
    }

    #[connector_test]
    async fn test_matches_single(runner: Runner) -> TestResult<()> {
        create_test_data(&runner).await?;

        ltree_query(
            &runner,
            "path",
            "matches",
            r#"["A|B.*{,}.C{1,2}.*{,}"]"#,
            vec![3, 5, 6],
            false,
        )
        .await?;

        Ok(())
    }

    #[connector_test]
    async fn test_not_matches_single(runner: Runner) -> TestResult<()> {
        create_test_data(&runner).await?;

        ltree_query(
            &runner,
            "path",
            "matches",
            r#"["A|B.*{,}.C{1,2}.*{,}"]"#,
            vec![1, 2, 4, 7, 8, 9],
            true,
        )
        .await?;

        Ok(())
    }

    #[connector_test]
    async fn test_matches_multiple(runner: Runner) -> TestResult<()> {
        create_test_data(&runner).await?;

        ltree_query(
            &runner,
            "path",
            "matches",
            r#"["A|B.*{,}.C{1,2}.*{,}", "*{,}.E"]"#,
            vec![3, 5, 6, 9],
            false,
        )
        .await?;

        Ok(())
    }

    #[connector_test]
    async fn test_not_matches_multiple(runner: Runner) -> TestResult<()> {
        create_test_data(&runner).await?;

        ltree_query(
            &runner,
            "path",
            "matches",
            r#"["A|B.*{,}.C{1,2}.*{,}", "*{,}.E"]"#,
            vec![1, 2, 4, 7, 8],
            true,
        )
        .await?;

        Ok(())
    }

    #[connector_test]
    async fn test_matches_fulltext_single(runner: Runner) -> TestResult<()> {
        create_test_data(&runner).await?;

        ltree_query(&runner, "path", "matches_fulltext", r#""B & !C""#, vec![2, 4], false).await?;

        Ok(())
    }

    #[connector_test]
    async fn test_not_matches_fulltext_single(runner: Runner) -> TestResult<()> {
        create_test_data(&runner).await?;

        ltree_query(
            &runner,
            "path",
            "matches_fulltext",
            r#""B & !C""#,
            vec![1, 3, 5, 6, 7, 8, 9],
            true,
        )
        .await?;

        Ok(())
    }

    async fn create_test_data(runner: &Runner) -> TestResult<()> {
        create_row(runner, r#"{ id: 1, path: "A" }"#).await?;
        create_row(runner, r#"{ id: 2, path: "A.B" }"#).await?;
        create_row(runner, r#"{ id: 3, path: "A.B.C" }"#).await?;
        create_row(runner, r#"{ id: 4, path: "B" }"#).await?;
        create_row(runner, r#"{ id: 5, path: "B.C" }"#).await?;
        create_row(runner, r#"{ id: 6, path: "B.C.D" }"#).await?;
        create_row(runner, r#"{ id: 7, path: "C" }"#).await?;
        create_row(runner, r#"{ id: 8, path: "C.D" }"#).await?;
        create_row(runner, r#"{ id: 9, path: "C.D.E" }"#).await?;

        Ok(())
    }

    async fn create_row(runner: &Runner, data: &str) -> TestResult<()> {
        runner
            .query(format!("mutation {{ createOnePath(data: {}) {{ id }} }}", data))
            .await?
            .assert_success();
        Ok(())
    }
}

async fn ltree_query(
    runner: &Runner,
    field: &str,
    operation: &str,
    comparator: &str,
    expected_ids: Vec<i32>,
    reverse: bool,
) -> TestResult<()> {
    let result = match reverse {
        true => {
            runner
                .query(format!(
                    indoc::indoc! {r#"
                query {{
                    findManyPath(where: {{
                        NOT: {{ {}: {{ {}: {} }} }}
                    }}) {{
                        id
                    }}
                }}
            "#},
                    field, operation, comparator,
                ))
                .await?
        }
        false => {
            runner
                .query(format!(
                    indoc::indoc! {r#"
                query {{
                    findManyPath(where: {{
                        {}: {{ {}: {} }}
                    }}) {{
                        id
                    }}
                }}
            "#},
                    field, operation, comparator,
                ))
                .await?
        }
    };

    result.assert_success();

    if expected_ids.is_empty() {
        assert_eq!(result.to_string(), r#"{"data":{"findManyPath":[]}}"#);
    } else {
        let stringified: Vec<_> = expected_ids
            .into_iter()
            .map(|id| format!(r#"{{"id":{}}}"#, id))
            .collect();

        assert_eq!(
            result.to_string(),
            format!(r#"{{"data":{{"findManyPath":[{}]}}}}"#, stringified.join(","))
        )
    }

    Ok(())
}
