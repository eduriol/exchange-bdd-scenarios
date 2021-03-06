# Exchange BDD Scenarios
A set of executable Gherkin scenarios written in pure Rust to test the Kraken Exchange API.  
## Requirements
- [Rust](https://www.rust-lang.org/tools/install) 1.54+
- Have a valid Kraken account with 2FA enabled for API requests.
## Run the scenarios
As usual, first step is cloning the repo :)
```
git clone https://github.com/eduriol/exchange-bdd-scenarios.git
cd exchange-bdd-scenarios
```
### Local execution
For security reasons, the API key and API secret are not stored anywhere. You need to set them beforehand as environment variables in order to successfully execute the scenarios:
```
export API_KEY=<API_KEY>
export API_SECRET=<API_SECRET>
```
Once this is done you can run the scenarios by simply running the following command (prefixed by the One Time Password needed for 2FA):
```
OTP=<One Time Password> cargo test
```
### Docker execution
#### Build Docker image
```
docker build --tag="exchange-bdd-scenarios:latest" .
```
#### Run Docker image
In the case of Docker execution, you need to pass the environment variables within the run command:
```
docker run --env OTP=<One Time Password> --env API_KEY=<API_KEY> --env API_SECRET=<API_SECRET> -it exchange-bdd-scenarios:latest
```
### Reports
The report showing the result of the scenarios execution can be seen in the standard output:

<img width="617" alt="Screenshot 2021-10-27 at 12 22 39" src="https://user-images.githubusercontent.com/5454201/139048082-d7f84507-0cd5-4e6f-8189-dc1b11944643.png">

### Contribution
If you'd like to contribute to the project, please send a [Pull Request](https://docs.github.com/en/github/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/creating-a-pull-request) or contact eduriol [at] gmail.com.
#### Code quality
As part of your contribution, it is expected that you check the standardization of your code with [rustfmt](https://github.com/rust-lang/rustfmt) prior to sending your changes.
