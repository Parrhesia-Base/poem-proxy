use std::sync::Arc;

use futures_util::{SinkExt, StreamExt};
use poem::{
    Request, Result, Response, Error, http::{ StatusCode, Method }, handler, web::{Data, websocket::{WebSocket, Message}}, Body, FromRequest, IntoResponse
};
use tokio_tungstenite::connect_async;

#[handler]
pub async fn proxy( 
    req: &Request, 
    target: Data<&String>, 
    method: Method,
    body: Body,
    ) -> Result<Response>
{
    // Have to clone this here for some reason
    let perm_target = target.clone();

    // If we need a websocket connection,
    if let Ok( ws ) = WebSocket::from_request_without_body( req ).await {

        return Ok( 
            ws.on_upgrade(move |socket| async move {
                let ( mut clientsink, mut clientstream ) = socket.split();
                
                // Start connection to server
                // let ( mut serversocket, _ ) = connect_async( perm_target.clone() ).await.unwrap();
                // let ( mut serversink, mut serverstream ) = serversocket.split();

                // Relay client messages to the server we are proxying
                tokio::spawn( async move {
                    while let Some( Ok( msg ) ) = clientstream.next().await {

                        // When a message is received,
                        // if let Message::Text( text ) = &msg {

                            // Echo it:
                            // println!( "Websocket client sent: '{}'", text );

                            // Send it right back!
                            // if clientsink.send( Message::Text( format!("{}", text ) ) ).await.is_err() {
                            //     break;
                            // }

                            // Forward it to the server
                            // serversink.send(
                            //     msg.into()
                            // ).await.unwrap();
                        // }



                        // call( &msg ).await;
                    }
                });
                
                // Relay server messages to the client
                // tokio::spawn( async move {
                //     while let Some( Ok( msg ) ) = serverstream.next().await {
                //         // if let tokio_tungstenite::tungstenite::Message::Text( text ) = &msg {
                //             clientsink.send(
                //                 msg.into()
                //             ).await.unwrap();
                //         // }
                //     }
                // })
            }).into_response()
        );
    } else {
        println!( "No websocket here!" );

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
                    .body( body.into_bytes().await.unwrap() )
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
