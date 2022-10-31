use poem::{
    Endpoint, Request, Result, Response, Error, http::StatusCode, Body
};

#[derive(Debug)]
pub struct ProxyEndpoint {
    target: String,
}

impl ProxyEndpoint {
    pub fn new( target: String ) -> Self {
        Self { target: target }
    }
}

// async fn serve_target( req: &Request ) -> Result<Response> {
//     Ok(

//     )
// }

#[async_trait::async_trait]
impl Endpoint for ProxyEndpoint {
    type Output = Response;

    async fn call( &self, req: Request ) -> Result<Self::Output> {

        let path = req
            .uri()
            .path()
            .trim_start_matches( (self.target.clone()+"/").as_str() )
            .trim_end_matches( "/" );
        
        let mut request_path = self.target.clone() + path.clone();
        println!( "Received request: {}", request_path );

        let client = reqwest::Client::new();
        let res = client.post( request_path )
            .headers( req.headers().clone() )
            .body( "poopy" )
            .send()
            .await;

        match res {
            Ok( result ) => {

                println!("Response: {:?}", result );

                let mut j = Response::default();
                j.set_status( result.status() );
                j.set_body( result.text().await.unwrap() );
                    // status: result.status(),
                    // headers: result.headers().clone(),
                    // body: Body::from_string( result.text().await.unwrap() ),
                    // ..Default::default()
                    // j.head
                    
                Ok( j )
            },
            Err( error ) => {
                Err( Error::from_string( error.to_string(), error.status().unwrap_or( StatusCode::EXPECTATION_FAILED ) ) )
            }
        }

        // return Err( Error::from_string( "Error!".to_owned(), StatusCode::NOT_IMPLEMENTED ) );
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
