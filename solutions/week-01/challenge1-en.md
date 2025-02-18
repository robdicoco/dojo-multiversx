<div align="left">
  <a href="../../README-en.md">
    <img src="../images/dojo_multiversx_banner.gif" alt="Challenge 1 Banner">
  </a>
</div>

[![🇺🇸 English](https://img.shields.io/badge/Lang-EN-blue)](challenge1-en.md)
[![🇧🇷 Portuguese](https://img.shields.io/badge/Lang-PT--BR-green)](challenge1.md)

[← README](../../README-en.md)

# 🎯 **Challenge #1: Installing Rust & First Impressions**  

![Challenge Badge](https://img.shields.io/badge/Challenge-1-blue?style=for-the-badge&logo=rust)
![Status](https://img.shields.io/badge/Status-Completed-brightgreen?style=for-the-badge)
![Difficulty](https://img.shields.io/badge/Difficulty-Easy-brightgreen?style=for-the-badge)

---

## 📢 **Objective**  
Complete the following tasks and share your experience! 🚀  

### 🔍 **Tasks**  

1️⃣ **Install Rust** 🦀  
   - Ensure Rust is correctly installed on your system.  
   - Verify by running `rustc --version` in the terminal.  

2️⃣ **Install xPortal (Wallet)** 💳  
   - Set up the **xPortal** wallet to explore blockchain functionality.  

3️⃣ **Social Media Post** 📝  
   - Publish a post on **LinkedIn** or **Twitter/X** sharing your impressions about the live session.  
   - Include what you learned and how your experience was! 🎉 

##### 📢 **Published Posts:**  

- **[Luciano - LinkedIn Post](https://www.linkedin.com/posts/lucenfort_ia-e-blockchain-v%C3%A3o-transformar-tudo-multiversx-activity-7294824804518359040-n1ts?utm_source=share&utm_medium=member_desktop&rcm=ACoAAElTUVMBgVSjIpZeL4ccPeYlJBCsVaw44hU)**  
- **[Emanoel - LinkedIn Post](https://www.linkedin.com/posts/emanoel-oliveira-br_dojostellar-nearx-stellar-activity-7295447426939678722-EIPC?utm_source=share&utm_medium=member_desktop&rcm=ACoAAElTUVMBgVSjIpZeL4ccPeYlJBCsVaw44hU)**  

---

## ⏱ **Milestones**  

### 📅 **Milestone 1: Setting Up Rust**  
- [x] Install Rust  
- [x] Verify installation with `rustc --version`  

### 📅 **Milestone 2: Setting Up xPortal Wallet**  
- [x] Download and install the **xPortal** wallet  
- [x] Confirm wallet setup  

### 📅 **Milestone 3: Social Media Post**  
- [x] Write a post about your first experience  
- [x] Publish it on **LinkedIn** or **Twitter/X**  

## 📎 **Useful Resources**  

🔗 [Official Rust Installation Guide](https://www.rust-lang.org/tools/install)  
🔗 [Download xPortal Wallet](https://www.xportal.com)  

### 📖 **Installation Guide: Rust and xPortal Wallet for MULTIVERSX Development**  

🛠️ Check out the complete guide on Medium:  

<p align="center">
  <a href="https://medium.com/@pavusa/how-to-install-rust-and-xportal-wallet-for-multiversx-development-e3bec1fae898" target="_blank">
    <strong>👉 How to Install Rust and xPortal Wallet for MULTIVERSX Development</strong>
  </a>
</p>

<p align="center">
  <a href="https://medium.com/@pavusa/how-to-install-rust-and-xportal-wallet-for-multiversx-development-e3bec1fae898" target="_blank">
    <img src="../images/post_medium_challenge1.png" alt="How to Install Rust and xPortal Wallet for MULTIVERSX Development" width="300">
  </a>
</p>

---

# 🎯 **Challenge #2: Creating a CRUD with Hyper**  

![Challenge Badge](https://img.shields.io/badge/Challenge-2-blue?style=for-the-badge&logo=rust)
![Status](https://img.shields.io/badge/Status-Completed-brightgreen?style=for-the-badge)
![Difficulty](https://img.shields.io/badge/Difficulty-Medium-yellow?style=for-the-badge)

### 📢 **Objective**  
Create a simple CRUD using **Hyper** to manage records, such as people and books.  
The object must have **at least two different data types** (e.g., name and age of a person).  

## 🚀 **Our Solution**: CRUD Hyper  

🔗 Available at: [CRUD Hyper - Crates.io](https://crates.io/crates/crud_hyper)  

CRUD Hyper is a web application that implements **Create, Read, Update, Delete** (CRUD) operations using **Rust's Hyper** library. This project demonstrates how to build a basic CRUD system with an in-memory database and serves as a learning resource for Rust web development.  

### 🌟 **Features**  

✅ **Create** ➜ Add new items to the database.  
✅ **Read** ➜ Retrieve all stored items.  
✅ **Update** ➜ Modify an existing item.  
✅ **Delete** ➜ Remove an item from the database.  
✅ **In-Memory Storage** ➜ Uses `Arc<Mutex<HashMap>>` for thread-safe storage.  
✅ **Asynchronous** ➜ Built with **Tokio** and **Hyper** for high-performance request handling.  

### 🛠 **Requirements**  

Before installing the project, ensure you have:  

- **Rust 1.60+** installed ([Install Rust](https://www.rust-lang.org/))  
- **Cargo**, Rust’s package manager  

### 📦 **Installation & Usage**  

Clone the repository and build the project:  

```sh
git clone https://github.com/yourusername/crud_hyper.git
cd crud_hyper
cargo build
```

To run the server locally:  

```sh
cargo run
```

The server will start on port **3000** by default.  

### 📡 **API Endpoints**  

#### 🔹 Create a new item  

**`POST /items`**  
- **Input:**  
  ```json
  { "name": "Alice", "age": 25 }
  ```  
- **Output:** JSON with the new item's ID  

#### 🔹 Retrieve all items  

**`GET /items`**  
- **Output:** JSON list of stored items  

#### 🔹 Update an item  

**`PUT /items/{id}`**  
- **Input:**  
  ```json
  { "name": "Carlos", "age": 30 }
  ```  
- **Output:** Updated item  

#### 🔹 Delete an item  

**`DELETE /items/{id}`**  
- **Output:** Confirmation of deletion  

---

### 👥 **MetaStakers Team**  

- [@robdicoco](https://github.com/robdicoco)  
- [@alfatektecnologia](https://github.com/alfatektecnologia)  
- [@lucenfort](https://github.com/lucenfort)  

---

## 📜 **License**  

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for more details.  

---

🔹 *Developed with 💙 and Rust for the Dojo MultiversX* 🔹  
