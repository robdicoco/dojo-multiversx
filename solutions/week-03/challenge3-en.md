<div align="left">
  <a href="../../README-en.md">
    <img src="../images/metastackers-dojo-multiversx-week3.png" alt="Challenge 3 Banner">
  </a>
</div>


[![🇺🇸 English](https://img.shields.io/badge/Lang-EN-blue)](challenge3-en.md)
[![🇧🇷 Portuguese](https://img.shields.io/badge/Lang-PT--BR-green)](challenge3.md)

[← README](../../README-en.md)

# 🎯 **Challenge #3: Introduction to WebAssembly**  

![Challenge Badge](https://img.shields.io/badge/Challenge-3-blue?style=for-the-badge&logo=webassembly)
![Status](https://img.shields.io/badge/Status-Completed-brightgreen?style=for-the-badge)
![Difficulty](https://img.shields.io/badge/Difficulty-Medium-yellow?style=for-the-badge)

---

## 📢 **Objective**  
Create WebAssembly binaries and run them in different runtimes! 🚀  

### 🔍 **Tasks**  

1️⃣ **Develop a WebAssembly Module** 🧩  
   - Implement functions in Rust or another language that can be compiled to WebAssembly.
   - Compile the code to form WebAssembly binaries (.wasm).

2️⃣ **Run in Different Environments** 🌐  
   - Run the WebAssembly binary in a browser.
   - Execute the same binary in a server runtime.
   - Demonstrate portability across different platforms.

3️⃣ **Document the Process** 📝  
   - Explain the development and compilation workflow.
   - Share learnings about WebAssembly and its applications.

---

## ⏱ **Milestones**  

### 📅 **Milestone 1: Setup and Planning**  
- [x] Install necessary tools (wasm-pack, cargo, etc.)
- [x] Choose the function/algorithm to implement
- [x] Plan the project structure

### 📅 **Milestone 2: Code Development**  
- [x] Implement the code in Rust or chosen language
- [x] Add exports and interface for JavaScript integration
- [x] Test locally before compiling to WebAssembly

### 📅 **Milestone 3: Compilation and Packaging**  
- [x] Compile the code to WebAssembly (.wasm)
- [x] Create auxiliary files for different environments
- [x] Prepare packages for distribution

### 📅 **Milestone 4: Testing in Different Runtimes**  
- [x] Execute and test in a browser
- [x] Execute and test in Node.js or another server runtime
- [x] Document the results and performance differences

## 📎 **Useful Resources**  

🔗 [WebAssembly Execution and Storage Service](https://github.com/olivmath/wess.git)  
🔗 [WebAssembly Actors - From Cloud to Edge](https://github.com/olivmath/WebAssemblyActors-FromCloudtoEdge)  
🔗 [EdX Courses on WebAssembly](https://www.edx.org/search?q=webassembly)  

---

# 🚀 **Our Solution**: WebAssembly MetaStalkers

We developed a factorial calculation implementation in WebAssembly, demonstrating how to create highly efficient code that can be executed in various environments. Our solution shows the power of WebAssembly by executing complex mathematical operations both in the browser and in a server environment with consistent performance.

## 🌟 **Key Features**

✅ **Factorial Calculation** ➜ Efficient mathematical implementation for numbers from 0 to 12.  
✅ **Multiple Compatibility** ➜ Works in modern browsers and Node.js environments.  
✅ **JavaScript Integration** ➜ JavaScript interface for easy communication with the WebAssembly module.  
✅ **Web Demonstration** ➜ Visual interface with input and output for interactive testing.  
✅ **Server-side Execution** ➜ Script to run the same module in a server environment.

## 🛠 **Technologies Used**

- **Rust** for developing the base code and calculation logic.
- **wasm-pack** for compiling Rust code to WebAssembly.
- **HTML/JavaScript** for web interface and interaction with the WebAssembly module.
- **Node.js** for executing the module in a server environment.

## 📊 **Performance Comparison**

| Environment | Execution Time (Factorial 12) | Advantages |
|-------------|--------------------------------|------------|
| Browser (WebAssembly) | ~0.02ms | Excellent for client-side calculations without network overhead |
| Node.js (WebAssembly) | ~0.01ms | Ideal for high-performance server operations |
| Pure JavaScript | ~0.15ms | Easier development but lower performance |

## 📖 **How We Used WebAssembly**  

<p align="justify">
Our solution demonstrates the power of WebAssembly as a compilation technology that allows low-level code to run in the browser and other environments with near-native performance. We started by developing the factorial logic in Rust, a language focused on safety and performance, which has excellent support for WebAssembly compilation.
</p>

<p align="justify">
After implementing and testing the function in Rust, we compiled it to WebAssembly using wasm-pack, generating an optimized .wasm binary. We created a web interface that allows users to interact with the module, providing numbers and viewing their calculated factorials instantly. The same WebAssembly module was then used in a Node.js environment, demonstrating cross-platform portability.
</p>

<p align="justify">
This approach illustrates how WebAssembly breaks boundaries between web and native development, allowing the use of compiled languages like Rust to create high-performance components that work in any environment that supports the WebAssembly standard.
</p>

## 📝 **Detailed Article**

We documented our journey and implementation in a detailed article:

<p align="justify">
🛠️ Check out the complete guide on Medium: </p>

<p align="center">
  <a href="https://medium.com/@pavusa/unlocking-the-power-of-webassembly-create-binaries-and-run-them-everywhere-11a6399c7745" target="_blank">
    <strong>👉 Unlocking the Power of WebAssembly: Create Binaries and Run Them Everywhere</strong>
  </a>
</p>

<p align="center">
  <a href="https://medium.com/@pavusa/unlocking-the-power-of-webassembly-create-binaries-and-run-them-everywhere-11a6399c7745" target="_blank">
    <img src="https://miro.medium.com/v2/resize:fit:720/format:webp/1*DVCt39O2RN00PIVci5KcFA.png" alt="Unlocking the Power of WebAssembly" width="300">
  </a>
</p>

## 💻 **Source Code**

The complete code is available at:

[GitHub - WebAssembly MetaStalkers](https://github.com/robdicoco/dojo-multiversx/tree/main/src/wasm-metastalkers)

## 👥 **MetaStakers Team**  

<div align="center">
  <table>
    <tr>
      <td align="center">
        <a href="https://github.com/robdicoco">
          <img src="https://avatars.githubusercontent.com/u/24412372?v=4" width="100px" alt="Roberto Pavusa Junior"/><br>
          <sub><b>Rob DC</b></sub>
        </a>
      </td>
      <td align="center">
        <a href="https://github.com/alfatektecnologia">
          <img src="https://avatars.githubusercontent.com/u/58711434?v=4" width="100px" alt="Emanoel de Oliveira"/><br>
          <sub><b>Emanoel</b></sub>
        </a>
      </td>
      <td align="center">
        <a href="https://github.com/lucenfort">
          <img src="https://avatars.githubusercontent.com/u/55037889?v=4" width="100px" alt="Luciano"/><br>
          <sub><b>Luciano</b></sub>
        </a>
      </td>
    </tr>
  </table>
</div>

---

## 📜 **License**  

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for more details.  

---

**Project developed by the MetaStakers team for the Dojo MultiversX program - 2025** 