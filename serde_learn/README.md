source: https://www.youtube.com/watch?v=YLZtw8_aLwA&list=PLDbRgZ0OOEpUkWDGqp91ODn0dk7LPBAUL

This video provides an introduction to the Serde crate in Rust for handling JSON data.  

Here's a list of the content covered:  

* Introduction to Serde: Explains what Serde is (a Rust crate) and its purpose (serialization and deserialization).
* Serialization: How to convert Rust data structures (structs) into JSON format.
* Deserialization: How to convert JSON text back into Rust data structures.
* Serde and Serde JSON Crates: Discusses the two main crates needed for JSON operations and how to add them to your project with Cargo.
* Deriving Serialize and Deserialize: Shows how to use attributes to make custom Rust structs serializable and deserializable.
* Customizing Serialization: Explores options for renaming fields (e.g., camelCase, PascalCase) and other container attributes.
* Nested Data Structures: Demonstrates how Serde handles structs that contain other structs.
* Error Handling: Explains how Serde returns Result types for serialization and deserialization, allowing for graceful error handling.
* Denying Unknown Fields: How to configure Serde to reject JSON input that contains fields not defined in the Rust struct.
* Field-Level Attributes: Briefly touches on applying attributes to individual struct fields.