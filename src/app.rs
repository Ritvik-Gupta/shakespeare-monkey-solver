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
        egui::SidePanel::left("population-form")
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Simulation Form");

                ui.vertical_centered_justified(|ui| {
                    ui.label("Enter a Term: ");
                    let target = &mut self.population_form.target_term;
                    ui.text_edit_singleline(target);
                    if target.len() > 20 {
                        *target = target[..20].to_string();
                    }
                });

                ui.add(
                    egui::Slider::new(&mut self.population_form.mutation_rate, 0..=100)
                        .text("Mut Rate")
                        .suffix("%"),
                );

                ui.add(
                    egui::Slider::new(&mut self.population_form.population_size, 10..=200)
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
                    self.running_simulation = Some(self.population_form.create_simulation())
                }
            });

        if let Some(simulation) = &mut self.running_simulation {
            egui::CentralPanel::default().show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for dna in simulation.population.iter() {
                        ui.label(dna.genes.iter().collect::<String>());
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
            && (10..=200).contains(&form.population_size)
            && form.target_term.len() > 0
    }
}
