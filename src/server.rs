use std::f32::consts::PI;
use std::fmt::Error;
use std::os::unix::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};

use calc::{calculator_server};
use calc::{DensityArguments, DensityResponse};

pub mod calc{
    tonic::include_proto!("calc");
}

#[derive (Debug,Default)]
pub struct CalculatorService
{

}

#[tonic::async_trait]
impl calculator_server::Calculator for CalculatorService
{
    async fn calculate_density(&self, request: Request<DensityArguments>)
    ->Result<Response<DensityResponse>, Status>
    {
        let req = request.into_inner();
        
        let result = (-req.x.powi(2)).exp() / f32::sqrt(2.0*PI);

        let reply = DensityResponse
        {
            result:result.into(),
        };

        Ok(Response::new(reply))
        
    }
}
#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {

    let addres:SocketAddr = "127.0.0.1:50051".parse()?;


    Ok(())

}
