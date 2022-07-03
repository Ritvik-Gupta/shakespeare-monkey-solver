use eframe::{egui, epi};

use crate::state::{Population, PopulationForm};

#[cfg_attr(
    feature = "persistence",
    derive(serde::Deserialize, serde::Serialize),
    serde(default)
)]
#[derive(Default)]
pub struct TemplateApp {
    population_form: PopulationForm,

    #[cfg_attr(feature = "persistence", serde(skip))]
    running_simulation: Option<Population>,
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "shakespeare-monkey-solver"
    }

    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        let has_sim_finished = match &mut self.running_simulation {
            Some(simulation) => {
                let has_sim_finished = simulation.compute_biased_fitness_if_not_finished();
                if !has_sim_finished {
                    simulation.update_generation();
                }
                has_sim_finished
            }
            None => false,
        };

        egui::SidePanel::left("config-panel")
            .resizable(false)
            .show(ctx, |ui| {
                egui::Grid::new("config-panel-grid")
                    .num_columns(2)
                    .striped(true)
                    .spacing([0.0, 0.0])
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.heading("Simulation Form");

                            ui.vertical_centered_justified(|ui| {
                                ui.label("Enter a Term: ");
                                ui.text_edit_singleline(&mut self.population_form.target_term);
                            });

                            ui.add(
                                egui::Slider::new(&mut self.population_form.mutation_rate, 0..=100)
                                    .text("Mut Rate")
                                    .suffix("%"),
                            );

                            ui.add(
                                egui::Slider::new(
                                    &mut self.population_form.population_size,
                                    10..=1000,
                                )
                                .text("Pop Size"),
                            );

                            let simulation_button = ui.add_enabled(
                                self.is_valid_form_state()
                                    && self
                                        .running_simulation
                                        .as_ref()
                                        .map(|simulation| self.population_form != simulation)
                                        .unwrap_or(true),
                                egui::Button::new("Create Simulation"),
                            );

                            if simulation_button.clicked() {
                                self.running_simulation =
                                    Some(self.population_form.create_simulation())
                            }
                        });

                        ui.end_row();

                        if let Some(simulation) = &mut self.running_simulation {
                            ui.vertical_centered(|ui| {
                                ui.horizontal_wrapped(|ui| {
                                    ui.label("Mutation Rate: ");
                                    ui.label(
                                        egui::RichText::new(
                                            (simulation.mutation_rate as f64 / 100.0).to_string(),
                                        )
                                        .color(egui::Color32::LIGHT_GRAY),
                                    );
                                });

                                ui.horizontal_wrapped(|ui| {
                                    ui.label("Population Size: ");
                                    ui.label(
                                        egui::RichText::new(
                                            simulation.population.len().to_string(),
                                        )
                                        .color(egui::Color32::LIGHT_GRAY),
                                    );
                                });

                                ui.horizontal_wrapped(|ui| {
                                    ui.label("Generation: ");
                                    ui.label(
                                        egui::RichText::new(simulation.generations.to_string())
                                            .color(egui::Color32::LIGHT_GRAY),
                                    );
                                });

                                ui.horizontal_wrapped(|ui| {
                                    ui.label("Target: ");
                                    ui.label(
                                        egui::RichText::new(&simulation.target_term)
                                            .color(egui::Color32::KHAKI),
                                    );
                                });
                            });
                            if !has_sim_finished {
                                ui.add(egui::Spinner::new());
                            }
                        }
                    });
            });

        if let Some(simulation) = &mut self.running_simulation {
            egui::CentralPanel::default().show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for dna in simulation.population.iter() {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;

                            for (idx, token) in simulation.target_term.char_indices() {
                                let mut label = egui::RichText::new(dna.genes[idx]);
                                if token == dna.genes[idx] {
                                    label = label.color(egui::Color32::LIGHT_GREEN);
                                }
                                ui.label(label);
                            }
                        });
                        ui.separator();
                    }
                });
            });
        }
    }
}

impl TemplateApp {
    fn is_valid_form_state(&self) -> bool {
        let form = &self.population_form;

        (0..=100).contains(&form.mutation_rate)
            && (10..=1000).contains(&form.population_size)
            && form.target_term.len() > 0
    }
}
