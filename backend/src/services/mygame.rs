use futures::stream::TryStreamExt;
use mongodb::{
    bson::doc,
    Collection,
};

use crate::models::MyList;

pub async fn fetch_all(mygames: &Collection<MyList>) -> Result<Vec<MyList>, mongodb::error::Error> {
    let cursor = mygames
        .find(doc! {})
        .await?;
    cursor.try_collect().await
}
