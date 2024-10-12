# RC4 Cipher Application

## Lab Objective

This lab focuses on implementing the RC4 (Rivest Cipher 4) stream cipher algorithm. The objective is to create an application that can encrypt and decrypt messages using RC4 in both binary and string modes. This implementation provides a practical understanding of stream ciphers and their application in cryptography.

## Problem Statement

Implement an RC4 cipher application with the following features:

1. Binary Mode:
   - Accept plaintext and key in binary format.
   - Implement RC4 algorithm for encryption and decryption.
   - Display the ciphertext in binary format.

2. String Mode:
   - Accept plaintext and key as ASCII strings.
   - Implement RC4 algorithm for encryption and decryption.
   - Display the ciphertext in hexadecimal format.

3. GUI Implementation:
   - Create a user-friendly interface for input and output.
   - Provide options to switch between binary and string modes.
   - Display intermediate steps (KSA and PRGA) for educational purposes.

## Implementation

This application is implemented in Rust using the `eframe` library for the graphical user interface. It provides an intuitive interface for users to encrypt and decrypt messages using the RC4 algorithm in both binary and string modes.

### Code Explanation (main.rs)

1. **Application Structure**:
   - The `RC4App` struct holds the state of the application, including plaintext, key, ciphertext, mode selection, and intermediate steps.

2. **GUI Implementation**:
   - The `eframe::App` trait is implemented for `RC4App`, defining how the UI is rendered and updated.
   - The `update` function creates the layout with input fields, mode selection, buttons, and output areas.

3. **RC4 Implementation**:
   - `ksa` function implements the Key Scheduling Algorithm.
   - `prga` function implements the Pseudo-Random Generation Algorithm.
   - `encrypt` and `decrypt` functions handle the RC4 encryption and decryption process for both modes.

4. **Key Components**:
   - Mode selection checkbox for switching between binary and string modes.
   - Text input areas for plaintext and key.
   - "Encrypt" and "Decrypt" buttons to perform operations.
   - "Reset" button to clear all inputs.
   - Text areas to display ciphertext and decrypted text.
   - Collapsible sections to show KSA and PRGA steps.

### Features

- Toggle between Binary and String modes
- Input fields for plaintext and key
- Encryption and decryption functionality
- Display of ciphertext (binary or hexadecimal)
- Display of decrypted text
- Visualization of KSA and PRGA steps
- Reset functionality to clear all inputs

### Binary Mode Example
![Screenshot 2024-10-12 192710](https://github.com/user-attachments/assets/bb3a13eb-fbb9-4477-8659-bed8e74ff7f3)

![Screenshot 2024-10-12 192746](https://github.com/user-attachments/assets/798f91fc-a3b3-4c7e-b95e-b3c3548c5ee6)



### String Mode Example

![Screenshot 2024-10-12 192819](https://github.com/user-attachments/assets/b4876b44-e07c-4f83-af22-99098cc7ba6e)


## Project Structure

```
rc4_cipher_app/
├── src/
│   └── main.rs
├── app_icon.ico
├── Cargo.toml
├── build.rs
├── rc4_cipher_app.exe
└── README.md
```

## How to Run

1. Simply double-click the `rc4_cipher_app.exe` file to run the application directly.

   or

2. If running from source:
   - Ensure you have Rust installed on your system.
   - Clone this repository.
   - Navigate to the project directory.
   - Run the following command:
     ```
     cargo run
     ```

## Building from Source

To build the application from source:

1. Ensure you have Rust and Cargo installed.
2. Clone this repository.
3. Navigate to the project directory.
4. Run the following command:
   ```
   cargo build --release
   ```
5. The executable will be created in the `target/release` directory.

## Dependencies

- eframe: For creating the graphical user interface
- winres: For setting the application icon (Windows only)
