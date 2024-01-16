use calc::calculator_client::CalculatorClient;
use calc::DensityArguments;

pub mod calc
{
    tonic::include_proto!("calc");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = CalculatorClient::connect("http://127.0.0.1:50051").await?;

    let request = tonic::Request::new(DensityArguments {
        x: 100.0.into(),
    });

    let response = client.calculate_density(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}