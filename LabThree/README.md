# Cipher Application

## Problem Statement

This application implements two cipher algorithms:

1. Additive Cipher (Caesar Cipher) for encryption:
   - Algorithm:
     - Read the plaintext from the user.
     - Read the key from the user.
     - For each character in plaintext, add the key and take mod 26.
     - Display the ciphertext.

2. Rail Fence Cipher:
   - Algorithm:
     - Read the plaintext.
     - Arrange the plaintext in a row-columnar matrix format.
     - Read the keyword depending on the number of columns of the plaintext.
     - Arrange the characters of the keyword in sorted order and the corresponding columns of the plaintext.
     - Read the characters row-wise or column-wise in the former order to get the ciphertext.

## Implementation

This application is implemented in Rust using the `eframe` library for the graphical user interface. It provides a simple and intuitive interface for users to encrypt messages using either the Additive Cipher or the Rail Fence Cipher.

### Code Explanation (main.rs)

1. **Application Structure**:
   - The `CipherApp` struct holds the state of the application, including plaintext, key, ciphertext, cipher type, and matrix for visualization.
   - The `CipherType` enum defines the two types of ciphers available.

2. **GUI Implementation**:
   - The `eframe::App` trait is implemented for `CipherApp`, defining how the UI is rendered and updated.
   - The `update` function creates the layout with input fields, buttons, and output areas.

3. **Cipher Implementations**:
   - `additive_cipher` function implements the Additive Cipher algorithm.
   - `RailFence` struct and its methods implement the Rail Fence Cipher algorithm.

4. **Key Components**:
   - Text input areas for plaintext and key.
   - Radio buttons for selecting the cipher type.
   - "Encrypt" button to perform encryption.
   - "Reset" button to clear all inputs.
   - Text area to display the ciphertext.
   - Matrix visualization for the Rail Fence Cipher.

5. **Matrix Visualization**:
   - For the Rail Fence Cipher, a grid is drawn to show how the plaintext is arranged before encryption.
   - This helps users understand the encryption process visually.

### Features

- Choice between Additive Cipher and Rail Fence Cipher
- Input fields for plaintext and key
- Encryption functionality
- Display of ciphertext
- Matrix visualization for the Rail Fence Cipher
- Reset functionality to clear all inputs

### Additive Cipher Example

![Screenshot 2024-09-26 130310](https://github.com/user-attachments/assets/8a451862-8f7e-4397-82e3-96912dbad65a)


### Rail Fence Cipher Example

![Screenshot 2024-09-26 130404](https://github.com/user-attachments/assets/14f41edc-4ac3-4ee9-a342-9fbee226be26)



## Project Structure

```
cipher_app/
├── src/
│   └── main.rs
├── app_icon.ico
├── Cargo.toml
├── build.rs
├── cipher_app.exe
└── README.md
```

## How to Run

   Simply double-click the `cipher_app.exe` file to run the application directly.

   or

1. Ensure you have Rust installed on your system.
2. Clone this repository.
3. Navigate to the project directory.
4. Run the following command:
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
