# OpenFoodFacts Viewer

A lightweight desktop application built with Rust and the egui library to explore food product data from the OpenFoodFacts API. Users can search for products by name and view details such as ingredients in a simple, intuitive GUI.

## Features

Product Search: Search for food products using keywords (e.g., "chocolate", "bread").
Detailed View: Display product details, including name and ingredients, with a single click.
Debug Logging: Log API requests and responses for troubleshooting.
Cross-Platform: Runs on Windows, macOS, and Linux via eframe.

## Screenshots

Search Results
    Search Screenshot (Add a screenshot here)  
Product Details
    Details Screenshot (Add a screenshot here)  

## Prerequisites

- Rust: Version 1.70 or higher (install via rustup).
- Git: For cloning the repository.

## Installation

Clone the Repository:

```bash
git clone https://github.com/yourusername/openfoodfacts-viewer.git
cd openfoodfacts-viewer
```

### Install Dependencies

Ensure your Cargo.toml contains:  

```toml
    [dependencies]
    eframe = "0.23.0"
    reqwest = { version = "0.11", features = ["json", "blocking"] }
    serde = { version = "1.0", features = ["derive"] }
    serde_json = "1.0"
    log = "0.4"
    env_logger = "0.9"
    winapi = { version = "0.3", features = ["winuser", "windef"] } # Windows only
```

Build the Project:

```bash
cargo build --release
```

## Usage

Run the Application:

- With debug logging:  

```bash
RUST_LOG=debug cargo run --release
```  

- Without debug logging:  

```bash
cargo run --release
```  

### Search for Products

Type a search term (e.g., "milk") in the search bar and click "Search".
Browse the list of products returned from the OpenFoodFacts API.

### View Details

Click a product name to see its details.
Use the "Back" button to return to the search results.

## Debugging
Check the console for logs when running with RUST_LOG=debug. Example output:

```log
DEBUG: Requesting search results from: https://world.openfoodfacts.org/cgi/search.pl?search_terms=milk&search_simple=1&json=1
DEBUG: Received response with status: 200 OK
INFO: Successfully parsed search results.
```

## Project Structure

```log
openfoodfacts-viewer/
├── Cargo.toml         # Project configuration and dependencies
├── src/
│   └── main.rs       # Application source code
├── screenshots/      # Placeholder for screenshots (optional)
└── README.md         # Project documentation
```

## API Usage

This project queries the OpenFoodFacts API:

- Search Endpoint: <https://world.openfoodfacts.org/cgi/search.pl>
- Details Endpoint: <https://world.openfoodfacts.org/api/v0/product/{code}.json>  

Fields like product_name and ingredients_text are optional to handle inconsistent API responses.

## Known Limitations

- Missing Data: Some products may lack product_name or other fields; fallbacks (e.g., "Unnamed Product") are used.  
- Basic Details: Only displays name and ingredients currently.  
- No Pagination: Search results are not paginated yet.  

## Contributing

### Contributions are welcome! Here's how to get involved

Fork the Repository: Click "Fork" on GitHub.
Clone Your Fork:  

```bash
git clone https://github.com/yourusername/openfoodfacts-viewer.git  
```  

Create a Branch:  

```bash
git checkout -b feature/your-feature-name
```  

Make Changes: Implement your feature or fix.
Commit and Push:  

```bash
git commit -m "Add your feature description"  
git push origin feature/your-feature-name  
```  

Submit a Pull Request: Open a PR on GitHub with a clear description.
Please follow Rust coding conventions and include tests if applicable.

## Future Enhancements

Add pagination for search results.
Display more product details (e.g., nutrition facts, allergens).
Support WebAssembly for browser deployment.
Improve UI with filters and styling.

## Troubleshooting

### Build Errors

On Windows, ensure winapi is included with winuser and windef features.
Run cargo update if dependency versions conflict.

### API Errors

Check debug logs (RUST_LOG=debug) for raw JSON responses if parsing fails.
Verify internet connectivity.

## Acknowledgments

OpenFoodFacts for the open food database.
egui for the GUI framework.
The Rust community for excellent tools and support.

## Customization Notes

Repository URL: Replace <https://github.com/mortifia/test_egui_with_openfoodfacts>  
Screenshots: Create a screenshots/ directory and add images (e.g., search.png, details.png) to visually showcase the app. Update the paths in the README accordingly.

## License

```txt
MIT License

Copyright (c) 2025 Guillaume Casal

Permission is hereby granted, free of charge, to any person obtaining a copy...
```

Badges: The shields (Rust version, license) enhance GitHub readability.