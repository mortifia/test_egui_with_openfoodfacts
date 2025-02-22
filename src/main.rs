use eframe::egui;
use serde::Deserialize;
use std::sync::mpsc;
use log::{debug, error, info};

// Structs for API responses
#[derive(Deserialize, Debug)]
struct Product {
    code: Option<String>,
    product_name: Option<String>,
}

#[derive(Deserialize)]
struct SearchResponse {
    products: Vec<Product>,
}

#[derive(Deserialize)]
struct ProductDetails {
    code: String,
    product_name: Option<String>,
    ingredients_text: Option<String>,
    brands: Option<String>,
    // other fields...
}

#[derive(Deserialize)]
struct ProductDetailsResponse {
    product: ProductDetails,
}

// Application state
enum View {
    SearchResults,
    ProductDetails,
}

enum Message {
    SearchResults(Vec<Product>),
    ProductDetails(ProductDetails),
    Error(String),
}

struct OpenFoodFactsViewer {
    search_term: String,
    search_results: Vec<Product>,
    selected_product: Option<ProductDetails>,
    view: View,
    is_loading: bool,
    error_message: Option<String>,
    message_sender: mpsc::Sender<Message>,
    message_receiver: mpsc::Receiver<Message>,
}

impl OpenFoodFactsViewer {
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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top panel with search bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Search:");
                ui.text_edit_singleline(&mut self.search_term);
                if ui.button("Search").clicked() {
                    self.is_loading = true;
                    self.error_message = None;
                    let sender = self.message_sender.clone();
                    let search_term = self.search_term.clone();
                    std::thread::spawn(move || {
                        let url = format!(
                            "https://world.openfoodfacts.org/cgi/search.pl?search_terms={}&search_simple=1&json=1",
                            search_term
                        );
                        debug!("Requesting search results from: {}", url);
                        match reqwest::blocking::get(&url) {
                            Ok(response) => {
                                debug!("Received response with status: {}", response.status());
                                let response_text = response.text().unwrap();
                                println!("Raw JSON: {}", response_text);
                                let product: Product = serde_json::from_str(&response_text).unwrap();
                                println!("Product: {:?}", product);
                                // Cloner le texte de la réponse avant de déplacer `response`
                                let response_json = serde_json::from_str::<serde_json::Value>(&response_text).unwrap();
                                match serde_json::from_value::<SearchResponse>(response_json) {
                                    Ok(search_response) => {
                                        info!("Successfully parsed search results.");
                                        sender.send(Message::SearchResults(search_response.products)).unwrap();
                                    }
                                    Err(e) => {
                                        error!("Failed to parse response: {}", e);
                                        sender.send(Message::Error(format!("Failed to parse response: {}", e))).unwrap();
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Failed to get response: {}", e);
                                sender.send(Message::Error(format!("Failed to get response: {}", e))).unwrap();
                            }
                        }
                    });
                }
            });
        });

        // Central panel for content
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
                                if ui.button(product.product_name.as_deref().unwrap_or("Unnamed product")).clicked() {
                                    self.view = View::ProductDetails;
                                    self.is_loading = true;
                                    let sender = self.message_sender.clone();
                                    let code = product.code.clone().unwrap_or_else(|| "unknown".to_string());
                                    std::thread::spawn(move || {
                                        let url = format!(
                                            "https://world.openfoodfacts.org/api/v0/product/{}.json",
                                            code
                                        );
                                        debug!("Requesting product details from: {}", url);
                                        match reqwest::blocking::get(&url) {
                                            Ok(response) => {
                                                debug!("Received response with status: {}", response.status());
                                                match response.json::<ProductDetailsResponse>() {
                                                    Ok(details_response) => {
                                                        info!("Successfully parsed product details.");
                                                        sender.send(Message::ProductDetails(details_response.product)).unwrap();
                                                    }
                                                    Err(e) => {
                                                        error!("Failed to parse details: {}", e);
                                                        sender.send(Message::Error(format!("Failed to parse details: {}", e))).unwrap();
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                error!("Details request failed: {}", e);
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
                            ui.heading(product.product_name.as_deref().unwrap_or("Unnamed product"));
                            ui.label(format!(
                                "Ingredients: {}",
                                product.ingredients_text.as_ref().unwrap_or(&"N/A".to_string())
                            ));
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

fn main() {
    // Initialize the logger
    env_logger::init();

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "OpenFoodFacts Viewer",
        options,
        Box::new(|cc| Ok(Box::new(OpenFoodFactsViewer::new(cc)))),
    );
}