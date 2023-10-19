use async_graphql::*;

struct Query;

#[Object]
impl Query {
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

async fn query_gql() {
    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    let res = schema.execute("{ add(a:10, b:20)}").await;

    println!("{:?}", res);
}

fn main() {
    query_gql();
}
