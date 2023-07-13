<div align="center">
  <img src="https://rustacean.net/assets/rustacean-orig-noshadow.png" width="15%">
  <h1>CodeLaughs-API</h1>
  <p>
    <strong>CodeLaughs-API is an API that provides Reddit memes related to programming and software development.</strong>
  </p>
  <p>
  
![rustc](https://img.shields.io/badge/rustc-1.64+-ab6000.svg)
![MIT](https://img.shields.io/badge/license-MIT-blue)
![actix-web](https://img.shields.io/badge/framework-Actix%20Web-yellowgreen)

  </p>
</div>

# Installation
1. Clone the repository:
```
git clone https://github.com/nXoji/CodeLaughs-API.git
```
2. Create `.env` file with the following variables:
```
HOST=127.0.0.1
PORT=8080
ROOT_PASS=qwerty1234
MONGODB_URI={your mongodb url}
DATABASE_NAME=CodeLaughs-API (your databse name)
COLLECTION_NAME=tokens (your collection name)
```
3. Start the project:
```
cargo run
```
---
# Documentation
To use CodeLaughs-API, you can send HTTP requests to the API endpoints. Here are the available endpoints:
- `/ping`: Use this endpoint to check the functionality of the API.
* `/api/get_meme`: Use this endpoint to get a random meme. Please include the following headers in your request:
  * `X-API-Key`: Include your API key in this header to authenticate the request.
+ `/api/create_token/{pass}`: Use this endpoint to generate new tokens. Replace {pass} with your root password. This endpoint will create and return new tokens that can be used for authentication.
---
# License
CodeLaughs-API is open source software under the [MIT License](https://opensource.org/license/mit/).
