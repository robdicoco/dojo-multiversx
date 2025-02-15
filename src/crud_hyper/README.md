# CRUD Hyper

A simple CRUD (Create, Read, Update, Delete) web application built using Rust's `hyper` library. This project demonstrates how to implement basic CRUD operations with an in-memory database and serves as a learning resource for Rust web development.

## Features

-   **Create**: Add new items to the database.
-   **Read**: Retrieve all items from the database.
-   **Update**: Modify the name of an existing item.
-   **Delete**: Remove an item from the database.
-   **In-Memory Storage**: Uses a thread-safe `HashMap` wrapped in an `Arc<Mutex>` for storage.
-   **Asynchronous**: Built with `tokio` and `hyper` for asynchronous request handling.

## Usage

### Prerequisites

-   Rust 1.60 or higher
-   Cargo (Rust's package manager)

### Installation

Clone the repository and build the project:

```bash
git clone https://github.com/yourusername/crud_hyper.git
cd crud_hyper
cargo build
```

---

## <a id="license"></a>📜 License

This project is licensed under the **MIT License**. Check the [LICENSE](./LICENSE) file for more details.

---

## Dojo MultiversX - Challenge 02

-   Rust Server
    -   Challenge: Create a CRUD (Create, Read, Update, Delete) to manage records .

<p align="left">🌟 Developed during the <strong>Dojo MultiversX</strong> program</p>

<p align="left">
  <img src="img/logometa.png" alt="Dojo MultiversX logo" width="200">
</p>

## <a id="participants"></a>👥 Participants

The **MetaStakers** team is composed of the following members:

<div align="left">
  <table>
    <tr>
      <td align="center">
        <a href="https://github.com/robdicoco">
          <img src="https://avatars.githubusercontent.com/u/24412372?v=4" width="100px" alt="Roberto Pavusa Junior's GitHub photo"/><br>
          <sub><b>robdicoco</b></sub>
        </a>
      </td>
      <td align="center">
        <a href="https://github.com/alfatektecnologia">
          <img src="https://avatars.githubusercontent.com/u/58711434?v=4" width="100px" alt="Emanoel de Oliveira's GitHub photo"/><br>
          <sub><b>alfatektecnologia</b></sub>
        </a>
      </td>
      <td align="center">
        <a href="https://github.com/lucenfort">
          <img src="https://avatars.githubusercontent.com/u/55037889?v=4" width="100px" alt="Luciano's GitHub photo"/><br>
          <sub><b>lucenfort</b></sub>
        </a>
      </td>
    </tr>
  </table>
</div>
