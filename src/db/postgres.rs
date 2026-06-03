use anyhow::Result;

use sqlx::{
    Pool,
    Postgres,
    Row,
};

use crate::tree::node::{
    Node,
    NodeType,
};

pub async fn load_schema(
    database_url: &str,
) -> Result<Node> {

    let pool =
        Pool::<Postgres>::connect(
            database_url
        )
        .await?;

    /*
        ROOT DATABASE NODE
    */

    let mut database =
        Node::new(
            "database",
            NodeType::Database,
            None,
        );

    /*
        LOAD TABLES + COLUMNS
    */

    let rows =
        sqlx::query(
            r#"
            SELECT
                table_name,
                column_name,
                data_type
            FROM information_schema.columns
            WHERE table_schema = 'public'
            ORDER BY table_name, ordinal_position
            "#
        )
        .fetch_all(&pool)
        .await?;

    for row in rows {

        let table_name: String =
            row.get("table_name");

        let column_name: String =
            row.get("column_name");

        let data_type: String =
            row.get("data_type");

        /*
            FIND TABLE
        */

        let table_index =
            database
                .children
                .iter()
                .position(|t| {
                    t.name == table_name
                });

        match table_index {

            Some(index) => {

                database.children[index]
                    .children
                    .push(
                        Node::new(
                            &column_name,
                            NodeType::Column,
                            Some(&data_type),
                        )
                    );
            }

            None => {

                let mut table =
                    Node::new(
                        &table_name,
                        NodeType::Table,
                        None,
                    );

                table.add_child(
                    Node::new(
                        &column_name,
                        NodeType::Column,
                        Some(&data_type),
                    )
                );

                database.add_child(table);
            }
        }
    }

    Ok(database)
}