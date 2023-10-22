use error_chain::error_chain;

error_chain! { //* Error chain macro */
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main] // * THis sets up tokio runtime for asynchornous tasks
async fn main() -> Result<()> {
    let res = reqwest::get("https://dummyjson.com/products/1").await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?; // * .text() method is asycnhronous
    println!("Body:\n{:#?}", body);

    println!("Hello Wolrd");
    Ok(())
}
