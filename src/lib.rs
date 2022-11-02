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
        println!( "Request headers: {:?}", req.headers().clone() );
        let res = client.get( request_path )
            .headers( req.headers().clone() )
            .body( "poopy" )
            .send()
            .await;

        match res {
            Ok( result ) => {


                let mut j = Response::default();
                
                // j.set_status( result.status() );
                // j.set_body( result.text().await.unwrap() );
                // j.set_body( result.text().await.unwrap() );
                // let headers = j.headers_mut();

                // *headers = result.headers().clone();
                
                // println!("Response: {:?}", &result.text().await );
                    // status: result.status(),
                    // headers: result.headers().clone(),
                    // body: Body::from_string( result.text().await.unwrap() ),
                    // ..Default::default()
                    // j.head
                // let l = result.extensions().clone_into( j.extensions_mut().get() );
                j.extensions().clone_from( &result.extensions() );
                // println!("Result extensions: {:?}", result.extensions() );
                // j.headers().clone_from( &result.headers() );
                // j.headers().extend( result.headers().get_all(key) );
                result.headers().iter().for_each(|(key, val)| {
                    j.headers_mut().insert( key, val.to_owned() );
                });

                j.set_status( result.status() );
                j.set_version( result.version() );

                j.set_body( result.bytes().await.unwrap() );

                println!("j headers: {:?}", j.headers() );
                // println!("Result status: {:?}", result.status() );
                // println!("Result body: {:?}", result.bytes().await );
                // println!("Result headers: {:?}", result );
                    
                Ok( j )
            },
            Err( error ) => {
                println!( "ERROR!" );
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
