use futures_util::{SinkExt, StreamExt};
use poem::{
    Request, Result, Response, Error, http::{ StatusCode, Method, HeaderMap }, handler, web::{Data, websocket::{WebSocket}}, Body, FromRequest, IntoResponse
};
use tokio_tungstenite::connect_async;

// The websocket-enabled proxy handler
#[handler]
pub async fn proxy( 
    req: &Request, 
    headers: &HeaderMap,
    target: Data<&String>, 
    method: Method,
    body: Body,
    ) -> Result<Response>
{
    // If we need a websocket connection,
    if let Ok( ws ) = WebSocket::from_request_without_body( req ).await {

        // Update to using websocket target
        let perm_target = target.clone().replace( "https", "wss" ).replace( "http", "ws" );
        
        // Generate websocket request:
        let mut w_request = http::Request::builder().uri( &perm_target );
        for (key, value) in headers.iter() {
            w_request = w_request.header( key, value ); 
        }

        // Start the websocket connection
        return Ok( 
            ws.on_upgrade(move |socket| async move {
                let ( mut clientsink, mut clientstream ) = socket.split();
                
                // Start connection to server
                // println!( "Starting connection to {}", perm_target );
                let ( mut serversocket, _ ) = connect_async( w_request.body(()).unwrap() ).await.unwrap();
                let ( mut serversink, mut serverstream ) = serversocket.split();

                // Relay client messages to the server we are proxying
                tokio::spawn( async move {
                    // println!( "Client thread spawned" );
                    while let Some( Ok( msg ) ) = clientstream.next().await {

                        // When a message is received, forward it to the server
                        // println!( "Received client message!" );
                        serversink.send(
                            msg.into()
                        ).await.unwrap();
                    };
                });
                
                // Relay server messages to the client
                tokio::spawn( async move {
                    // println!( "Server thread spawned!" );
                    while let Some( Ok( msg ) ) = serverstream.next().await {
                        // println!( "Received server message!" );
                        clientsink.send(
                            msg.into()
                        ).await.unwrap();
                    }
                });
            }).into_response()
        );
    } 
    
    // Not using websocket:
    else {
        
        // Update the uri to point to the proxied server
        let request_uri = target.to_owned() + &req.uri().to_string();

        // Now generate a request for the proxied server, based on information
        // that we have from the current request
        let client = reqwest::Client::new();
        let res = match method {
            Method::GET => {
                client.get( request_uri )
                    .headers( req.headers().clone() )
                    .body( body.into_bytes().await.unwrap() )
                    .send()
                    .await
            },
            _ => {
                todo!();
                // return Err( Error::from_string( "Unknown method!", StatusCode::EXPECTATION_FAILED ) )
            }
        };

        // Check on the response and forward everything from the server to our client,
        // including headers and the body of the response, among other things.
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
                Ok( j )
            },

            // The request to the back-end server failed. Why?
            Err( error ) => {
                Err( Error::from_string( error.to_string(), error.status().unwrap_or( StatusCode::EXPECTATION_FAILED ) ) )
            }
        }
    }
}

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
