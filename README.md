# Idle Legends Greek Heroes Backend

This project is the backend for an idle game created for a game jam. The game focuses on making heroes more powerful until they become gods. This backend is written in Rust and interacts with the OpenAI API to generate text and images based on player and hero data.

## Project Overview

The backend web server receives player and hero data, generates prompts based on that data, and uses the OpenAI API to generate corresponding text and images. The server then returns the generated content to the client.

## Requirements

### Receive Data
- `PlayerID` (string)
- `PlayerFeats` (array of strings)
- `HeroID` (string)
- `HeroFeats` (array of strings)
- `NewCreation` (boolean)

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
    cargo run
    ```

## Endpoints

### `/generate` (POST)

- **Description**: Accepts player and hero data, processes it, and returns the generated text and image.
- **Request**:
    ```json
    {
        "PlayerID": "string",
        "PlayerFeats": ["string", "string"],
        "HeroID": "string",
        "HeroFeats": ["string", "string"],
        "NewCreation": true
    }
    ```
- **Response**:
    ```json
    {
        "text": "string",
        "image_url": "string"
    }
    ```

## Considerations

1. **Prompt Variety**: Ensure prompts are diverse and cover different scenarios for hero creation and feats.
2. **Error Handling**: Implement robust error handling, especially for API calls.
3. **Rate Limiting**: Add basic rate limiting once the basics are functional to prevent API spamming.
4. **Scalability**: Ensure the server can handle multiple concurrent requests efficiently.

## Next Steps

1. **Define Endpoints**:
   - `/generate` (POST): Accepts player and hero data, processes it, and returns the text and image.

2. **Basic Server Setup**:
   - Initialize a new Rust project with `actix-web`.
   - Define the `/generate` endpoint.

3. **Prompt Generation Logic**:
   - Implement the logic to generate prompts based on the received data.

4. **API Integration**:
   - Set up the OpenAI API interaction.
   - Handle responses from the API.

5. **Response Handling**:
   - Structure the server response to include both the text and image.

Once the basics are in place, we can iterate on improvements like rate limiting and more sophisticated prompt generation.

## Author

Vincent KERDRAON

## License

This project is licensed under the Creative Commons Attribution-NonCommercial 4.0 International License.

[![License: CC BY-NC 4.0](https://img.shields.io/badge/License-CC%20BY--NC%204.0-lightgrey.svg)](https://creativecommons.org/licenses/by-nc/4.0/)
