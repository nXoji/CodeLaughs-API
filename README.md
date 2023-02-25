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

# Documentation
To use CodeLaughs-API, you can send HTTP requests to the API endpoints. Here are the available endpoints:
- `/ping`: to check the functionality
* `/api/get_meme`: to get random meme

# Installation
1. Clone the repository:
```
git clone https://github.com/nXoji/CodeLaughs-API.git
```
2. Create `.env` file with the following variables:
```
HOST=127.0.0.1
PORT=8080
```
3. Start the project:
```
cargo run
```

# License
CodeLaughs-API is open source software under the [MIT License](https://opensource.org/license/mit/).
