use calc::calculator_client::CalculatorClient;
use calc::DensityArguments;

pub mod calc
{
    tonic::include_proto!("calc");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let mut buffer = String::new();
    println!("Enter x");

    std::io::stdin().read_line(&mut buffer).expect("Not a valid string");
    let num_input:f32 = buffer.trim().parse().expect("Not a valid f32");

    let mut client = CalculatorClient::connect("http://127.0.0.1:50051").await?;

    let request = tonic::Request::new(DensityArguments {
        x: num_input.into(),
    });

    let response = client.calculate_density(request).await?;

    println!("Density={:}", response.into_inner().result);

    Ok(())
}