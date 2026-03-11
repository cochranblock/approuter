// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! egui GUI. kova gui. Professional theme per THEME.md.
//! f113=gui_run

use eframe::egui;
use std::sync::{mpsc, Arc};
use tokio::sync::broadcast;

use crate::theme::{self, colors, layout};

/// f113=gui_run. Run native egui GUI. Demo mode records to ~/.kova/demos/.
pub fn run(demo: bool) -> anyhow::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([720.0, 560.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Kova",
        options,
        Box::new(move |cc| {
            theme::apply(&cc.egui_ctx);
            Ok(Box::new(KovaApp::new(cc, demo)))
        }),
    )
    .map_err(|e| anyhow::anyhow!("eframe: {}", e))
}

struct KovaApp {
    input: String,
    messages: Vec<crate::Message>,
    store: Option<crate::storage::t12>,
    system_prompt: String,
    persona: String,
    show_prompts: bool,
    show_backlog: bool,
    pending_intent: Option<crate::t0>,
    /// Current project for code gen. Discovered from projects_root; user selects from dropdown.
    current_project: std::path::PathBuf,
    #[cfg(feature = "inference")]
    llm_receiver: Option<mpsc::Receiver<std::sync::Arc<str>>>,
    #[cfg(feature = "inference")]
    router_receiver: Option<mpsc::Receiver<crate::RouterResult>>,
    #[cfg(feature = "inference")]
    router_pending_user_msg: Option<String>,
    #[cfg(feature = "inference")]
    pipeline_receiver: Option<broadcast::Receiver<Arc<str>>>,
    last_applied: Option<String>,
    #[cfg(feature = "inference")]
    clarification_pending: bool,
    #[cfg(feature = "inference")]
    clarification_choices: Option<Vec<String>>,
    #[cfg(feature = "inference")]
    restatement_pending: bool,
    #[cfg(feature = "inference")]
    restatement_pending_msg: Option<String>,
    /// Demo mode: record actions to ~/.kova/demos/
    demo_recording: Option<DemoRecording>,
}

impl DemoRecording {
    fn push(&mut self, kind: &str, data: serde_json::Value) {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        let mut m = serde_json::Map::new();
        m.insert("kind".into(), serde_json::Value::String(kind.to_string()));
        m.insert("ts_ms".into(), serde_json::json!(ts));
        if let Some(obj) = data.as_object() {
            for (k, v) in obj {
                m.insert(k.clone(), v.clone());
            }
        }
        self.actions.push(serde_json::Value::Object(m));
    }
    fn save(&self) {
        let dir = std::env::var("KOVA_DEMO_DIR")
            .ok()
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| {
                dirs::home_dir()
                    .unwrap_or_else(|| std::path::PathBuf::from("."))
                    .join(".kova")
                    .join("demos")
            });
        let _ = std::fs::create_dir_all(&dir);
        let safe = self.name.replace(['/', '\\', ':'], "_");
        let path = dir.join(format!("{}.json", safe));
        let payload = serde_json::json!({
            "name": self.name,
            "source": "egui",
            "actions": self.actions,
            "started_at": self.started_at
        });
        if let Ok(json) = serde_json::to_string_pretty(&payload) {
            let _ = std::fs::write(&path, json);
            eprintln!("Demo saved: {}", path.display());
        }
    }
}

struct DemoRecording {
    name: String,
    actions: Vec<serde_json::Value>,
    started_at: String,
}

fn response_for_input(input: &str) -> (String, Option<crate::t0>) {
    match crate::f62(input) {
        Some(intent) => {
            let name = crate::intent_name(&intent.s0);
            (format!("Run {}? (y/n)", name), Some(intent))
        }
        None => ("".into(), None),
    }
}

fn is_confirm(s: &str) -> bool {
    let t = s.trim().to_lowercase();
    t == "y" || t == "yes"
}

/// Shared logic for building system prompt with Cursor rules. Testable.
fn build_system_prompt_impl(system: &str, persona: &str, project: &std::path::Path) -> String {
    let cursor = crate::cursor_prompts::load_cursor_prompts(project);
    if cursor.is_empty() {
        format!("{}\n\n{}", system, persona)
    } else {
        format!(
            "{}\n\n{}\n\n--- Cursor rules ---\n{}",
            system, persona, cursor
        )
    }
}

impl KovaApp {
    fn new(_cc: &eframe::CreationContext<'_>, demo: bool) -> Self {
        let system_prompt = crate::load_prompt("system");
        let persona = crate::load_prompt("persona");
        let store = crate::storage::t12::f39(crate::sled_path()).ok();
        let messages = store
            .as_ref()
            .and_then(|s| crate::f74(s).ok())
            .unwrap_or_default();
        let demo_recording = if demo {
            Some(DemoRecording {
                name: format!("egui-{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs()),
                actions: Vec::new(),
                started_at: format!("{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs()),
            })
        } else {
            None
        };
        Self {
            input: String::new(),
            messages,
            store,
            system_prompt,
            persona,
            show_prompts: false,
            show_backlog: false,
            pending_intent: None,
            current_project: crate::default_project(),
            #[cfg(feature = "inference")]
            llm_receiver: None,
            #[cfg(feature = "inference")]
            router_receiver: None,
            #[cfg(feature = "inference")]
            router_pending_user_msg: None,
            #[cfg(feature = "inference")]
            pipeline_receiver: None,
            last_applied: None,
            #[cfg(feature = "inference")]
            clarification_pending: false,
            #[cfg(feature = "inference")]
            clarification_choices: None,
            #[cfg(feature = "inference")]
            restatement_pending: false,
            #[cfg(feature = "inference")]
            restatement_pending_msg: None,
            demo_recording,
        }
    }

    fn build_system_prompt(&self) -> String {
        build_system_prompt_impl(&self.system_prompt, &self.persona, &self.current_project)
    }

    fn run_intent(
        &mut self,
        intent: crate::t0,
        project: std::path::PathBuf,
        approuter_dir: Option<std::path::PathBuf>,
    ) {
        if let Some(ref mut rec) = self.demo_recording {
            rec.push("api_call", serde_json::json!({
                "method": "run_intent",
                "path": crate::intent_name(&intent.s0),
                "project": project.display().to_string()
            }));
        }
        let plan = crate::t3::f14(&intent, project, approuter_dir);
        let exec = crate::t6;
        match exec.f15(&plan) {
            Ok(results) => {
                let all_ok = results.iter().all(|r| r.s11);
                let summary: Vec<String> = results
                    .iter()
                    .map(|r| {
                        let mark = if r.s11 { "✓" } else { "✗" };
                        format!("{} {} {}", mark, r.s10, r.s13)
                    })
                    .collect();
                let response = if results.is_empty() {
                    "No actions for this intent.".into()
                } else if all_ok {
                    format!("Done.\n{}", summary.join("\n"))
                } else {
                    format!("Failed.\n{}", summary.join("\n"))
                };
                self.messages.push(crate::Message {
                    role: "assistant".into(),
                    content: response.clone(),
                });
                self.persist("assistant", &response);
            }
            Err(e) => {
                let response = format!("Error: {}", e);
                self.messages.push(crate::Message {
                    role: "assistant".into(),
                    content: response.clone(),
                });
                self.persist("assistant", &response);
            }
        }
    }

    fn persist(&self, role: &str, content: &str) {
        if let Some(ref s) = self.store {
            let _ = crate::f73(s, role, content);
        }
    }
}

impl eframe::App for KovaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(feature = "inference")]
        {
            if let Some(rx) = &mut self.router_receiver {
                match rx.try_recv() {
                    Ok(result) => {
                        self.router_receiver = None;
                        let to_persist = if let Some(m) = self.messages.last_mut() {
                            match &result {
                                crate::RouterResult::NeedsClarification { .. } => {
                                    let orig = self.router_pending_user_msg.as_deref().unwrap_or("");
                                    let q = result.clarification_question(orig);
                                    m.content = q.clone();
                                    self.clarification_pending = true;
                                    self.clarification_choices =
                                        result.clarification_choices().map(|s| s.to_vec());
                                    Some(m.content.clone())
                                }
                                crate::RouterResult::Error(e) => {
                                    m.content = format!("Router error: {}", e);
                                    Some(m.content.clone())
                                }
                                crate::RouterResult::CodeGen => {
                                    let user_msg = self.router_pending_user_msg.take().unwrap_or_default();
                                    let (action, target) = if let Some(in_pos) = user_msg.find(" in ") {
                                        let (a, t) = user_msg.split_at(in_pos);
                                        (a.trim(), t[" in ".len()..].trim())
                                    } else if let Some(to_pos) = user_msg.find(" to ") {
                                        let (a, t) = user_msg.split_at(to_pos);
                                        (a.trim(), t[" to ".len()..].trim())
                                    } else {
                                        (user_msg.as_str(), "")
                                    };
                                    let restatement = crate::elicitor::build_restatement(action, target);
                                    m.content = restatement.clone();
                                    self.restatement_pending = true;
                                    self.restatement_pending_msg = Some(user_msg);
                                    Some(restatement)
                                }
                                _ if result.use_coder() || matches!(result, crate::RouterResult::Run) =>
                                {
                                    m.content.clear();
                                    let user_msg = self.router_pending_user_msg.take().unwrap_or_default();
                                    if let Some(path) = crate::f78(crate::ModelRole::Coder) {
                                        let system = self.build_system_prompt();
                                        let hist: Vec<(String, String)> = Vec::new();
                                        let rx = crate::inference::f76(
                                            &path,
                                            &system,
                                            &hist,
                                            &user_msg,
                                        );
                                        self.llm_receiver = Some(rx);
                                        None
                                    } else {
                                        m.content =
                                            "No coder model. Run: kova model install".into();
                                        Some(m.content.clone())
                                    }
                                }
                                _ => {
                                    m.content = "Unexpected classification.".into();
                                    Some(m.content.clone())
                                }
                            }
                        } else {
                            None
                        };
                        if let Some(c) = to_persist {
                            self.persist("assistant", &c);
                        }
                        ctx.request_repaint();
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        self.router_receiver = None;
                    }
                    Err(mpsc::TryRecvError::Empty) => {}
                }
            }
            if self.router_receiver.is_some() {
                ctx.request_repaint();
            }

            if let Some(rx) = &mut self.pipeline_receiver {
                loop {
                    match rx.try_recv() {
                        Ok(content) => {
                            if let Some(m) = self.messages.last_mut() {
                                m.content = (*content).to_string();
                            }
                            ctx.request_repaint();
                        }
                        Err(broadcast::error::TryRecvError::Lagged(_)) => continue,
                        Err(broadcast::error::TryRecvError::Closed) => {
                            if let Some(m) = self.messages.last() {
                                self.persist("assistant", &m.content);
                            }
                            self.pipeline_receiver = None;
                            break;
                        }
                        Err(broadcast::error::TryRecvError::Empty) => break,
                    }
                }
                ctx.request_repaint();
            }
            if self.pipeline_receiver.is_some() {
                ctx.request_repaint();
            }

            while let Some(rx) = &mut self.llm_receiver {
                match rx.try_recv() {
                    Ok(token) => {
                        if let Some(m) = self.messages.last_mut() {
                            m.content.push_str(&token);
                        }
                        ctx.request_repaint();
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        let content = self
                            .messages
                            .last()
                            .map(|m| m.content.clone())
                            .unwrap_or_default();
                        if !content.is_empty() {
                            self.persist("assistant", &content);
                        }
                        self.llm_receiver = None;
                        break;
                    }
                    Err(mpsc::TryRecvError::Empty) => break,
                }
            }
            if self.llm_receiver.is_some() {
                ctx.request_repaint();
            }
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(colors::BG).inner_margin(egui::Margin::same(layout::MARGIN)))
            .show(ctx, |ui| {
            ui.add_space(layout::GAP);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Kova").color(colors::PRIMARY).size(22.0).strong());
                ui.add_space(layout::MARGIN);
                ui.separator();
                ui.add_space(layout::GAP);
                ui.label(egui::RichText::new("Project").color(colors::MUTED).small());
                let projects = crate::discover_projects();
                let current_name = self.current_project.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("?").to_string();
                egui::ComboBox::from_id_salt("project_selector")
                    .selected_text(&current_name)
                    .show_ui(ui, |ui| {
                        for p in &projects {
                            let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("?");
                            if ui.selectable_label(name == current_name, name).clicked() {
                                self.current_project = p.clone();
                                ui.close_menu();
                            }
                        }
                        if projects.is_empty() {
                            ui.label(egui::RichText::new("(none)").color(colors::MUTED));
                        }
                    });
                ui.add_space(layout::GAP);
                if ui.button("Prompts").clicked() {
                    self.show_prompts = !self.show_prompts;
                }
                if ui.button("Backlog").clicked() {
                    self.show_backlog = !self.show_backlog;
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new("~/.kova/prompts/").color(colors::MUTED).small());
                });
            });
            ui.add_space(layout::GAP);
            if self.show_backlog {
                let backlog_path = crate::backlog_path();
                let backlog = std::fs::read_to_string(&backlog_path)
                    .ok()
                    .and_then(|s| serde_json::from_str::<crate::Backlog>(&s).ok())
                    .unwrap_or_default();
                theme::panel_frame().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Backlog").color(colors::PRIMARY).strong());
                        ui.add_space(layout::GAP);
                        if ui.button("Run all").clicked() {
                        let default_approuter = std::env::var("HOME")
                            .ok()
                            .map(|h| std::path::PathBuf::from(h).join("approuter"));
                        for entry in &backlog.items {
                            if let Some(intent) = crate::entry_to_intent(entry) {
                                let project = entry
                                    .project
                                    .as_ref()
                                    .map(std::path::PathBuf::from)
                                    .unwrap_or_else(|| self.current_project.clone());
                                let approuter_dir = entry
                                    .approuter_dir
                                    .as_ref()
                                    .map(std::path::PathBuf::from)
                                    .or_else(|| default_approuter.clone());
                                self.run_intent(intent, project, approuter_dir);
                            }
                        }
                    }
                    });
                    ui.add_space(layout::PADDING_SM);
                    egui::ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
                        for entry in &backlog.items {
                            theme::message_frame().show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new(&entry.intent).color(colors::PRIMARY));
                                    ui.label(egui::RichText::new("·").color(colors::MUTED));
                                    ui.label(egui::RichText::new(entry.project.as_deref().unwrap_or("-")).color(colors::MUTED).small());
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        if ui.button("Run").clicked() {
                                if let Some(intent) = crate::entry_to_intent(entry) {
                                    let project = entry
                                        .project
                                        .as_ref()
                                        .map(std::path::PathBuf::from)
                                        .unwrap_or_else(|| self.current_project.clone());
                                    let approuter_dir = entry
                                        .approuter_dir
                                        .as_ref()
                                        .map(std::path::PathBuf::from)
                                        .or_else(|| {
                                            std::env::var("HOME")
                                                .ok()
                                                .map(|h| std::path::PathBuf::from(h).join("approuter"))
                                        });
                                            self.run_intent(intent, project, approuter_dir);
                                        }
                                    }
                                });
                                });
                            });
                        }
                    });
                });
                ui.add_space(layout::GAP);
            }
            if self.show_prompts {
                theme::panel_frame().show(ui, |ui| {
                    egui::CollapsingHeader::new(egui::RichText::new("system.md").color(colors::PRIMARY))
                        .default_open(true)
                        .show(ui, |ui| {
                            egui::ScrollArea::vertical().max_height(120.0).show(ui, |ui| {
                                ui.label(egui::RichText::new(&self.system_prompt).color(colors::TEXT).monospace());
                            });
                        });
                    egui::CollapsingHeader::new(egui::RichText::new("persona.md").color(colors::PRIMARY))
                        .default_open(false)
                        .show(ui, |ui| {
                            egui::ScrollArea::vertical().max_height(80.0).show(ui, |ui| {
                                ui.label(egui::RichText::new(&self.persona).color(colors::TEXT).monospace());
                            });
                        });
                });
                ui.add_space(layout::GAP);
            }
            theme::input_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Chat").color(colors::PRIMARY).strong());
                let mut send = false;
                ui.text_edit_singleline(&mut self.input);
                if ui.input(|i| i.key_pressed(egui::Key::Enter)) && !self.input.is_empty() {
                    send = true;
                }
                if ui.button("Send").clicked() && !self.input.is_empty() {
                    send = true;
                }
                if send {
                    let msg = std::mem::take(&mut self.input);
                    if !msg.is_empty() {
                        if let Some(ref mut rec) = self.demo_recording {
                            if is_confirm(&msg) {
                                let intent = self.pending_intent.as_ref().map(|i| crate::intent_name(&i.s0).to_string()).unwrap_or_default();
                                rec.push("egui_confirm", serde_json::json!({ "intent": intent }));
                            } else {
                                rec.push("egui_send", serde_json::json!({ "text": msg }));
                            }
                        }
                        self.messages.push(crate::Message {
                            role: "user".into(),
                            content: msg.clone(),
                        });
                        self.persist("user", &msg);

                        if is_confirm(&msg) {
                            if let Some(intent) = self.pending_intent.take() {
                                let project = self.current_project.clone();
                                let approuter_dir = std::env::var("HOME")
                                    .ok()
                                    .map(|h| std::path::PathBuf::from(h).join("approuter"));
                                self.run_intent(intent, project, approuter_dir);
                            } else {
                                let response: String = "No pending intent to run.".into();
                                self.messages.push(crate::Message {
                                    role: "assistant".into(),
                                    content: response.clone(),
                                });
                                self.persist("assistant", &response);
                            }
                        } else {
                            #[cfg(feature = "inference")]
                            let is_restatement = self.restatement_pending;

                            #[cfg(feature = "inference")]
                            if is_restatement {
                                self.restatement_pending = false;
                                let pending = self.restatement_pending_msg.take().unwrap_or_default();
                                let reply = crate::elicitor::parse_reply(&msg, None);
                                match reply {
                                    crate::elicitor::ElicitorReply::Confirm(true) => {
                                        let coder = crate::f78(crate::ModelRole::Coder);
                                        let fix = crate::f78(crate::ModelRole::Fix)
                                            .or_else(|| crate::f78(crate::ModelRole::Coder));
                                        if let (Some(cp), Some(fp)) = (coder, fix) {
                                            let system = self.build_system_prompt();
                                            let project = self.current_project.clone();
                                            let max_retries = crate::orchestration_max_fix_retries();
                                            let run_clippy = crate::orchestration_run_clippy();
                                            let rx = crate::pipeline::f81(
                                                &cp, &fp, &system, &pending,
                                                &project, max_retries, run_clippy, None,
                                            );
                                            self.pipeline_receiver = Some(rx);
                                            if let Some(m) = self.messages.last_mut() {
                                                m.content = "Generating…".into();
                                            }
                                        } else {
                                            self.messages.push(crate::Message {
                                                role: "assistant".into(),
                                                content: "No coder model. Run: kova model install".into(),
                                            });
                                        }
                                    }
                                    _ => {
                                        self.messages.push(crate::Message {
                                            role: "assistant".into(),
                                            content: "Cancelled.".into(),
                                        });
                                        self.persist("assistant", "Cancelled.");
                                    }
                                }
                            } else {
                            #[cfg(feature = "inference")]
                            let is_clarification = self.clarification_pending;

                            #[cfg(feature = "inference")]
                            let to_route_opt = if is_clarification {
                                let orig = self.router_pending_user_msg.take().unwrap_or_default();
                                self.clarification_pending = false;
                                let choices = self.clarification_choices.take();
                                let reply = crate::elicitor::parse_reply(
                                    &msg,
                                    choices.as_ref().map(|c| c.len()),
                                );
                                match reply {
                                    crate::elicitor::ElicitorReply::Cancel => {
                                        self.messages.push(crate::Message {
                                            role: "assistant".into(),
                                            content: "Cancelled.".into(),
                                        });
                                        self.persist("assistant", "Cancelled.");
                                        None
                                    }
                                    crate::elicitor::ElicitorReply::Choice(idx) => {
                                        let s = choices
                                            .as_ref()
                                            .and_then(|ch| ch.get(idx))
                                            .map(|s| s.as_str())
                                            .unwrap_or(&msg);
                                        Some(format!("{} in {}", orig, s))
                                    }
                                    _ => {
                                        let s = if msg.contains(".rs") {
                                            format!("{} in {}", orig, msg)
                                        } else {
                                            format!("{} {}", orig, msg)
                                        };
                                        Some(s)
                                    }
                                }
                            } else {
                                Some(msg.clone())
                            };
                            #[cfg(not(feature = "inference"))]
                            let to_route_opt = Some(msg.clone());

                            #[cfg(feature = "inference")]
                            let skip_keyword = is_clarification;
                            #[cfg(not(feature = "inference"))]
                            let skip_keyword = false;

                            if let Some(to_route) = to_route_opt {
                            let (response, intent) = if skip_keyword {
                                (String::new(), None)
                            } else {
                                response_for_input(&msg)
                            };
                            self.pending_intent = intent;
                            if !response.is_empty() {
                                self.messages.push(crate::Message {
                                    role: "assistant".into(),
                                    content: response.clone(),
                                });
                                self.persist("assistant", &response);
                            } else if let Some(router_path) = crate::f78(crate::ModelRole::Router) {
                                #[cfg(feature = "inference")]
                                {
                                    let rx = crate::f79(&router_path, &to_route);
                                    self.messages.push(crate::Message {
                                        role: "assistant".into(),
                                        content: "Classifying…".into(),
                                    });
                                    self.router_receiver = Some(rx);
                                    self.router_pending_user_msg = Some(to_route);
                                }
                            } else if let Some(model_path) = crate::inference_model_path() {
                                #[cfg(feature = "inference")]
                                {
                                    let system = self.build_system_prompt();
                                    let hist: Vec<(String, String)> = Vec::new();
                                    let rx = crate::inference::f76(
                                        &model_path,
                                        &system,
                                        &hist,
                                        &msg,
                                    );
                                    self.messages.push(crate::Message {
                                        role: "assistant".into(),
                                        content: String::new(),
                                    });
                                    self.llm_receiver = Some(rx);
                                }
                            } else {
                                #[cfg(feature = "inference")]
                                let fallback = "No local model. Run: kova model install";
                                #[cfg(not(feature = "inference"))]
                                let fallback = "Build with --features inference for local LLM.";
                                self.messages.push(crate::Message {
                                    role: "assistant".into(),
                                    content: fallback.into(),
                                });
                                self.persist("assistant", fallback);
                            }
                            }
                            // else: cancelled - already pushed message above
                        }
                        }
                    }
                }
            });
            });
            ui.add_space(layout::GAP);
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (i, m) in self.messages.iter().enumerate() {
                    let (prefix, color) = if m.role == "user" {
                        ("You", colors::PRIMARY)
                    } else {
                        ("Assistant", colors::SECONDARY)
                    };
                    theme::message_frame().show(ui, |ui| {
                    ui.label(egui::RichText::new(prefix).color(color).strong());
                    let content = {
                        #[cfg(feature = "inference")]
                        let (llm_waiting, router_waiting) =
                            (self.llm_receiver.is_some(), self.router_receiver.is_some());
                        #[cfg(not(feature = "inference"))]
                        let (llm_waiting, router_waiting) = (false, false);
                        if m.role == "assistant" && i == self.messages.len() - 1 {
                            #[cfg(feature = "inference")]
                            let pipeline_waiting = self.pipeline_receiver.is_some();
                            #[cfg(not(feature = "inference"))]
                            let pipeline_waiting = false;
                            if router_waiting {
                                "Classifying…"
                            } else if pipeline_waiting {
                                "Generating…"
                            } else if m.content.is_empty() && llm_waiting {
                                "Thinking…"
                            } else {
                                &m.content
                            }
                        } else {
                            &m.content
                        }
                    };
                    ui.add_space(layout::PADDING_SM);
                    ui.label(egui::RichText::new(content).color(colors::TEXT).monospace());

                    #[cfg(feature = "inference")]
                    if m.role == "assistant" {
                        if let Some(code) = crate::pipeline::extract_rust_block(&m.content) {
                            ui.add_space(layout::PADDING_SM);
                            let user_msg = self
                                .messages
                                .iter()
                                .enumerate()
                                .rev()
                                .find(|(j, x)| *j < i && x.role == "user")
                                .map(|(_, x)| x.content.as_str())
                                .unwrap_or("");
                            let hint = crate::context_loader::f83(user_msg);
                            let project = &self.current_project;
                            let target = crate::output::f85(project, hint.as_deref());
                            let current = std::fs::read_to_string(&target).unwrap_or_default();

                            ui.horizontal(|ui| {
                                if ui.button("Copy").clicked() {
                                    ui.ctx().output_mut(|o| o.copied_text = code.clone());
                                }
                                if ui.button("Apply").clicked() {
                                    let parent = target.parent().unwrap_or(project);
                                    if let Err(e) = std::fs::create_dir_all(parent) {
                                        self.last_applied =
                                            Some(format!("Apply failed: {}", e));
                                    } else if let Err(e) = std::fs::write(&target, &code) {
                                        self.last_applied =
                                            Some(format!("Apply failed: {}", e));
                                    } else {
                                        self.last_applied =
                                            Some(format!("Applied to {}", target.display()));
                                    }
                                }
                                if let Some(ref msg) = self.last_applied {
                                    let c = if msg.starts_with("Applied") {
                                        colors::TERTIARY
                                    } else {
                                        colors::SECONDARY
                                    };
                                    ui.label(egui::RichText::new(msg).color(c));
                                }
                            });

                            egui::CollapsingHeader::new(format!("Show diff ({})", i))
                                .default_open(false)
                                .show(ui, |ui| {
                                    let diff = crate::output::f84(&current, &code);
                                    egui::ScrollArea::vertical()
                                        .max_height(200.0)
                                        .show(ui, |ui| {
                                            ui.label(egui::RichText::new(diff).color(colors::TEXT).monospace());
                                        });
                                });
                        }
                    }
                    });
                    ui.add_space(layout::GAP);
                }
            });
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        if let Some(rec) = self.demo_recording.take() {
            rec.save();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Prove GUI build_system_prompt includes baked Cursor rules when enabled.
    /// Uses temp dir for KOVA_PROJECTS_ROOT to avoid slow discover_projects on home.
    #[test]
    fn gui_build_system_prompt_includes_baked() {
        let tmp = tempfile::TempDir::new().unwrap();
        std::env::set_var("KOVA_PROJECTS_ROOT", tmp.path());
        let out = build_system_prompt_impl("System", "Persona", tmp.path());
        std::env::remove_var("KOVA_PROJECTS_ROOT");
        if crate::cursor_prompts_enabled() {
            assert!(
                out.contains("--- Cursor rules ---"),
                "GUI prompt must include Cursor rules when enabled"
            );
            assert!(
                out.contains("Blocking Only") || out.contains("f81"),
                "GUI prompt must include baked content"
            );
        }
    }
}
