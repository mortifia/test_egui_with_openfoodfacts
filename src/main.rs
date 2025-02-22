use eframe::egui;
use serde::Deserialize;
use std::sync::mpsc;

// Structs to deserialize API responses

/// Represents a product in the search results.
#[derive(Deserialize)]
struct Product {
    code: String,
    product_name: String,
}

/// Represents the response from the search API endpoint.
#[derive(Deserialize)]
struct SearchResponse {
    products: Vec<Product>,
}

/// Represents detailed information about a product.
#[derive(Deserialize)]
struct ProductDetails {
    product_name: String,
    ingredients_text: Option<String>,
}

/// Represents the response from the product details API endpoint.
#[derive(Deserialize)]
struct ProductDetailsResponse {
    product: ProductDetails,
}

// Application state management

/// Enum to track the current view (search results or product details).
enum View {
    SearchResults,
    ProductDetails,
}

/// Enum to represent messages sent from background threads to the UI thread.
enum Message {
    SearchResults(Vec<Product>),
    ProductDetails(ProductDetails),
    Error(String),
}

/// Main application struct holding all state.
struct OpenFoodFactsViewer {
    /// Current search term entered by the user.
    search_term: String,
    /// List of products from the latest search.
    search_results: Vec<Product>,
    /// Details of the currently selected product.
    selected_product: Option<ProductDetails>,
    /// Current view (search results or product details).
    view: View,
    /// Flag indicating if data is being loaded.
    is_loading: bool,
    /// Optional error message to display.
    error_message: Option<String>,
    /// Sender for sending messages from background threads.
    message_sender: mpsc::Sender<Message>,
    /// Receiver for receiving messages in the UI thread.
    message_receiver: mpsc::Receiver<Message>,
}

impl OpenFoodFactsViewer {
    /// Creates a new instance of the application.
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            search_term: String::new(),
            search_results: Vec::new(),
            selected_product: None,
            view: View::SearchResults,
            is_loading: false,
            error_message: None,
            message_sender: sender,
            message_receiver: receiver,
        }
    }
}

impl eframe::App for OpenFoodFactsViewer {
    /// Updates the UI each frame.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top panel with search bar and button
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Search:");
                ui.text_edit_singleline(&mut self.search_term);
                if ui.button("Search").clicked() {
                    self.is_loading = true;
                    self.error_message = None;
                    let sender = self.message_sender.clone();
                    let search_term = self.search_term.clone();
                    // Spawn a background thread for the search API request
                    std::thread::spawn(move || {
                        let url = format!(
                            "https://world.openfoodfacts.org/cgi/search.pl?search_terms={}&search_simple=1&json=1",
                            search_term
                        );
                        match reqwest::blocking::get(&url) {
                            Ok(response) => {
                                match response.json::<SearchResponse>() {
                                    Ok(search_response) => {
                                        sender.send(Message::SearchResults(search_response.products)).unwrap();
                                    }
                                    Err(e) => {
                                        sender.send(Message::Error(format!("Failed to parse response: {}", e))).unwrap();
                                    }
                                }
                            }
                            Err(e) => {
                                sender.send(Message::Error(format!("Request failed: {}", e))).unwrap();
                            }
                        }
                    });
                }
            });
        });

        // Central panel for displaying content
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.is_loading {
                ui.label("Loading...");
            } else if let Some(error) = &self.error_message {
                ui.label(format!("Error: {}", error));
            } else {
                match self.view {
                    View::SearchResults => {
                        ui.heading("Search Results");
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for product in &self.search_results {
                                if ui.button(&product.product_name).clicked() {
                                    self.view = View::ProductDetails;
                                    self.is_loading = true;
                                    let sender = self.message_sender.clone();
                                    let code = product.code.clone();
                                    // Spawn a background thread for the product details API request
                                    std::thread::spawn(move || {
                                        let url = format!(
                                            "https://world.openfoodfacts.org/api/v0/product/{}.json",
                                            code
                                        );
                                        match reqwest::blocking::get(&url) {
                                            Ok(response) => {
                                                match response.json::<ProductDetailsResponse>() {
                                                    Ok(details_response) => {
                                                        sender.send(Message::ProductDetails(details_response.product)).unwrap();
                                                    }
                                                    Err(e) => {
                                                        sender.send(Message::Error(format!("Failed to parse details: {}", e))).unwrap();
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                sender.send(Message::Error(format!("Details request failed: {}", e))).unwrap();
                                            }
                                        }
                                    });
                                }
                            }
                        });
                    }
                    View::ProductDetails => {
                        if let Some(product) = &self.selected_product {
                            ui.heading(&product.product_name);
                            if let Some(ingredients) = &product.ingredients_text {
                                ui.label(format!("Ingredients: {}", ingredients));
                            } else {
                                ui.label("Ingredients: N/A");
                            }
                        }
                        if ui.button("Back").clicked() {
                            self.view = View::SearchResults;
                            self.selected_product = None;
                        }
                    }
                }
            }
        });

        // Handle messages from background threads
        while let Ok(message) = self.message_receiver.try_recv() {
            match message {
                Message::SearchResults(results) => {
                    self.search_results = results;
                    self.is_loading = false;
                }
                Message::ProductDetails(details) => {
                    self.selected_product = Some(details);
                    self.is_loading = false;
                }
                Message::Error(err) => {
                    self.error_message = Some(err);
                    self.is_loading = false;
                }
            }
        }
    }
}

/// Entry point of the application.
fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "OpenFoodFacts Viewer",
        options,
        Box::new(|cc| Box::new(OpenFoodFactsViewer::new(cc))),
    );
}