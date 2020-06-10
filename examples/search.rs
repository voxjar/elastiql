use elasticsearch::{Elasticsearch, SearchParts};
use elastiql::search::{
    query::{BooleanQuery, CompoundQuery, Query, TermQuery},
    HighlightOptions, Request, Response,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    id: String,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Elasticsearch::default();

    let query = TermQuery::new("id", "test_user_id");
    // which, when passed to `Request.builder().query()`, is short for:
    let query = CompoundQuery::builder()
        .boolean(
            BooleanQuery::builder()
                .filter(vec![Query::builder().term(query).build()])
                .build(),
        )
        .build();

    let request = Request::builder()
        .query(query)
        .size(1)
        .version(true)
        .highlight(HighlightOptions::default())
        .build();

    let response = client
        .search(SearchParts::None)
        .body(request)
        .allow_no_indices(true)
        .send()
        .await?
        .error_for_status_code()?;

    let search_response: Response<User> = response.json().await?;

    match search_response {
        Response::Err { error, .. } => Err(error.ty.into()),
        Response::Ok(results) => {
            for hit in results.hits.hits.iter() {
                println!("{:#?}", &hit);

                let user: &User = &hit.source;
                println!("{:#?}", &user);
            }

            Ok(())
        }
    }
}
