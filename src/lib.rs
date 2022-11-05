use poem::{
    Request, Result, Response, Error, http::{ StatusCode, Method }, handler, web::Data, Body
};

#[handler]
pub async fn proxy( 
    req: &Request, 
    target: Data<&String>, 
    method: Method,
    body: Body,
    ) -> Result<Response>
{
    let target = target.to_owned();

    let path = req
        .uri()
        .path()
        .trim_start_matches( (target.clone()+"/").as_str() )
        .trim_end_matches( "/" );
    
    let mut request_path = target.clone() + path.clone();
    println!( "Received request: {}", request_path );

    let client = reqwest::Client::new();
    println!( "Request headers: {:?}", req.headers().clone() );

    let res = match method {
        Method::GET => {
            client.get( request_path )
                .headers( req.headers().clone() )
                .body( "body.into_bytes().await.unwrap()" )
                .send()
                .await
        },
        _ => {
            println!( "Unknown method: {}", method );
            return Err( Error::from_string( "Unknown method!", StatusCode::EXPECTATION_FAILED ) )
        }
    };

    match res {
        Ok( result ) => {
            let mut j = Response::default();
            j.extensions().clone_from( &result.extensions() );
            result.headers().iter().for_each(|(key, val)| {
                j.headers_mut().insert( key, val.to_owned() );
            });
            j.set_status( result.status() );
            j.set_version( result.version() );
            j.set_body( result.bytes().await.unwrap() );
            println!("j headers: {:?}", j.headers() );
            Ok( j )
        },
        Err( error ) => {
            println!( "ERROR!" );
            Err( Error::from_string( error.to_string(), error.status().unwrap_or( StatusCode::EXPECTATION_FAILED ) ) )
        }
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
