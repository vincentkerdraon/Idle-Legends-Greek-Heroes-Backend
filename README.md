# Idle Legends Greek Heroes Backend

This project is the backend for an idle game created for a game jam. The game focuses on making ancient Greek heroes more powerful until they become gods. This backend is written in Rust and interacts with the OpenAI API to generate text and images based on player and hero data.

## Project Overview

The backend web server receives player and hero data, generates prompts based on that data, and uses the OpenAI API to generate corresponding text and images. The server then returns the generated content to the client.

## Features

### Process Data
- Generate a variety of prompts based on the received data.
- Prompts will cover:
  - Greeting a new hero with background and physical features.
  - Describing a hero's feat, like building a remarkable statue.

### API Interaction
- Use the OpenAI API to generate a text response and an image based on the prompt.

### Return Data
- Respond with the generated text and image.

### Session Management
- Handle data in-session without persistent storage.

## Setup

1. **Install Rust**: Make sure you have Rust installed. You can download it from [here](https://www.rust-lang.org/tools/install).

2. **Clone the repository**: 
    ```bash
    git clone https://gitlab.com/eclypsaine/Idle-Legends-Greek-Heroes-Backend.git
    cd Idle-Legends-Greek-Heroes-Backend
    ```

3. **Install dependencies**: 
    ```bash
    cargo build
    ```

4. **Run the server**:
    ```bash
    export OPENAI_SECRET="redacted"
    export BIND="127.0.0.1:8080"
    cargo run
    ```

## Endpoints

### `/generate` (POST)

- **Description**: Accepts player and hero data, processes it, and returns the generated text and image.
- **Request**:
    ```json
    {
        "player_id": "string",
        "hero_id": "string",
        "feat_id": "string"
    }
    ```
- **Response**:
    ```json
    {
        "text": "string",
        "image_url": "string"
    }
    ```

## Author

Vincent KERDRAON

## License

This project is licensed under the Creative Commons Attribution-NonCommercial 4.0 International License.

[![License: CC BY-NC 4.0](https://img.shields.io/badge/License-CC%20BY--NC%204.0-lightgrey.svg)](https://creativecommons.org/licenses/by-nc/4.0/)
