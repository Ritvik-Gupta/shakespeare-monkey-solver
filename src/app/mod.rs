mod state;

use eframe::{egui, epi};
use state::{
    biased_scale::BiasedScaleStore::*, population_builder::PopulationBuilder,
    population_store::PopulationStore,
};

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Default)]
pub struct TemplateApp {
    population_form: PopulationBuilder,

    #[cfg_attr(feature = "persistence", serde(skip))]
    running_simulation: Option<PopulationStore>,
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
        if let Some(simulation) = &mut self.running_simulation {
            simulation.compute_biased_fitness();
            if !simulation.has_finished {
                simulation.update_generation();
                ctx.request_repaint();
            }
        }

        egui::SidePanel::left("config-panel")
            .resizable(false)
            .show(ctx, |ui| {
                egui::Grid::new("config-panel-grid")
                    .num_columns(1)
                    .striped(true)
                    .spacing([10.0, 10.0])
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            let form = &mut self.population_form;

                            ui.heading("Simulation Form");

                            ui.vertical_centered_justified(|ui| {
                                ui.label("Enter a Term: ");
                                ui.text_edit_singleline(&mut form.target_term);
                            });

                            ui.add(
                                egui::Slider::new(&mut form.mutation_rate, 0..=50)
                                    .text("Mut Rate")
                                    .suffix("%"),
                            );

                            ui.add(
                                egui::Slider::new(&mut form.population_size, 10..=500)
                                    .text("Pop Size"),
                            );

                            egui::ComboBox::new("", "Biased Scale")
                                .selected_text(format!("Bias: {:?}", form.biased_scale))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut form.biased_scale,
                                        Multiplicative,
                                        format!("{:?}", Multiplicative),
                                    );
                                    ui.selectable_value(
                                        &mut form.biased_scale,
                                        Order,
                                        format!("{:?}", Order),
                                    );
                                    ui.selectable_value(
                                        &mut form.biased_scale,
                                        Exponential,
                                        format!("{:?}", Exponential),
                                    );
                                });

                            let mut slider = egui::Slider::new(
                                &mut form.scale_factor,
                                match form.biased_scale {
                                    Multiplicative | Order => 1.0..=5.0,
                                    Exponential => 1.2..=2.50,
                                },
                            )
                            .text("Bias Scale");
                            slider = match form.biased_scale {
                                Multiplicative => slider.prefix("N*"),
                                Order => slider.prefix("N^"),
                                Exponential => slider.suffix("^N"),
                            };
                            ui.add(slider);

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
                                    Some(self.population_form.build_simulation())
                            }
                        });

                        ui.end_row();

                        if let Some(simulation) = &mut self.running_simulation {
                            ui.vertical_centered(|ui| {
                                ui.horizontal_wrapped(|ui| {
                                    ui.label("Mutation Rate: ");
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "{}%",
                                            simulation.mutation_rate
                                        ))
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
                                    let mut label = egui::RichText::new(
                                        simulation.generation_counter.to_string(),
                                    )
                                    .color(egui::Color32::LIGHT_GRAY);
                                    if simulation.has_finished {
                                        label = label.color(egui::Color32::GOLD);
                                    }

                                    ui.label(label);
                                });

                                ui.horizontal_wrapped(|ui| {
                                    ui.label("Target: ");
                                    ui.label(
                                        egui::RichText::new(&simulation.target_term)
                                            .color(egui::Color32::GOLD),
                                    );
                                });
                            });
                        }
                    });
            });

        if let Some(simulation) = &mut self.running_simulation {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    let best_candidate = &simulation.population[simulation.best_candidate];

                    for (idx, token) in simulation.target_term.char_indices() {
                        let mut label = egui::RichText::new(best_candidate.genes[idx]).underline();
                        if token == best_candidate.genes[idx] {
                            label = label.color(egui::Color32::GOLD);
                        }
                        ui.label(label);
                    }
                });
                ui.separator();
                ui.separator();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for candidate in simulation.population.iter() {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;

                            for (idx, token) in simulation.target_term.char_indices() {
                                let mut label = egui::RichText::new(candidate.genes[idx]);
                                if token == candidate.genes[idx] {
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

        (0..=50).contains(&form.mutation_rate)
            && (10..=500).contains(&form.population_size)
            && form.target_term.len() > 0
    }
}
