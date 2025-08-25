# Discord Clone Project

This is a project aiming to create a clone of the popular communication platform, Discord. The project is currently in the development phase, focusing on core features. My goal is to rebuild the fundamental functionality of Discord using modern web technologies.

**Important Note:** The project does not yet have WebSocket support. Therefore, features like instant messaging and real-time updates are not active. These features are planned for the next phase of the development roadmap.

## 🚀 Current Features

The following features are partially or fully completed in the current version of the project:

- **User Authentication:** Sign-up and login functionality.
- **Server List:** Listing the servers a user has joined.
- **Channel Management:** Displaying text channels within a server.
- **Messaging Interface:** Basic UI for sending and viewing messages (not real-time).

## 🗺️ Roadmap (Future Features)

The main features planned for future development include:

- [ ] **WebSocket Integration:** Real-time messaging and notifications.
- [ ] **Voice Channels:** Voice chat functionality via WebRTC.
- [ ] **Direct Messages:** One-on-one messaging between users.
- [ ] **User Profiles:** Profile pictures, status, and user settings.
- [ ] **Invite System:** Inviting users to servers.
- [ ] **Role & Permission Management:** Server-side authorization.

## ⚙️ Setup and Launch

To run the project on your local machine, follow these steps:

1.  **Clone the project:**

    ```bash
    git clone https://github.com/erendirek/inert-server.git
    ```

2.  **Navigate to the project directory:**

    ```bash
    cd inert-server
    ```

3.  **Build the project and install dependencies:**
    Cargo will automatically handle the dependencies listed in `Cargo.toml`.

    ```bash
    cargo build
    ```

4.  **Set up environment variables:**
    Copy the `.env.example` file to a new file named `.env` and enter your own database and other configuration details.

5.  **Run the application:**
    ```bash
    cargo run
    ```

The application will run by default at `http://localhost:8080` (or the port specified in your configuration).

## 🤝 Contributing

This project is open for contributions. If you would like to contribute, please open an issue to state your ideas or submit a pull request. We value all contributions and feedback!
