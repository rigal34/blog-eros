// Les traces d'un chien invisible : de temps en temps, quelques empreintes
// de pattes apparaissent l'une après l'autre sur l'écran, comme si un chien
// passait, puis elles s'effacent doucement.

use gloo_timers::callback::Timeout;
use yew::prelude::*;

// Délais en millisecondes
const PREMIERE_MARCHE: u32 = 4_000;
const ATTENTE_MIN: f64 = 9_000.0;
const ATTENTE_MAX: f64 = 18_000.0;
const INTERVALLE_PAS: u32 = 380;
// Doit être égal à la durée de l'animation .empreinte dans style.css
const DUREE_EMPREINTE: u32 = 12_000;

// Distances en pixels
const AVANCE_PAR_PAS: f64 = 26.0;
const ECART_GAUCHE_DROITE: f64 = 13.0;

struct Empreinte {
    id: u32,
    x: f64,
    y: f64,
    angle: f64,
}

struct Marche {
    x: f64,
    y: f64,
    angle: f64,
    pas_restants: u32,
    gauche: bool,
}

pub enum Msg {
    Depart,
    Pas,
    Effacer(u32),
}

pub struct Empreintes {
    empreintes: Vec<Empreinte>,
    marche: Option<Marche>,
    prochain_id: u32,
}

fn hasard_entre(min: f64, max: f64) -> f64 {
    min + js_sys::Math::random() * (max - min)
}

fn taille_ecran() -> (f64, f64) {
    let fenetre = web_sys::window();
    let largeur = fenetre
        .as_ref()
        .and_then(|f| f.inner_width().ok())
        .and_then(|v| v.as_f64())
        .unwrap_or(1280.0);
    let hauteur = fenetre
        .as_ref()
        .and_then(|f| f.inner_height().ok())
        .and_then(|v| v.as_f64())
        .unwrap_or(800.0);
    (largeur, hauteur)
}

// Les personnes qui ont demandé moins d'animations ne voient pas les traces.
fn animations_reduites() -> bool {
    web_sys::window()
        .and_then(|f| f.match_media("(prefers-reduced-motion: reduce)").ok().flatten())
        .map(|m| m.matches())
        .unwrap_or(false)
}

fn programmer(ctx: &Context<Empreintes>, delai: u32, msg: Msg) {
    let link = ctx.link().clone();
    Timeout::new(delai, move || link.send_message(msg)).forget();
}

fn patte() -> Html {
    html! {
        <svg viewBox="0 0 24 24" width="24" height="24" focusable="false">
            <ellipse cx="12" cy="16" rx="5" ry="4.5" />
            <circle cx="5.8" cy="9.6" r="2.2" />
            <circle cx="9.8" cy="6.4" r="2.3" />
            <circle cx="14.2" cy="6.4" r="2.3" />
            <circle cx="18.2" cy="9.6" r="2.2" />
        </svg>
    }
}

impl Component for Empreintes {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        if !animations_reduites() {
            programmer(ctx, PREMIERE_MARCHE, Msg::Depart);
        }
        Self {
            empreintes: Vec::new(),
            marche: None,
            prochain_id: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Depart => {
                let (largeur, hauteur) = taille_ecran();
                self.marche = Some(Marche {
                    x: hasard_entre(largeur * 0.1, largeur * 0.9),
                    y: hasard_entre(hauteur * 0.15, hauteur * 0.85),
                    angle: hasard_entre(0.0, std::f64::consts::TAU),
                    pas_restants: hasard_entre(10.0, 16.0) as u32,
                    gauche: true,
                });
                ctx.link().send_message(Msg::Pas);
                false
            }
            Msg::Pas => {
                let Some(marche) = self.marche.as_mut() else {
                    return false;
                };

                // La patte se pose un peu à gauche ou à droite du chemin
                let cote = if marche.gauche { -1.0 } else { 1.0 };
                let id = self.prochain_id;
                self.prochain_id += 1;
                self.empreintes.push(Empreinte {
                    id,
                    x: marche.x - cote * ECART_GAUCHE_DROITE * marche.angle.sin(),
                    y: marche.y + cote * ECART_GAUCHE_DROITE * marche.angle.cos(),
                    angle: marche.angle,
                });
                programmer(ctx, DUREE_EMPREINTE, Msg::Effacer(id));

                // Le chien avance, en changeant un peu de direction
                marche.x += AVANCE_PAR_PAS * marche.angle.cos();
                marche.y += AVANCE_PAR_PAS * marche.angle.sin();
                marche.angle += hasard_entre(-0.12, 0.12);
                marche.gauche = !marche.gauche;
                marche.pas_restants = marche.pas_restants.saturating_sub(1);

                let (largeur, hauteur) = taille_ecran();
                let sorti = marche.x < -30.0
                    || marche.y < -30.0
                    || marche.x > largeur + 30.0
                    || marche.y > hauteur + 30.0;

                if marche.pas_restants == 0 || sorti {
                    self.marche = None;
                    let attente = hasard_entre(ATTENTE_MIN, ATTENTE_MAX) as u32;
                    programmer(ctx, attente, Msg::Depart);
                } else {
                    programmer(ctx, INTERVALLE_PAS, Msg::Pas);
                }
                true
            }
            Msg::Effacer(id) => {
                self.empreintes.retain(|e| e.id != id);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="empreintes" aria-hidden="true">
                { for self.empreintes.iter().map(|e| {
                    // Le dessin pointe vers le haut : on le tourne dans le sens de la marche
                    let style = format!(
                        "left:{:.0}px;top:{:.0}px;transform:translate(-50%,-50%) rotate({:.0}deg);",
                        e.x,
                        e.y,
                        e.angle.to_degrees() + 90.0
                    );
                    html! { <span key={e.id} class="empreinte" {style}>{ patte() }</span> }
                }) }
            </div>
        }
    }
}
