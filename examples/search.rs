use nekosbest::{
    SearchQuery, SearchQueryKind,
    client::{Client, ClientConfig},
};

#[tokio::main]
async fn main() {
    let cl = Client::new(ClientConfig::default());
    let r = nekosbest::search_with_client(
        &cl,
        SearchQuery::new("Senko", SearchQueryKind::Gif)
            .amount(2)
            .category(nekosbest::Category::Pat),
    )
    .await
    .unwrap();

    dbg!(&r.0);
}
