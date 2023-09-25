
#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn it_works() {
        let client_id = "";
        let client_secret = "";
        StreamWaveClient::from_app(&client_id, &client_secret)
    }
}
