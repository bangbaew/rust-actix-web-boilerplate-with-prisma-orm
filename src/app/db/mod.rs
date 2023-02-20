pub mod prisma;
use self::prisma::PrismaClient;

pub async fn get_prisma_client() -> PrismaClient {

    let client = prisma::new_client().await;
    if let Err(e) = client {
        panic!("{}", e)
    }

    client.unwrap()
}